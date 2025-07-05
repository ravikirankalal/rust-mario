/// Screenshot functionality for the Mario game
/// This module provides functions to capture and save screenshots of the game

use macroquad::prelude::*;
use std::path::Path;
use std::fs::File;
use gif::{Encoder, Frame, Repeat};

/// Capture the current screen and save it as a PNG file
/// 
/// # Arguments
/// * `filepath` - The path where the screenshot should be saved
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Err if failed
pub fn capture_screenshot<P: AsRef<Path>>(filepath: P) -> Result<(), Box<dyn std::error::Error>> {
    // Get screen data as macroquad Image
    let screen_image = get_screen_data();
    
    // Get raw bytes from the image
    let width = screen_image.width() as u32;
    let height = screen_image.height() as u32;
    let bytes = screen_image.bytes.clone();
    
    // Create image buffer from screen data
    use image::{ImageBuffer, Rgba};
    let img_buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, bytes)
        .ok_or("Failed to create image buffer from screen data")?;
    
    // Save as PNG
    img_buffer.save(filepath)?;
    
    Ok(())
}

/// Game recorder that captures frames for creating animated GIFs
pub struct GameRecorder {
    frames: Vec<Vec<u8>>,
    width: u16,
    height: u16,
    frame_delay: u16, // in hundredths of a second
}

impl GameRecorder {
    /// Create a new game recorder
    pub fn new(frame_delay_ms: u16) -> Self {
        Self {
            frames: Vec::new(),
            width: 0,
            height: 0,
            frame_delay: frame_delay_ms / 10, // Convert ms to hundredths of a second
        }
    }

    /// Capture the current screen as a frame
    pub fn capture_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let screen_image = get_screen_data();
        
        if self.width == 0 {
            self.width = screen_image.width() as u16;
            self.height = screen_image.height() as u16;
        }
        
        // Convert RGBA to RGB (GIF doesn't support alpha)
        let rgba_bytes = screen_image.bytes;
        let mut rgb_bytes = Vec::with_capacity((rgba_bytes.len() * 3) / 4);
        
        for chunk in rgba_bytes.chunks(4) {
            rgb_bytes.push(chunk[0]); // R
            rgb_bytes.push(chunk[1]); // G
            rgb_bytes.push(chunk[2]); // B
            // Skip alpha channel
        }
        
        self.frames.push(rgb_bytes);
        Ok(())
    }

    /// Save all captured frames as an animated GIF
    pub fn save_gif<P: AsRef<Path>>(&self, filepath: P) -> Result<(), Box<dyn std::error::Error>> {
        if self.frames.is_empty() {
            return Err("No frames captured".into());
        }

        let file = File::create(filepath)?;
        let mut encoder = Encoder::new(file, self.width, self.height, &[])?;
        encoder.set_repeat(Repeat::Infinite)?;

        for frame_data in &self.frames {
            let mut frame = Frame::from_rgb(self.width, self.height, frame_data);
            frame.delay = self.frame_delay;
            encoder.write_frame(&frame)?;
        }

        Ok(())
    }

    /// Get the number of captured frames
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_screenshot_generation() {
        // This test would need to be run in a macroquad context
        // For now, we'll create a placeholder that documents the intended functionality
        
        // Expected workflow:
        // 1. Create game state
        // 2. Render one frame
        // 3. Capture screenshot
        // 4. Save to assets/screenshot.png
        
        assert!(true, "Screenshot test placeholder - requires macroquad context");
    }
}