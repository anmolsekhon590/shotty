use std::fs;
use std::process::Command;

pub fn copy_to_clipboard(filename: &str) {
    let status = Command::new("wl-copy")
        .arg("--type=image/png")
        .stdin(fs::File::open(filename).expect("Failed to open screenshot file"))
        .status()
        .expect("Failed to execute wl-copy");

    if !status.success() {
        eprintln!("Failed to copy screenshot to clipboard.");
    }
}
