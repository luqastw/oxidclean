# Usage Guide

## Available Commands

### scan - Scan System

Analyzes the system and identifies orphan packages.

```bash
# Basic scan
oxidclean scan

# Sort by size
oxidclean scan --sort-by-size

# JSON output
oxidclean scan --json

# Export to file
oxidclean scan --export report.json
```

### clean - Remove Packages

Removes orphan packages interactively.

```bash
# Interactive mode (asks confirmation for each package)
sudo oxidclean clean

# Auto-confirm all removals
sudo oxidclean clean --yes

# Simulate removals (doesn't execute)
oxidclean clean --dry-run
```

### analyze - Analyze Package

Analyzes dependencies of a specific package.

```bash
# Analyze vim
oxidclean analyze vim

# JSON output
oxidclean analyze vim --json
```

Shows:
- Direct dependencies
- Transitive dependencies
- Packages that depend on it (reverse dependencies)
- Total size with dependencies

### cache - Manage Cache

Manages pacman's package cache.

```bash
# View cache statistics
oxidclean cache --stats

# Clean old versions (keeps 3 by default)
sudo oxidclean cache --clean

# Clean keeping only 1 version
sudo oxidclean cache --clean --keep 1

# Simulate cleanup
oxidclean cache --clean --dry-run

# Clean without confirmation
sudo oxidclean cache --clean --yes
```

### list - List Orphans

Lists all orphan packages (simple mode).

```bash
# Basic list
oxidclean list

# Sort by size
oxidclean list --sort-by-size
```

## Global Flags

| Flag | Description |
|------|-------------|
| `-v, --verbose` | Verbose mode - shows more details |
| `-q, --quiet` | Quiet mode - results only |
| `-h, --help` | Shows help |
| `-V, --version` | Shows version |

## Practical Examples

### Basic cleanup workflow

```bash
# 1. View system report
oxidclean scan

# 2. Review orphans (dry-run mode)
oxidclean clean --dry-run

# 3. Remove orphans interactively
sudo oxidclean clean

# 4. Clean cache
sudo oxidclean cache --clean
```

### Automation with scripts

```bash
# List orphans (for script usage)
oxidclean list --quiet

# Remove all orphans automatically
sudo oxidclean clean --yes

# Export JSON report for processing
oxidclean scan --json > /tmp/report.json
```

### Analyzing a package before removal

```bash
# See what depends on a package
oxidclean analyze libcups

# If nothing depends on it, safe to remove
# If something depends, it shows in "reverse_dependencies"
```

## Risk Levels

OxidClean classifies packages in three levels:

| Level | Color | Description |
|-------|-------|-------------|
| Safe | Green | Safe to remove |
| Caution | Yellow | Review before removing (system related) |
| Critical | Red | Never remove (protected packages) |

## Protected Packages

By default, the following packages are protected:

- `base`, `linux`, `linux-lts`, `linux-zen`, `linux-hardened`
- `linux-firmware`, `pacman`, `systemd`
- `glibc`, `coreutils`, `bash`, `shadow`, `util-linux`

You can add more packages in the configuration file.
