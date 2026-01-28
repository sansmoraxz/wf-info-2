# WF Info

Alternative warframe companion app. Linux supported.
## Usage

### Option 1: Wrapper Mode (No sudo required with default kernel settings)

Run as a parent process that launches Warframe as a child. This allows memory access without sudo on most systems:

```bash
# Build the project
cargo build --release

# Launch Warframe through the wrapper
./target/release/wf-info-2 -- /path/to/Warframe.x64.exe [warframe args]
```

**For Steam:**
Set as launch options:
```
/path/to/wf-info-2 -- %command%
```

The monitor will automatically exit when Warframe closes.

### Option 2: Standalone Mode (Requires elevated permissions)

Run independently and monitor an already-running Warframe instance:

```bash
# With capabilities (recommended):
sudo setcap cap_sys_ptrace=eip ./target/release/wf-info-2
./target/release/wf-info-2

# Or with sudo:
sudo ./target/release/wf-info-2

# Or relax ptrace restrictions (security risk):
sudo sysctl kernel.yama.ptrace_scope=0
./target/release/wf-info-2
```

### Control API (optional)

The control API exposes a lightweight, line-delimited JSON protocol over TCP/Unix sockets/Windows named pipes.
Enable it by setting one or more environment variables:

```
WF_INFO_API_TCP=127.0.0.1:47410
WF_INFO_API_UNIX=/tmp/wf-info-2.sock
WF_INFO_API_NPIPE=wf-info-2-control
```

Defaults (when no env vars are set):
- Linux/macOS: unix socket at `${XDG_RUNTIME_DIR}/wf-info-2/control.sock` (falls back to cache/data dirs)
- Windows: named pipe `\\.\pipe\wf-info-2-control`
- Other platforms: TCP `127.0.0.1:47410`

Requests are JSON objects terminated by a newline. Responses are JSON objects with `ok`/`error`.

Supported operations:
- `ping`
- `inventory.load` (manual load of inventory data)
- `inventory.filter` (filter/view inventory items)
- `inventory.meta.get`
- `inventory.stale.update` (mark inventory as stale)
- `screenshot.trigger` (record a screenshot event)

Example (TCP):
```
echo '{"id":1,"op":"inventory.load","params":{"path":"testdata/pretty.json"}}' | nc 127.0.0.1 47410
echo '{"id":2,"op":"inventory.filter","params":{"category":"suits","contains":"prime","include_details":true}}' | nc 127.0.0.1 47410
```

Example (Unix socket):
```
echo '{"id":1,"op":"ping"}' | socat - UNIX-CONNECT:/tmp/wf-info-2.sock
```

Example (Windows named pipe):
```
echo {\"id\":1,\"op\":\"ping\"} | ncat --exec \"cmd /c type con\" --no-shutdown \\\\.\\pipe\\wf-info-2-control
```

### CLI helper (bin)

Build the helper:
```
cargo build --bin wf-info-cli
```

Examples:
```
./target/debug/wf-info-cli --tcp 127.0.0.1:47410 ping --pretty
./target/debug/wf-info-cli --unix /tmp/wf-info-2.sock inventory-load --path testdata/pretty.json
./target/debug/wf-info-cli --npipe wf-info-2-control ping
./target/debug/wf-info-cli --tcp 127.0.0.1:47410 inventory-filter --category suits --contains prime --include-details --limit 10
./target/debug/wf-info-cli --tcp 127.0.0.1:47410 inventory-stale --timestamp 1737840000 --reason "manual reset"
```

Optional item details are pulled from `warframe-items-data/json`. Override the lookup path with
`WF_ITEMDATA_DIR`.
