# Troubleshooting

## Common Problems

### "Pacman database not found"

**Error:**
```
Pacman database not found or corrupted at: /var/lib/pacman/local
```

**Cause:** OxidClean can't access pacman's database.

**Solutions:**
1. Verify you're on an Arch Linux or derivative system
2. Check permissions:
   ```bash
   ls -la /var/lib/pacman/local
   ```
3. Sync the database:
   ```bash
   sudo pacman -Sy
   ```

### "Insufficient permissions"

**Error:**
```
Insufficient permissions: Operation requires root permissions
```

**Cause:** Removal commands need root.

**Solution:**
```bash
sudo oxidclean clean
```

### "Unsupported system"

**Error:**
```
Unsupported system: expected Arch Linux or derivative
```

**Cause:** OxidClean detected the system is not Arch Linux.

**Solutions:**
1. Verify `/etc/os-release` exists
2. Confirm it's an Arch-based system:
   ```bash
   cat /etc/os-release | grep -i arch
   ```

### Scan is too slow

**Cause:** First run or expired cache.

**Solutions:**
1. Cache lasts 5 minutes - subsequent runs are faster
2. Check if disk isn't slow
3. For many packages (>3000), a few seconds is normal

### Package removed but still appears as orphan

**Cause:** Scan cache is outdated.

**Solution:**
```bash
# Force a new scan (wait for cache to expire or restart)
# Or simply run scan again after a few minutes
oxidclean scan
```

## FAQ

### Can OxidClean damage my system?

OxidClean has several protections:
- Critical packages are protected (base, linux, pacman, etc.)
- Interactive mode asks confirmation for each removal
- Dry-run mode allows simulation before execution
- All operations are logged

### What's the difference between OxidClean and `pacman -Qdtq`?

OxidClean offers additional features:
- Risk classification per package
- Complete dependency analysis
- Cache management
- Visual interface with colors and tables
- Critical package protection
- Operation logging

### Can I use it on Manjaro/EndeavourOS?

Yes! OxidClean works on any Arch Linux-based distribution.

### How do I backup before cleaning?

```bash
# List explicitly installed packages
pacman -Qqe > packages-explicit.txt

# List all packages
pacman -Qq > packages-all.txt

# Export report
oxidclean scan --export backup-report.json
```

### Pacman cache is taking too much space

Use the cache command:
```bash
# View statistics
oxidclean cache --stats

# Clean keeping only 1 version
sudo oxidclean cache --clean --keep 1

# For more aggressive cleanup, use paccache directly
sudo paccache -rk1
```

## Logs and Debug

### Log locations

```bash
# Operations log
~/.config/oxidclean/history.log

# Configuration
~/.config/oxidclean/config.toml
```

### Verbose mode

For more information during execution:
```bash
oxidclean -v scan
```

### Report bugs

If you find a bug:
1. Run with `--verbose` and capture output
2. Include system information:
   ```bash
   uname -a
   cat /etc/os-release
   pacman --version
   ```
3. Open an issue in the project repository
