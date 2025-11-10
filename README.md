# Shotty

**Shotty** is a simple Wayland screenshot tool that allows you to take region-based screenshots using `grim` and `slurp`. It automatically saves the screenshot and copies it to the clipboard.

## 🚀 Features
- Select a screen region using `slurp`
- Capture a screenshot with `grim`
- Copy the screenshot to the clipboard using `wl-clipboard`
- **Screen freeze support** - Prevent dropdowns and UI elements from closing during screenshot selection (requires `hyprpicker`)
- Fullscreen and monitor-specific screenshot support
- Saves screenshots in `~/Pictures/screenshots/` by default

## 📦 Dependencies & Installation

### Core Dependencies (Required)
- `grim` (Wayland screenshot utility)
- `slurp` (Wayland region selector)  
- `wl-clipboard` (Wayland clipboard tool)

### Optional Dependencies
- `hyprpicker` (for screen freeze functionality)

### Installation Commands by Distribution

#### Arch Linux
```bash
# Core dependencies
sudo pacman -S grim slurp wl-clipboard

# Optional: For screen freeze feature
sudo pacman -S hyprpicker
```

#### Ubuntu/Debian
```bash
# Core dependencies
sudo apt update
sudo apt install grim slurp wl-clipboard-tools

# Optional: hyprpicker (may need to build from source on older versions)
# On Ubuntu 24.10+:
sudo apt install hyprpicker
# Or build from source (see manual installation section below)
```

#### Fedora
```bash
# Core dependencies
sudo dnf install grim slurp wl-clipboard

# Optional: For screen freeze feature
sudo dnf install hyprpicker
```

#### openSUSE
```bash
# Core dependencies
sudo zypper install grim slurp wl-clipboard

# Optional: For screen freeze feature
sudo zypper install hyprpicker
```

#### Gentoo
```bash
# Core dependencies
sudo emerge media-gfx/grim gui-apps/slurp gui-apps/wl-clipboard

# Optional: For screen freeze feature (available in GURU overlay)
sudo eselect repository enable guru
sudo emaint sync -r guru
sudo emerge gui-apps/hyprpicker
```

#### Manual Installation (hyprpicker from source)
If `hyprpicker` is not available in your distribution's repositories:

```bash
# Install build dependencies first
# For Arch:
sudo pacman -S cmake ninja pango cairo wayland wayland-protocols hyprutils hyprwayland-scanner libxkbcommon

# For Ubuntu/Debian:
sudo apt install cmake ninja-build libpango1.0-dev libcairo2-dev libwayland-dev wayland-protocols libxkbcommon-dev

# Clone and build hyprpicker
git clone https://github.com/hyprwm/hyprpicker.git
cd hyprpicker
cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -S . -B ./build
cmake --build ./build --config Release --target hyprpicker
sudo cmake --install ./build
```

## 🎯 Usage

```bash
# Basic region screenshot
shotty

# Fullscreen screenshot
shotty -f

# Screenshot specific monitor
shotty -o DP-1

# Region screenshot with screen freeze (prevents dropdowns from closing)
shotty -z

# Combine flags
shotty --freeze --fullscreen
```

### Screen Freeze Feature
The `--freeze` (or `-z`) flag uses `hyprpicker` to freeze the screen content before taking a region screenshot. This is particularly useful for:
- Capturing dropdown menus that would normally close when you click elsewhere
- Screenshot context menus  
- Capturing tooltips and hover states
- Any UI elements that would normally disappear when clicking elsewhere

**How it works:**
1. Run `shotty -z`
2. The screen content freezes immediately 
3. You can now safely select the region without dropdowns/menus disappearing
4. The screenshot captures the frozen content

This feature works similarly to Windows' Snipping Tool screen freeze functionality and is perfect for capturing ephemeral UI elements like dropdown menus in your Zen browser!

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

```bash
cargo install --git https://github.com/anmolsekhon590/shotty.git
```

This will install `shotty` to `~/.cargo/bin/`.

**Important:** Ensure `~/.cargo/bin` is in your `$PATH`. Add this to your shell configuration file (`~/.bashrc`, `~/.zshrc`, or `~/.config/fish/config.fish`):

```bash
# For bash/zsh
export PATH="$HOME/.cargo/bin:$PATH"

# For fish
fish_add_path $HOME/.cargo/bin
```

Then reload your shell:
```bash
source ~/.bashrc  # or source ~/.zshrc, or restart your terminal
```

### Option 2: Install System-wide
To install Shotty for all users (after building from source):

```bash
# Build the project first
cargo build --release

# Install to /usr/local/bin (no PATH configuration needed)
sudo cp target/release/shotty /usr/local/bin/
```

### Option 3: Install from Local Repository
If you've cloned the repository and want to install from the local source:

```bash
# Install directly from the local directory
cargo install --path .

# Or install to /usr/local/bin
cargo install --path . --root /usr/local
```

## 📜 License

Shotty is licensed under the GNU General Public License v3.0 (GPL-3.0).

See the full license text in the [LICENSE](https://github.com/anmolsekhon590/shotty/blob/main/LICENSE) file.
