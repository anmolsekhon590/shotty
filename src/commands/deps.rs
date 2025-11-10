use std::process::Command;

/// Check if a command exists in the system PATH
fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check all required dependencies and return helpful error messages
pub fn check_dependencies() -> Result<(), String> {
    let mut missing = Vec::new();

    if !command_exists("grim") {
        missing.push("grim");
    }

    if !command_exists("slurp") {
        missing.push("slurp");
    }

    if !command_exists("wl-copy") {
        missing.push("wl-copy (wl-clipboard)");
    }

    if !missing.is_empty() {
        let mut error_msg = String::from("Missing required dependencies:\n");
        for dep in &missing {
            error_msg.push_str(&format!("  - {}\n", dep));
        }
        error_msg.push_str("\nPlease install them using your package manager:\n");
        error_msg.push_str("  Arch Linux:  sudo pacman -S grim slurp wl-clipboard\n");
        error_msg.push_str("  Debian/Ubuntu: sudo apt install grim slurp wl-clipboard-tools\n");
        error_msg.push_str("  Fedora:      sudo dnf install grim slurp wl-clipboard\n");
        error_msg.push_str("  openSUSE:    sudo zypper install grim slurp wl-clipboard\n");
        error_msg.push_str("  Gentoo:      sudo emerge media-gfx/grim gui-apps/slurp gui-apps/wl-clipboard\n");

        return Err(error_msg);
    }

    Ok(())
}
