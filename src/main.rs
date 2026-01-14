use notify::RecursiveMode;
use notify_debouncer_mini::{DebounceEventResult, new_debouncer};
use regex::Regex;
use std::env;
use std::fs::{File, metadata};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

fn find_ee_log() -> Option<PathBuf> {
    // Common Warframe installation paths on Linux (Steam/Proton)
    let home = env::var("HOME").ok()?;

    // Try Steam Proton path
    let steam_path = PathBuf::from(&home)
        .join(".steam/steam/steamapps/compatdata/230410/pfx/drive_c/users/steamuser/AppData/Local/Warframe/EE.log");

    if steam_path.exists() {
        return Some(steam_path);
    }

    // Try custom path from environment variable
    if let Ok(custom_path) = env::var("WARFRAME_EE_LOG") {
        let path = PathBuf::from(custom_path);
        if path.exists() {
            return Some(path);
        }
    }

    None
}

struct AccountInfo {
    username: String,
    account_id: String,
}

fn parse_line_for_account(line: &str) -> Option<AccountInfo> {
    // Regex patterns to find account ID and username
    // Pattern 1: "Logged in Username (accountid)"
    let login_regex = Regex::new(r"Sys \[Info\]: Logged in (\S+) \(([A-Fa-f0-9]+)\)").ok()?;
    // Pattern 2: "Player name changed to Username ... AccountId: accountid"
    let account_regex =
        Regex::new(r"Player name changed to (\S+).*AccountId:\s*([A-Fa-f0-9]+)").ok()?;

    // Check for "Logged in" pattern
    if let Some(caps) = login_regex.captures(line) {
        if let (Some(username), Some(id)) = (caps.get(1), caps.get(2)) {
            return Some(AccountInfo {
                username: username.as_str().to_string(),
                account_id: id.as_str().to_string(),
            });
        }
    }

    // Check for "Player name changed" pattern
    if let Some(caps) = account_regex.captures(line) {
        if let (Some(username), Some(id)) = (caps.get(1), caps.get(2)) {
            return Some(AccountInfo {
                username: username.as_str().to_string(),
                account_id: id.as_str().to_string(),
            });
        }
    }

    None
}

fn parse_account_id(log_path: &Path) -> Result<Option<AccountInfo>, anyhow::Error> {
    let file = File::open(log_path)?;
    let reader = BufReader::new(file);

    let mut account_info: Option<AccountInfo> = None;

    // Read file from end to get most recent login status
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    for line in lines.iter().rev() {
        if let Some(info) = parse_line_for_account(line) {
            account_info = Some(info);
            break;
        }
    }

    Ok(account_info)
}

fn watch_log_file(log_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Watching log file for login events: {}", log_path.display());
    log::info!("Press Ctrl+C to stop");

    let log_filename = log_path.file_name().ok_or("Invalid log path")?;
    let mut last_size = metadata(log_path)?.len();
    let mut last_position = last_size;

    // Set up file watcher - watch parent directory to detect file recreation
    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        move |res: DebounceEventResult| {
            if let Ok(events) = res {
                for event in events {
                    let _ = tx.send(event);
                }
            }
        },
    )?;

    // Watch the parent directory to catch file deletion and recreation
    let parent_dir = log_path.parent().ok_or("Invalid log path")?;
    debouncer
        .watcher()
        .watch(parent_dir, RecursiveMode::NonRecursive)?;

    loop {
        // Wait for file change event
        if let Ok(event) = rx.recv() {
            // Filter events: only process if it's for our target file
            let is_our_file = event
                .path
                .file_name()
                .map(|name| name == log_filename)
                .unwrap_or(false);

            if !is_our_file {
                continue;
            }

            log::debug!("Event for EE.log: {:?}", event);

            // Check if file still exists
            if !log_path.exists() {
                log::info!("File deleted, waiting for recreation");

                // Wait for file to be recreated
                while !log_path.exists() {
                    std::thread::sleep(Duration::from_millis(100));
                }

                log::info!("File recreated, launcher restarted");

                last_size = 0;
                last_position = 0;
                continue;
            }

            let current_size = match metadata(log_path) {
                Ok(meta) => meta.len(),
                Err(_) => continue, // File might be temporarily unavailable
            };

            // Check if file was truncated/cleared (launcher restart without deletion)
            if current_size < last_size {
                log::info!("File truncated, launcher restarted");
                last_size = 0;
                last_position = 0;
                continue;
            }

            // Only process if there's new content
            if current_size > last_position {
                log::debug!(
                    "Reading from position {} to {}",
                    last_position,
                    current_size
                );

                // Reopen file to get fresh handle
                let mut read_file = File::open(log_path)?;
                read_file.seek(SeekFrom::Start(last_position))?;

                let reader = BufReader::new(read_file);

                // Process new lines
                for line_result in reader.lines() {
                    if let Ok(line) = line_result {
                        log::debug!("New line: {}", line);
                        if let Some(AccountInfo {
                            username,
                            account_id,
                        }) = parse_line_for_account(&line)
                        {
                            log::info!("User logged in: username={}, account_id={}", username, account_id);
                        }
                    }
                }

                last_position = current_size;
            }

            last_size = current_size;
        }
    }
}

fn main() {
    env_logger::init();
    log::info!("Warframe Account Info Scanner started");

    // Check for --watch flag
    let watch_mode = env::args().any(|arg| arg == "--watch" || arg == "-w");

    // Check for custom log path as command line argument
    let log_path = env::args()
        .nth(if watch_mode { 2 } else { 1 })
        .or_else(|| env::args().nth(if watch_mode { 1 } else { 2 }))
        .filter(|arg| !arg.starts_with('-'))
        .map(PathBuf::from)
        .or_else(|| find_ee_log())
        .unwrap_or_else(|| {
            eprintln!("Error: Could not find EE.log file.");
            eprintln!("Please specify the path as an argument or set WARFRAME_EE_LOG environment variable.");
            eprintln!("\nUsage: wf-info-2 [--watch] [path/to/EE.log]");
            eprintln!("  --watch, -w    Watch file for new login events");
            std::process::exit(1);
        });

    log::info!("Log file path: {}", log_path.display());

    if watch_mode {
        // Live monitoring mode
        match parse_account_id(&log_path) {
            Ok(Some(AccountInfo {
                username,
                account_id,
            })) => {
                log::info!("Current session: username={}, account_id={}", username, account_id);
            }
            Ok(None) => {
                log::info!("No active login session found");
            }
            Err(e) => {
                log::warn!("Error reading log file: {}", e);
            }
        }

        if let Err(e) = watch_log_file(&log_path) {
            log::error!("Error watching file: {}", e);
            std::process::exit(1);
        }
    } else {
        // One-time scan mode
        match parse_account_id(&log_path) {
            Ok(Some(AccountInfo {
                username,
                account_id,
            })) => {
                log::info!("User logged in: username={}, account_id={}", username, account_id);
            }
            Ok(None) => {
                log::info!("No active login session found");
            }
            Err(e) => {
                log::error!("Error reading log file: {}", e);
                std::process::exit(1);
            }
        }
    }
}
