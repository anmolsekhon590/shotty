use std::fs;
use std::process::Command;

pub fn copy_to_clipboard(filename: &str) {
    let file = match fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open screenshot file: {}", e);
            return;
        }
    };

    let status = match Command::new("wl-copy")
        .arg("--type=image/png")
        .stdin(file)
        .status()
    {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Failed to execute wl-copy: {}", e);
            eprintln!("Make sure wl-clipboard is installed and in your PATH.");
            return;
        }
    };

    if !status.success() {
        eprintln!("Failed to copy screenshot to clipboard.");
    }
}
