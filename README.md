# WF Info

Alternative Warframe companion app.

Yes it also works with both linux and windows.

This app is designed to run as a background daemon that monitors the Warframe process and provides an API for fetching inventory data, account information, and other game-related details. It can be used in conjunction with a CLI client or integrated into other applications.

> _WARNING_: DE has not officially granted permission to access Warframe's process, so use the `memory` feature with caution. It is used to load live inventory data using your account from DE's API but could potentially lead to risk of loosing access to your account. Use proper judgement and ensure you understand the implications of enabling this feature.
> 
> _NOTE_: The above warning does not apply if you don't build with the `memory` feature enabled.

It's not necessary to have the `memory` feature to use this tool, all it provides is just some additional APIs (mentioned below). If you feel like you don't want to risk your account, but still use this tool you may skip it (only your actual inventory tracking via this tool will be unavailable not trade watch, nor warframe market usage, nor screenshots or fissure runs). In fact you may use other apps like [Overwolf's Allecaframe](https://www.overwolf.com/app/alejandro_cabrerizo-alecaframe), and load their exported inventory data. (Please note that the above warning still apply for Overwolf or any other third party tool that touches inventory)

## Building

```bash
WF_PROFILE_KEY=change-me cargo build --release --workspace
```

The build produces two user-facing binaries:
- `wf-info-daemon` - The main daemon that monitors Warframe
- `wf-info-cli` - CLI client to interact with the daemon

To build only the binaries:

```bash
WF_PROFILE_KEY=change-me cargo build --release -p wf-info-daemon -p wf-info-cli
```

Or with memory features enabled (needed for live inventory)

```bash
WF_PROFILE_KEY=change-me cargo build --release -p wf-info-daemon --features memory
```

## Daemon Usage (Linux)

### Option 1: Wrapper Mode (No sudo required with default kernel settings)

Run as a parent process that launches Warframe as a child. This allows the daemon to automatically monitor the Warframe process without needing elevated permissions.

```bash
./target/release/wf-info-daemon -- /path/to/Warframe.x64.exe [warframe args]
```

**For Steam:** Set as launch options:
```
/path/to/wf-info-daemon -- %command%
```

The daemon will automatically exit when Warframe closes.

### Option 2: Standalone Mode

Run independently and monitor an already-running Warframe instance:

```bash
./target/release/wf-info-daemon
```

Please note that most linux distros prevent reading memory from another process unless it's a child process. So for the memory feature you have options:-

```bash
# With capabilities (recommended):
sudo setcap cap_sys_ptrace=eip ./target/release/wf-info-daemon
./target/release/wf-info-daemon

# Or with sudo:
sudo ./target/release/wf-info-daemon

# Or relax ptrace restrictions (security risk, not recommended):
sudo sysctl kernel.yama.ptrace_scope=0
./target/release/wf-info-daemon
```

## Daemon Usage (Windows)

As above. There's no special restrictions that prevent reading the game data unless it's running with elevated privileges.

## Control API

The daemon exposes a line-delimited JSON protocol over TCP, Unix sockets, or Windows named pipes.

### Configuration

Set endpoint via CLI flags or environment variables:

| Flag | Environment Variable | Example |
|------|---------------------|---------|
| `--tcp` | `WF_INFO_API_TCP` | `127.0.0.1:47410` |
| `--unix` | `WF_INFO_API_UNIX` | `${XDG_RUNTIME_DIR}/wf-info-2/control.sock` |
| `--npipe` | `WF_INFO_API_NPIPE` | `wf-info-2-control` |

**Defaults (when no options are set):**
- Linux/macOS: Unix socket at `${XDG_RUNTIME_DIR}/wf-info-2/control.sock`
- Windows: Named pipe `\\.\pipe\wf-info-2-control`
- Other platforms: TCP `127.0.0.1:47410`

### Supported Operations

| Operation | Description |
|-----------|-------------|
| `ping` | Health check |
| `subscribe` | Subscribe to daemon events (streaming) |
| `inventory.load` | Load inventory from file or JSON |
| `inventory.filter` | Filter and search inventory items |
| `inventory.meta.get` | Get inventory metadata |
| `inventory.stale.update` | Mark inventory as stale |
| `inventory.refresh` | Refresh inventory from game API (only available with `memory` feature) |
| `screenshot.trigger` | Capture and return a screenshot |
| `wfm.price` | Get live warframe.market prices for an item (with set part breakdown) |
| `wfm.refresh` | Force refresh the warframe.market item cache |
| `wfm.signin` | Sign in to warframe.market (email/password) |
| `wfm.signout` | Sign out from warframe.market |
| `wfm.signstatus` | Get/set warframe.market online status |

### Examples

**TCP:**
```bash
echo '{"id":1,"op":"ping"}' | nc 127.0.0.1 47410
echo '{"id":2,"op":"inventory.filter","params":{"category":"suits","contains":"prime","include_details":true}}' | nc 127.0.0.1 47410
```

**Unix socket:**
```bash
echo '{"id":1,"op":"ping"}' | socat - UNIX-CONNECT:${XDG_RUNTIME_DIR}/wf-info-2/control.sock
```

**Windows named pipe:**
```powershell
echo {"id":1,"op":"ping"} | ncat --exec "cmd /c type con" --no-shutdown \\.\pipe\wf-info-2-control
```

## CLI Client

The `wf-info-cli` binary provides a convenient interface to the daemon.

### Commands

| Command | Description |
|---------|-------------|
| `ping` | Ping the daemon |
| `watch` | Subscribe to events (streaming) |
| `inventory-load` | Load inventory data |
| `inventory-filter` | Filter inventory items |
| `inventory-meta` | Get inventory metadata |
| `inventory-stale` | Mark inventory as stale |
| `inventory-refresh` | Refresh inventory from game |
| `screenshot` | Trigger screenshot capture |
| `wfm-price` | Get live warframe.market prices for an item |
| `wfm-refresh` | Force refresh warframe.market item cache |
| `wfm-signin` | Sign in to warframe.market |
| `wfm-signout` | Sign out from warframe.market |
| `wfm-status` | Check/set warframe.market auth status |
| `call` | Call any operation by name |

### Examples

```bash
# Ping the daemon
./target/release/wf-info-cli --tcp 127.0.0.1:47410 ping --pretty

# Load inventory from file
./target/release/wf-info-cli inventory-load --path testdata/inventory/sample_inventory.json

# Filter inventory
./target/release/wf-info-cli inventory-filter --category suits --contains prime --include-details --limit 10

# Mark inventory as stale
./target/release/wf-info-cli inventory-stale --timestamp 1737840000 --reason "manual reset"

# Watch for events
./target/release/wf-info-cli watch --events account_login,inventory_fetched

# Trigger screenshot
./target/release/wf-info-cli screenshot

# Get market prices for an item (includes set parts and inventory counts)
./target/release/wf-info-cli wfm-price --search "frost prime" --pretty
./target/release/wf-info-cli wfm-price --item-type "/Lotus/Powersuits/Frost/FrostPrime" --pretty

# Force refresh warframe.market cache
./target/release/wf-info-cli wfm-refresh --pretty

# Filter inventory with market price data
./target/release/wf-info-cli inventory-filter --category suits --contains prime --include-details --include-market --pretty

# Sign in to warframe.market
./target/release/wf-info-cli wfm-signin --email user@example.com --password mypassword

# Check auth status and token validity
./target/release/wf-info-cli wfm-status --pretty

# Set warframe.market online status
./target/release/wf-info-cli wfm-status --status online

# Sign out from warframe.market
./target/release/wf-info-cli wfm-signout
```

_**Note:** Screenshot capture targets the Warframe window when possible and returns `image/bmp`. X11 and XWayland use the X11 screenshot API, while native Wayland uses the PipeWire screencast protocol. Although pure wayland is supported, due to overheads I would recommend using X11 or XWayland if you want to use the screenshot feature to be optimally performant._

## Events

The daemon emits events that clients can subscribe to via the `subscribe` operation or `watch` command:

- `account_login` - Player account login detected
- `account_logout` - Player account logout detected
- `inventory_fetched` - Inventory loaded successfully
- `inventory_stale` - Inventory marked as stale
- `profile_updated` - Profile data updated
- `screenshot_triggered` - Screenshot captured
- `dm_tab_opened` - New DM chat tab opened

## Environment Variables

| Variable | Description |
|----------|-------------|
| `WF_INFO_API_TCP` | TCP endpoint for control API |
| `WF_INFO_API_UNIX` | Unix socket endpoint (Unix only) |
| `WF_INFO_API_NPIPE` | Named pipe endpoint (Windows only) |
| `WF_SKIP_AUTO_CALLBACK` | Skip callback events viz. market status set and auto inventory state updates |
| `WF_ITEM_DATA_BASE_URL` | Override the upstream base URL used to refresh cached item-data JSON files |
| `WARFRAME_APP_CONFIG` | Custom path to Warframe config directory |
| `RUST_LOG` | Logging level (e.g., `debug`, `info`) |

## Build-time Environment Variables

| Variable | Description |
|----------|-------------|
| `WF_PROFILE_KEY` | Required at build time; used as the encryption key source for cached profile/auth data |
