# Architecture Overview

## Technology Stack

### Frontend
- **Vue 3** - UI framework with Composition API
- **TypeScript** - Type-safe JavaScript
- **TailwindCSS** - Utility-first styling
- **Pinia** - State management
- **Vue Router** - Page navigation

### Backend
- **Rust** - Systems programming language
- **Tauri 2.0** - Desktop app framework
- **Reqwest** - HTTP client for RPC calls
- **Serde JSON** - JSON serialization

### Communication
- **JSON-RPC** - Protocol for communicating with JunoCash daemon
- **Tauri Commands** - Bridge between Rust backend and Vue frontend

## Project Structure

```
junocash-wallet/
├── src/                        # Vue frontend
│   ├── components/             # Reusable UI components
│   │   ├── Sidebar.vue         # Navigation sidebar
│   │   └── Status.vue          # Node status indicator
│   ├── views/                  # Page components
│   │   ├── Dashboard.vue       # Balance overview
│   │   ├── Send.vue            # Send transactions
│   │   ├── Receive.vue         # Generate addresses
│   │   ├── History.vue         # Transaction history
│   │   ├── Mining.vue          # Mining controls
│   │   └── Settings.vue        # Configuration
│   ├── stores/                 # Pinia state stores
│   │   ├── nodeStore.ts        # Node settings/state
│   │   └── walletStore.ts      # Wallet data/actions
│   ├── router/                 # Vue Router config
│   │   └── index.ts            # Route definitions
│   ├── App.vue                 # Root component
│   └── main.ts                 # Entry point
│
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── commands/           # Tauri command handlers
│   │   │   ├── mining.rs       # Mining RPC commands
│   │   │   ├── node.rs         # Node launch/control
│   │   │   └── wallet.rs       # Wallet RPC commands
│   │   ├── rpc.rs              # RPC client implementation
│   │   ├── state.rs            # Application state (node process)
│   │   ├── lib.rs              # Library setup
│   │   └── main.rs             # App entry point
│   ├── Cargo.toml              # Rust dependencies
│   └── tauri.conf.json         # Tauri configuration
│
├── docs/                       # Documentation
├── assets/                     # Screenshots/images
└── README.md                   # Main readme
```

## How It Works

### Application Flow

```
User Interaction (Vue UI)
    ↓
Tauri Command (invoke)
    ↓
Rust Handler (commands/*.rs)
    ↓
RPC Client (rpc.rs)
    ↓
JunoCash Daemon (junocashd)
    ↓
Response back through chain
    ↓
UI Updates (Pinia store)
```

### Example: Sending a Transaction

1. User fills form in `Send.vue`
2. Calls `wallet.sendTransaction()` in `walletStore.ts`
3. Store invokes Tauri command: `invoke('send_transaction', {...})`
4. Rust handler in `wallet.rs::send_transaction()` receives call
5. Handler calls `call_rpc("z_sendmany", ...)` in `rpc.rs`
6. RPC client makes HTTP request to junocashd
7. Daemon processes transaction, returns operation ID
8. Operation ID travels back through chain to UI
9. UI shows success state with operation ID

## State Management

### Node Store (`nodeStore.ts`)

Manages node configuration and connection:

```typescript
{
  binPath: string,          // Path to junocashd binary
  dataDir: string,          // Blockchain data directory
  rpcUser: string,          // RPC username
  rpcPass: string,          // RPC password
  rpcPort: number,          // RPC port (default: 18232)
  randomxFastMode: boolean, // RandomX optimization
  isConnected: boolean      // Connection status
}
```

Stored in: `localStorage`

### Wallet Store (`walletStore.ts`)

Manages wallet data and operations:

```typescript
{
  balance: {
    total: number,
    transparent: number,
    private: number
  },
  transactions: Array,      // Transaction history
  addresses: Array,         // Generated addresses
  isLoading: boolean,       // Operation in progress
  lastError: string         // Last error message
}
```

Actions:
- `fetchBalance()` - Get current balances
- `fetchTransactions()` - Load transaction history
- `fetchAddresses()` - Get all wallet addresses
- `sendTransaction()` - Send JUNO to address
- `generateAddress()` - Create new address

## RPC Commands

### Wallet Commands

**`get_balance`**
- RPC: `z_gettotalbalance`
- Returns: `{ total, transparent, private }`
- Used by: Dashboard, Send page

**`list_transactions`**
- RPC: `listtransactions "*" 50 0`
- Returns: Array of transaction objects
- Used by: Dashboard, History page

**`send_transaction`**
- RPC: `z_sendmany`
- Params: from_address, recipients, minconf, fee
- Returns: Operation ID
- Used by: Send page

**`get_new_address`**
- RPC: `z_getnewaddress`
- Params: address_type ("unified")
- Returns: New address string
- Used by: Receive page

**`get_all_addresses`**
- RPC: `listaddresses`
- Returns: Hierarchical address structure
- Used by: Receive page, Mining auto-shield

**`get_block_count`**
- RPC: `getblockcount`
- Returns: Current block height
- Used by: Status component

### Mining Commands

**`get_mining_info`**
- RPC: `getmininginfo`
- Returns: `{ localsolps, networksolps, difficulty, blocks, generate }`
- Used by: Mining page (polling every 5s)

**`set_mining`**
- RPC: `setgenerate`
- Params: enabled (bool), threads (int)
- Returns: Status message
- Used by: Mining page toggle

**`get_block_subsidy`**
- RPC: `getblocksubsidy`
- Params: block_height
- Returns: Block reward amount
- Used by: Mining page reward display

**`shield_coinbase`**
- RPC: `z_shieldcoinbase`
- Params: from ("*"), to_address
- Returns: Operation ID
- Used by: Mining auto-shield

### Node Commands

**`launch_node`**
- Spawns junocashd process
- Params: bin_path, data_dir, rpc_port, rpc_user, rpc_pass, randomx_fast_mode
- Returns: Success message
- Stores process handle in `NodeState`

**`stop_node`**
- Kills junocashd process
- Returns: Status message
- Called on app exit

**`get_default_juno_paths`**
- Returns platform-specific default paths
- Used by: Settings page initialization

## RPC Client Implementation

Located in `src-tauri/src/rpc.rs`:

```rust
pub async fn call_rpc(
    method: &str,
    params: Vec<Value>,
    port: u16,
    user: &str,
    pass: &str,
) -> Result<Value, String>
```

**How it works:**
1. Constructs JSON-RPC request with id, method, params
2. Uses HTTP Basic Auth with RPC credentials
3. POSTs to `http://{host}:{port}` (configurable host)
4. Parses response, extracts result or error
5. Returns result or error string

**Error handling:**
- Network errors (node not running)
- Auth errors (wrong credentials)
- RPC errors (invalid params, insufficient funds)
- JSON parsing errors

## Process Management

The wallet can launch and manage the junocashd process.

**State tracking:**
```rust
pub struct NodeState {
    pub process: Mutex<Option<Child>>, // Child process handle
}
```

**Lifecycle:**
1. User clicks "Launch Node" in Settings
2. `launch_node` command spawns junocashd with flags
3. Process handle stored in `NodeState`
4. Wallet keeps node running while open
5. On app exit, `RunEvent::ExitRequested` kills process

**Flags passed to junocashd:**
- `-datadir=<path>` - Data directory
- `-rpcport=<port>` - RPC port
- `-rpcuser=<user>` - RPC username
- `-rpcpassword=<pass>` - RPC password
- `-daemon=0` - Don't daemonize (stay as child process)
- `-randomxfastmode` - Optional, if enabled in settings

## Security Considerations

### Credential Storage
- RPC credentials stored in browser `localStorage`
- OS handles encryption of localStorage
- Not as secure as native credential managers
- Acceptable for local-only connections

### Private Key Management
- Private keys NEVER leave junocashd
- Wallet only issues RPC commands
- No key material in wallet app
- Signing happens in daemon

### Network Security
- RPC listens on configurable host (default: localhost/127.0.0.1)
- No remote connections by default (when using localhost)
- HTTP Basic Auth required for all requests
- No TLS (not needed for local connections)

## Build Process

### Development
```bash
bun run tauri dev
```
- Starts Vite dev server for hot reload
- Launches Tauri in dev mode
- Opens DevTools automatically

### Production
```bash
bun run tauri build
```
- Builds Vue app with Vite
- Compiles Rust code with optimizations
- Creates platform-specific bundles
- Output in `src-tauri/target/release/bundle/`

### Platform Targets
- **Linux**: AppImage, .deb
- **Windows**: .msi, portable .exe
- **macOS**: .dmg, .app bundle

## Performance Optimizations

### Polling
- Mining stats: Every 5 seconds
- Balance/transactions: On-demand + manual refresh
- Block height: Every 10 seconds

### State Updates
- Reactive Vue updates (no manual DOM manipulation)
- Pinia computed properties for derived state
- Debounced input validation

### RPC Efficiency
- Batching not used (simple one-at-a-time)
- Could be improved with `batch` RPC calls
- Connection pooling handled by reqwest

## Extensibility

### Adding New RPC Commands

1. Add command in `src-tauri/src/commands/wallet.rs`:
```rust
#[command]
pub async fn your_command(
    param: String,
    port: u16,
    user: String,
    pass: String,
) -> Result<Value, String> {
    call_rpc("rpc_method", vec![json!(param)], port, &user, &pass).await
}
```

2. Register in `src-tauri/src/main.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    your_command,
    // ... other commands
])
```

3. Call from Vue:
```typescript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('your_command', {
    param: 'value',
    port: node.rpcPort,
    user: node.rpcUser,
    pass: node.rpcPass
});
```

### Adding New Pages

1. Create `src/views/YourPage.vue`
2. Add route in `src/router/index.ts`
3. Add nav link in `src/components/Sidebar.vue`

### Styling Guidelines

- Use TailwindCSS utility classes
- Glass morphism effects: `glass` custom class
- Color palette: Indigo/Purple for primary, Emerald for success
- Dark theme only (no light mode toggle)
- Animations: Tailwind's built-in + custom keyframes

## Testing Strategy

Currently no automated tests (open for contribution).

Suggested testing approach:
- **Unit tests**: Rust RPC client logic
- **Integration tests**: Tauri commands end-to-end
- **E2E tests**: Playwright or Tauri's testing tools
- **Manual testing**: Full user flows before releases

## Future Improvements

Potential enhancements:
- Multi-language support (i18n)
- Hardware wallet integration
- Address book
- QR code scanning
- Transaction export (CSV)
- Custom RPC port configuration
- Tor/VPN integration
- Mobile companion app
- Multi-wallet support
