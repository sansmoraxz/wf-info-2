use chrono::Utc;
use notify::RecursiveMode;
use notify_debouncer_mini::{DebounceEventResult, new_debouncer};
#[cfg(feature = "memory")]
use serde_json::json;
use std::fs::{File, metadata};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

use crate::account::AccountInfo;
use crate::control::{AccountLoginEvent, AccountLogoutEvent, DaemonEvent, ProfileUpdatedEvent};
use crate::logs::{self, LogEntryParser, LogEvent};

use crate::storage;
use crate::{api, control};

#[cfg(feature = "memory")]
use crate::{control::InventoryFetchedEvent, inventory_refresh, process};

pub async fn observe_warframe_activity(
    app_config_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Watching for Warframe activity...");

    if !app_config_path.exists() {
        // wait for folder to be created
        log::info!("Waiting for Warframe config folder to be created...");
        while !app_config_path.exists() {
            sleep(Duration::from_millis(500)).await;
        }
    }

    // wait for the log file to be created
    log::info!("Waiting for EE.log to be created...");
    let log_path = app_config_path.join("EE.log");
    while !log_path.exists() {
        sleep(Duration::from_millis(500)).await;
    }
    log::info!("EE.log found at {:?}", log_path);

    let mut current_account_id: Option<String> = None;
    let mut log_parser = LogEntryParser::new();
    let log_filename = log_path.file_name().ok_or("Invalid log path")?.to_owned();
    let initial_size = metadata(&log_path)?.len();
    let mut last_size = initial_size;
    let mut last_position = initial_size;

    // Keep a persistent file handle to avoid reopening on every event
    let mut read_file = File::open(&log_path)?;
    read_file.seek(SeekFrom::Start(last_position))?;

    // Set up file watcher - watch parent directory to detect file recreation
    let (tx, mut rx) = mpsc::channel(100);
    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        move |res: DebounceEventResult| {
            if let Ok(events) = res {
                for event in events {
                    let _ = tx.blocking_send(event);
                }
            }
        },
    )?;

    // Watch the parent directory of the log file for changes
    debouncer
        .watcher()
        .watch(&app_config_path, RecursiveMode::NonRecursive)?;

    loop {
        // Wait for file change event
        if let Some(event) = rx.recv().await {
            // Filter events: only process if it's for our target file
            let is_our_file = event
                .path
                .file_name()
                .map(|name| name == log_filename.as_os_str())
                .unwrap_or(false);

            if !is_our_file {
                continue;
            }

            log::trace!("Event for EE.log: {:?}", event);

            // Check if file still exists
            if !log_path.exists() {
                log::info!("File deleted, waiting for recreation");

                // Wait for file to be recreated with exponential backoff
                let mut backoff = Duration::from_millis(100);
                let max_backoff = Duration::from_secs(15);
                while !log_path.exists() {
                    sleep(backoff).await;
                    backoff = (backoff * 2).min(max_backoff);
                }

                log::info!("File recreated, game restarted");

                last_size = 0;
                last_position = 0;
                current_account_id = None;
                log_parser.reset();
                read_file = File::open(&log_path)?;
                continue;
            }

            let current_size = match metadata(&log_path) {
                Ok(meta) => meta.len(),
                Err(_) => continue, // File might be temporarily unavailable
            };

            // Check if file was truncated/cleared (game restart without deletion)
            if current_size < last_size {
                log::info!("File truncated, game restarted");
                last_size = 0;
                last_position = 0;
                current_account_id = None;
                log_parser.reset();
                read_file = File::open(&log_path)?;
                continue;
            }

            // Only process if there's new content
            if current_size > last_position {
                log::debug!(
                    "Reading from position {} to {}",
                    last_position,
                    current_size
                );

                read_file.seek(SeekFrom::Start(last_position))?;

                let reader = BufReader::new(&read_file);

                // Feed lines through the state machine, collecting complete entries
                let mut complete_entries: Vec<String> = Vec::new();
                for line_result in reader.lines() {
                    if let Ok(line) = line_result {
                        log::trace!("New line: {}", line);
                        if let Some(entry) = log_parser.feed_line(&line) {
                            complete_entries.push(entry);
                        }
                    }
                }
                // Flush the last buffered entry
                if let Some(entry) = log_parser.flush() {
                    complete_entries.push(entry);
                }

                // Process complete multi-line entries
                for entry in complete_entries {
                    log::trace!("Complete log entry: {}", entry);
                    match logs::parse_log_line(&entry) {
                        Some(LogEvent::Login(AccountInfo {
                            username,
                            account_id,
                        })) => {
                            if current_account_id.as_deref() == Some(&account_id) {
                                log::debug!(
                                    "Duplicate login event for account_id={}",
                                    account_id
                                );
                                continue;
                            }

                            current_account_id = Some(account_id.clone());
                            control::set_current_account(Some(account_id.clone()));
                            log::info!(
                                "User logged in: username={}, account_id={}",
                                username,
                                account_id
                            );

                            // Emit account login event
                            control::emit(DaemonEvent::AccountLogin(AccountLoginEvent {
                                timestamp: Utc::now(),
                                account_id: account_id.clone(),
                                username: username.clone(),
                            }));

                            let acc_id = account_id.clone();
                            let user_name = username.clone();
                            tokio::spawn(async move {
                                // 1. Fetch Profile
                                match api::fetch_player_profile(&acc_id).await {
                                    Ok(profile) => {
                                        log::info!(
                                            "Fetched profile for {}: {:?}",
                                            user_name,
                                            profile
                                        );
                                        if let Err(e) =
                                            storage::save_encrypted_profile(&profile)
                                        {
                                            log::error!(
                                                "Failed to save profile for {}: {}",
                                                user_name,
                                                e
                                            );
                                        } else {
                                            // Emit profile updated event
                                            control::emit(DaemonEvent::ProfileUpdated(
                                                ProfileUpdatedEvent {
                                                    timestamp: Utc::now(),
                                                    account_id: acc_id.clone(),
                                                },
                                            ));
                                        }
                                    }
                                    Err(e) => {
                                        log::error!(
                                            "Failed to fetch profile for {}: {}",
                                            user_name,
                                            e
                                        );
                                    }
                                }

                                // 2. Scan Memory & Fetch Nonces and Inventory (if memory feature enabled)
                                #[cfg(feature = "memory")]
                                if let Some(pid) = process::get_warframe_pid() {
                                    log::info!(
                                        "Warframe running (PID: {}), attempting to extract inventory auth...",
                                        pid
                                    );

                                    match inventory_refresh::fetch_inventory_from_process(
                                        &acc_id,
                                        pid,
                                        5,
                                        Duration::from_secs(3),
                                    )
                                    .await
                                    {
                                        Ok(Some(result)) => {
                                            log::info!(
                                                "Successfully extracted auth: {}",
                                                result.auth.to_query_string()
                                            );

                                            if let Err(e) =
                                                storage::save_inventory(&result.inventory)
                                            {
                                                log::error!("Failed to save inventory: {}", e);
                                            } else {
                                                if let Err(e) = storage::touch_inventory_updated(
                                                    Some("auto"),
                                                ) {
                                                    log::warn!(
                                                        "Failed to update inventory metadata: {}",
                                                        e
                                                    );
                                                }

                                                // Emit inventory fetched event
                                                let summary = json!({
                                                    "suits": result.inventory.suits.len(),
                                                    "long_guns": result.inventory.long_guns.len(),
                                                    "pistols": result.inventory.pistols.len(),
                                                    "melee": result.inventory.melee.len(),
                                                });
                                                control::emit(DaemonEvent::InventoryFetched(
                                                    InventoryFetchedEvent {
                                                        timestamp: Utc::now(),
                                                        source: "auto".to_string(),
                                                        summary,
                                                    },
                                                ));
                                            }
                                        }
                                        Ok(None) => {
                                            log::warn!(
                                                "Could not extract auth data from process memory"
                                            );
                                            log::info!(
                                                "Tip: Make sure you're logged into Warframe"
                                            );
                                        }
                                        Err(e) => {
                                            log::error!("Memory scan error: {}", e);
                                            log::info!(
                                                "Tip: Grant necessary permissions or try running with sudo"
                                            );
                                        }
                                    }
                                } else {
                                    log::info!(
                                        "Warframe not running - skipping inventory fetch"
                                    );
                                }
                            });
                        }
                        Some(LogEvent::Logout) => {
                            current_account_id = None;
                            control::set_current_account(None);
                            log::info!("User logged out");

                            // Emit account logout event
                            control::emit(DaemonEvent::AccountLogout(AccountLogoutEvent {
                                timestamp: Utc::now(),
                            }));

                            if let Err(e) = storage::delete_profile() {
                                log::error!("Failed to delete profile: {}", e);
                            }
                        }
                        None => {}
                    }
                }

                last_position = current_size;
            }

            last_size = current_size;
        }
    }
}
