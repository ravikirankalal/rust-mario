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
    }

    /// Handle player input for movement and jumping
    fn handle_input(&mut self) {
        // Horizontal movement
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.velocity_x = -PLAYER_SPEED;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.velocity_x = PLAYER_SPEED;
        } else {
            self.velocity_x = 0.0;
        }

        // Jumping
        if (is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W)) && self.on_ground {
            self.velocity_y = -JUMP_STRENGTH;
            self.on_ground = false;
        }
    }

    /// Draw the player
    pub fn draw(&self) {
        // Draw player as a red square (representing Mario)
        draw_rectangle(self.x, self.y, self.width, self.height, RED);
        // Add a border for visual clarity
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, MAROON);
        
        // Add simple "eyes" to show direction
        draw_circle(self.x + 5.0, self.y + 5.0, 2.0, WHITE);
        draw_circle(self.x + 15.0, self.y + 5.0, 2.0, WHITE);
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

/// Main game state and logic
pub struct SimpleLevel {
    player: Player,
    platforms: Vec<Platform>,
    goal: Goal,
    game_won: bool,
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

        Self {
            player: Player::new(50.0, 50.0),
            platforms,
            goal: Goal::new(870.0, 140.0),
            game_won: false,
            camera_x: 0.0,
        }
    }

    /// Update the game state
    pub fn update(&mut self, delta_time: f32) {
        if !self.game_won {
            self.player.update(&self.platforms, delta_time);
            
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
        
        // Draw goal
        let goal_copy = Goal::new(self.goal.x + camera_offset, self.goal.y);
        goal_copy.draw();
        
        // Draw player
        let player_copy = Player::new(self.player.x + camera_offset, self.player.y);
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