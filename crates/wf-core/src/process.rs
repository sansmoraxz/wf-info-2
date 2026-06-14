#[cfg(all(feature = "memory", target_os = "linux"))]
use anyhow::Context;
use std::collections::HashSet;
use std::time::Duration;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};
use tokio::time::sleep;

#[cfg(feature = "memory")]
use {anyhow::Result, memchr::memmem, std::collections::HashMap};

#[cfg(all(feature = "memory", target_os = "linux"))]
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

/// Checks if Warframe is the main game process (not launcher)
fn is_warframe_game_process(process: &sysinfo::Process) -> bool {
    let name_match = process
        .name()
        .to_string_lossy()
        .contains("Warframe.x64.exe");
    let cmd_match = process
        .cmd()
        .iter()
        .any(|arg| arg.to_string_lossy().contains("Warframe.x64.exe"));

    if name_match || cmd_match {
        // Exclude launcher which uses Preprocess.log
        let is_launcher = process
            .cmd()
            .iter()
            .any(|arg| arg.to_string_lossy().contains("Preprocess.log"));
        !is_launcher
    } else {
        false
    }
}

fn process_refresh_kind() -> ProcessRefreshKind {
    ProcessRefreshKind::nothing().with_cmd(UpdateKind::OnlyIfNotSet)
}

fn refresh_all_process_commands(system: &mut System) {
    system.refresh_processes_specifics(ProcessesToUpdate::All, true, process_refresh_kind());
}

fn refresh_process_command(system: &mut System, pid: sysinfo::Pid) {
    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        true,
        process_refresh_kind(),
    );
}

fn find_warframe_pid(system: &System) -> Option<u32> {
    system
        .processes()
        .values()
        .find(|p| is_warframe_game_process(p))
        .map(|p| p.pid().as_u32())
}

fn find_all_warframe_pids(system: &System) -> Vec<u32> {
    system
        .processes()
        .values()
        .filter(|p| is_warframe_game_process(p))
        .map(|p| p.pid().as_u32())
        .collect()
}

pub async fn wait_for_warframe_start() -> u32 {
    log::info!("Waiting for Warframe to start...");
    let mut system = System::new();

    loop {
        refresh_all_process_commands(&mut system);

        if let Some(pid) = find_warframe_pid(&system) {
            log::info!("Warframe process detected (PID: {}).", pid);
            return pid;
        }

        sleep(Duration::from_secs(5)).await;
    }
}

pub async fn wait_for_new_warframe_start(existing_pids: &HashSet<u32>) -> u32 {
    log::info!(
        "Waiting for launched Warframe game process; excluding existing PIDs: {:?}",
        existing_pids
    );
    let mut system = System::new();

    loop {
        refresh_all_process_commands(&mut system);

        if let Some(pid) = find_all_warframe_pids(&system)
            .into_iter()
            .find(|pid| !existing_pids.contains(pid))
        {
            log::info!("Launched Warframe game process detected (PID: {}).", pid);
            return pid;
        }

        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn wait_for_warframe_exit(pid: u32) {
    while is_warframe_pid(pid) {
        sleep(Duration::from_secs(1)).await;
    }
}

/// Finds the Warframe game process PID if running
pub fn get_warframe_pid() -> Option<u32> {
    let mut system = System::new();
    refresh_all_process_commands(&mut system);

    find_warframe_pid(&system)
}

pub fn get_all_warframe_pids() -> Vec<u32> {
    let mut system = System::new();
    refresh_all_process_commands(&mut system);

    find_all_warframe_pids(&system)
}

/// Checks whether a PID still belongs to the Warframe game process.
pub fn is_warframe_pid(pid: u32) -> bool {
    let pid = sysinfo::Pid::from_u32(pid);
    let mut system = System::new();
    refresh_process_command(&mut system, pid);

    system
        .process(pid)
        .map(is_warframe_game_process)
        .unwrap_or(false)
}

pub fn terminate_process(pid: u32) -> bool {
    let pid = sysinfo::Pid::from_u32(pid);
    let mut system = System::new();
    refresh_process_command(&mut system, pid);

    system
        .process(pid)
        .map(|process| process.kill())
        .unwrap_or(false)
}

/// Authorization query string containing accountId and nonce
#[derive(Debug, Clone)]
pub struct AuthQuery {
    pub account_id: String,
    pub nonce: String,
}

impl AuthQuery {
    /// Returns the full query string for API requests
    pub fn to_query_string(&self) -> String {
        format!("?accountId={}&nonce={}", self.account_id, self.nonce)
    }
}

/// Scans process memory for authorization data (accountId + nonce).
/// This reads /proc/{pid}/maps and /proc/{pid}/mem on Linux.
/// Requires appropriate permissions
#[cfg(all(feature = "memory", target_os = "linux"))]
pub fn scan_memory_for_auth(pid: u32, account_id: &str) -> Result<Option<AuthQuery>> {
    log::info!(
        "Scanning memory for auth data (PID: {}, accountId: {})",
        pid,
        account_id
    );

    // Needle: ?accountId=<id>&nonce= — followed by ASCII digits
    let needle = format!("?accountId={}&nonce=", account_id);
    let needle_bytes = needle.as_bytes();
    let finder = memmem::Finder::new(needle_bytes);

    // Read memory mappings
    let maps_path = format!("/proc/{}/maps", pid);
    let maps_file =
        File::open(&maps_path).context("Failed to open /proc/maps (try running with sudo)")?;
    let maps_reader = BufReader::new(maps_file);

    // Open process memory
    let mem_path = format!("/proc/{}/mem", pid);
    let mut mem_file =
        File::open(&mem_path).context("Failed to open /proc/mem (try running with sudo)")?;

    // Track candidates and their occurrence count (like the C++ version)
    let mut candidates: HashMap<String, u32> = HashMap::new();
    const REQUIRED_MATCHES: u32 = 3;

    // 4MB buffer for reading memory regions
    let mut buffer = vec![0u8; 4 * 1024 * 1024];

    for line in maps_reader.lines() {
        let line: String = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let range_str = parts[0];
        let perms = parts[1];

        // Only scan readable memory regions
        if !perms.contains('r') {
            continue;
        }

        // Parse address range (e.g., "7f0b5c000000-7f0b5c021000")
        let mut range_parts = range_str.split('-');
        let start_hex = range_parts.next().unwrap_or("0");
        let end_hex = range_parts.next().unwrap_or("0");

        let start = u64::from_str_radix(start_hex, 16).unwrap_or(0);
        let end = u64::from_str_radix(end_hex, 16).unwrap_or(0);
        let region_size = (end - start) as usize;

        // Skip empty or excessively large regions
        if region_size == 0 || region_size > 500 * 1024 * 1024 {
            continue;
        }

        // Read region in chunks
        let mut offset = 0usize;
        while offset < region_size {
            let chunk_size = std::cmp::min(buffer.len(), region_size - offset);
            let read_addr = start + offset as u64;

            if mem_file.seek(SeekFrom::Start(read_addr)).is_err() {
                break;
            }

            match mem_file.read(&mut buffer[..chunk_size]) {
                Ok(bytes_read) if bytes_read > 0 => {
                    let chunk = &buffer[..bytes_read];
                    let mut search_from = 0;
                    while let Some(pos) = finder.find(&chunk[search_from..]) {
                        let nonce_start = search_from + pos + needle_bytes.len();
                        let nonce_end = chunk[nonce_start..]
                            .iter()
                            .position(|b| !b.is_ascii_digit())
                            .map_or(bytes_read, |i| nonce_start + i);

                        if nonce_end > nonce_start {
                            let nonce =
                                String::from_utf8_lossy(&chunk[nonce_start..nonce_end]).to_string();
                            let auth_str = format!("{}:{}", account_id, nonce);

                            let count = candidates.entry(auth_str.clone()).or_insert(0);
                            *count += 1;

                            log::debug!("Found candidate auth (count={}): {}", count, auth_str);

                            if *count >= REQUIRED_MATCHES {
                                log::info!("Confirmed auth data after {} matches", count);
                                log::debug!("Auth data: accountId={}, nonce={}", account_id, nonce);
                                return Ok(Some(AuthQuery {
                                    account_id: account_id.to_string(),
                                    nonce,
                                }));
                            }
                        }
                        search_from += pos + needle_bytes.len();
                    }
                }
                _ => break,
            }

            offset += chunk_size;
        }
    }

    if candidates.is_empty() {
        log::warn!("No auth data found in process memory");
    } else {
        log::warn!(
            "Found {} candidate(s) but none confirmed (need {} matches)",
            candidates.len(),
            REQUIRED_MATCHES
        );
    }

    Ok(None)
}

/// Scans process memory for authorization data (accountId + nonce) on Windows.
/// Uses Windows API to enumerate and read process memory regions.
/// Requires appropriate process access rights (PROCESS_VM_READ | PROCESS_QUERY_INFORMATION)
#[cfg(all(feature = "memory", target_os = "windows"))]
pub fn scan_memory_for_auth(pid: u32, account_id: &str) -> Result<Option<AuthQuery>> {
    use winapi::shared::minwindef::{FALSE, LPVOID};
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::memoryapi::ReadProcessMemory;
    use winapi::um::memoryapi::VirtualQueryEx;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::winnt::{
        MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
        PAGE_READONLY, PAGE_READWRITE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
    };

    log::info!(
        "Scanning Windows memory for auth data (PID: {}, accountId: {})",
        pid,
        account_id
    );

    // Needle: ?accountId=<id>&nonce= — followed by ASCII digits
    let needle = format!("?accountId={}&nonce=", account_id);
    let needle_bytes = needle.as_bytes();
    let finder = memmem::Finder::new(needle_bytes);

    // Open process with read permissions
    let process_handle =
        unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, FALSE, pid) };

    if process_handle.is_null() {
        anyhow::bail!("Failed to open process (try running as Administrator)");
    }

    // Ensure handle is closed when we exit
    let _handle_guard = scopeguard::guard(process_handle, |handle| unsafe {
        CloseHandle(handle);
    });

    // Track candidates and their occurrence count
    let mut candidates: HashMap<String, u32> = HashMap::new();
    const REQUIRED_MATCHES: u32 = 3;

    // 4MB buffer for reading memory regions
    let mut buffer = vec![0u8; 4 * 1024 * 1024];
    let mut address: usize = 0;
    let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };

    // Enumerate memory regions
    loop {
        let result = unsafe {
            VirtualQueryEx(
                process_handle,
                address as LPVOID,
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };

        if result == 0 {
            break; // No more memory regions
        }

        // Check if region is committed and readable
        let is_committed = mbi.State == MEM_COMMIT;
        let is_readable = matches!(
            mbi.Protect,
            PAGE_READONLY | PAGE_READWRITE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE
        );

        if is_committed && is_readable {
            let region_size = mbi.RegionSize;
            let region_base = mbi.BaseAddress as usize;

            // Skip empty or excessively large regions
            if region_size > 0 && region_size <= 500 * 1024 * 1024 {
                // Read region in chunks
                let mut offset = 0usize;
                while offset < region_size {
                    let chunk_size = std::cmp::min(buffer.len(), region_size - offset);
                    let read_addr = (region_base + offset) as LPVOID;
                    let mut bytes_read: usize = 0;

                    let success = unsafe {
                        ReadProcessMemory(
                            process_handle,
                            read_addr,
                            buffer.as_mut_ptr() as LPVOID,
                            chunk_size,
                            &mut bytes_read,
                        )
                    };

                    if success != FALSE && bytes_read > 0 {
                        let chunk = &buffer[..bytes_read];
                        let mut search_from = 0;
                        while let Some(pos) = finder.find(&chunk[search_from..]) {
                            let nonce_start = search_from + pos + needle_bytes.len();
                            let nonce_end = chunk[nonce_start..]
                                .iter()
                                .position(|b| !b.is_ascii_digit())
                                .map_or(bytes_read, |i| nonce_start + i);

                            if nonce_end > nonce_start {
                                let nonce = String::from_utf8_lossy(&chunk[nonce_start..nonce_end])
                                    .to_string();
                                let auth_str = format!("{}:{}", account_id, nonce);

                                let count = candidates.entry(auth_str.clone()).or_insert(0);
                                *count += 1;

                                log::debug!("Found candidate auth (count={}): {}", count, auth_str);

                                if *count >= REQUIRED_MATCHES {
                                    log::info!("Confirmed auth data after {} matches", count);
                                    log::debug!(
                                        "Auth data: accountId={}, nonce={}",
                                        account_id,
                                        nonce
                                    );
                                    return Ok(Some(AuthQuery {
                                        account_id: account_id.to_string(),
                                        nonce,
                                    }));
                                }
                            }
                            search_from += pos + needle_bytes.len();
                        }
                    } else {
                        break; // Failed to read, move to next region
                    }

                    offset += chunk_size;
                }
            }
        }

        // Move to next region
        address = (mbi.BaseAddress as usize) + mbi.RegionSize;
    }

    if candidates.is_empty() {
        log::warn!("No auth data found in process memory");
    } else {
        log::warn!(
            "Found {} candidate(s) but none confirmed (need {} matches)",
            candidates.len(),
            REQUIRED_MATCHES
        );
    }

    Ok(None)
}

/// Scans process memory for authorization data - stub for unsupported platforms
#[cfg(all(
    feature = "memory",
    not(any(target_os = "linux", target_os = "windows"))
))]
pub fn scan_memory_for_auth(_pid: u32, _account_id: &str) -> Result<Option<AuthQuery>> {
    anyhow::bail!("Memory scanning is not supported on this platform")
}

/// Attempts to extract auth data with retries, waiting for it to appear in memory
#[cfg(feature = "memory")]
pub async fn scan_memory_for_auth_with_retry(
    pid: u32,
    account_id: &str,
    max_retries: u32,
    retry_delay: Duration,
) -> Result<Option<AuthQuery>> {
    for attempt in 1..=max_retries {
        log::info!("Memory scan attempt {}/{}", attempt, max_retries);

        match scan_memory_for_auth(pid, account_id) {
            Ok(Some(auth)) => return Ok(Some(auth)),
            Ok(None) => {
                if attempt < max_retries {
                    log::info!("Auth not found, retrying in {:?}...", retry_delay);
                    sleep(retry_delay).await;
                }
            }
            Err(e) => {
                log::error!("Memory scan error: {}", e);
                return Err(e);
            }
        }
    }

    Ok(None)
}
