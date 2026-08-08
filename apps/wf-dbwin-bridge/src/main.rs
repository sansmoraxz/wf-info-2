//! Relays `OutputDebugString` frames captured via `DBWIN_BUFFER` to stdout.
//! Runs inside a wine prefix so the host daemon can read untruncated game
//! logs from this process's stdout.

#[cfg(windows)]
use std::io;

#[cfg(windows)]
fn main() -> io::Result<()> {
    use std::io::{Write as _, stdout};
    use std::time::Duration;

    let mut monitor = wf_dbwin::DbwinMonitor::new()?;
    let stdout = stdout();

    loop {
        if let Some(frame) = monitor.wait_for_message(Duration::from_millis(250))? {
            // Verbatim, no added newline: a long line spans multiple DBWIN
            // frames and the host-side LineAssembler must see it unbroken.
            let mut out = stdout.lock();
            out.write_all(frame.text.as_bytes())?;
            out.flush()?;
        }
    }
}

#[cfg(not(windows))]
fn main() {
    use std::process::exit;

    eprintln!("wf-dbwin-bridge only runs on Windows (or inside a wine prefix)");
    exit(1);
}
