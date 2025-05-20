use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use anyhow::Result;

/// Logs security events with timestamps to a text file
/// 
/// # Arguments
/// * `event` - The security event description to log
/// 
/// # Returns
/// * `Result<()>` - Ok if successful, Err if file operations fail
/// 
/// # Example
/// ```
/// logging::log_event("Motion detected in zone 1")?;
/// ```
pub fn log_event(event: &str) -> Result<()> {
    // Get current timestamp in local timezone
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    
    // Format log entry with timestamp
    let log_entry = format!("[{}] {}\n", now, event);
    
    // Open log file in append mode, create if doesn't exist
    let mut file = OpenOptions::new()
        .create(true)        // Create file if it doesn't exist
        .append(true)       // Append to existing file
        .open("security_log.txt")?;  // File path
    
    // Write the log entry
    file.write_all(log_entry.as_bytes())?;
    
    Ok(())
}

/// Logs a security event with additional verification status
/// 
/// # Arguments
/// * `event` - The security event description
/// * `verified` - Whether LED pattern verification passed
/// 
/// # Returns
/// * `Result<()>` - Ok if successful, Err if file operations fail
pub fn log_verified_event(event: &str, verified: bool) -> Result<()> {
    let status = if verified { "VERIFIED" } else { "TAMPER DETECTED" };
    log_event(&format!("{} - {}", event, status))
}