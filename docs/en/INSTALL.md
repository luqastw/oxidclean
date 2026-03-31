# Installation

## Requirements

- Arch Linux or derivative (Manjaro, EndeavourOS, etc.)
- Rust 1.70+ (for compilation)
- Root permissions (for package removal)

## Installation Methods

### Via Cargo (Recommended for developers)

```bash
cargo install oxidclean
```

### Via AUR (Arch User Repository)

```bash
# With yay
yay -S oxidclean

# With paru
paru -S oxidclean

# Manually
git clone https://aur.archlinux.org/oxidclean.git
cd oxidclean
makepkg -si
```

### Manual Build

```bash
# Clone repository
git clone https://github.com/your-username/oxidclean.git
cd oxidclean

# Build release
cargo build --release

# Install (optional)
sudo cp target/release/oxidclean /usr/local/bin/
```

## Verification

After installation, verify it's working:

```bash
oxidclean --version
oxidclean --help
```

## Initial Configuration

OxidClean works without configuration, but you can create a config file:

```bash
mkdir -p ~/.config/oxidclean
```

Create `~/.config/oxidclean/config.toml`:

```toml
# Packages to ignore in analysis
ignored_packages = []

# Protected packages (will never be removed)
# Defaults already include: base, linux, pacman, systemd, glibc, etc.
protected_packages = []

# Interactive mode by default
interactive = true

# Cache versions to keep per package
cache_keep_versions = 3
```

## Uninstallation

### Via Cargo

```bash
cargo uninstall oxidclean
```

### Via AUR

```bash
sudo pacman -Rns oxidclean
```

### Manual

```bash
sudo rm /usr/local/bin/oxidclean
rm -rf ~/.config/oxidclean
```
