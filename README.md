# JunoCash Wallet

A modern desktop wallet for JunoCash built with Tauri, Vue 3, and Rust.

## Features

- **Portfolio Dashboard**: View total balance, shielded and transparent funds with visual breakdowns
- **Send & Receive**: Manage transactions with support for both transparent and shielded addresses
- **Integrated Mining**: Built-in CPU miner with configurable thread allocation
- **Auto-Shielding**: Automatically shield mined coinbase transactions for enhanced privacy
- **Transaction History**: Complete activity log with filtering and search
- **Node Management**: Launch and manage your own JunoCash node directly from the wallet
- **Balance Privacy**: Toggle balance visibility for privacy in public settings
- **System Tray**: Close to tray for background operation

## Technology Stack

- **Frontend**: Vue 3, TypeScript, TailwindCSS
- **Backend**: Rust, Tauri 2.0
- **RPC**: Direct communication with JunoCash daemon

## Prerequisites

- Node.js 18+ and Bun (or npm/pnpm/yarn)
- Rust 1.70+
- JunoCash daemon (junocashd)

## Installation

### From Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/junocash-wallet.git
cd junocash-wallet
```

2. Install frontend dependencies:
```bash
bun install
```

3. Build the application:
```bash
bun run tauri build
```

The built application will be in `src-tauri/target/release/`.

### Development

Run in development mode:
```bash
bun run tauri dev
```

## Configuration

### First Time Setup

1. Launch the wallet
2. Navigate to Settings
3. Configure your node paths:
   - **Executable Path**: Location of `junocashd` binary
   - **Data Directory**: Blockchain storage location
4. Set RPC credentials (must match your `junocash.conf`)
5. Either launch the node from the wallet or connect to an existing instance

### Node Configuration

If running an external node, ensure your `junocash.conf` contains:

```conf
server=1
rpcuser=your_username
rpcpassword=your_password
rpcport=18232
```

### RandomX Fast Mode

When launching the node from the wallet, you can enable RandomX Fast Mode for improved mining performance. This requires approximately 2GB of additional RAM but significantly increases hashrate.

Enable in Settings > Performance Options before launching the node.

## Mining

### Starting Mining

1. Navigate to the Mining page
2. Configure CPU threads (or use Auto for ~50% of available cores)
3. Click the power button to start mining
4. Monitor hashrate and network statistics in real-time

### Auto-Shielding

To automatically shield mined rewards:

1. Enable the Auto-Shielding toggle
2. Select a destination shielded address
3. Shields will be triggered every 5 minutes while mining

Manual shielding is also available via the "Trigger Manual Shield" button.

## Building for Distribution

### Linux

```bash
bun run tauri build
```

Outputs:
- AppImage: `src-tauri/target/release/bundle/appimage/`
- Debian package: `src-tauri/target/release/bundle/deb/`

### Windows

```bash
bun run tauri build
```

Outputs:
- Installer: `src-tauri\target\release\bundle\msi\`
- Portable: `src-tauri\target\release\`

### macOS

```bash
bun run tauri build
```

Outputs:
- DMG: `src-tauri/target/release/bundle/dmg/`
- App bundle: `src-tauri/target/release/bundle/macos/`

## Architecture

### Frontend Structure

```
src/
├── components/     # Reusable UI components
├── views/          # Page components
├── stores/         # Pinia state management
├── router/         # Vue Router configuration
└── assets/         # Static assets
```

### Backend Structure

```
src-tauri/src/
├── commands/       # Tauri command handlers
│   ├── mining.rs
│   ├── node.rs
│   └── wallet.rs
├── rpc.rs          # RPC client implementation
├── state.rs        # Application state
└── main.rs         # Entry point
```

## RPC Commands

The wallet communicates with `junocashd` via JSON-RPC:

- `getbalance` - Retrieve wallet balances
- `listtransactions` - Fetch transaction history
- `getnewaddress` - Generate new addresses
- `sendtoaddress` - Send transparent transactions
- `z_sendmany` - Send shielded transactions
- `getmininginfo` - Mining statistics
- `setgenerate` - Start/stop mining
- `z_shieldcoinbase` - Shield mined rewards

## Security Considerations

- RPC credentials are stored in localStorage (encrypted by OS)
- Private keys are managed by the JunoCash daemon
- Always verify addresses before sending transactions
- Keep your system and wallet software updated

## Troubleshooting

### Cannot Connect to Node

- Verify `junocashd` is running
- Check RPC credentials match your configuration
- Ensure RPC port (default 18232) is not blocked
- Check `debug.log` in your JunoCash data directory

### Mining Not Starting

- Verify node is fully synced
- Check that the wallet is unlocked
- Ensure sufficient CPU resources are available
- Review node logs for errors


## Support

For issues and feature requests, please use the GitHub issue tracker.

For general questions, join the junocash discord server:
- Discord: [discord.gg/junocash](discord.gg/junocash)

or reach me via wtv links on my site 
