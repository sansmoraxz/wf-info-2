use anyhow::{Context, Result};
use regex::bytes::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::time::Duration;
use sysinfo::System;
use tokio::time::sleep;

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

pub async fn wait_for_warframe_start() {
    log::info!("Waiting for Warframe to start...");
    let mut system = System::new();

    loop {
        system.refresh_all();

        let running = system
            .processes()
            .values()
            .any(|p| is_warframe_game_process(p));

        if running {
            log::info!("Warframe process detected.");
            break;
        }

        sleep(Duration::from_secs(5)).await;
    }
}

/// Finds the Warframe game process PID if running
pub fn get_warframe_pid() -> Option<u32> {
    let mut system = System::new();
    system.refresh_all();

    system
        .processes()
        .values()
        .find(|p| is_warframe_game_process(p))
        .map(|p| p.pid().as_u32())
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
/// Requires appropriate permissions (typically root/sudo).
pub fn scan_memory_for_auth(pid: u32, account_id: &str) -> Result<Option<AuthQuery>> {
    log::info!(
        "Scanning memory for auth data (PID: {}, accountId: {})",
        pid,
        account_id
    );

    // Pattern: ?accountId=<24 chars>&nonce=<digits>
    let pattern_str = format!(r"\?accountId={}&nonce=([0-9]+)", regex::escape(account_id));
    let re = Regex::new(&pattern_str).context("Failed to build regex pattern")?;

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
        let line = line?;
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
                    // Search for all matches in this chunk
                    for captures in re.captures_iter(&buffer[..bytes_read]) {
                        if let Some(nonce_match) = captures.get(1) {
                            let nonce = String::from_utf8_lossy(nonce_match.as_bytes()).to_string();
                            let auth_str = format!("{}:{}", account_id, nonce);

                            let count = candidates.entry(auth_str.clone()).or_insert(0);
                            *count += 1;

                            log::debug!("Found candidate auth (count={}): {}", count, auth_str);

                            // Like the C++ version, require multiple matches for confidence
                            if *count >= REQUIRED_MATCHES {
                                log::info!("Confirmed auth data after {} matches", count);
                                log::debug!("Auth data: accountId={}, nonce={}", account_id, nonce);
                                return Ok(Some(AuthQuery {
                                    account_id: account_id.to_string(),
                                    nonce,
                                }));
                            }
                        }
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

/// Attempts to extract auth data with retries, waiting for it to appear in memory
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
