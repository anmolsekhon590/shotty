use clap::Parser;

/// Shotty: A simple Wayland screenshot wrapper
#[derive(Parser)]
#[command(
    version,
    about = "A simple Wayland screenshot wrapper",
    long_about = "A wayland screenshot wrapper for grim and slurp. Requires compositor to support the unstable wayland screencopy protocol"
)]
pub struct Args {
    /// Take a fullscreen screenshot
    #[arg(short = 'f', long = "fullscreen")]
    pub fullscreen: bool,

    /// Take a screenshot of a specific monitor (e.g., `--output DP-1`)
    #[arg(short = 'o', long = "output", value_name = "MONITOR_NAME")]
    pub output: Option<String>,
}
