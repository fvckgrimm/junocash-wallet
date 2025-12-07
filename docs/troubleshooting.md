# Troubleshooting Guide

## Connection Issues

### "Cannot Connect to Node"

**Check if node is running:**

Linux/Mac:
```bash
ps aux | grep junocashd
```

Windows (Command Prompt):
```cmd
tasklist | findstr junocashd
```

If not running, start it from Settings or manually.

**Verify RPC credentials match:**
1. Check wallet Settings (RPC User & Password)
2. Check `junocash.conf` file in data directory
3. They must be identical

**Check RPC port:**
- Default is `18232`
- Make sure no firewall is blocking it
- If running node on different computer, ensure port is accessible

**Try restarting:**
1. Stop the node (if running)
2. Close wallet
3. Relaunch wallet
4. Click "Save & Launch Node" in Settings

### "Node Starting But Wallet Won't Connect"

Wait 10-30 seconds. Node needs time to initialize.

If still failing after 1 minute:
1. Check `debug.log` in your data directory
2. Look for errors near the bottom
3. Common issues: corrupted data, wrong paths, permission errors

## Balance/Transaction Issues

### Balance Shows 0 But I Have Coins

**Check both balances:**
- Dashboard shows Transparent AND Shielded separately
- Mined coins appear in Transparent first
- Must shield them to move to Shielded

**Node might not be synced:**
- Check block height on Dashboard
- Compare to network block height (check explorer or ask community)
- If behind, wait for sync to complete

**Refresh balance:**
- Click the Refresh button on Dashboard
- Wait a few seconds
- If still wrong, restart wallet

### Transaction Not Appearing

**Check confirmations:**
- New transactions need confirmations to show
- Wait 2-5 minutes
- Click Refresh on Dashboard or History page

**Check correct address type:**
- Sent to shielded? Check Shielded balance
- Sent to transparent? Check Transparent balance

**View transaction details:**
If you have the TXID or Operation ID, check it on a block explorer or use:
```bash
junocash-cli z_viewtransaction "txid..."
```

### Can't Spend Mined Coins

Mined coins must be:
1. **Matured** (100 confirmations, ~2.5 hours)
2. **Shielded** (moved from transparent to shielded)

Go to Mining page and use Auto-Shielding or manual shield button.

## Mining Issues

### Mining Won't Start

**Node not synced:**
- Most common issue
- Check block height matches network
- Wait for full sync before mining

**Wrong settings:**
- Go to Settings
- Verify paths are correct
- Click "Save & Connect Only" to test connection

**Try command line:**
```bash
junocash-cli setgenerate true 2
```
If this works but wallet button doesn't, report the bug.

### Hashrate Shows 0 or Very Low

**Just started mining:**
- Wait 10-20 seconds for hashrate to stabilize
- Initial reading can be inaccurate

**CPU throttling:**
- Check CPU temperature (use monitoring tool)
- If too hot, CPU slows down automatically
- Reduce thread count or improve cooling

**Wrong algorithm:**
- JunoCash uses Equihash
- Make sure you're mining JunoCash, not another coin

### Computer Freezes When Mining

**Too many threads:**
- Reduce thread count to 2-4
- Leave some CPU for system tasks
- Don't use 100% of available threads

**Insufficient RAM:**
- RandomX Fast Mode needs 2GB extra RAM
- Disable it in Settings if you have less than 8GB total
- Regular mining uses much less memory

**Overheating:**
- Monitor CPU temperature
- Clean dust from fans/heatsink
- Improve case airflow
- Reduce threads

## Wallet UI Issues

### Wallet Window Won't Open

Check if running in system tray:
- Look for tray icon near clock
- Click it to show window
- Or right-click → Show Window

### App Crashes on Launch

**Delete settings and restart:**

Linux:
```bash
rm ~/.config/junocash-wallet/settings.json
```

Windows:
```cmd
del %APPDATA%\junocash-wallet\settings.json
```

Then relaunch and reconfigure.

**Check logs:**

Linux/Mac:
```bash
~/.local/share/junocash-wallet/logs/
```

Windows:
```cmd
%APPDATA%\junocash-wallet\logs\
```

### Balance Visibility Toggle Not Working

Clear localStorage:
1. Press F12 (open developer tools)
2. Go to Console tab
3. Type: `localStorage.clear()`
4. Press Enter
5. Restart wallet

Note: This will clear all settings, you'll need to reconfigure.

## Node Issues

### Node Won't Start From Wallet

**Check binary path:**
- Settings → Executable Path
- Click search button and verify file exists
- Make sure it's the actual `junocashd` file, not a folder

**Permission denied (Linux/Mac):**
```bash
chmod +x /path/to/junocashd
```

**Port already in use:**
- Another instance might be running
- Check with `ps aux | grep junocashd`
- Kill it: `pkill junocashd`
- Or change RPC port in config

**Data directory issues:**
- Check Data Directory path in Settings
- Make sure you have write permissions
- Ensure enough disk space (10GB+ free)

### Blockchain Sync Stuck

**Taking very long:**
- Initial sync can take hours
- Check `debug.log` for progress
- Make sure internet connection is stable

**Actually stuck (no progress for 30+ min):**
1. Stop node
2. Delete `peers.dat` in data directory
3. Restart node
4. It will find new peers

**Corrupted blockchain:**
Last resort - redownload blockchain:
1. Stop node
2. Delete these folders in data directory:
   - `blocks/`
   - `chainstate/`
3. Keep `wallet.dat` (don't delete this!)
4. Restart node, will resync from scratch

## Finding Log Files

**Wallet logs:**
- Linux: `~/.local/share/junocash-wallet/logs/`
- Windows: `%APPDATA%\junocash-wallet\logs\`
- macOS: `~/Library/Application Support/junocash-wallet/logs/`

**Node logs:**
Look for `debug.log` in your data directory:
- Linux: `~/.junocash/debug.log`
- Windows: `%APPDATA%\JunoCash\debug.log`
- macOS: `~/Library/Application Support/JunoCash/debug.log`

**View last 50 lines:**
```bash
tail -n 50 debug.log
```

## Getting Help

If none of this works:

1. **Check logs** (see above)
2. **Take screenshots** of error messages
3. **Note your setup:**
   - OS (Windows/Linux/Mac)
   - Wallet version
   - Node version
   - What you were doing when it broke

4. **Ask for help:**
   - GitHub Issues: [github.com/fvckgrimm/junocash-wallet/issues](https://github.com/fvckgrimm/junocash-wallet/issues)
   - Discord: [discord.gg/junocash](https://discord.gg/junocash)

**When asking:**
- Describe the problem clearly
- Include error messages (paste full text or screenshot)
- Say what you already tried
- Don't share private keys or passwords

## Common Error Messages

**"Error: method not found"**
- Node RPC not responding correctly
- Check node is fully started
- Verify RPC credentials

**"Error: insufficient funds"**
- Not enough balance for transaction + fee
- Remember: 0.0001 JUNO fee per transaction
- Check both transparent and shielded balances

**"Error: Invalid address"**
- Address format is wrong
- JunoCash addresses start with `j1...` (shielded) or `t1...` (transparent)
- Copy-paste carefully, no extra spaces

**"Operation failed"**
- Generic error from node
- Check `debug.log` for details
- Usually means transaction parameters were invalid

## Preventive Maintenance

**Back up regularly:**
- Wallet settings are in localStorage (auto-backed by OS)
- Node wallet is in `wallet.dat` (backup this file!)
- Keep backup on USB drive or cloud storage

**Keep updated:**
- Update wallet when new version releases
- Update node (junocashd) when available
- Check GitHub releases page regularly

**Monitor disk space:**
- Blockchain grows over time
- Keep 20GB+ free in data directory
- Or move data directory to bigger drive

**Clean up old data:**
Node debug.log can get huge:
```bash
# Linux/Mac - Keep only last 1000 lines
tail -n 1000 debug.log > debug.log.tmp && mv debug.log.tmp debug.log
```
