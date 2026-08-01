use clap::{Args, Parser};
#[cfg(unix)]
use std::ffi::OsString;
#[cfg(unix)]
use std::path::PathBuf;
#[cfg(windows)]
use std::pin::Pin;
#[cfg(unix)]
use std::process::Stdio;
use std::sync::Arc;
#[cfg(windows)]
use std::time::Duration;
use std::time::SystemTime;
use tokio::process::Command;
use tokio::signal;
#[cfg(windows)]
use tokio::task::JoinHandle;
#[cfg(windows)]
use tokio::time::{Sleep, sleep};

use wf_control::watcher::{AutoCallbacks, GameLifecycleTracker};
use wf_control::{
    self, AppState, ControlConfig, ControlEndpoint, DaemonEvent, GameStartEvent, ScreenshotConfig,
    SystemQuitEvent,
};
#[cfg(windows)]
use wf_core::logs::DbwinLogSource;
#[cfg(unix)]
use wf_core::logs::WineDebugLogSource;
use wf_core::process;
use wf_itemdata::item_data_fetch;

/// Warframe Account Info Scanner daemon
#[derive(Parser, Debug)]
#[command(name = "wf-info-daemon")]
#[command(about = "Warframe Account Info Scanner daemon")]
#[command(after_help = "Examples:\n  \
    wf-info-daemon -- %command%                        Launch Warframe as child process\n  \
    wf-info-daemon --tcp 127.0.0.1:9999 -- %command%   With custom API endpoint")]
struct Cli {
    #[command(flatten)]
    server: ServerArgs,

    #[command(flatten)]
    screenshot: ScreenshotArgs,

    /// Warframe command and arguments to launch as child process.
    /// Use -- separator before the command.
    #[arg(last = true, required = true)]
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

#[derive(Args, Debug, Clone)]
struct ScreenshotArgs {
    /// Force native Wayland ScreenCast capture in Wayland sessions, even when Warframe is visible through XWayland.
    #[arg(long, env = "WF_INFO_SCREENSHOT_NATIVE_WAYLAND")]
    native_wayland_screenshot: bool,
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

        if endpoints.is_empty() {
            return ControlConfig::from_env();
        }

        Some(ControlConfig { endpoints })
    }
}

fn auto_callbacks_from_env() -> AutoCallbacks {
    if std::env::var("WF_SKIP_AUTO_CALLBACK").is_ok_and(|v| v.eq_ignore_ascii_case("TRUE")) {
        AutoCallbacks::Skip
    } else {
        AutoCallbacks::Enabled
    }
}

fn emit_game_start(state: &AppState) {
    state.emit(DaemonEvent::GameStart(GameStartEvent {
        timestamp: SystemTime::now().into(),
    }));
}

async fn handle_game_exit(
    state: &AppState,
    lifecycle: &GameLifecycleTracker,
    auto_callbacks: AutoCallbacks,
) {
    let reason = lifecycle.exit_reason();
    log::info!("Warframe game process exited: reason={:?}", reason);
    state.emit(DaemonEvent::SystemQuit(SystemQuitEvent {
        timestamp: SystemTime::now().into(),
        reason,
    }));

    if auto_callbacks == AutoCallbacks::Enabled {
        wf_control::wfm_auth::set_status_if_connected(state, wf_control::wfm_auth::Status::Invisible)
            .await;
    }
}

#[cfg(unix)]
fn exit_from_child_result(
    result: Result<Result<std::process::ExitStatus, std::io::Error>, tokio::task::JoinError>,
) -> ! {
    match result {
        Ok(Ok(status)) => {
            log::info!("Warframe process exited with status: {}", status);
            std::process::exit(status.code().unwrap_or(0));
        }
        Ok(Err(e)) => {
            log::error!("Error waiting for Warframe process: {}", e);
            std::process::exit(1);
        }
        Err(e) => {
            log::error!("Child process task failed: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(unix)]
fn merged_winedebug_value(existing: Option<OsString>) -> OsString {
    match existing {
        Some(current) if !current.is_empty() => {
            let mut merged = current;
            merged.push(",warn+debugstr");
            merged
        }
        _ => OsString::from("warn+debugstr"),
    }
}

#[cfg(windows)]
async fn wait_for_new_game_pid_or_launcher_exit(
    existing_pids: &std::collections::HashSet<u32>,
    launcher_pid: u32,
    child_handle: &mut JoinHandle<Result<std::process::ExitStatus, std::io::Error>>,
) -> u32 {
    let wait_for_pid = process::wait_for_new_warframe_start(existing_pids, launcher_pid);
    tokio::pin!(wait_for_pid);

    let mut timeout: Option<Pin<Box<Sleep>>> = None;

    loop {
        tokio::select! {
            pid = &mut wait_for_pid => {
                return pid;
            }
            result = &mut *child_handle, if timeout.is_none() => {
                match result {
                    Ok(Ok(status)) => {
                        log::info!("Warframe launcher exited with status: {}", status);
                        timeout = Some(Box::pin(sleep(process::handoff_grace())));
                    }
                    Ok(Err(e)) => {
                        log::error!("Error waiting for Warframe launcher process: {}", e);
                        std::process::exit(1);
                    }
                    Err(e) => {
                        log::error!("Warframe launcher task failed: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            _ = async {
                if let Some(timeout) = timeout.as_mut() {
                    timeout.await;
                }
            }, if timeout.is_some() => {
                log::error!("Timed out waiting for the Warframe game process after the launcher exited");
                std::process::exit(1);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let auto_callbacks = auto_callbacks_from_env();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Warframe Account Info Scanner started");

    let state = AppState::new(ScreenshotConfig {
        native_wayland_capture: cli.screenshot.native_wayland_screenshot,
    });

    if let Err(e) = item_data_fetch::update_cache().await {
        log::warn!("Failed to update item data cache: {}", e);
    }

    wf_control::wfm_auth::try_restore_session(&state).await;

    let _control_server = match cli.server.into_control_config() {
        Some(cfg) => match wf_control::start_control_server(cfg, Arc::clone(&state)).await {
            Ok(server) => server,
            Err(e) => {
                log::error!("Failed to start control API: {}", e);
                wf_control::ControlServer::empty()
            }
        },
        None => {
            log::warn!("No control API endpoints configured");
            wf_control::ControlServer::empty()
        }
    };

    #[cfg(windows)]
    let mut log_source = DbwinLogSource::new().unwrap_or_else(|e| {
        eprintln!("Error: Failed to start DBWIN monitor: {}", e);
        std::process::exit(1);
    });

    let existing_warframe_pids: std::collections::HashSet<u32> =
        process::get_all_warframe_pids().into_iter().collect();

    log::info!(
        "Launching Warframe as child process: {:?}",
        cli.warframe_cmd
    );
    let mut command = Command::new(&cli.warframe_cmd[0]);
    command.args(&cli.warframe_cmd[1..]);
    #[cfg(unix)]
    {
        command.stderr(Stdio::piped());
        command.env(
            "WINEDEBUG",
            merged_winedebug_value(std::env::var_os("WINEDEBUG")),
        );
    }
    let mut child = command.spawn().unwrap_or_else(|e| {
        eprintln!("Error: Failed to launch Warframe: {}", e);
        std::process::exit(1);
    });

    #[cfg(unix)]
    let log_source = WineDebugLogSource::new(child.stderr.take().unwrap_or_else(|| {
        eprintln!("Error: Failed to capture Wine debug stderr.");
        std::process::exit(1);
    }));

    let launcher_pid = child.id().unwrap_or_else(|| {
        eprintln!("Error: Warframe launched without a PID.");
        std::process::exit(1);
    });
    log::info!("Warframe launcher spawned with PID: {}", launcher_pid);

    let mut child_handle = tokio::spawn(async move { child.wait().await });

    #[cfg(unix)]
    let warframe_pid = {
        let wait_for_pid =
            process::wait_for_new_warframe_start(&existing_warframe_pids, launcher_pid);
        tokio::pin!(wait_for_pid);

        tokio::select! {
            pid = &mut wait_for_pid => pid,
            result = &mut child_handle => exit_from_child_result(result),
        }
    };
    #[cfg(windows)]
    let warframe_pid = wait_for_new_game_pid_or_launcher_exit(
        &existing_warframe_pids,
        launcher_pid,
        &mut child_handle,
    )
    .await;

    #[cfg(windows)]
    {
        log::info!(
            "Using Warframe game PID for DBWIN filtering: {}",
            warframe_pid
        );
        log_source.set_pid_filter(warframe_pid);
    }

    #[cfg(unix)]
    log::info!(
        "Using Wine debugstr stderr transport with Warframe game PID: {}",
        warframe_pid
    );

    emit_game_start(&state);

    if auto_callbacks == AutoCallbacks::Skip {
        log::info!("Skipping auto set of warframe market status...");
    } else {
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            let mut rx = state.subscribe();
            loop {
                match rx.recv().await {
                    Ok(DaemonEvent::AccountLogin(_)) => {
                        wf_control::wfm_auth::set_status_if_connected(
                            &state,
                            wf_control::wfm_auth::Status::Ingame,
                        )
                        .await;
                    }
                    Ok(DaemonEvent::AccountLogout(_)) => {
                        wf_control::wfm_auth::set_status_if_connected(
                            &state,
                            wf_control::wfm_auth::Status::Invisible,
                        )
                        .await;
                    }
                    Ok(_) => {}
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        log::warn!("WFM auto-status missed {} events", n);
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        });
    }

    let lifecycle = GameLifecycleTracker::default();
    let watcher_lifecycle = lifecycle.clone();
    let watcher_state = Arc::clone(&state);
    let mut log_watcher = tokio::spawn(async move {
        if let Err(e) = wf_control::watcher::observe_warframe_activity_with_lifecycle(
            watcher_state,
            log_source,
            Some(warframe_pid),
            auto_callbacks,
            watcher_lifecycle,
        )
        .await
        {
            log::error!("Error reading live log source: {}", e);
        }
    });

    let game_exit =
        process::wait_for_warframe_exit(warframe_pid, launcher_pid, &existing_warframe_pids);
    tokio::pin!(game_exit);

    let game_exited = tokio::select! {
        _ = signal::ctrl_c() => {
            log::info!("Received Ctrl+C, shutting down...");
            false
        }
        watcher = &mut log_watcher => {
            if let Err(e) = watcher {
                log::error!("Log watcher task failed: {}", e);
            } else {
                log::info!("Log watcher exited");
            }
            tokio::select! {
                _ = signal::ctrl_c() => {
                    log::info!("Received Ctrl+C, shutting down...");
                    false
                }
                _ = &mut game_exit => true,
            }
        }
        _ = &mut game_exit => true,
    };

    if game_exited {
        handle_game_exit(&state, &lifecycle, auto_callbacks).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launch_command_is_required() {
        assert!(Cli::try_parse_from(["wf-info-daemon"]).is_err());
    }

    #[test]
    fn launch_command_is_captured_after_separator() {
        let cli = Cli::try_parse_from([
            "wf-info-daemon",
            "--tcp",
            "127.0.0.1:9999",
            "--",
            "wine",
            "Warframe.x64.exe",
        ])
        .unwrap();

        assert_eq!(cli.warframe_cmd, vec!["wine", "Warframe.x64.exe"]);
    }

    #[tokio::test]
    async fn lifecycle_helpers_emit_start_and_unexpected_quit() {
        let state = AppState::default();
        let mut events = state.subscribe();

        emit_game_start(&state);
        assert!(matches!(
            events.recv().await.unwrap(),
            DaemonEvent::GameStart(_)
        ));

        handle_game_exit(&state, &GameLifecycleTracker::default(), AutoCallbacks::Skip).await;
        assert!(matches!(
            events.recv().await.unwrap(),
            DaemonEvent::SystemQuit(SystemQuitEvent {
                reason: wf_control::SystemQuitReason::Unexpected,
                ..
            })
        ));
    }

    #[cfg(unix)]
    #[test]
    fn merged_winedebug_value_adds_debugstr_channel() {
        assert_eq!(
            merged_winedebug_value(None),
            OsString::from("warn+debugstr")
        );
        assert_eq!(
            merged_winedebug_value(Some(OsString::from("fixme-all"))),
            OsString::from("fixme-all,warn+debugstr")
        );
    }
}
