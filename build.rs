use std::env;
use std::path::PathBuf;

fn main() {
    // Only show the message when building in release mode or running cargo install.
    if env::var("PROFILE").unwrap_or_default() != "release" {
        return;
    }

    // Warn if ~/.cargo/bin is not on PATH, since `cargo install` installs there.
    let Ok(path_var) = env::var("PATH") else {
        return;
    };
    let home = env::var("HOME").unwrap_or_default();
    let cargo_bin = PathBuf::from(&home).join(".cargo/bin");
    let cargo_bin_str = cargo_bin.to_string_lossy();

    let cargo_bin_on_path = path_var
        .split(':')
        .any(|p| p == cargo_bin_str || p == "$HOME/.cargo/bin" || p.contains(".cargo/bin"));

    if cargo_bin_on_path {
        return;
    }

    for line in [
        "",
        "╭─────────────────────────────────────────────────────────────╮",
        "│  ⚠️  ~/.cargo/bin is not in your PATH                      │",
        "│                                                             │",
        "│  After installation, add this to your shell config:         │",
        "│                                                             │",
        "│    export PATH=\"$HOME/.cargo/bin:$PATH\"                    │",
        "│                                                             │",
        "│  Then reload: source ~/.bashrc (or restart terminal)        │",
        "│                                                             │",
        "│  Or install system-wide:                                    │",
        "│    sudo cp target/release/shotty /usr/local/bin/           │",
        "╰─────────────────────────────────────────────────────────────╯",
        "",
    ] {
        println!("cargo:warning={line}");
    }
}
