//! 10-second game recording test
//! 
//! This test creates a 10-second recording of the game showing
//! Mario moving around, jumping, and interacting with enemies and platforms.

use rust_mario::simple_level::SimpleLevel;
use rust_mario::screenshot::GameRecorder;
use macroquad::prelude::*;

/// Window configuration for recording
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Mario - 10 Second Recording Test".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

/// Simulated input for the recording - creates a scripted gameplay sequence
fn get_simulated_input(time: f32) -> (bool, bool, bool) {
    match time {
        // First 2 seconds: move right
        t if t < 2.0 => (false, true, false),
        // 2-3 seconds: jump while moving right
        t if t >= 2.0 && t < 3.0 => (false, true, true),
        // 3-4 seconds: continue right
        t if t >= 3.0 && t < 4.0 => (false, true, false),
        // 4-5 seconds: move left
        t if t >= 4.0 && t < 5.0 => (true, false, false),
        // 5-6 seconds: jump left
        t if t >= 5.0 && t < 6.0 => (true, false, true),
        // 6-7 seconds: move right again
        t if t >= 6.0 && t < 7.0 => (false, true, false),
        // 7-8 seconds: big jump
        t if t >= 7.0 && t < 8.0 => (false, true, true),
        // 8-10 seconds: final approach to goal
        _ => (false, true, false),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("Starting 10-second recording test...");
    
    // Create the game
    let mut game = SimpleLevel::new();
    
    // Create recorder (capture every 100ms = 10 FPS for reasonable file size)
    let mut recorder = GameRecorder::new(100);
    
    let start_time = get_time();
    let mut last_capture_time = start_time;
    let recording_duration = 10.0; // 10 seconds
    let capture_interval = 0.1; // Capture every 100ms
    
    println!("Recording gameplay for {} seconds...", recording_duration);
    
    while get_time() - start_time < recording_duration {
        let current_time = get_time();
        let elapsed = current_time - start_time;
        
        // Simulate input based on elapsed time
        let (should_move_left, should_move_right, should_jump) = get_simulated_input(elapsed as f32);
        
        // We can't directly inject input into macroquad, so we'll modify the player directly
        // This is a test-specific approach
        if should_move_left {
            game.player.velocity_x = -200.0;
            game.player.facing_right = false;
        } else if should_move_right {
            game.player.velocity_x = 200.0;
            game.player.facing_right = true;
        } else {
            game.player.velocity_x = 0.0;
        }
        
        if should_jump && game.player.on_ground {
            game.player.velocity_y = -300.0;
            game.player.on_ground = false;
        }
        
        // Update game state
        game.update(get_frame_time());
        
        // Draw the game
        game.draw();
        
        // Capture frame at regular intervals
        if current_time - last_capture_time >= capture_interval {
            match recorder.capture_frame() {
                Ok(()) => {},
                Err(e) => eprintln!("Failed to capture frame: {}", e),
            }
            last_capture_time = current_time;
        }
        
        next_frame().await;
    }
    
    println!("Recording complete! Captured {} frames", recorder.frame_count());
    
    // Ensure assets directory exists
    if let Err(e) = std::fs::create_dir_all("assets") {
        eprintln!("Failed to create assets directory: {}", e);
        return;
    }
    
    // Save the recording as a GIF
    let gif_path = "assets/10_second_recording.gif";
    match recorder.save_gif(gif_path) {
        Ok(()) => {
            println!("Recording saved successfully to {}", gif_path);
            println!("You can now view the 10-second gameplay recording!");
        },
        Err(e) => {
            eprintln!("Failed to save recording: {}", e);
        }
    }
    
    // Wait a moment before closing
    for _ in 0..30 {
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recording_functionality() {
        // Test that we can create a recorder and it starts with no frames
        let recorder = GameRecorder::new(100);
        assert_eq!(recorder.frame_count(), 0);
        
        // Verify assets directory can be created
        std::fs::create_dir_all("assets").expect("Failed to create assets directory");
        
        // Test that the game can be initialized
        let game = SimpleLevel::new();
        assert!(!game.game_won);
        
        println!("Recording test infrastructure verified");
        println!("Run 'cargo run --bin recording_test' to generate the actual 10-second recording");
    }
}