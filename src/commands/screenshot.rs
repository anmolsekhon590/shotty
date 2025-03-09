use crate::commands::config::ShottyConfig;
use chrono::Local;
use std::fs;
use std::process::Command;

pub fn take_screenshot(fullscreen: bool, output: Option<String>, config: &ShottyConfig) -> String {
    let home_dir = std::env::var("HOME").expect("Could not get HOME directory");
    let screenshot_dir = config.screenshot_dir.replace("~", &home_dir);
    fs::create_dir_all(&screenshot_dir).expect("Failed to create screenshot directory");

    let timestamp = Local::now().format(&config.timestamp_format).to_string();
    let filename = format!("{}/screenshot-{}.png", screenshot_dir, timestamp);

    let selection = if fullscreen {
        String::from("")
    } else if let Some(ref monitor_name) = output {
        monitor_name.clone()
    } else {
        let output = Command::new("slurp")
            .output()
            .expect("Failed to execute slurp")
            .stdout;
        let region = String::from_utf8_lossy(&output).to_string();
        if region.trim().is_empty() {
            eprintln!("No selection made.");
            return "".to_string();
        }
        region
    };

    let mut grim_args = vec![];

    if let Some(output_name) = output.as_ref() {
        grim_args.extend(["-o", output_name]);
    } else if !fullscreen {
        grim_args.extend(["-g", selection.trim()]);
    }

    let grim_status = Command::new("grim")
        .args(&grim_args)
        .arg(&filename)
        .status()
        .expect("Failed to execute grim");

    if !grim_status.success() {
        eprintln!("Failed to capture screenshot.");
        return "".to_string();
    }

    filename
}
