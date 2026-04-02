# OxidClean

[![CI](https://github.com/luqastw/oxidclean/actions/workflows/ci.yml/badge.svg)](https://github.com/luqastw/oxidclean/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A terminal tool for identifying and removing orphan packages on Arch Linux and derivatives.

## About

OxidClean identifies packages installed as dependencies that are no longer required by any other package, and helps you safely reclaim disk space on your Arch Linux system.

## Features

- **Orphan Detection** - Identifies dependency packages no longer needed by any installed software
- **Risk Classification** - Packages categorized as Safe, Caution, or Critical before removal
- **Dependency Analysis** - Full dependency graph with cycle detection
- **Cache Management** - Clean old package versions from pacman cache
- **Package Protection** - Critical system packages are protected against accidental removal
- **Visual Interface** - Colored tables, progress bars, and visual symbols

## Installation

```bash
# Via Cargo
cargo install oxidclean

# Via AUR (when available)
yay -S oxidclean
```

## Quick Start

```bash
# Scan system and view report
oxidclean scan

# List orphan packages
oxidclean list

# Remove orphans interactively
sudo oxidclean clean

# Analyze a specific package
oxidclean analyze vim

# View cache statistics
oxidclean cache --stats

# Clean package cache
sudo oxidclean cache --clean
```

## Commands

| Command | Description |
|---------|-------------|
| `scan` | Scans the system and displays orphan package report |
| `clean` | Removes orphan packages interactively |
| `analyze <pkg>` | Analyzes dependencies of a specific package |
| `cache` | Manages pacman cache |
| `list` | Lists all orphan packages |
| `completion <shell>` | Generates shell completion scripts |

## Example Output

```
╔═══════════════════════════════════════════════════════════╗
║              OxidClean - System Report                    ║
╚═══════════════════════════════════════════════════════════╝

📊 Statistics
   Total installed packages: 1396
   Orphan packages found:    31
   Recoverable space:        976.42 MB

📦 Orphan Packages
┌─────────────────┬─────────┬──────────┬────────┐
│ Name            │ Version │ Size     │ Risk   │
├─────────────────┼─────────┼──────────┼────────┤
│ lib32-libpng    │ 1.6.43  │ 456.2 KB │ Safe   │
│ python-pip      │ 24.0    │ 12.3 MB  │ Safe   │
│ nvidia-utils    │ 550.67  │ 89.1 MB  │ Caution│
└─────────────────┴─────────┴──────────┴────────┘
```

## Documentation

- [Installation](docs/en/INSTALL.md)
- [Usage Guide](docs/en/USAGE.md)
- [Troubleshooting](docs/en/TROUBLESHOOTING.md)

## Development

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

### Benchmarks

```bash
cargo bench
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or pull request.
