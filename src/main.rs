mod account;
mod api;
mod inventory;
mod logs;
mod process;
mod profile;
mod storage;
mod utils;
mod watcher;

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Warframe Account Info Scanner started");

    // Find EE.log file
    let log_path = logs::find_ee_log().unwrap_or_else(|| {
        eprintln!("Error: Could not find EE.log file.");
        eprintln!(
            "Please ensure Warframe is installed or set WARFRAME_EE_LOG environment variable."
        );
        std::process::exit(1);
    });

    log::info!("Log file path: {}", log_path.display());

    process::wait_for_warframe_start().await;

    if let Err(e) = watcher::watch_log_file(log_path).await {
        log::error!("Error watching file: {}", e);
        std::process::exit(1);
    }
}
