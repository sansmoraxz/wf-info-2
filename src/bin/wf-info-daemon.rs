use clap::{Args, Parser};
#[cfg(unix)]
use std::path::PathBuf;
use tokio::process::Command;
use tokio::signal;

use wf_info_2::control::{ControlConfig, ControlEndpoint};
use wf_info_2::*;

/// Warframe Account Info Scanner daemon
#[derive(Parser, Debug)]
#[command(name = "wf-info-daemon")]
#[command(about = "Warframe Account Info Scanner daemon")]
#[command(after_help = "Examples:\n  \
    wf-info-daemon -- %command%                        Launch Warframe as child process\n  \
    wf-info-daemon --tcp 127.0.0.1:9999 -- %command%   With custom API endpoint\n  \
    wf-info-daemon                                     Monitor existing Warframe process")]
struct Cli {
    #[command(flatten)]
    server: ServerArgs,

    /// Warframe command and arguments to launch as child process.
    /// Use -- separator before the command.
    /// If not provided, scans for an existing Warframe process.
    #[arg(last = true)]
    warframe_cmd: Vec<String>,
}

#[derive(Args, Debug, Clone)]
struct ServerArgs {
    /// TCP address to listen on
    #[arg(long, env = "WF_INFO_API_TCP")]
    tcp: Option<String>,

    /// Unix socket path to listen on
    #[cfg(unix)]
    #[arg(long, env = "WF_INFO_API_UNIX")]
    unix: Option<PathBuf>,

    /// Named pipe name to listen on
    #[cfg(windows)]
    #[arg(long, env = "WF_INFO_API_NPIPE")]
    npipe: Option<String>,
}

impl ServerArgs {
    fn into_control_config(self) -> Option<ControlConfig> {
        let mut endpoints = Vec::new();

        if let Some(addr) = self.tcp {
            endpoints.push(ControlEndpoint::Tcp(addr));
        }

        #[cfg(unix)]
        if let Some(path) = self.unix {
            endpoints.push(ControlEndpoint::Unix(path));
        }

        #[cfg(windows)]
        if let Some(pipe) = self.npipe {
            endpoints.push(ControlEndpoint::Npipe(pipe));
        }

        // If no CLI args or env vars provided, use defaults
        if endpoints.is_empty() {
            return ControlConfig::from_env();
        }

        Some(ControlConfig { endpoints })
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Warframe Account Info Scanner started");

    let cli = Cli::parse();

    let _control_server = match cli.server.into_control_config() {
        Some(cfg) => match control::start_control_server(cfg).await {
            Ok(server) => server,
            Err(e) => {
                log::error!("Failed to start control API: {}", e);
                control::ControlServer::empty()
            }
        },
        None => {
            log::warn!("No control API endpoints configured");
            control::ControlServer::empty()
        }
    };

    // Check if we should launch Warframe as a child process
    let warframe_cmd = if cli.warframe_cmd.is_empty() {
        None
    } else {
        Some(cli.warframe_cmd)
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
    let (child_handle, warframe_pid) = if let Some(cmd_args) = warframe_cmd {
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
        let handle = tokio::spawn(async move {
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
        });
        (Some(handle), None)
    } else {
        // No command provided, wait for Warframe to start on its own
        log::info!("No launch command provided, waiting for existing Warframe process...");
        let pid = process::wait_for_warframe_start().await;
        (None, Some(pid))
    };

    // Start watching the log file
    let log_watcher = tokio::spawn(async move {
        if let Err(e) = watcher::observe_warframe_activity(wf_config, warframe_pid).await {
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
