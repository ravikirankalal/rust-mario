//! Screenshot Generator
//! 
//! This binary generates a screenshot of the Rust Mario game
//! and saves it to assets/screenshot.png for documentation purposes.

use macroquad::prelude::*;
use rust_mario::simple_level::SimpleLevel;

/// Window configuration for screenshot generation
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Mario - Screenshot Generator".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("Generating screenshot...");
    
    // Create a new game level
    let game = SimpleLevel::new();
    
    // Render a few frames to ensure everything is initialized properly
    for _ in 0..5 {
        clear_background(SKYBLUE);
        game.draw();
        next_frame().await;
    }
    
    // Take the screenshot
    let screenshot_path = "assets/screenshot.png";
    match game.take_screenshot(screenshot_path) {
        Ok(()) => {
            println!("Screenshot successfully saved to {}", screenshot_path);
        },
        Err(e) => {
            eprintln!("Failed to take screenshot: {}", e);
            std::process::exit(1);
        }
    }
    
    // Wait a moment to ensure file is written
    for _ in 0..10 {
        next_frame().await;
    }
    
    println!("Screenshot generation complete!");
}