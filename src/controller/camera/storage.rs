use opencv::prelude::*;
use chrono::Local;
use std::path::Path;
use anyhow::{Result, Context};

/// Stores video frames to disk with timestamped filenames
pub struct VideoStorage {
    output_dir: String,
    frame_counter: u32,
}

impl VideoStorage {
    /// Creates a new VideoStorage instance
    /// 
    /// # Arguments
    /// * `output_dir` - Directory to store captured frames
    /// 
    /// # Returns
    /// * `Result<Self>` - New VideoStorage instance or error
    pub fn new(output_dir: &str) -> Result<Self> {
        // Create directory if it doesn't exist
        std::fs::create_dir_all(output_dir)
            .context(format!("Failed to create output directory: {}", output_dir))?;
        
        Ok(Self {
            output_dir: output_dir.to_string(),
            frame_counter: 0,
        })
    }

    /// Saves a video frame to disk
    /// 
    /// # Arguments
    /// * `frame` - The video frame to save
    /// * `event_type` - Description of why the frame was captured
    /// 
    /// # Returns
    /// * `Result<String>` - Path to saved file or error
    pub fn save_frame(&mut self, frame: &Mat, event_type: &str) -> Result<String> {
        // Generate filename with timestamp and event type
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!(
            "{}/frame_{}_{}_{}.jpg",
            self.output_dir,
            timestamp,
            event_type,
            self.frame_counter
        );
        
        // Sanitize filename (replace spaces with underscores)
        let filename = filename.replace(" ", "_");
        
        // Save the frame as JPEG
        opencv::imgcodecs::imwrite(
            &filename,
            frame,
            &opencv::core::Vector::new()
        ).context("Failed to save frame")?;
        
        self.frame_counter += 1;
        Ok(filename)
    }

    /// Saves a sequence of frames as a video file
    /// 
    /// # Arguments
    /// * `frames` - Vector of frames to save
    /// * `event_type` - Description of the event
    /// * `fps` - Frames per second for output video
    /// 
    /// # Returns
    /// * `Result<String>` - Path to saved video file
    pub fn save_video(&self, frames: &[Mat], event_type: &str, fps: f64) -> Result<String> {
        if frames.is_empty() {
            return Err(anyhow::anyhow!("No frames to save"));
        }

        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!(
            "{}/video_{}_{}.avi",
            self.output_dir,
            timestamp,
            event_type.replace(" ", "_")
        );

        // Get frame size from first frame
        let frame_size = frames[0].size()?;
        
        // Create video writer
        let mut writer = opencv::videoio::VideoWriter::new(
            &filename,
            opencv::videoio::VideoWriter::fourcc('M', 'J', 'P', 'G')?,
            fps,
            frame_size,
            true
        ).context("Failed to create video writer")?;

        // Write each frame
        for frame in frames {
            writer.write(frame)?;
        }

        Ok(filename)
    }
}