use config::{Config, File};
use std::process::Command;
use std::fs;
use std::env;
use chrono::Local;
use directories::ProjectDirs;

#[derive(Debug, serde::Deserialize)]
struct ShottyConfig {
    screenshot_dir: String,
    timestamp_format: String,
    clipboard: bool,
}

fn load_config() -> ShottyConfig {
    let proj_dirs = ProjectDirs::from("com", "anmolsekhon590", "shotty")
        .expect("Could not get project directory");
    let config_path = proj_dirs.config_dir().join("config.toml");

    if !config_path.exists() {
        fs::create_dir_all(proj_dirs.config_dir()).expect("Failed to create config directory");
        fs::write(
            &config_path,
            r#"screenshot_dir = "~/Pictures/screenshots"
timestamp_format = "%Y-%m-%d_%H-%M-%S"
clipboard = true
"#,
        )
        .expect("Failed to write default config");
    }

    Config::builder()
        .add_source(File::from(config_path))
        .build()
        .expect("Failed to load config")
        .try_deserialize()
        .expect("Invalid config format")
}

fn main() {
    let config = load_config();

    let home_dir = env::var("HOME").expect("Could not get HOME directory");
    let screenshot_dir = config.screenshot_dir.replace("~", &home_dir);
    fs::create_dir_all(&screenshot_dir).expect("Failed to create screenshot directory");

    let timestamp = Local::now().format(&config.timestamp_format).to_string();
    let filename = format!("{}/screenshot-{}.png", screenshot_dir, timestamp);

    let slurp_output = Command::new("slurp")
        .output()
        .expect("Failed to execute slurp");

    let selection = String::from_utf8_lossy(&slurp_output.stdout);
    if selection.trim().is_empty() {
        eprintln!("No selection made.");
        return;
    }

    let grim_status = Command::new("grim")
        .args(["-g", selection.trim(), &filename])
        .status()
        .expect("Failed to execute grim");

    if !grim_status.success() {
        eprintln!("Grim failed to capture screenshot.");
        return;
    }

    if config.clipboard {
        let wlcopy_status = Command::new("wl-copy")
            .arg("--type=image/png")
            .stdin(fs::File::open(&filename).expect("Failed to open screenshot file"))
            .status()
            .expect("Failed to execute wl-copy");

        if !wlcopy_status.success() {
            eprintln!("Failed to copy screenshot to clipboard.");
        }
    }
}

