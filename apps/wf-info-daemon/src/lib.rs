//! Warframe Account Info Scanner daemon.

use clap::{Args, Parser};
use std::collections::HashSet;
use std::env;
use std::io;
#[cfg(unix)]
use std::path::PathBuf;
#[cfg(windows)]
use std::pin::Pin;
use std::process::{ExitStatus, exit};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::process::{Child, Command};
use tokio::signal;
use tokio::sync::broadcast::error::RecvError;
#[cfg(unix)]
use tokio::task::JoinError;
use tokio::task::JoinHandle;
#[cfg(windows)]
use tokio::time::{Sleep, sleep};

use wf_control::watcher::{self, AutoCallbacks, GameLifecycleTracker};
use wf_control::wfm_auth::{Status, WfmHandle, set_status_if_connected, try_restore_session};
use wf_control::{
    self, ControlConfig, ControlEndpoint, ControlServer, DaemonEvent, EventBus, GameStartEvent,
    Handles, ScreenshotConfig, SystemQuitEvent, WaylandCapture,
};
#[cfg(windows)]
use wf_core::logs::DbwinLogSource;
#[cfg(unix)]
use wf_core::logs::WineDbwinBridgeSource;
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
    if env::var("WF_SKIP_AUTO_CALLBACK").is_ok_and(|v| v.eq_ignore_ascii_case("TRUE")) {
        AutoCallbacks::Skip
    } else {
        AutoCallbacks::Enabled
    }
}

fn emit_game_start(events: &EventBus) {
    events.emit(DaemonEvent::GameStart(GameStartEvent {
        timestamp: SystemTime::now().into(),
    }));
}

async fn wait_for_game_exit(mut game: process::RunningGame, lifecycle: &GameLifecycleTracker) {
    loop {
        let exited = game.pid_exited().await;
        // A requested quit means no bootstrap-handoff successor is coming,
        // so skip the grace-window scan instead of idling through it.
        if lifecycle.is_quit_requested() {
            return;
        }
        match exited.into_successor().await {
            Some(next) => game = next,
            None => return,
        }
    }
}

async fn handle_game_exit(
    events: &EventBus,
    wfm: &WfmHandle,
    lifecycle: &GameLifecycleTracker,
    auto_callbacks: AutoCallbacks,
) {
    let reason = lifecycle.exit_reason();
    log::info!("Warframe game process exited: reason={reason:?}");
    events.emit(DaemonEvent::SystemQuit(SystemQuitEvent {
        timestamp: SystemTime::now().into(),
        reason,
    }));

    if auto_callbacks == AutoCallbacks::Enabled {
        set_status_if_connected(wfm, Status::Invisible).await;
    }
}

#[cfg(unix)]
fn exit_from_child_result(result: Result<Result<ExitStatus, io::Error>, JoinError>) -> ! {
    match result {
        Ok(Ok(status)) => {
            log::info!("Warframe process exited with status: {status}");
            exit(status.code().unwrap_or(0));
        }
        Ok(Err(e)) => {
            log::error!("Error waiting for Warframe process: {e}");
            exit(1);
        }
        Err(e) => {
            log::error!("Child process task failed: {e}");
            exit(1);
        }
    }
}

#[cfg(unix)]
async fn wait_for_game_start_or_launcher_exit(
    launcher: process::Launcher,
    child_handle: &mut JoinHandle<Result<ExitStatus, io::Error>>,
) -> process::RunningGame {
    let game_started = launcher.game_started();
    tokio::pin!(game_started);

    tokio::select! {
        game = &mut game_started => game,
        result = &mut *child_handle => exit_from_child_result(result),
    }
}

#[cfg(windows)]
async fn wait_for_game_start_or_launcher_exit(
    launcher: process::Launcher,
    child_handle: &mut JoinHandle<Result<ExitStatus, io::Error>>,
) -> process::RunningGame {
    let game_started = launcher.game_started();
    tokio::pin!(game_started);

    let mut timeout: Option<Pin<Box<Sleep>>> = None;

    loop {
        tokio::select! {
            game = &mut game_started => {
                return game;
            }
            result = &mut *child_handle, if timeout.is_none() => {
                match result {
                    Ok(Ok(status)) => {
                        log::info!("Warframe launcher exited with status: {status}");
                        timeout = Some(Box::pin(sleep(process::handoff_grace())));
                    }
                    Ok(Err(e)) => {
                        log::error!("Error waiting for Warframe launcher process: {e}");
                        exit(1);
                    }
                    Err(e) => {
                        log::error!("Warframe launcher task failed: {e}");
                        exit(1);
                    }
                }
            }
            _ = async {
                if let Some(timeout) = timeout.as_mut() {
                    timeout.await;
                }
            }, if timeout.is_some() => {
                log::error!("Timed out waiting for the Warframe game process after the launcher exited");
                exit(1);
            }
        }
    }
}

async fn start_control_api(server: ServerArgs, cx: &Handles) -> ControlServer {
    let Some(cfg) = server.into_control_config() else {
        log::warn!("No control API endpoints configured");
        return ControlServer::empty();
    };
    match wf_control::start_control_server(cfg, cx.clone()).await {
        Ok(server) => server,
        Err(e) => {
            log::error!("Failed to start control API: {e}");
            ControlServer::empty()
        }
    }
}

fn launch_warframe(warframe_cmd: &[String]) -> Child {
    log::info!("Launching Warframe as child process: {warframe_cmd:?}");
    let Some((program, program_args)) = warframe_cmd.split_first() else {
        // Unreachable in practice: clap marks the launch command as required.
        eprintln!("Error: Missing Warframe launch command.");
        exit(1);
    };
    let mut command = Command::new(program);
    command.args(program_args);
    command.spawn().unwrap_or_else(|e| {
        eprintln!("Error: Failed to launch Warframe: {e}");
        exit(1);
    })
}

fn spawn_wfm_auto_status(cx: &Handles) {
    let mut rx = cx.events.subscribe();
    let wfm = cx.wfm.clone();
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(DaemonEvent::AccountLogin(_)) => {
                    set_status_if_connected(&wfm, Status::Ingame).await;
                }
                Ok(DaemonEvent::AccountLogout(_)) => {
                    set_status_if_connected(&wfm, Status::Invisible).await;
                }
                Ok(_) => {}
                Err(RecvError::Lagged(n)) => {
                    log::warn!("WFM auto-status missed {n} events");
                }
                Err(RecvError::Closed) => break,
            }
        }
    });
}

/// Wait for the game to exit or Ctrl+C; returns whether the game exited.
async fn run_until_shutdown(
    game: process::RunningGame,
    lifecycle: &GameLifecycleTracker,
    mut log_watcher: JoinHandle<()>,
) -> bool {
    let game_exit = wait_for_game_exit(game, lifecycle);
    tokio::pin!(game_exit);

    tokio::select! {
        _ = signal::ctrl_c() => {
            log::info!("Received Ctrl+C, shutting down...");
            false
        }
        watcher = &mut log_watcher => {
            if let Err(e) = watcher {
                log::error!("Log watcher task failed: {e}");
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
        () = &mut game_exit => true,
    }
}

/// Parse CLI arguments, launch Warframe, and run the daemon until the game
/// exits or Ctrl+C is received.
pub async fn run() {
    let cli = Cli::parse();
    let auto_callbacks = auto_callbacks_from_env();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Warframe Account Info Scanner started");

    let cx = Handles::new(ScreenshotConfig {
        wayland_capture: if cli.screenshot.native_wayland_screenshot {
            WaylandCapture::ForceNative
        } else {
            WaylandCapture::PreferXWayland
        },
    });

    if let Err(e) = item_data_fetch::update_cache(&cx.http).await {
        log::warn!("Failed to update item data cache: {e}");
    }

    try_restore_session(&cx.wfm).await;

    let _control_server = start_control_api(cli.server, &cx).await;

    #[cfg(windows)]
    let mut log_source = DbwinLogSource::new().unwrap_or_else(|e| {
        eprintln!("Error: Failed to start DBWIN monitor: {e}");
        exit(1);
    });

    let existing_warframe_pids: HashSet<u32> =
        process::get_all_warframe_pids().into_iter().collect();

    let mut child = launch_warframe(&cli.warframe_cmd);

    let launcher_pid = child.id().unwrap_or_else(|| {
        eprintln!("Error: Warframe launched without a PID.");
        exit(1);
    });
    log::info!("Warframe launcher spawned with PID: {launcher_pid}");

    let mut child_handle = tokio::spawn(async move { child.wait().await });

    let launcher = process::Launcher::new(launcher_pid, existing_warframe_pids);
    let game = wait_for_game_start_or_launcher_exit(launcher, &mut child_handle).await;

    #[cfg(windows)]
    {
        log::info!(
            "Using Warframe game PID for DBWIN filtering: {}",
            game.pid()
        );
        log_source.set_pid_filter(game.pid());
    }

    #[cfg(unix)]
    let log_source = WineDbwinBridgeSource::spawn_for_game(
        game.pid(),
        include_bytes!(env!("WF_DBWIN_BRIDGE_EXE")),
    )
    .unwrap_or_else(|e| {
        eprintln!("Error: Failed to launch DBWIN bridge in the game's wine prefix: {e}");
        exit(1);
    });

    emit_game_start(&cx.events);

    if auto_callbacks == AutoCallbacks::Skip {
        log::info!("Skipping auto set of warframe market status...");
    } else {
        spawn_wfm_auto_status(&cx);
    }

    let lifecycle = GameLifecycleTracker::default();
    let watcher_lifecycle = lifecycle.clone();
    let watcher_events = cx.events.clone();
    let watcher_http = cx.http.clone();
    let watcher_screenshot = Arc::clone(&cx.screenshot);
    let game_pid = game.pid();
    let log_watcher = tokio::spawn(async move {
        if let Err(e) = watcher::observe_warframe_activity_with_lifecycle(
            watcher_events,
            watcher_http,
            watcher_screenshot,
            log_source,
            Some(game_pid),
            auto_callbacks,
            watcher_lifecycle,
        )
        .await
        {
            log::error!("Error reading live log source: {e}");
        }
    });

    if run_until_shutdown(game, &lifecycle, log_watcher).await {
        handle_game_exit(&cx.events, &cx.wfm, &lifecycle, auto_callbacks).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launch_command_is_required() {
        Cli::try_parse_from(["wf-info-daemon"]).unwrap_err();
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
        let bus = EventBus::new();
        let wfm = WfmHandle::spawn();
        let mut events = bus.subscribe();

        emit_game_start(&bus);
        assert!(matches!(
            events.recv().await.unwrap(),
            DaemonEvent::GameStart(_)
        ));

        handle_game_exit(
            &bus,
            &wfm,
            &GameLifecycleTracker::default(),
            AutoCallbacks::Skip,
        )
        .await;
        assert!(matches!(
            events.recv().await.unwrap(),
            DaemonEvent::SystemQuit(SystemQuitEvent {
                reason: wf_control::SystemQuitReason::Unexpected,
                ..
            })
        ));
    }
}
