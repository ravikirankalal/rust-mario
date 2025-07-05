//! Rust Mario Game
//! 
//! A simple Mario-like platformer game built with macroquad.
//! Run this to start the game and enjoy jumping around!

mod simple_level;

use macroquad::prelude::*;

/// Window configuration for the game
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Mario - Simple Level".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Run the simple Mario level
    simple_level::run_simple_level().await;
}
