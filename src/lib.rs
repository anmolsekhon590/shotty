use config::{Config, File};
use directories::ProjectDirs;
use std::fs;

#[derive(Debug, serde::Deserialize)]
pub struct ShottyConfig {
    pub screenshot_dir: String,
    pub timestamp_format: String,
    pub clipboard: bool,
}

pub fn load_config() -> ShottyConfig {
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
