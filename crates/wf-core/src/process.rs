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

fn is_descendant_of(system: &System, pid: u32, ancestor_pid: u32) -> bool {
    let ancestor = sysinfo::Pid::from_u32(ancestor_pid);
    let mut current = sysinfo::Pid::from_u32(pid);
    // Bounded walk guards against parent-chain cycles from PID reuse.
    for _ in 0..64 {
        let Some(parent) = system.process(current).and_then(|p| p.parent()) else {
            return false;
        };
        if parent == ancestor {
            return true;
        }
        current = parent;
    }
    false
}

fn find_new_warframe_pid(
    system: &System,
    existing_pids: &HashSet<u32>,
    launcher_pid: u32,
) -> Option<u32> {
    let candidates: Vec<u32> = find_all_warframe_pids(system)
        .into_iter()
        .filter(|pid| !existing_pids.contains(pid))
        .collect();
    candidates
        .iter()
        .copied()
        .find(|pid| is_descendant_of(system, *pid, launcher_pid))
        // Wine/Steam can reparent the game process out of the launcher's
        // tree (e.g. under Steam's subreaper), so fall back to any new
        // Warframe game process.
        .or_else(|| candidates.first().copied())
}

pub async fn wait_for_new_warframe_start(existing_pids: &HashSet<u32>, launcher_pid: u32) -> u32 {
    log::info!(
        "Waiting for launched Warframe game process under launcher PID {}; excluding existing PIDs: {:?}",
        launcher_pid,
        existing_pids
    );
    let mut system = System::new();

    loop {
        refresh_all_process_commands(&mut system);

        if let Some(pid) = find_new_warframe_pid(&system, existing_pids, launcher_pid) {
            log::info!("Launched Warframe game process detected (PID: {}).", pid);
            return pid;
        }

        sleep(Duration::from_secs(1)).await;
    }
}

const DEFAULT_HANDOFF_GRACE: Duration = Duration::from_secs(10);

/// How long to keep scanning for a successor Warframe process after the
/// tracked one dies before declaring the game exited: the bootstrap
/// Warframe.x64.exe hands off to the real game process, and on slow systems
/// the successor may not be up yet. Tradeoff: a genuine quit is only reported
/// after this window. Tunable via WF_HANDOFF_GRACE_SECS.
pub fn handoff_grace() -> Duration {
    std::env::var("WF_HANDOFF_GRACE_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_HANDOFF_GRACE)
}

pub async fn wait_for_warframe_exit(pid: u32, launcher_pid: u32, existing_pids: &HashSet<u32>) {
    let mut tracked = pid;
    loop {
        while is_warframe_pid(tracked) {
            sleep(Duration::from_secs(1)).await;
        }

        let deadline = tokio::time::Instant::now() + handoff_grace();
        let successor = loop {
            let mut system = System::new();
            refresh_all_process_commands(&mut system);
            if let Some(next) = find_new_warframe_pid(&system, existing_pids, launcher_pid) {
                break Some(next);
            }
            if tokio::time::Instant::now() >= deadline {
                break None;
            }
            sleep(Duration::from_secs(1)).await;
        };

        match successor {
            Some(next) => {
                log::info!(
                    "Tracked Warframe PID {} exited; continuing with successor PID {}",
                    tracked,
                    next
                );
                tracked = next;
            }
            None => return,
        }
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthQuery {
    pub account_id: String,
    pub nonce: String,
}

#[cfg(feature = "memory")]
const AUTH_PREFIX: &[u8] = b"?accountId=";
#[cfg(feature = "memory")]
const NONCE_PREFIX: &[u8] = b"&nonce=";
#[cfg(feature = "memory")]
const ACCOUNT_ID_LEN: usize = 24;
#[cfg(feature = "memory")]
const REQUIRED_AUTH_ALLOCATIONS: u32 = 3;
#[cfg(feature = "memory")]
const CHUNK_OVERLAP: usize = 256;

/// Extracts valid authorization values from one readable allocation.
///
/// Warframe account IDs are 24-character hexadecimal object IDs. A nonce must
/// immediately follow the ID and contain at least one ASCII digit.
#[cfg(feature = "memory")]
fn auth_candidates_in_allocation(allocation: &[u8]) -> HashSet<AuthQuery> {
    auth_candidates_in_bytes(allocation, NonceBoundary::EndOfBytesTerminates)
}

/// Whether a nonce running up to the end of the byte slice counts as
/// terminated. A whole allocation is a natural boundary; a read chunk is not,
/// since the nonce may continue in the next chunk.
#[cfg(feature = "memory")]
#[derive(Clone, Copy, PartialEq, Eq)]
enum NonceBoundary {
    EndOfBytesTerminates,
    RequireTerminator,
}

#[cfg(feature = "memory")]
fn auth_candidates_in_bytes(allocation: &[u8], boundary: NonceBoundary) -> HashSet<AuthQuery> {
    let mut candidates = HashSet::new();
    let finder = memmem::Finder::new(AUTH_PREFIX);
    let mut search_from = 0;

    while let Some(relative_pos) = finder.find(&allocation[search_from..]) {
        let prefix_pos = search_from + relative_pos;
        let account_start = prefix_pos + AUTH_PREFIX.len();
        let account_end = account_start + ACCOUNT_ID_LEN;
        let nonce_prefix_end = account_end + NONCE_PREFIX.len();

        if nonce_prefix_end <= allocation.len() {
            let account_id = &allocation[account_start..account_end];
            let nonce_prefix = &allocation[account_end..nonce_prefix_end];

            if account_id.iter().all(u8::is_ascii_hexdigit) && nonce_prefix == NONCE_PREFIX {
                let nonce_start = nonce_prefix_end;
                let nonce_end = allocation[nonce_start..]
                    .iter()
                    .position(|byte| !byte.is_ascii_digit())
                    .map_or(allocation.len(), |offset| nonce_start + offset);

                let nonce_is_terminated =
                    nonce_end < allocation.len() || boundary == NonceBoundary::EndOfBytesTerminates;
                if nonce_end > nonce_start && nonce_is_terminated {
                    // Both slices have been validated as ASCII above.
                    let account_id = String::from_utf8_lossy(account_id).into_owned();
                    let nonce =
                        String::from_utf8_lossy(&allocation[nonce_start..nonce_end]).into_owned();
                    candidates.insert(AuthQuery { account_id, nonce });
                }
            }
        }

        search_from = prefix_pos + AUTH_PREFIX.len();
    }

    candidates
}

#[cfg(feature = "memory")]
#[derive(Default)]
struct AuthCandidateTracker {
    allocation_counts: HashMap<AuthQuery, u32>,
}

#[cfg(feature = "memory")]
impl AuthCandidateTracker {
    fn observe_allocation(
        &mut self,
        allocation_candidates: HashSet<AuthQuery>,
    ) -> Option<AuthQuery> {
        for candidate in allocation_candidates {
            let count = self.allocation_counts.entry(candidate.clone()).or_insert(0);
            *count += 1;
            log::debug!(
                "Found authorization candidate in {} readable allocation(s)",
                count
            );
            if *count >= REQUIRED_AUTH_ALLOCATIONS {
                return Some(candidate);
            }
        }
        None
    }
}

#[cfg(feature = "memory")]
fn add_chunk_candidates(
    allocation_candidates: &mut HashSet<AuthQuery>,
    previous_tail: &mut Vec<u8>,
    chunk: &[u8],
) {
    let mut searchable = Vec::with_capacity(previous_tail.len() + chunk.len());
    searchable.extend_from_slice(previous_tail);
    searchable.extend_from_slice(chunk);
    // A digit at the end of a read chunk may be only part of the nonce. It is
    // retained in `previous_tail` and accepted after a terminator is observed.
    allocation_candidates.extend(auth_candidates_in_bytes(
        &searchable,
        NonceBoundary::RequireTerminator,
    ));

    let tail_start = searchable.len().saturating_sub(CHUNK_OVERLAP);
    previous_tail.clear();
    previous_tail.extend_from_slice(&searchable[tail_start..]);
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
pub fn scan_memory_for_auth(pid: u32) -> Result<Option<AuthQuery>> {
    log::info!("Scanning memory for account authorization (PID: {})", pid);

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
    let mut tracker = AuthCandidateTracker::default();

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
        let mut allocation_candidates = HashSet::new();
        let mut previous_tail = Vec::new();
        while offset < region_size {
            let chunk_size = std::cmp::min(buffer.len(), region_size - offset);
            let read_addr = start + offset as u64;

            if mem_file.seek(SeekFrom::Start(read_addr)).is_err() {
                break;
            }

            match mem_file.read(&mut buffer[..chunk_size]) {
                Ok(bytes_read) if bytes_read > 0 => {
                    let chunk = &buffer[..bytes_read];
                    add_chunk_candidates(&mut allocation_candidates, &mut previous_tail, chunk);
                }
                _ => break,
            }

            offset += chunk_size;
        }
        allocation_candidates.extend(auth_candidates_in_allocation(&previous_tail));

        if let Some(auth) = tracker.observe_allocation(allocation_candidates) {
            log::info!(
                "Confirmed account authorization in {} readable allocations",
                REQUIRED_AUTH_ALLOCATIONS
            );
            return Ok(Some(auth));
        }
    }

    if tracker.allocation_counts.is_empty() {
        log::warn!("No auth data found in process memory");
    } else {
        log::warn!(
            "Found {} candidate(s) but none confirmed (need {} matches)",
            tracker.allocation_counts.len(),
            REQUIRED_AUTH_ALLOCATIONS
        );
    }

    Ok(None)
}

/// Scans process memory for authorization data (accountId + nonce) on Windows.
/// Uses Windows API to enumerate and read process memory regions.
/// Requires appropriate process access rights (PROCESS_VM_READ | PROCESS_QUERY_INFORMATION)
#[cfg(all(feature = "memory", target_os = "windows"))]
pub fn scan_memory_for_auth(pid: u32) -> Result<Option<AuthQuery>> {
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
        "Scanning Windows memory for account authorization (PID: {})",
        pid
    );

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
    let mut tracker = AuthCandidateTracker::default();

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
                let mut allocation_candidates = HashSet::new();
                let mut previous_tail = Vec::new();
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
                        add_chunk_candidates(&mut allocation_candidates, &mut previous_tail, chunk);
                    } else {
                        break; // Failed to read, move to next region
                    }

                    offset += chunk_size;
                }
                allocation_candidates.extend(auth_candidates_in_allocation(&previous_tail));

                if let Some(auth) = tracker.observe_allocation(allocation_candidates) {
                    log::info!(
                        "Confirmed account authorization in {} readable allocations",
                        REQUIRED_AUTH_ALLOCATIONS
                    );
                    return Ok(Some(auth));
                }
            }
        }

        // Move to next region
        address = (mbi.BaseAddress as usize) + mbi.RegionSize;
    }

    if tracker.allocation_counts.is_empty() {
        log::warn!("No auth data found in process memory");
    } else {
        log::warn!(
            "Found {} candidate(s) but none confirmed (need {} matches)",
            tracker.allocation_counts.len(),
            REQUIRED_AUTH_ALLOCATIONS
        );
    }

    Ok(None)
}

/// Scans process memory for authorization data - stub for unsupported platforms
#[cfg(all(
    feature = "memory",
    not(any(target_os = "linux", target_os = "windows"))
))]
pub fn scan_memory_for_auth(_pid: u32) -> Result<Option<AuthQuery>> {
    anyhow::bail!("Memory scanning is not supported on this platform")
}

/// Attempts to extract auth data with retries, waiting for it to appear in memory
#[cfg(feature = "memory")]
pub async fn scan_memory_for_auth_with_retry(
    pid: u32,
    max_retries: u32,
    retry_delay: Duration,
) -> Result<Option<AuthQuery>> {
    for attempt in 1..=max_retries {
        log::info!("Memory scan attempt {}/{}", attempt, max_retries);

        match scan_memory_for_auth(pid) {
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

#[cfg(all(test, feature = "memory"))]
// clippy's allow-*-in-tests exemption doesn't recognize cfg(all(test, ...)).
#[allow(clippy::expect_used)]
mod memory_tests {
    use super::*;

    const ACCOUNT_A: &str = "2baaaaaaaaaaaaaaaaaaaaaa";
    const ACCOUNT_B: &str = "3cbbbbbbbbbbbbbbbbbbbbbb";

    fn query(account_id: &str, nonce: &str) -> String {
        format!("prefix?accountId={account_id}&nonce={nonce}\0suffix")
    }

    fn only_candidate(allocation: &[u8]) -> AuthQuery {
        auth_candidates_in_allocation(allocation)
            .into_iter()
            .next()
            .expect("expected an authorization candidate")
    }

    #[test]
    fn extracts_a_valid_authorization_candidate() {
        let candidate = only_candidate(query(ACCOUNT_A, "1234567890").as_bytes());
        assert_eq!(candidate.account_id, ACCOUNT_A);
        assert_eq!(candidate.nonce, "1234567890");
    }

    #[test]
    fn rejects_malformed_authorization_candidates() {
        let cases = [
            "?accountId=2gaaaaaaaaaaaaaaaaaaaaaa&nonce=123",
            "?accountId=2baaaaaaaaaaaaaaaaaaaaa&nonce=123",
            "?accountId=2baaaaaaaaaaaaaaaaaaaaaa&other=123",
            "?accountId=2baaaaaaaaaaaaaaaaaaaaaa&nonce=",
            "?accountId=2baaaaaaaaaaaaaaaaaaaaaa&nonce=abc",
        ];

        for allocation in cases {
            assert!(
                auth_candidates_in_allocation(allocation.as_bytes()).is_empty(),
                "unexpected candidate from {allocation}"
            );
        }
    }

    #[test]
    fn requires_three_distinct_readable_allocations() {
        let allocation = query(ACCOUNT_A, "123");
        let candidates = auth_candidates_in_allocation(allocation.as_bytes());
        let mut tracker = AuthCandidateTracker::default();

        assert!(tracker.observe_allocation(candidates.clone()).is_none());
        assert!(tracker.observe_allocation(candidates.clone()).is_none());
        let confirmed = tracker.observe_allocation(candidates).unwrap();
        assert_eq!(confirmed.account_id, ACCOUNT_A);
        assert_eq!(confirmed.nonce, "123");
    }

    #[test]
    fn repeated_values_in_one_allocation_count_only_once() {
        let allocation = format!(
            "{}{}{}",
            query(ACCOUNT_A, "123"),
            query(ACCOUNT_A, "123"),
            query(ACCOUNT_A, "123")
        );
        let candidates = auth_candidates_in_allocation(allocation.as_bytes());
        assert_eq!(candidates.len(), 1);

        let mut tracker = AuthCandidateTracker::default();
        assert!(tracker.observe_allocation(candidates).is_none());
    }

    #[test]
    fn conflicting_candidates_are_counted_independently() {
        let candidate_a = auth_candidates_in_allocation(query(ACCOUNT_A, "123").as_bytes());
        let candidate_b = auth_candidates_in_allocation(query(ACCOUNT_B, "456").as_bytes());
        let mut tracker = AuthCandidateTracker::default();

        assert!(tracker.observe_allocation(candidate_a.clone()).is_none());
        assert!(tracker.observe_allocation(candidate_b.clone()).is_none());
        assert!(tracker.observe_allocation(candidate_a.clone()).is_none());
        assert!(tracker.observe_allocation(candidate_b).is_none());
        let confirmed = tracker.observe_allocation(candidate_a).unwrap();
        assert_eq!(confirmed.account_id, ACCOUNT_A);
        assert_eq!(confirmed.nonce, "123");
    }

    #[test]
    fn detects_a_candidate_split_across_read_chunks() {
        let allocation = query(ACCOUNT_A, "123456");
        let split_at = allocation.find("123456").unwrap() + 3;
        let mut candidates = HashSet::new();
        let mut tail = Vec::new();

        add_chunk_candidates(
            &mut candidates,
            &mut tail,
            &allocation.as_bytes()[..split_at],
        );
        add_chunk_candidates(
            &mut candidates,
            &mut tail,
            &allocation.as_bytes()[split_at..],
        );

        assert_eq!(candidates.len(), 1);
        let candidate = candidates.into_iter().next().unwrap();
        assert_eq!(candidate.account_id, ACCOUNT_A);
        assert_eq!(candidate.nonce, "123456");
    }
}
