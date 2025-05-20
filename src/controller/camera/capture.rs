use opencv::{
    core,
    highgui,
    imgproc,
    videoio,
    prelude::*,
};
use anyhow::{Context, Result};
use crate::controller::camera::logging;
use crate::controller::camera::storage;

/// Motion detection and video capture system
pub struct MotionDetector {
    cap: videoio::VideoCapture,    // Video capture device
    background: Option<core::Mat>, // Background model for motion detection
    threshold: f64,                // Threshold for motion detection
    min_contour_area: f64,         // Minimum contour area to consider as motion
    storage: storage::VideoStorage,// Video storage handler
    led_pattern: Option<Vec<bool>> // Expected LED pattern (for tamper detection)
}

impl MotionDetector {
    /// Creates a new MotionDetector instance
    /// 
    /// # Arguments
    /// * `camera_index` - Index of the camera device
    /// * `threshold` - Sensitivity threshold for motion detection
    /// * `min_contour_area` - Minimum area (in pixels) to consider as motion
    /// * `storage_dir` - Directory to store captured footage
    /// 
    /// # Returns
    /// * `Result<Self>` - New MotionDetector instance or error
    pub fn new(
        camera_index: i32,
        threshold: f64,
        min_contour_area: f64,
        storage_dir: &str,
    ) -> Result<Self> {
        // Initialize video capture device
        let cap = videoio::VideoCapture::new(camera_index, videoio::CAP_ANY)
            .context("Failed to open camera")?;
        
        if !cap.is_opened()? {
            return Err(anyhow::anyhow!("Could not open camera at index {}", camera_index));
        }

        // Initialize video storage
        let storage = storage::VideoStorage::new(storage_dir)
            .context("Failed to initialize video storage")?;

        Ok(Self {
            cap,
            background: None,
            threshold,
            min_contour_area,
            storage,
            led_pattern: None,
        })
    }

    /// Captures a single frame from the camera
    /// 
    /// # Returns
    /// * `Result<Option<Mat>>` - Captured frame or None if capture failed
    pub fn capture_frame(&mut self) -> Result<Option<core::Mat>> {
        let mut frame = core::Mat::default();
        self.cap.read(&mut frame)?;
        
        // Check if frame is empty (capture failed)
        if frame.empty()? {
            return Ok(None);
        }

        Ok(Some(frame))
    }

    /// Processes a frame to detect motion
    /// 
    /// # Arguments
    /// * `frame` - The frame to analyze
    /// 
    /// # Returns
    /// * `Result<bool>` - True if motion was detected, False otherwise
    pub fn detect_motion(&mut self, frame: &core::Mat) -> Result<bool> {
        // Convert to grayscale for motion analysis
        let mut gray = core::Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        
        // Apply Gaussian blur to reduce noise
        let mut blurred = core::Mat::default();
        imgproc::gaussian_blur(
            &gray, 
            &mut blurred, 
            core::Size::new(21, 21), // Kernel size
            0.0,                     // Sigma X
            0.0,                     // Sigma Y
            core::BORDER_DEFAULT      // Border type
        )?;

        // Initialize background model if this is the first frame
        if self.background.is_none() {
            self.background = Some(blurred.clone());
            return Ok(false);
        }

        // Compute absolute difference between current frame and background
        let mut diff = core::Mat::default();
        core::absdiff(&self.background.as_ref().unwrap(), &blurred, &mut diff)?;
        
        // Apply threshold to create binary image
        let mut thresh = core::Mat::default();
        imgproc::threshold(
            &diff, 
            &mut thresh, 
            self.threshold,       // Threshold value
            255.0,               // Maximum value
            imgproc::THRESH_BINARY // Threshold type
        )?;
        
        // Find contours in the thresholded image
        let mut contours = opencv::types::VectorOfMat::new();
        imgproc::find_contours(
            &thresh,
            &mut contours,
            imgproc::RETR_EXTERNAL,      // Retrieve only external contours
            imgproc::CHAIN_APPROX_SIMPLE,// Contour approximation method
            core::Point::default(),      // Offset
        )?;

        // Check each contour for significant motion
        for i in 0..contours.len() {
            let contour = contours.get(i)?;
            let area = imgproc::contour_area(&contour, false)?;
            
            if area > self.min_contour_area {
                // Update background model
                self.background = Some(blurred.clone());
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Monitors the video feed continuously for motion events
    pub fn monitor(&mut self) -> Result<()> {
        let mut event_frames = Vec::new();
        let mut in_event = false;
        
        loop {
            if let Some(frame) = self.capture_frame()? {
                // Detect motion in the frame
                let motion_detected = self.detect_motion(&frame)?;
                
                if motion_detected {
                    if !in_event {
                        // Start of new motion event
                        logging::log_event("Motion detected - starting capture")?;
                        in_event = true;
                    }
                    
                    // Save the frame
                    let saved_path = self.storage.save_frame(&frame, "motion")?;
                    event_frames.push(frame.clone());
                    
                    // Here we would add LED pattern verification later
                    // let verified = self.verify_led_pattern(&frame)?;
                    // logging::log_verified_event("Motion captured", verified);
                } else if in_event {
                    // End of motion event
                    logging::log_event("Motion event ended")?;
                    
                    // Save the collected frames as a video
                    if !event_frames.is_empty() {
                        self.storage.save_video(&event_frames, "motion_event", 15.0)?;
                        event_frames.clear();
                    }
                    
                    in_event = false;
                }
            }
        }
    }

    /// Placeholder for future LED pattern verification
    fn verify_led_pattern(&self, frame: &core::Mat) -> Result<bool> {
        // TODO: Implement LED pattern detection and verification
        Ok(true)
    }
}