# WF Info

Alternative Warframe companion app.

Yes it also works with both linux and windows.

This app is designed to run as a background daemon that monitors Warframe and provides an API for inventory data, account activity, and other game-related details. It can be used in conjunction with a CLI client or integrated into other applications.

> _WARNING_: DE has not officially granted permission to access Warframe's process, so use the `memory` feature with caution. It is used to load live inventory data using your account from DE's API but could potentially lead to risk of loosing access to your account. Use proper judgement and ensure you understand the implications of enabling this feature.
> 
> _NOTE_: The above warning does not apply if you don't build with the `memory` feature enabled.

The `memory` feature is optional. Without it, login/logout detection, trade watching, warframe.market usage, screenshots, fissure runs, and loading an exported inventory all continue to work. Live inventory refresh, account-ID resolution, and automatic profile refresh require `memory`, because current Warframe logs no longer expose the account ID. You may use another app such as [Overwolf's Allecaframe](https://www.overwolf.com/app/alejandro_cabrerizo-alecaframe) and load its exported inventory data instead. (The warning above still applies to Overwolf or any other third-party tool that touches inventory.)

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

Or with memory features enabled (needed for live inventory and profile refresh)

```bash
WF_PROFILE_KEY=change-me cargo build --release -p wf-info-daemon --features memory
```

## Daemon Usage (Linux)

Run the daemon as the parent process that launches Warframe as a child.

```bash
./target/release/wf-info-daemon -- /path/to/Warframe.x64.exe [warframe args]
```

**For Steam:** Set as launch options:
```
/path/to/wf-info-daemon -- %command%
```

The daemon will automatically exit when Warframe closes.

### Linux Log Transport

On Linux/Wine/Proton, the daemon captures live Warframe log lines from Wine's `OutputDebugString` debug channel (`warn+debugstr`). This avoids the multi-second `EE.log` flush delay and gives near-live event detection without requiring a separate helper process.

The daemon sets `WINEDEBUG=warn+debugstr` for the launched process automatically. So no need to set this value externally.

### Linux Memory Feature Notes

Most Linux distros prevent reading memory from unrelated processes unless they are child processes or the reader has additional permissions. The supported launch/wrapper mode above makes Warframe a child process of the daemon, which is the intended setup for the `memory` feature.

In unusual setups, such as mixed privilege levels or distro-specific ptrace restrictions, you may still need to set extra permissions:

```bash
sudo setcap cap_sys_ptrace=eip ./target/release/wf-info-daemon
```

Or relax ptrace restrictions (security risk, not recommended):

```bash
sudo sysctl kernel.yama.ptrace_scope=0
```

Or run the daemon with elevated permissions (also not recommended):

```bash
sudo ./target/release/wf-info-daemon -- /path/to/Warframe.x64.exe
```

## Daemon Usage (Windows)

Run the daemon as the parent process that launches Warframe:

```powershell
.\target\release\wf-info-daemon.exe -- "C:\Path\To\Warframe.x64.exe"
```

On native Windows, the daemon captures live log lines through the standard DBWIN / `OutputDebugString` monitor protocol. There are no special restrictions unless the game is launched elevated while the daemon is not.

## Control API

The daemon exposes a line-delimited JSON protocol over TCP, Unix sockets, or Windows named pipes.

### Configuration

Set endpoint and daemon behavior via CLI flags or environment variables:

| Flag | Environment Variable | Example |
|------|---------------------|---------|
| `--tcp` | `WF_INFO_API_TCP` | `127.0.0.1:47410` |
| `--unix` | `WF_INFO_API_UNIX` | `${XDG_RUNTIME_DIR}/wf-info-2/control.sock` |
| `--npipe` | `WF_INFO_API_NPIPE` | `wf-info-2-control` |
| `--native-wayland-screenshot` | `WF_INFO_SCREENSHOT_NATIVE_WAYLAND` | `true` |

`--native-wayland-screenshot` only works when the daemon is built with the `native-wayland-screenshot` Cargo feature.

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

> **Note:** Screenshot capture targets the Warframe window when possible and returns `image/bmp`. X11 and XWayland use the X11 screenshot API, while native Wayland uses the PipeWire screencast protocol.
>
> In a Wayland session, XWayland/X11 capture is preferred when Warframe is visible through XWayland. Start the daemon with `--native-wayland-screenshot` or set `WF_INFO_SCREENSHOT_NATIVE_WAYLAND=true` to force native Wayland/PipeWire capture instead.
>
> The first screenshot capture after launch may be slow as it has to setup the necessary infrastructure (e.g. PipeWire screencast stream), lookup the Warframe window, and so on. Subsequent captures should be much faster.
>
> Native Wayland capture must be built with the `native-wayland-screenshot` Cargo feature and requires a working `xdg-desktop-portal` ScreenCast backend, PipeWire, and the GStreamer PipeWire plugin (`pipewiresrc`). It will ask for screencapture permission on the first screenshot request, so make sure to allow it.

## Events

The daemon emits events that clients can subscribe to via the `subscribe` operation or `watch` command:

- `game_start` - Warframe game process detected
- `account_login` - Player account login detected
- `account_logout` - Player account logout detected
- `system_quit` - Warframe game process exited (`reason` is `requested` or `unexpected`)
- `inventory_fetched` - Inventory loaded successfully (requires a `memory` build)
- `inventory_stale` - Inventory marked as stale (requires a `memory` build)
- `profile_updated` - Profile data updated (requires a `memory` build)
- `screenshot_triggered` - Screenshot captured
- `dm_tab_opened` - New DM chat tab opened

## Environment Variables

| Variable | Description |
|----------|-------------|
| `WF_INFO_API_TCP` | TCP endpoint for control API |
| `WF_INFO_API_UNIX` | Unix socket endpoint (Unix only) |
| `WF_INFO_API_NPIPE` | Named pipe endpoint (Windows only) |
| `WF_INFO_SCREENSHOT_NATIVE_WAYLAND` | Force native Wayland/PipeWire screenshot capture in Wayland sessions instead of preferring XWayland/X11 |
| `WF_SKIP_AUTO_CALLBACK` | Skip callback events viz. market status set and auto inventory state updates |
| `WF_ITEM_DATA_BASE_URL` | Override the upstream base URL used to refresh cached item-data JSON files |
| `WARFRAME_APP_CONFIG` | Custom path to Warframe config directory |
| `RUST_LOG` | Logging level (e.g., `debug`, `info`) |

## Build-time Environment Variables

| Variable | Description |
|----------|-------------|
| `WF_PROFILE_KEY` | Required at build time; used as the encryption key source for cached profile/auth data |
