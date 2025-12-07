# Configuration Guide

## First Time Setup

### Step 1: Install JunoCash Node

Before using the wallet, you need the JunoCash daemon (node software).

**Download from:** [github.com/junocash/junocash/releases](https://github.com/junocash/junocash/releases)

**Installation:**
- **Linux**: Extract and move `junocashd` to `/usr/bin/` or keep it anywhere
- **Windows**: Extract to `C:\Program Files\JunoCash\` or any folder
- **macOS**: Extract to `/usr/local/bin/` or Applications folder

### Step 2: Configure the Wallet

1. Launch JunoCash Wallet
2. Go to **Settings** (bottom of sidebar)
3. Fill in these fields:

**Executable Path:**
- Click the search button and locate your `junocashd` file
- Example Linux: `/usr/bin/junocashd`
- Example Windows: `C:\Program Files\JunoCash\junocashd.exe`

**Data Directory:**
- Where blockchain data is stored (can be large, 10GB+)
- Click folder button to choose location
- Default locations work fine:
  - Linux: `~/.junocash`
  - Windows: `%APPDATA%\JunoCash`
  - macOS: `~/Library/Application Support/JunoCash`

**RPC User & Password:**
- Username: `junocash` (or whatever you want)
- Password: Pick a strong password
- Remember these, you'll need them later

### Step 3: Launch the Node

Two options:

**Option A: Launch from Wallet (Recommended)**
- Click **"Save & Launch Node"** in Settings
- Wallet will start the node for you
- Keep wallet open while using

**Option B: Run Node Separately**
- Useful if you want node running 24/7
- See "Running External Node" below

## Running External Node

If you want to run the node separately (in background, on another computer, etc):

### Create Config File

Create a file called `junocash.conf` in your data directory:

**Linux/Mac:** `~/.junocash/junocash.conf`
**Windows:** `%APPDATA%\JunoCash\junocash.conf`

Add these lines:
```conf
server=1
rpcuser=junocash
rpcpassword=your_password_here
rpcport=18232
```

Replace `your_password_here` with the same password you put in wallet settings.

### Start the Node

**Linux/Mac:**
```bash
junocashd -daemon
```

**Windows:**
Double-click `junocashd.exe` or run in Command Prompt:
```cmd
junocashd.exe
```

### Connect Wallet

In wallet Settings, click **"Save & Connect Only"**. 

The wallet will connect to your running node.

## RandomX Fast Mode

Makes mining faster but uses 2GB extra RAM.

**When to enable:**
- You have 8GB+ RAM
- You plan to mine regularly
- You want maximum hashrate

**How to enable:**
1. Go to Settings
2. Toggle **"RandomX Fast Mode"** ON
3. Click **"Save & Launch Node"**

Note: Only works when launching node from wallet.

## Testnet vs Mainnet

By default, everything runs on mainnet (real coins).

**To use testnet** (for testing):
1. Start node with `-testnet` flag:
```bash
junocashd -testnet
```
2. Wallet automatically detects testnet

Testnet coins have no value. Good for learning.

## Port Configuration

Default RPC port: `18232`

If you need to change it:
1. Edit `junocash.conf`: `rpcport=YOUR_PORT`
2. Currently wallet uses 18232 (hardcoded)
3. Contact developer if you need custom port support

## Security Tips

- Keep RPC password strong (16+ characters)
- Don't share RPC credentials
- Wallet stores settings in browser localStorage (OS-encrypted)
- Back up your wallet regularly
- Node data directory can be 10GB+, plan storage accordingly

## What's Next?

Once configured:
- **Dashboard** shows your balance
- **Receive** generates addresses for incoming funds
- **Send** to transfer coins
- **Mining** to earn rewards
- **History** to view all transactions
