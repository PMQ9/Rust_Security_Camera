mod controller;
mod tuning;

use anyhow::Result;
use controller::camera::capture::MotionDetector;

fn main() -> Result<()> {
    // Initialize motion detector with:
    // - Camera index 0 (usually default webcam)
    // - Threshold of 25.0 (sensitivity to changes)
    // - Minimum contour area of 1000.0 pixels (size of motion to detect)
    // - Storage directory "captured_footage"
    let mut detector = MotionDetector::new(0, 25.0, 1000.0, "captured_footage")?;
    
    println!("Starting surveillance system...");
    println!("Motion events will be logged to security_log.txt");
    println!("Captured footage will be saved to captured_footage/");
    
    // Start monitoring (this runs indefinitely)
    detector.monitor()?;

    Ok(())
}
