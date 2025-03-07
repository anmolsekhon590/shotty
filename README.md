# Shotty

**Shotty** is a simple Wayland screenshot tool that allows you to take region-based screenshots using `grim` and `slurp`. It automatically saves the screenshot and copies it to the clipboard.

## 🚀 Features
- Select a screen region using `slurp`
- Capture a screenshot with `grim`
- Copy the screenshot to the clipboard using `wl-clipboard`
- Saves screenshots in `~/Pictures/screenshots/` by default

## 📦 Dependencies
Before building **Shotty**, make sure you have the following dependencies installed:

- `grim` (Wayland screenshot utility)
- `slurp` (Wayland region selector)
- `wl-clipboard` (Wayland clipboard tool)

## **🔧 Building From Source**

```
git clone https://github.com/anmolsekhon590/shotty.git
cd shotty
cargo build --release
```

> Note: make sure you have Rust installed in your system

## 🚀 Installing Shotty

### Option 1: Install via Cargo (Recommended)
If you have Rust installed, you can install Shotty directly using Cargo:

```
cargo install --git https://github.com/anmolsekhon590/shotty.git
```

This will install `shotty` to `~/.cargo/bin/`. Ensure it's in your `$PATH`:

### Option 2: Install System-wide
To install Shotty for all users:

```
sudo cp target/release/shotty /usr/local/bin/
```

## 📜 License

Shotty is licensed under the GNU General Public License v3.0 (GPL-3.0).

See the full license text in the [LICENSE](https://github.com/anmolsekhon590/shotty/blob/main/LICENSE) file.
