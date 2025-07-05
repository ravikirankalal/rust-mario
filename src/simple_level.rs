//! Simple Mario Game Level
//! 
//! This module implements a minimal Mario-like platformer game with:
//! - A controllable player character (red square representing Mario)
//! - Static platforms to jump on
//! - A goal zone (green flag) to reach for winning
//! - Basic physics (gravity, jumping, collision detection)
//! 
//! Controls:
//! - Arrow keys or WASD for movement
//! - Space or Up arrow for jumping
//! - ESC to quit
//! 
//! The game is designed to be easily extendable for future features like
//! enemies, coins, power-ups, multiple levels, etc.

use macroquad::prelude::*;

/// Game constants for easy tuning
const GRAVITY: f32 = 800.0;           // Pixels per second squared
const JUMP_STRENGTH: f32 = 300.0;     // Initial jump velocity
const PLAYER_SPEED: f32 = 200.0;      // Horizontal movement speed
const PLAYER_SIZE: f32 = 20.0;        // Player width and height
const PLATFORM_HEIGHT: f32 = 20.0;    // Platform thickness
const GOAL_SIZE: f32 = 30.0;          // Goal flag size

/// Represents a rectangular platform that the player can stand on
#[derive(Debug, Clone)]
pub struct Platform {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Platform {
    /// Create a new platform
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if a point is inside this platform
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width &&
        y >= self.y && y <= self.y + self.height
    }

    /// Check if this platform intersects with a rectangle
    pub fn intersects(&self, x: f32, y: f32, width: f32, height: f32) -> bool {
        x < self.x + self.width &&
        x + width > self.x &&
        y < self.y + self.height &&
        y + height > self.y
    }

    /// Draw the platform
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, BROWN);
        // Add a slight border for visual appeal
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, DARKBROWN);
    }
}

/// Animation states for the player
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
    Jumping,
}

/// Represents the player character (Mario)
#[derive(Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub on_ground: bool,
    pub width: f32,
    pub height: f32,
    pub facing_right: bool,
    pub animation_state: AnimationState,
    pub animation_timer: f32,
}

impl Player {
    /// Create a new player at the specified position
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            velocity_x: 0.0,
            velocity_y: 0.0,
            on_ground: false,
            width: PLAYER_SIZE,
            height: PLAYER_SIZE,
            facing_right: true,
            animation_state: AnimationState::Idle,
            animation_timer: 0.0,
        }
    }

    /// Update player physics and handle input
    pub fn update(&mut self, platforms: &[Platform], delta_time: f32) {
        // Handle input
        self.handle_input();

        // Apply gravity
        if !self.on_ground {
            self.velocity_y += GRAVITY * delta_time;
        }

        // Update position
        let new_x = self.x + self.velocity_x * delta_time;
        let new_y = self.y + self.velocity_y * delta_time;

        // Check horizontal collisions
        let mut can_move_x = true;
        for platform in platforms {
            if platform.intersects(new_x, self.y, self.width, self.height) {
                can_move_x = false;
                break;
            }
        }

        if can_move_x {
            self.x = new_x;
        } else {
            self.velocity_x = 0.0;
        }

        // Check vertical collisions
        let mut can_move_y = true;
        self.on_ground = false;

        for platform in platforms {
            if platform.intersects(self.x, new_y, self.width, self.height) {
                // Landing on top of platform
                if self.velocity_y > 0.0 && self.y <= platform.y {
                    self.y = platform.y - self.height;
                    self.velocity_y = 0.0;
                    self.on_ground = true;
                    can_move_y = false;
                }
                // Hitting platform from below
                else if self.velocity_y < 0.0 && self.y >= platform.y + platform.height {
                    self.y = platform.y + platform.height;
                    self.velocity_y = 0.0;
                    can_move_y = false;
                }
            }
        }

        if can_move_y && !self.on_ground {
            self.y = new_y;
        }

        // Keep player on screen (basic boundary checking)
        if self.x < 0.0 {
            self.x = 0.0;
            self.velocity_x = 0.0;
        }
        if self.x + self.width > screen_width() {
            self.x = screen_width() - self.width;
            self.velocity_x = 0.0;
        }

        // Reset if player falls off screen
        if self.y > screen_height() {
            self.x = 50.0;
            self.y = 50.0;
            self.velocity_x = 0.0;
            self.velocity_y = 0.0;
        }

        // Apply friction when on ground
        if self.on_ground {
            self.velocity_x *= 0.8;
        }

        // Update animation state and timer
        self.animation_timer += delta_time;
        self.update_animation_state();
    }

    /// Update animation state based on player movement
    fn update_animation_state(&mut self) {
        if !self.on_ground {
            self.animation_state = AnimationState::Jumping;
        } else if self.velocity_x.abs() > 10.0 {
            self.animation_state = AnimationState::Walking;
        } else {
            self.animation_state = AnimationState::Idle;
        }
    }

    /// Handle player input for movement and jumping
    fn handle_input(&mut self) {
        // Horizontal movement
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.velocity_x = -PLAYER_SPEED;
            self.facing_right = false;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.velocity_x = PLAYER_SPEED;
            self.facing_right = true;
        } else {
            self.velocity_x = 0.0;
        }

        // Jumping
        if (is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W)) && self.on_ground {
            self.velocity_y = -JUMP_STRENGTH;
            self.on_ground = false;
        }
    }

    /// Draw the player with enhanced Mario-like graphics
    pub fn draw(&self) {
        let x = self.x;
        let y = self.y;
        let w = self.width;
        let h = self.height;

        // Animation-based slight bobbing for walking
        let walking_offset = if self.animation_state == AnimationState::Walking {
            (self.animation_timer * 8.0).sin() * 1.0
        } else {
            0.0
        };
        let draw_y = y + walking_offset;

        // Mario's body (overalls)
        draw_rectangle(x + 2.0, draw_y + 8.0, w - 4.0, h - 8.0, BLUE);
        draw_rectangle_lines(x + 2.0, draw_y + 8.0, w - 4.0, h - 8.0, 1.0, DARKBLUE);

        // Mario's shirt (red)
        draw_rectangle(x + 4.0, draw_y + 10.0, w - 8.0, 6.0, RED);

        // Mario's head (skin color - light brown)
        let head_color = Color::new(0.96, 0.85, 0.73, 1.0); // Peach/skin color
        draw_circle(x + w / 2.0, draw_y + 6.0, 6.0, head_color);

        // Mario's hat (red)
        draw_rectangle(x + 3.0, draw_y + 1.0, w - 6.0, 6.0, RED);
        draw_rectangle_lines(x + 3.0, draw_y + 1.0, w - 6.0, 6.0, 1.0, MAROON);

        // Hat emblem (M)
        draw_circle(x + w / 2.0, draw_y + 3.0, 2.5, WHITE);
        draw_text("M", x + w / 2.0 - 2.0, draw_y + 5.5, 8.0, RED);

        // Eyes (direction-aware)
        let eye_offset = if self.facing_right { 1.0 } else { -1.0 };
        draw_circle(x + w / 2.0 - 2.0 + eye_offset, draw_y + 6.0, 1.0, BLACK);
        draw_circle(x + w / 2.0 + 2.0 + eye_offset, draw_y + 6.0, 1.0, BLACK);

        // Mustache
        draw_rectangle(x + w / 2.0 - 3.0, draw_y + 8.0, 6.0, 2.0, Color::new(0.4, 0.2, 0.1, 1.0));

        // Arms based on animation
        let arm_swing = if self.animation_state == AnimationState::Walking {
            (self.animation_timer * 6.0).sin() * 2.0
        } else {
            0.0
        };

        // Left arm
        draw_rectangle(x - 1.0, draw_y + 10.0 + arm_swing, 4.0, 8.0, head_color);
        // Right arm  
        draw_rectangle(x + w - 3.0, draw_y + 10.0 - arm_swing, 4.0, 8.0, head_color);

        // Feet/shoes (brown)
        let foot_color = Color::new(0.4, 0.2, 0.1, 1.0);
        draw_rectangle(x + 1.0, draw_y + h - 2.0, 6.0, 3.0, foot_color);
        draw_rectangle(x + w - 7.0, draw_y + h - 2.0, 6.0, 3.0, foot_color);

        // Jumping pose adjustments
        if self.animation_state == AnimationState::Jumping {
            // Arms up when jumping
            draw_rectangle(x - 2.0, draw_y + 6.0, 4.0, 6.0, head_color);
            draw_rectangle(x + w - 2.0, draw_y + 6.0, 4.0, 6.0, head_color);
        }
    }

    /// Check if player intersects with a rectangle (for goal detection)
    pub fn intersects(&self, x: f32, y: f32, width: f32, height: f32) -> bool {
        self.x < x + width &&
        self.x + self.width > x &&
        self.y < y + height &&
        self.y + self.height > y
    }
}

/// Represents the goal that the player needs to reach
#[derive(Debug)]
pub struct Goal {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Goal {
    /// Create a new goal at the specified position
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            width: GOAL_SIZE,
            height: GOAL_SIZE * 2.0,
        }
    }

    /// Draw the goal as a flag
    pub fn draw(&self) {
        // Flag pole
        draw_rectangle(self.x + self.width * 0.8, self.y, 3.0, self.height, DARKGRAY);
        
        // Flag
        draw_rectangle(self.x, self.y, self.width * 0.8, self.height * 0.4, GREEN);
        draw_rectangle_lines(self.x, self.y, self.width * 0.8, self.height * 0.4, 2.0, DARKGREEN);
        
        // Flag text
        draw_text("GOAL", self.x + 2.0, self.y + 15.0, 12.0, WHITE);
    }
}

/// Represents a decorative tree in the environment
#[derive(Debug, Clone)]
pub struct Tree {
    pub x: f32,
    pub y: f32,
    pub height: f32,
}

impl Tree {
    /// Create a new tree at the specified position
    pub fn new(x: f32, y: f32, height: f32) -> Self {
        Self { x, y, height }
    }

    /// Draw the tree
    pub fn draw(&self) {
        let trunk_width = 8.0;
        let trunk_height = self.height * 0.4;
        let crown_radius = self.height * 0.3;

        // Tree trunk (brown)
        let trunk_color = Color::new(0.4, 0.2, 0.1, 1.0);
        draw_rectangle(
            self.x - trunk_width / 2.0,
            self.y - trunk_height,
            trunk_width,
            trunk_height,
            trunk_color,
        );

        // Tree crown (green circles for leaves)
        let crown_y = self.y - trunk_height - crown_radius;
        draw_circle(self.x, crown_y, crown_radius, GREEN);
        draw_circle(self.x - crown_radius * 0.3, crown_y + crown_radius * 0.2, crown_radius * 0.8, DARKGREEN);
        draw_circle(self.x + crown_radius * 0.3, crown_y + crown_radius * 0.2, crown_radius * 0.8, DARKGREEN);
    }
}

/// Enemy movement direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyDirection {
    Left,
    Right,
}

/// Represents a simple enemy (Goomba-like)
#[derive(Debug, Clone)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub direction: EnemyDirection,
    pub speed: f32,
    pub patrol_start: f32,
    pub patrol_end: f32,
}

impl Enemy {
    /// Create a new enemy
    pub fn new(x: f32, y: f32, patrol_start: f32, patrol_end: f32) -> Self {
        Self {
            x,
            y,
            width: 16.0,
            height: 16.0,
            direction: EnemyDirection::Right,
            speed: 30.0,
            patrol_start,
            patrol_end,
        }
    }

    /// Update enemy movement
    pub fn update(&mut self, delta_time: f32) {
        // Simple patrol AI
        match self.direction {
            EnemyDirection::Right => {
                self.x += self.speed * delta_time;
                if self.x >= self.patrol_end {
                    self.direction = EnemyDirection::Left;
                }
            }
            EnemyDirection::Left => {
                self.x -= self.speed * delta_time;
                if self.x <= self.patrol_start {
                    self.direction = EnemyDirection::Right;
                }
            }
        }
    }

    /// Draw the enemy (Goomba-like)
    pub fn draw(&self) {
        let x = self.x;
        let y = self.y;
        let w = self.width;
        let h = self.height;

        // Body (brown mushroom-like)
        let body_color = Color::new(0.5, 0.3, 0.1, 1.0);
        draw_rectangle(x + 2.0, y + h * 0.3, w - 4.0, h * 0.7, body_color);
        
        // Head (round, darker brown)
        let head_color = Color::new(0.4, 0.2, 0.05, 1.0);
        draw_circle(x + w / 2.0, y + h * 0.25, w * 0.4, head_color);

        // Eyes (angry looking)
        draw_circle(x + w * 0.35, y + h * 0.2, 2.0, WHITE);
        draw_circle(x + w * 0.65, y + h * 0.2, 2.0, WHITE);
        draw_circle(x + w * 0.35, y + h * 0.2, 1.0, BLACK);
        draw_circle(x + w * 0.65, y + h * 0.2, 1.0, BLACK);

        // Eyebrows (angry)
        draw_line(x + w * 0.3, y + h * 0.15, x + w * 0.4, y + h * 0.1, 2.0, BLACK);
        draw_line(x + w * 0.6, y + h * 0.1, x + w * 0.7, y + h * 0.15, 2.0, BLACK);

        // Feet
        draw_rectangle(x, y + h - 3.0, 5.0, 3.0, BLACK);
        draw_rectangle(x + w - 5.0, y + h - 3.0, 5.0, 3.0, BLACK);
    }

    /// Check if enemy intersects with a rectangle
    pub fn intersects(&self, x: f32, y: f32, width: f32, height: f32) -> bool {
        x < self.x + self.width &&
        x + width > self.x &&
        y < self.y + self.height &&
        y + height > self.y
    }
}

/// Main game state and logic
pub struct SimpleLevel {
    pub player: Player,
    platforms: Vec<Platform>,
    goal: Goal,
    trees: Vec<Tree>,
    enemies: Vec<Enemy>,
    pub game_won: bool,
    camera_x: f32,
}

impl SimpleLevel {
    /// Create a new game level
    pub fn new() -> Self {
        let mut platforms = Vec::new();
        
        // Create a simple level layout
        // Ground platforms
        platforms.push(Platform::new(0.0, 400.0, 200.0, PLATFORM_HEIGHT));
        platforms.push(Platform::new(250.0, 450.0, 150.0, PLATFORM_HEIGHT));
        platforms.push(Platform::new(450.0, 350.0, 100.0, PLATFORM_HEIGHT));
        platforms.push(Platform::new(600.0, 300.0, 120.0, PLATFORM_HEIGHT));
        platforms.push(Platform::new(750.0, 250.0, 100.0, PLATFORM_HEIGHT));
        
        // Some floating platforms
        platforms.push(Platform::new(200.0, 300.0, 80.0, PLATFORM_HEIGHT));
        platforms.push(Platform::new(350.0, 200.0, 80.0, PLATFORM_HEIGHT));
        platforms.push(Platform::new(500.0, 150.0, 80.0, PLATFORM_HEIGHT));
        
        // Final platform with goal
        platforms.push(Platform::new(850.0, 200.0, 100.0, PLATFORM_HEIGHT));

        // Add decorative trees
        let mut trees = Vec::new();
        trees.push(Tree::new(100.0, 400.0, 40.0));
        trees.push(Tree::new(300.0, 450.0, 35.0));
        trees.push(Tree::new(520.0, 350.0, 45.0));
        trees.push(Tree::new(700.0, 250.0, 38.0));
        trees.push(Tree::new(950.0, 200.0, 42.0));

        // Add enemies
        let mut enemies = Vec::new();
        enemies.push(Enemy::new(220.0, 400.0 - 16.0, 210.0, 380.0)); // Ground patrol
        enemies.push(Enemy::new(470.0, 350.0 - 16.0, 460.0, 540.0)); // Platform patrol
        enemies.push(Enemy::new(620.0, 300.0 - 16.0, 610.0, 710.0)); // Longer patrol

        Self {
            player: Player::new(50.0, 50.0),
            platforms,
            goal: Goal::new(870.0, 140.0),
            trees,
            enemies,
            game_won: false,
            camera_x: 0.0,
        }
    }

    /// Update the game state
    pub fn update(&mut self, delta_time: f32) {
        if !self.game_won {
            self.player.update(&self.platforms, delta_time);
            
            // Update enemies
            for enemy in &mut self.enemies {
                enemy.update(delta_time);
            }
            
            // Check enemy collisions (simple reset for now)
            for enemy in &self.enemies {
                if enemy.intersects(self.player.x, self.player.y, self.player.width, self.player.height) {
                    // Reset player position on enemy collision
                    self.player.x = 50.0;
                    self.player.y = 50.0;
                    self.player.velocity_x = 0.0;
                    self.player.velocity_y = 0.0;
                }
            }
            
            // Simple camera follow
            let target_camera_x = self.player.x - screen_width() / 2.0;
            self.camera_x = self.camera_x + (target_camera_x - self.camera_x) * 0.1;
            
            // Keep camera within bounds
            self.camera_x = self.camera_x.max(0.0);
            
            // Check if player reached the goal
            if self.player.intersects(self.goal.x, self.goal.y, self.goal.width, self.goal.height) {
                self.game_won = true;
            }
        }
    }

    /// Draw the game
    pub fn draw(&self) {
        clear_background(SKYBLUE);
        
        // Apply camera offset
        let camera_offset = -self.camera_x;
        
        // Draw trees (background elements)
        for tree in &self.trees {
            let tree_with_offset = Tree::new(
                tree.x + camera_offset,
                tree.y,
                tree.height,
            );
            tree_with_offset.draw();
        }
        
        // Draw platforms
        for platform in &self.platforms {
            let platform_with_offset = Platform::new(
                platform.x + camera_offset,
                platform.y,
                platform.width,
                platform.height,
            );
            platform_with_offset.draw();
        }
        
        // Draw enemies
        for enemy in &self.enemies {
            let mut enemy_with_offset = enemy.clone();
            enemy_with_offset.x += camera_offset;
            enemy_with_offset.draw();
        }
        
        // Draw goal
        let goal_copy = Goal::new(self.goal.x + camera_offset, self.goal.y);
        goal_copy.draw();
        
        // Draw player (on top of everything)
        let mut player_copy = Player::new(self.player.x + camera_offset, self.player.y);
        player_copy.facing_right = self.player.facing_right;
        player_copy.animation_state = self.player.animation_state;
        player_copy.animation_timer = self.player.animation_timer;
        player_copy.draw();
        
        // Draw UI
        self.draw_ui();
    }

    /// Draw the user interface
    fn draw_ui(&self) {
        // Instructions
        draw_text("Use Arrow Keys or WASD to move, Space/Up to jump", 10.0, 30.0, 20.0, WHITE);
        draw_text("Reach the green flag to win! ESC to quit", 10.0, 55.0, 20.0, WHITE);
        
        // Win message
        if self.game_won {
            let win_text = "Congratulations! You reached the goal!";
            let text_width = measure_text(win_text, None, 40, 1.0).width;
            let x = (screen_width() - text_width) / 2.0;
            let y = screen_height() / 2.0;
            
            // Background for text
            draw_rectangle(x - 10.0, y - 30.0, text_width + 20.0, 50.0, Color::new(0.0, 0.0, 0.0, 0.7));
            draw_text(win_text, x, y, 40.0, GOLD);
            draw_text("Press ESC to quit", x + 50.0, y + 30.0, 20.0, WHITE);
        }
    }

    /// Check if the game should quit
    pub fn should_quit(&self) -> bool {
        is_key_pressed(KeyCode::Escape)
    }

    /// Generate a screenshot of the current game state and save it to the specified path
    /// 
    /// # Arguments
    /// * `filepath` - The path where the screenshot should be saved
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Err if failed
    pub fn take_screenshot<P: AsRef<std::path::Path>>(&self, filepath: P) -> Result<(), Box<dyn std::error::Error>> {
        // First render the current game state to ensure the screen has the latest frame
        self.draw();
        
        // Capture and save the screenshot
        crate::screenshot::capture_screenshot(filepath)
    }
}

/// Main game loop for the simple level
pub async fn run_simple_level() {
    let mut game = SimpleLevel::new();
    
    loop {
        let delta_time = get_frame_time();
        
        // Update game state
        game.update(delta_time);
        
        // Draw everything
        game.draw();
        
        // Check for quit
        if game.should_quit() {
            break;
        }
        
        next_frame().await;
    }
}