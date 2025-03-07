use chrono::Local;
use clap::Parser;
use shotty::load_config;
use std::env;
use std::fs;
use std::process::{Command, Stdio};

/// Shotty: A simple Wayland screenshot tool
#[derive(Parser)]
#[clap(
    name = "Shotty",
    version = "0.1.0",
    about = "A simple Wayland screenshot tool"
)]
struct Args {
    /// Take a fullscreen screenshot
    #[clap(long)]
    fullscreen: bool,
}

fn main() {
    let args = Args::parse();
    let config = load_config();
    let home_dir = env::var("HOME").expect("Could not get HOME directory");
    let screenshot_dir = config.screenshot_dir.replace("~", &home_dir);
    fs::create_dir_all(&screenshot_dir).expect("Failed to create screenshot directory");

    let timestamp = Local::now().format(&config.timestamp_format).to_string();
    let filename = format!("{}/screenshot-{}.png", screenshot_dir, timestamp);

    let selection = if args.fullscreen {
        String::from("")
    } else {
        let output = Command::new("slurp")
            .output()
            .expect("Failed to execute slurp")
            .stdout;
        let region = String::from_utf8_lossy(&output).to_string();
        if region.trim().is_empty() {
            eprintln!("No selection made.");
            return;
        }
        region
    };

    let mut grim_args = vec![];
    if !args.fullscreen {
        grim_args.extend(["-g", selection.trim()]);
    }

    let mut grim_process = Command::new("grim")
        .args(&grim_args)
        .stdout(Stdio::piped())
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

    let save_status = Command::new("grim")
        .args(&grim_args)
        .arg(&filename)
        .status()
        .expect("Failed to execute grim for saving file");

    if !save_status.success() {
        eprintln!("Failed to save screenshot to file.");
    }
}
