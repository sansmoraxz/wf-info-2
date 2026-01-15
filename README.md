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
