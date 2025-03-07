use chrono::Local;
use shotty::load_config;
use std::env;
use std::fs;
use std::process::{Command, Stdio};

fn main() {
    let config = load_config();
    let home_dir = env::var("HOME").expect("Could not get HOME directory");
    let screenshot_dir = config.screenshot_dir.replace("~", &home_dir);
    fs::create_dir_all(&screenshot_dir).expect("Failed to create screenshot directory");

    let timestamp = Local::now().format(&config.timestamp_format).to_string();
    let filename = format!("{}/screenshot-{}.png", screenshot_dir, timestamp);

    // Get region selection from slurp
    let selection = Command::new("slurp")
        .output()
        .expect("Failed to execute slurp")
        .stdout;

    let selection = String::from_utf8_lossy(&selection);
    if selection.trim().is_empty() {
        eprintln!("No selection made.");
        return;
    }

    // Capture screenshot and pipe to wl-copy
    let mut grim_process = Command::new("grim")
        .args(["-g", selection.trim(), "-"]) // Output to stdout
        .stdout(Stdio::piped()) // Pipe output
        .spawn()
        .expect("Failed to execute grim");

    if let Some(output) = grim_process.stdout.take() {
        let wlcopy_status = Command::new("wl-copy")
            .stdin(output)
            .status()
            .expect("Failed to execute wl-copy");

        if !wlcopy_status.success() {
            eprintln!("Failed to copy screenshot to clipboard.");
        }
    }

    let grim_status = grim_process
        .wait()
        .expect("Failed to wait for grim process");
    if !grim_status.success() {
        eprintln!("Grim failed to capture screenshot.");
        return;
    }

    // Save screenshot to disk always
    let save_status = Command::new("grim")
        .args(["-g", selection.trim(), &filename])
        .status()
        .expect("Failed to execute grim for saving file");

    if !save_status.success() {
        eprintln!("Failed to save screenshot to file.");
    }
}
