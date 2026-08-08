use std::future::Future;
use std::io;
use std::mem;
use std::pin::Pin;
#[cfg(windows)]
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
#[cfg(windows)]
use std::thread::{self, JoinHandle};
#[cfg(windows)]
use std::time::Duration;

#[cfg(unix)]
use std::env;
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt as _;
#[cfg(unix)]
use std::path::PathBuf;
#[cfg(unix)]
use std::process::{self, Stdio};

#[cfg(unix)]
use tokio::io::{AsyncBufReadExt as _, AsyncReadExt as _, BufReader};
#[cfg(unix)]
use tokio::process::{Child, Command};
#[cfg(windows)]
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};
#[cfg(windows)]
use wf_dbwin::{DbwinFrame, DbwinMonitor};

#[cfg(unix)]
use crate::wine::WineContext;

#[cfg(windows)]
const DBWIN_POLL_INTERVAL: Duration = Duration::from_millis(250);

pub trait LogSource: Send {
    fn recv_chunk(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<String>>> + Send + '_>>;
}

#[derive(Debug, Default)]
pub struct LineAssembler {
    pending: String,
}

impl LineAssembler {
    pub fn push_chunk(&mut self, chunk: &str) -> String {
        self.pending.push_str(chunk);

        let Some(last_newline) = self.pending.rfind('\n') else {
            return String::new();
        };

        let split_at = last_newline + 1;
        let trailing = self.pending.split_off(split_at);
        let complete = mem::take(&mut self.pending);
        self.pending = trailing;
        complete
    }

    #[cfg(test)]
    pub(crate) fn pending_fragment(&self) -> &str {
        &self.pending
    }
}

/// Reads game logs by running the `wf-dbwin-bridge.exe` helper inside the
/// game's wine prefix and streaming its stdout. The bridge relays
/// `OutputDebugString` frames verbatim, so long lines arrive untruncated.
#[cfg(unix)]
pub struct WineDbwinBridgeSource {
    child: Child,
    buffer: Vec<u8>,
}

#[cfg(unix)]
impl WineDbwinBridgeSource {
    /// Launches the bridge helper in the same wine prefix as `game_pid`.
    ///
    /// `bridge_exe` is the embedded `wf-dbwin-bridge.exe` image, extracted to
    /// a runtime directory before launch. The `WF_DBWIN_BRIDGE` environment
    /// variable overrides it with an on-disk helper.
    pub fn spawn_for_game(game_pid: u32, bridge_exe: &[u8]) -> io::Result<Self> {
        let context = WineContext::for_pid(game_pid).map_err(io::Error::other)?;
        let bridge = bridge_exe_path(bridge_exe)?;

        log::info!(
            "Launching DBWIN bridge {} via {}",
            bridge.display(),
            context.wine_binary.display()
        );
        log::debug!("DBWIN bridge env: {:?}", context.env);

        let mut command = Command::new(&context.wine_binary);
        command
            .arg(&bridge)
            .envs(context.env)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let mut child = command.spawn()?;

        if let Some(stderr) = child.stderr.take() {
            tokio::spawn(async move {
                let mut lines = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    log::debug!("DBWIN bridge stderr: {line}");
                }
            });
        }

        Ok(Self {
            child,
            buffer: vec![0; wf_dbwin::DBWIN_BUFFER_SIZE],
        })
    }
}

#[cfg(unix)]
impl LogSource for WineDbwinBridgeSource {
    fn recv_chunk(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<String>>> + Send + '_>> {
        Box::pin(async move {
            let Some(stdout) = self.child.stdout.as_mut() else {
                return Err(io::Error::other("DBWIN bridge stdout not captured"));
            };

            loop {
                let read = stdout.read(&mut self.buffer).await?;
                if read == 0 {
                    let status = self.child.wait().await?;
                    log::info!("DBWIN bridge exited: {status}");
                    return Ok(None);
                }
                let chunk =
                    String::from_utf8_lossy(self.buffer.get(..read).unwrap_or_default())
                        .into_owned();
                if chunk.is_empty() {
                    continue;
                }
                log::trace!("DBWIN bridge chunk: {chunk:?}");
                return Ok(Some(chunk));
            }
        })
    }
}

/// Extracts the embedded bridge image to a runtime directory, skipping the
/// write when an identical file is already present from a previous run.
#[cfg(unix)]
fn bridge_exe_path(bridge_exe: &[u8]) -> io::Result<PathBuf> {
    if let Some(path) = env::var_os("WF_DBWIN_BRIDGE") {
        return Ok(PathBuf::from(path));
    }

    // XDG_RUNTIME_DIR is typically mounted noexec, and wine cannot map a PE
    // from a noexec filesystem — use the cache dir instead.
    let dir = env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
        .unwrap_or_else(env::temp_dir)
        .join("wf-info");
    fs::create_dir_all(&dir)?;
    let bridge = dir.join("wf-dbwin-bridge.exe");

    let up_to_date = fs::read(&bridge).is_ok_and(|existing| existing == bridge_exe);
    if !up_to_date {
        // Write-then-rename so a concurrent daemon (or a wine process still
        // mapping the old PE) never observes a partially written file.
        let staging = dir.join(format!("wf-dbwin-bridge.exe.{}", process::id()));
        fs::write(&staging, bridge_exe)?;
        // Wine maps unix permissions to NT execute access; without +x it
        // refuses to launch the PE and falls back to a failing ShellExecute.
        fs::set_permissions(&staging, fs::Permissions::from_mode(0o755))?;
        fs::rename(&staging, &bridge)?;
    }
    Ok(bridge)
}

#[cfg(windows)]
pub struct DbwinLogSource {
    pid_filter: Option<u32>,
    receiver: UnboundedReceiver<io::Result<DbwinFrame>>,
    shutdown: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

#[cfg(windows)]
impl DbwinLogSource {
    pub fn new() -> io::Result<Self> {
        let mut monitor = DbwinMonitor::new()?;
        let (sender, receiver) = unbounded_channel();
        let shutdown = Arc::new(AtomicBool::new(false));
        let worker_shutdown = Arc::clone(&shutdown);

        let worker = thread::Builder::new()
            .name("dbwin-log-source".to_owned())
            .spawn(move || {
                while !worker_shutdown.load(Ordering::Relaxed) {
                    match monitor.wait_for_message(DBWIN_POLL_INTERVAL) {
                        Ok(Some(frame)) => {
                            if sender.send(Ok(frame)).is_err() {
                                break;
                            }
                        }
                        Ok(None) => {}
                        Err(error) => {
                            if sender.send(Err(error)).is_err() {
                                log::debug!("dbwin receiver dropped before error delivery");
                            }
                            break;
                        }
                    }
                }
            })?;

        Ok(Self {
            pid_filter: None,
            receiver,
            shutdown,
            worker: Some(worker),
        })
    }

    pub fn set_pid_filter(&mut self, pid: u32) {
        self.pid_filter = Some(pid);
    }
}

#[cfg(windows)]
impl Drop for DbwinLogSource {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(worker) = self.worker.take()
            && worker.join().is_err()
        {
            log::warn!("dbwin log source worker thread panicked");
        }
    }
}

#[cfg(windows)]
impl LogSource for DbwinLogSource {
    fn recv_chunk(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<String>>> + Send + '_>> {
        Box::pin(async move {
            while let Some(message) = self.receiver.recv().await {
                let frame = message?;
                if self.pid_filter.is_some_and(|pid| frame.pid != pid) {
                    continue;
                }
                if frame.text.is_empty() {
                    continue;
                }
                let mut text = frame.text;
                if !text.ends_with('\n') {
                    text.push('\n');
                }
                return Ok(Some(text));
            }

            Ok(None)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LineAssembler;

    #[test]
    fn line_assembler_returns_complete_line() {
        let mut assembler = LineAssembler::default();
        assert_eq!(
            assembler.push_chunk("0.049 Sys [Diag]: Build Label\r\n"),
            "0.049 Sys [Diag]: Build Label\r\n"
        );
        assert_eq!(assembler.pending_fragment(), "");
    }

    #[test]
    fn line_assembler_reassembles_split_line() {
        let mut assembler = LineAssembler::default();
        assert_eq!(assembler.push_chunk("84.333 Sys [Info]: Player name"), "");
        assert_eq!(
            assembler.push_chunk(" changed to Jasper123\r\n"),
            "84.333 Sys [Info]: Player name changed to Jasper123\r\n"
        );
        assert_eq!(assembler.pending_fragment(), "");
    }

    #[test]
    fn line_assembler_handles_multiple_lines_and_retains_partial_tail() {
        let mut assembler = LineAssembler::default();
        let chunk = "0.049 Sys [Diag]: Build Label\r\n72.458 Sys [Info]: Logged in Jasper";
        assert_eq!(
            assembler.push_chunk(chunk),
            "0.049 Sys [Diag]: Build Label\r\n"
        );
        assert_eq!(
            assembler.pending_fragment(),
            "72.458 Sys [Info]: Logged in Jasper"
        );

        assert_eq!(
            assembler.push_chunk("123\r\n84.333 Sys [Info]: Profile hash"),
            "72.458 Sys [Info]: Logged in Jasper123\r\n"
        );
        assert_eq!(
            assembler.pending_fragment(),
            "84.333 Sys [Info]: Profile hash"
        );
    }
}
