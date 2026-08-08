//! Windows `DBWIN_BUFFER` (`OutputDebugString`) capture primitives shared by
//! the native daemon and the wine-prefix bridge helper.

#[cfg(windows)]
mod monitor {
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
    // winapi gates this behind its "winerror" feature; not worth enabling
    // for a single constant.
    const WAIT_TIMEOUT: u32 = 258;

    pub struct DbwinMonitor {
        mapping: HANDLE,
        view: *mut u8,
        buffer_ready: HANDLE,
        data_ready: HANDLE,
    }

    unsafe impl Send for DbwinMonitor {}

    impl DbwinMonitor {
        pub fn new() -> io::Result<Self> {
            let buffer_size = u32::try_from(DBWIN_BUFFER_SIZE)
                .map_err(|_| io::Error::other("DBWIN buffer size exceeds u32 range"))?;

            unsafe {
                let mapping = CreateFileMappingA(
                    INVALID_HANDLE_VALUE,
                    null_mut(),
                    PAGE_READWRITE,
                    0,
                    buffer_size,
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

        pub fn wait_for_message(&mut self, timeout: Duration) -> io::Result<Option<DbwinFrame>> {
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

#[cfg(windows)]
pub use monitor::DbwinMonitor;

pub const DBWIN_BUFFER_SIZE: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbwinFrame {
    pub pid: u32,
    pub text: String,
}

#[must_use]
pub fn decode_dbwin_frame(buffer: &[u8]) -> Option<DbwinFrame> {
    let (pid_bytes, payload) = buffer.split_at_checked(size_of::<u32>())?;
    let pid = u32::from_le_bytes(pid_bytes.try_into().ok()?);
    let end = payload
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(payload.len());

    let text = String::from_utf8_lossy(payload.get(..end)?).into_owned();
    if text.is_empty() {
        return None;
    }

    Some(DbwinFrame { pid, text })
}

#[cfg(test)]
mod tests {
    use super::decode_dbwin_frame;

    #[test]
    fn decode_dbwin_frame_reads_pid_and_message() {
        let mut frame = Vec::new();
        frame.extend_from_slice(&1234_u32.to_le_bytes());
        frame.extend_from_slice(b"72.458 Sys [Info]: Logged in sample_account\0ignored");

        let decoded = decode_dbwin_frame(&frame).unwrap();
        assert_eq!(decoded.pid, 1234, "pid should be decoded from LE prefix");
        assert_eq!(
            decoded.text, "72.458 Sys [Info]: Logged in sample_account",
            "text should stop at the NUL terminator"
        );
    }

    #[test]
    fn decode_dbwin_frame_rejects_empty_or_truncated_payloads() {
        assert!(decode_dbwin_frame(&[]).is_none(), "empty buffer");
        assert!(decode_dbwin_frame(&[1, 2, 3]).is_none(), "short buffer");

        let mut frame = Vec::new();
        frame.extend_from_slice(&77_u32.to_le_bytes());
        frame.push(0);
        assert!(decode_dbwin_frame(&frame).is_none(), "empty payload");
    }
}
