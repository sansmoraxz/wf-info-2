use std::future::Future;
use std::io;
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
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader, Lines};
#[cfg(windows)]
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};

#[cfg(windows)]
const DBWIN_BUFFER_SIZE: usize = 4096;
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
        let complete = std::mem::take(&mut self.pending);
        self.pending = trailing;
        complete
    }

    pub fn pending_fragment(&self) -> &str {
        &self.pending
    }
}

#[cfg(unix)]
pub struct WineDebugLogSource<R> {
    lines: Lines<BufReader<R>>,
}

#[cfg(unix)]
impl<R> WineDebugLogSource<R>
where
    R: AsyncRead + Unpin + Send,
{
    pub fn new(reader: R) -> Self {
        Self {
            lines: BufReader::new(reader).lines(),
        }
    }
}

#[cfg(unix)]
impl<R> LogSource for WineDebugLogSource<R>
where
    R: AsyncRead + Unpin + Send,
{
    fn recv_chunk(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = io::Result<Option<String>>> + Send + '_>> {
        Box::pin(async move {
            while let Some(line) = self.lines.next_line().await? {
                log::trace!("wine debug stderr raw line: {line:?}");
                if let Some(message) = decode_wine_debug_line(&line) {
                    log::debug!("wine debugstr accepted payload: {message:?}");
                    return Ok(Some(message));
                }
                log::trace!("wine debug stderr ignored line");
            }

            log::debug!("wine debug stderr source reached EOF");
            Ok(None)
        })
    }
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
        let mut monitor = platform::DbwinMonitor::new()?;
        let (sender, receiver) = unbounded_channel();
        let shutdown = Arc::new(AtomicBool::new(false));
        let worker_shutdown = Arc::clone(&shutdown);

        let worker = thread::Builder::new()
            .name("dbwin-log-source".to_string())
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
                            let _ = sender.send(Err(error));
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
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
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

#[cfg(windows)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct DbwinFrame {
    pid: u32,
    text: String,
}

#[cfg(windows)]
fn decode_dbwin_frame(buffer: &[u8]) -> Option<DbwinFrame> {
    if buffer.len() < std::mem::size_of::<u32>() {
        return None;
    }

    let pid = u32::from_le_bytes(buffer[..4].try_into().ok()?);
    let payload = &buffer[4..];
    let end = payload
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(payload.len());
    if end == 0 {
        return None;
    }

    let text = String::from_utf8_lossy(&payload[..end]).into_owned();
    if text.is_empty() {
        return None;
    }

    Some(DbwinFrame { pid, text })
}

#[cfg(unix)]
fn decode_wine_debug_line(line: &str) -> Option<String> {
    const ANSI_MARKER: &str = "warn:debugstr:OutputDebugStringA ";
    const WIDE_MARKER: &str = "warn:debugstr:OutputDebugStringW ";

    if let Some((_, payload)) = line.split_once(ANSI_MARKER) {
        log::trace!("matched OutputDebugStringA line");
        return decode_wine_debug_payload(payload);
    }

    if let Some((_, payload)) = line.split_once(WIDE_MARKER) {
        log::trace!("matched OutputDebugStringW line");
        return decode_wine_debug_payload(payload);
    }

    None
}

#[cfg(unix)]
fn decode_wine_debug_payload(payload: &str) -> Option<String> {
    let payload = payload.strip_prefix('L').unwrap_or(payload);
    let payload = payload.strip_prefix('"')?;

    let mut decoded = Vec::new();
    let mut chars = payload.chars();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => return Some(String::from_utf8_lossy(&decoded).into_owned()),
            '\\' => {
                let escaped = chars.next()?;
                match escaped {
                    '\\' => decoded.push(b'\\'),
                    '"' => decoded.push(b'"'),
                    '\'' => decoded.push(b'\''),
                    'a' => decoded.push(0x07),
                    'b' => decoded.push(0x08),
                    'f' => decoded.push(0x0c),
                    'n' => decoded.push(b'\n'),
                    'r' => decoded.push(b'\r'),
                    't' => decoded.push(b'\t'),
                    'v' => decoded.push(0x0b),
                    'x' => {
                        let hi = chars.next()?;
                        let lo = chars.next()?;
                        let value = u8::from_str_radix(&format!("{hi}{lo}"), 16).ok()?;
                        decoded.push(value);
                    }
                    other => {
                        let mut buf = [0u8; 4];
                        decoded.extend_from_slice(other.encode_utf8(&mut buf).as_bytes());
                    }
                }
            }
            other => {
                let mut buf = [0u8; 4];
                decoded.extend_from_slice(other.encode_utf8(&mut buf).as_bytes());
            }
        }
    }

    log::debug!("failed to decode wine debug payload: {payload:?}");
    None
}

#[cfg(windows)]
mod platform {
    use super::{DBWIN_BUFFER_SIZE, DbwinFrame, decode_dbwin_frame};
    use std::io;
    use std::ptr::null_mut;
    use std::slice;
    use std::time::Duration;

    use winapi::ctypes::c_void;
    use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
    use winapi::um::memoryapi::{FILE_MAP_READ, MapViewOfFile, UnmapViewOfFile};
    use winapi::um::synchapi::{CreateEventA, SetEvent, WaitForSingleObject};
    use winapi::um::winbase::{CreateFileMappingA, WAIT_OBJECT_0};
    use winapi::um::winnt::{HANDLE, PAGE_READWRITE};

    const DBWIN_BUFFER_NAME: &[u8] = b"DBWIN_BUFFER\0";
    const DBWIN_BUFFER_READY_NAME: &[u8] = b"DBWIN_BUFFER_READY\0";
    const DBWIN_DATA_READY_NAME: &[u8] = b"DBWIN_DATA_READY\0";
    const WAIT_TIMEOUT: u32 = 258;

    pub(super) struct DbwinMonitor {
        mapping: HANDLE,
        view: *mut u8,
        buffer_ready: HANDLE,
        data_ready: HANDLE,
    }

    unsafe impl Send for DbwinMonitor {}

    impl DbwinMonitor {
        pub(super) fn new() -> io::Result<Self> {
            unsafe {
                let mapping = CreateFileMappingA(
                    INVALID_HANDLE_VALUE,
                    null_mut(),
                    PAGE_READWRITE,
                    0,
                    DBWIN_BUFFER_SIZE as u32,
                    DBWIN_BUFFER_NAME.as_ptr().cast(),
                );
                if mapping.is_null() {
                    return Err(io::Error::last_os_error());
                }

                let view = MapViewOfFile(mapping, FILE_MAP_READ, 0, 0, DBWIN_BUFFER_SIZE);
                if view.is_null() {
                    CloseHandle(mapping);
                    return Err(io::Error::last_os_error());
                }

                let buffer_ready =
                    CreateEventA(null_mut(), 0, 0, DBWIN_BUFFER_READY_NAME.as_ptr().cast());
                if buffer_ready.is_null() {
                    UnmapViewOfFile(view);
                    CloseHandle(mapping);
                    return Err(io::Error::last_os_error());
                }

                let data_ready =
                    CreateEventA(null_mut(), 0, 0, DBWIN_DATA_READY_NAME.as_ptr().cast());
                if data_ready.is_null() {
                    CloseHandle(buffer_ready);
                    UnmapViewOfFile(view);
                    CloseHandle(mapping);
                    return Err(io::Error::last_os_error());
                }

                if SetEvent(buffer_ready) == 0 {
                    CloseHandle(data_ready);
                    CloseHandle(buffer_ready);
                    UnmapViewOfFile(view);
                    CloseHandle(mapping);
                    return Err(io::Error::last_os_error());
                }

                Ok(Self {
                    mapping,
                    view: view.cast(),
                    buffer_ready,
                    data_ready,
                })
            }
        }

        pub(super) fn wait_for_message(
            &mut self,
            timeout: Duration,
        ) -> io::Result<Option<DbwinFrame>> {
            let millis = u32::try_from(timeout.as_millis()).unwrap_or(u32::MAX);

            unsafe {
                match WaitForSingleObject(self.data_ready, millis) {
                    WAIT_OBJECT_0 => {
                        let raw = slice::from_raw_parts(self.view.cast::<u8>(), DBWIN_BUFFER_SIZE);
                        let frame = decode_dbwin_frame(raw);
                        if SetEvent(self.buffer_ready) == 0 {
                            return Err(io::Error::last_os_error());
                        }
                        Ok(frame)
                    }
                    WAIT_TIMEOUT => Ok(None),
                    _ => Err(io::Error::last_os_error()),
                }
            }
        }
    }

    impl Drop for DbwinMonitor {
        fn drop(&mut self) {
            unsafe {
                CloseHandle(self.data_ready);
                CloseHandle(self.buffer_ready);
                UnmapViewOfFile(self.view.cast::<c_void>());
                CloseHandle(self.mapping);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LineAssembler;
    #[cfg(windows)]
    use super::decode_dbwin_frame;
    #[cfg(unix)]
    use super::decode_wine_debug_line;

    #[cfg(windows)]
    #[test]
    fn decode_dbwin_frame_reads_pid_and_message() {
        let mut frame = Vec::new();
        frame.extend_from_slice(&1234u32.to_le_bytes());
        frame.extend_from_slice(b"72.458 Sys [Info]: Logged in sample_account\0ignored");

        let decoded = decode_dbwin_frame(&frame).unwrap();
        assert_eq!(decoded.pid, 1234);
        assert_eq!(decoded.text, "72.458 Sys [Info]: Logged in sample_account");
    }

    #[cfg(windows)]
    #[test]
    fn decode_dbwin_frame_rejects_empty_or_truncated_payloads() {
        assert!(decode_dbwin_frame(&[]).is_none());
        assert!(decode_dbwin_frame(&[1, 2, 3]).is_none());

        let mut frame = Vec::new();
        frame.extend_from_slice(&77u32.to_le_bytes());
        frame.push(0);
        assert!(decode_dbwin_frame(&frame).is_none());
    }

    #[cfg(unix)]
    #[test]
    fn decode_wine_debug_line_reads_outputdebugstring_payload() {
        let line = r#"34924.788:00c4:00c8:warn:debugstr:OutputDebugStringA "72.458 Sys [Info]: Logged in Jasper123\r\n""#;
        assert_eq!(
            decode_wine_debug_line(line).unwrap(),
            "72.458 Sys [Info]: Logged in Jasper123\r\n"
        );
    }

    #[cfg(unix)]
    #[test]
    fn decode_wine_debug_line_handles_wide_prefix_and_hex_escapes() {
        let line = r#"0024:warn:debugstr:OutputDebugStringW L"Player name changed to Jasper123\xee\x80\x80 Clan: TestC#963\r\n""#;
        let decoded = decode_wine_debug_line(line).unwrap();
        assert!(decoded.contains("Player name changed to Jasper123"));
        assert!(decoded.ends_with("Clan: TestC#963\r\n"));
    }

    #[cfg(unix)]
    #[test]
    fn decode_wine_debug_line_ignores_non_debugstr_noise() {
        assert!(decode_wine_debug_line("ntsync: up and running.").is_none());
        assert!(decode_wine_debug_line("fixme:heap:RtlSetHeapInformation stub").is_none());
    }

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
