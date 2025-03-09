use clap::Parser;

/// Shotty: A simple Wayland screenshot tool
#[derive(Parser)]
#[clap(
    name = "Shotty",
    version = "0.1.0",
    about = "A simple Wayland screenshot tool"
)]
pub struct Args {
    /// Take a fullscreen screenshot
    #[clap(long)]
    pub fullscreen: bool,

    /// Take a screenshot of a specific monitor (e.g., `--output DP-1`)
    #[clap(long, value_name = "MONITOR_NAME")]
    pub output: Option<String>,
}
