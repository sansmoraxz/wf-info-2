# WF Info

Alternative Warframe companion app with Linux support.

This app is designed to run as a background daemon that monitors the Warframe process and provides an API for fetching inventory data, account information, and other game-related details. It can be used in conjunction with a CLI client or integrated into other applications.

> <font color="red">_Warning_</font>: DE has not officially granted permission to access Warframe's process, so use the `memory` feature with caution. It is used to load live inventory data using your account from DE's API but could potentially lead to risk of loosing access to your account. Use proper judgement and ensure you understand the implications of enabling this feature.
> 
> For inventory you may use other apps like [Overwolf's Allecaframe](https://www.overwolf.com/app/alejandro_cabrerizo-alecaframe), and load their exported inventory data into this app for filtering and querying without needing the `memory` feature.

This warning does not apply for other features like screenshot capture, log monitoring, etc. which uses standard OS APIs and does not interact with the game process directly.

## Building

```bash
cargo build --release
```

This produces two binaries:
- `wf-info-daemon` - The main daemon that monitors Warframe
- `wf-info-cli` - CLI client to interact with the daemon

## Usage (Linux)

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

### Option 2: Standalone Mode (Requires elevated permissions)

Run independently and monitor an already-running Warframe instance:

```bash
# With capabilities (recommended):
sudo setcap cap_sys_ptrace=eip ./target/release/wf-info-daemon
./target/release/wf-info-daemon

# Or with sudo:
sudo ./target/release/wf-info-daemon

# Or relax ptrace restrictions (security risk):
sudo sysctl kernel.yama.ptrace_scope=0
./target/release/wf-info-daemon
```

## Control API

The daemon exposes a line-delimited JSON protocol over TCP, Unix sockets, or Windows named pipes.

### Configuration

Set endpoint via CLI flags or environment variables:

| Flag | Environment Variable | Example |
|------|---------------------|---------|
| `--tcp` | `WF_INFO_API_TCP` | `127.0.0.1:47410` |
| `--unix` | `WF_INFO_API_UNIX` | `/tmp/wf-info-2.sock` |
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
| `inventory.refresh` | Refresh inventory from game API (with `memory` feature enabled) |
| `screenshot.trigger` | Capture and return a screenshot |

### Examples

**TCP:**
```bash
echo '{"id":1,"op":"ping"}' | nc 127.0.0.1 47410
echo '{"id":2,"op":"inventory.filter","params":{"category":"suits","contains":"prime","include_details":true}}' | nc 127.0.0.1 47410
```

**Unix socket:**
```bash
echo '{"id":1,"op":"ping"}' | socat - UNIX-CONNECT:/tmp/wf-info-2.sock
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
| `call` | Call any operation by name |

### Examples

```bash
# Ping the daemon
./target/release/wf-info-cli --tcp 127.0.0.1:47410 ping --pretty

# Load inventory from file
./target/release/wf-info-cli inventory-load --path testdata/pretty.json

# Filter inventory
./target/release/wf-info-cli inventory-filter --category suits --contains prime --include-details --limit 10

# Mark inventory as stale
./target/release/wf-info-cli inventory-stale --timestamp 1737840000 --reason "manual reset"

# Watch for events
./target/release/wf-info-cli watch --events account_login,inventory_fetched

# Trigger screenshot
./target/release/wf-info-cli screenshot
```

_**Note:** The screenshot will only do fullscreen capture, to avoid unintended consequences, ensure that the game window is active when capturing._

## Events

The daemon emits events that clients can subscribe to via the `subscribe` operation or `watch` command:

- `account_login` - Player account login detected
- `account_logout` - Player account logout detected
- `inventory_fetched` - Inventory loaded successfully
- `inventory_stale` - Inventory marked as stale
- `profile_updated` - Profile data updated
- `screenshot_triggered` - Screenshot captured

## Environment Variables

| Variable | Description |
|----------|-------------|
| `WF_INFO_API_TCP` | TCP endpoint for control API |
| `WF_INFO_API_UNIX` | Unix socket endpoint (Unix only) |
| `WF_INFO_API_NPIPE` | Named pipe endpoint (Windows only) |
| `WF_ITEMDATA_DIR` | Path to warframe-items-data JSON directory |
| `WARFRAME_APP_CONFIG` | Custom path to Warframe config directory |
| `RUST_LOG` | Logging level (e.g., `debug`, `info`) |
