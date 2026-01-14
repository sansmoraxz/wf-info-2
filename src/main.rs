use std::env;
use std::path::PathBuf;

mod account;
mod api;
mod logs;
mod process;
mod profile;
mod storage;
mod watcher;

use account::AccountInfo;

#[tokio::main]
async fn main() {
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
        .or_else(|| logs::find_ee_log())
        .unwrap_or_else(|| {
            eprintln!("Error: Could not find EE.log file.");
            eprintln!("Please specify the path as an argument or set WARFRAME_EE_LOG environment variable.");
            eprintln!("\nUsage: wf-info-2 [--watch] [path/to/EE.log]");
            eprintln!("  --watch, -w    Watch file for new login events");
            std::process::exit(1);
        });

    log::info!("Log file path: {}", log_path.display());

    if watch_mode {
        process::wait_for_warframe_start().await;

        // Live monitoring mode
        let initial_account_id = match logs::parse_account_id(&log_path) {
            Ok(Some(AccountInfo {
                username,
                account_id,
            })) => {
                log::info!(
                    "Current session: username={}, account_id={}",
                    username,
                    account_id
                );
                match api::fetch_player_profile(&account_id).await {
                    Ok(profile) => {
                        log::info!("Fetched profile: {:?}", profile);
                        if let Err(e) = storage::save_encrypted_profile(&profile) {
                            log::error!("Failed to save profile: {}", e);
                        }
                    }
                    Err(e) => log::error!("Failed to fetch profile: {}", e),
                }
                Some(account_id)
            }
            Ok(None) => {
                log::info!("No active login session found");
                None
            }
            Err(e) => {
                log::warn!("Error reading log file: {}", e);
                None
            }
        };

        if let Err(e) = watcher::watch_log_file(log_path, initial_account_id).await {
            log::error!("Error watching file: {}", e);
            std::process::exit(1);
        }
    } else {
        // One-time scan mode
        match logs::parse_account_id(&log_path) {
            Ok(Some(AccountInfo {
                username,
                account_id,
            })) => {
                log::info!(
                    "User logged in: username={}, account_id={}",
                    username,
                    account_id
                );
                match api::fetch_player_profile(&account_id).await {
                    Ok(profile) => {
                        log::info!("Fetched profile: {:?}", profile);
                        if let Err(e) = storage::save_encrypted_profile(&profile) {
                            log::error!("Failed to save profile: {}", e);
                        }
                    }
                    Err(e) => log::error!("Failed to fetch profile: {}", e),
                }
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
