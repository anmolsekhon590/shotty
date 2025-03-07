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
git clone https://github.com/YOUR_USERNAME/shotty.git
cd shotty
cargo build --release
```

> Note: make sure you have Rust installed in your system

## 🚀 Installing Shotty

To install it system-wide:

```
sudo cp target/release/shotty /usr/local/bin/
```

