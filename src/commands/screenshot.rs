use crate::commands::config::ShottyConfig;
use chrono::Local;
use std::fs;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const LOCK_FILE_PATH: &str = "/tmp/shotty-selection-lock";

/// Check if a selection is already in progress by reading the lock file
/// and verifying the stored PID is still running
fn is_selection_in_progress() -> bool {
    if let Ok(content) = fs::read_to_string(LOCK_FILE_PATH) {
        if let Ok(pid) = content.trim().parse::<u32>() {
            // Check if process with this PID is still running using kill signal 0
            // This returns 0 if process exists, ESRCH if it doesn't
            unsafe {
                return libc::kill(pid as libc::pid_t, 0) == 0;
            }
        }
    }
    false
}

/// Acquire the selection lock, cleaning stale locks if needed
fn acquire_selection_lock() -> Result<(), String> {
    // Clean up stale lock if exists (either no file or stale PID)
    let _ = fs::remove_file(LOCK_FILE_PATH);
    
    // Write current PID to lock file
    fs::write(LOCK_FILE_PATH, std::process::id().to_string())
        .map_err(|e| format!("Failed to create lock file: {}", e))
}

/// Release the selection lock
fn release_selection_lock() {
    let _ = fs::remove_file(LOCK_FILE_PATH);
}

pub fn take_screenshot(fullscreen: bool, output: Option<String>, config: &ShottyConfig, freeze: bool) -> String {
    let home_dir = std::env::var("HOME").expect("Could not get HOME directory");
    let screenshot_dir = config.screenshot_dir.replace("~", &home_dir);
    fs::create_dir_all(&screenshot_dir).expect("Failed to create screenshot directory");

    // Check if selection is already in progress (only for region screenshots)
    let needs_selection = !fullscreen && output.is_none();
    if needs_selection {
        if is_selection_in_progress() {
            eprintln!("Screenshot selection is already active.");
            eprintln!("If this is incorrect, run: rm {}", LOCK_FILE_PATH);
            return String::new();
        }
        if let Err(e) = acquire_selection_lock() {
            eprintln!("{}", e);
            return String::new();
        }
    }

    let timestamp = Local::now().format(&config.timestamp_format).to_string();
    let filename = format!("{}/screenshot-{}.png", screenshot_dir, timestamp);

    // Helper to cleanup and return early
    macro_rules! cleanup_and_return {
        ($msg:expr) => {{
            if needs_selection {
                release_selection_lock();
            }
            $msg.to_string()
        }};
    }

    // Start screen freeze if requested and taking a region screenshot
    let freeze_process = if freeze && !fullscreen && output.is_none() {
        match Command::new("hyprpicker")
            .arg("-r")  // render-inactive flag
            .arg("-z")  // no-zoom to reduce visual interference
            .stderr(Stdio::null())  // Suppress error messages like "PBUFFER null"
            .stdout(Stdio::null())  // Suppress output
            .spawn()
        {
            Ok(child) => {
                // Give hyprpicker a moment to start rendering
                thread::sleep(Duration::from_millis(200));
                Some(child)
            }
            Err(_) => {
                eprintln!("Warning: hyprpicker not found, screen freeze unavailable");
                None
            }
        }
    } else {
        None
    };

    let selection = if fullscreen {
        String::from("")
    } else if let Some(ref monitor_name) = output {
        monitor_name.clone()
    } else {
        let output = match Command::new("slurp").output() {
            Ok(output) => output.stdout,
            Err(e) => {
                // Kill freeze process if slurp fails
                if let Some(mut child) = freeze_process {
                    let _ = child.kill();
                }
                eprintln!("Failed to execute slurp: {}", e);
                eprintln!("Make sure slurp is installed and in your PATH.");
                return cleanup_and_return!("");
            }
        };
        let region = String::from_utf8_lossy(&output).to_string();
        if region.trim().is_empty() {
            // Kill freeze process if selection was cancelled
            if let Some(mut child) = freeze_process {
                let _ = child.kill();
            }
            eprintln!("No selection made.");
            return cleanup_and_return!("");
        }
        region
    };

    let mut grim_args = vec![];

    if let Some(output_name) = output.as_ref() {
        grim_args.extend(["-o", output_name]);
    } else if !fullscreen {
        grim_args.extend(["-g", selection.trim()]);
    }

    let grim_status = match Command::new("grim")
        .args(&grim_args)
        .arg(&filename)
        .status()
    {
        Ok(status) => status,
        Err(e) => {
            // End screen freeze
            if let Some(mut child) = freeze_process {
                let _ = child.kill();
            }
            eprintln!("Failed to execute grim: {}", e);
            eprintln!("Make sure grim is installed and in your PATH.");
            return cleanup_and_return!("");
        }
    };

    // End screen freeze
    if let Some(mut child) = freeze_process {
        let _ = child.kill();
    }

    if !grim_status.success() {
        eprintln!("Failed to capture screenshot.");
        return cleanup_and_return!("");
    }

    // Release lock before returning successful result
    if needs_selection {
        release_selection_lock();
    }

    filename
}
