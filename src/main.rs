use std::env;
use tokio::process::Command;
use tokio::signal;

use wf_info_2::*;

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Warframe Account Info Scanner started");

    let args: Vec<String> = env::args().collect();

    // Check if we should launch Warframe as a child process
    // Format: wf-info-2 -- /path/to/warframe args...
    let warframe_cmd = if args.len() > 2 && args[1] == "--" {
        Some(&args[2..])
    } else if args.len() > 1 && args[1] == "--help" {
        println!("Usage:");
        println!(
            "  {} -- <warframe_command> [args...]   Launch Warframe as child process",
            args[0]
        );
        println!(
            "  {}                                   Monitor existing Warframe process",
            args[0]
        );
        println!();
        println!("Example:");
        println!(
            "  {} -- /path/to/Warframe.x64.exe -log:Preprocess.log",
            args[0]
        );
        std::process::exit(0);
    } else if args.len() > 1 {
        eprintln!("Error: Invalid arguments. Use '--' separator before Warframe command.");
        eprintln!("Usage: {} -- <warframe_command> [args...]", args[0]);
        std::process::exit(1);
    } else {
        None
    };

    // Find warframe config folder
    let wf_config = logs::find_wf_app_config().unwrap_or_else(|| {
        eprintln!("Error: Could not find Warframe config folder.");
        eprintln!(
            "Please ensure Warframe is installed or set WARFRAME_APP_CONFIG environment variable."
        );
        std::process::exit(1);
    });

    log::info!("Warframe config folder: {:?}", wf_config);

    // If command line args provided, launch Warframe as child process
    let child_handle = if let Some(cmd_args) = warframe_cmd {
        log::info!("Launching Warframe as child process: {:?}", cmd_args);

        let mut child = Command::new(&cmd_args[0])
            .args(&cmd_args[1..])
            .spawn()
            .unwrap_or_else(|e| {
                eprintln!("Error: Failed to launch Warframe: {}", e);
                std::process::exit(1);
            });

        log::info!("Warframe launched with PID: {:?}", child.id());

        // Spawn task to monitor child process exit
        Some(tokio::spawn(async move {
            match child.wait().await {
                Ok(status) => {
                    log::info!("Warframe process exited with status: {}", status);
                    std::process::exit(status.code().unwrap_or(0));
                }
                Err(e) => {
                    log::error!("Error waiting for Warframe process: {}", e);
                    std::process::exit(1);
                }
            }
        }))
    } else {
        // No command provided, wait for Warframe to start on its own
        log::info!("No launch command provided, waiting for existing Warframe process...");
        process::wait_for_warframe_start().await;
        None
    };

    // Start watching the log file
    let log_watcher = tokio::spawn(async move {
        if let Err(e) = watcher::observe_warframe_activity(wf_config).await {
            log::error!("Error watching file: {}", e);
        }
    });

    // Wait for Ctrl+C
    tokio::select! {
        _ = signal::ctrl_c() => {
            log::info!("Received Ctrl+C, shutting down...");
        }
        _ = log_watcher => {
            log::info!("Log watcher exited");
        }
        result = async {
            if let Some(handle) = child_handle {
                handle.await
            } else {
                // Never completes if no child
                std::future::pending::<Result<(), tokio::task::JoinError>>().await
            }
        } => {
            if let Ok(())  = result {
                log::info!("Child process task completed");
            }
        }
    }
}

#[cfg(test)]
mod tests;
