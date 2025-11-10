use std::env;
use std::path::PathBuf;

fn main() {
    // Only show the message when building in release mode or running cargo install
    let profile = env::var("PROFILE").unwrap_or_default();

    // Check if ~/.cargo/bin is in PATH
    if let Ok(path_var) = env::var("PATH") {
        let home = env::var("HOME").unwrap_or_default();
        let cargo_bin = PathBuf::from(&home).join(".cargo/bin");
        let cargo_bin_str = cargo_bin.to_string_lossy();

        if !path_var.split(':').any(|p| p == cargo_bin_str || p == "$HOME/.cargo/bin" || p.contains(".cargo/bin")) {
            if profile == "release" {
                println!("cargo:warning=");
                println!("cargo:warning=╭─────────────────────────────────────────────────────────────╮");
                println!("cargo:warning=│  ⚠️  ~/.cargo/bin is not in your PATH                      │");
                println!("cargo:warning=│                                                             │");
                println!("cargo:warning=│  After installation, add this to your shell config:         │");
                println!("cargo:warning=│                                                             │");
                println!("cargo:warning=│    export PATH=\"$HOME/.cargo/bin:$PATH\"                    │");
                println!("cargo:warning=│                                                             │");
                println!("cargo:warning=│  Then reload: source ~/.bashrc (or restart terminal)        │");
                println!("cargo:warning=│                                                             │");
                println!("cargo:warning=│  Or install system-wide:                                    │");
                println!("cargo:warning=│    sudo cp target/release/shotty /usr/local/bin/           │");
                println!("cargo:warning=╰─────────────────────────────────────────────────────────────╯");
                println!("cargo:warning=");
            }
        }
    }
}
