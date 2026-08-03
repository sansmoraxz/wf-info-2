use std::env;
#[cfg(unix)]
use std::fs;
use std::io;
#[cfg(unix)]
use std::path::PathBuf;

use tokio::io::{
    AsyncBufReadExt as _, AsyncRead, AsyncWrite, AsyncWriteExt as _, BufReader, Lines, split,
};
use tokio::net::TcpListener;
use tokio::sync::broadcast::error::RecvError;
use tokio::task::JoinHandle;

#[cfg(unix)]
use tokio::net::UnixListener;
#[cfg(windows)]
use tokio::net::windows::named_pipe::ServerOptions;

use super::events::{DaemonEvent, EventMessage};
use super::requests::{self, Handles};
use super::subscription::EventFilter;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Failed to bind TCP control socket at {addr}")]
    BindTcp {
        addr: String,
        #[source]
        source: io::Error,
    },
    #[cfg(unix)]
    #[error("Failed to create unix socket dir {path}")]
    CreateSocketDir {
        path: String,
        #[source]
        source: io::Error,
    },
    #[cfg(unix)]
    #[error("Failed to remove existing unix socket {path}")]
    RemoveStaleSocket {
        path: String,
        #[source]
        source: io::Error,
    },
    #[cfg(unix)]
    #[error("Failed to bind unix control socket {path}")]
    BindUnix {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("Failed to serialize response")]
    SerializeResponse(#[source] serde_json::Error),
    #[error("Failed to serialize event")]
    SerializeEvent(#[source] serde_json::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Debug, Clone)]
pub enum ControlEndpoint {
    Tcp(String),
    #[cfg(unix)]
    Unix(PathBuf),
    #[cfg(windows)]
    Npipe(String),
}

#[derive(Debug, Clone)]
pub struct ControlConfig {
    pub endpoints: Vec<ControlEndpoint>,
}

impl ControlConfig {
    #[must_use]
    pub fn from_env() -> Option<Self> {
        let mut endpoints = Vec::new();

        if let Ok(addr) = env::var("WF_INFO_API_TCP") {
            endpoints.push(ControlEndpoint::Tcp(addr));
        }

        #[cfg(unix)]
        {
            if let Ok(path) = env::var("WF_INFO_API_UNIX") {
                endpoints.push(ControlEndpoint::Unix(PathBuf::from(path)));
            }
        }

        #[cfg(windows)]
        {
            if let Ok(pipe) = env::var("WF_INFO_API_NPIPE") {
                endpoints.push(ControlEndpoint::Npipe(pipe));
            }
        }

        if endpoints.is_empty() {
            endpoints.extend(default_control_endpoints());
        }

        if endpoints.is_empty() {
            None
        } else {
            Some(Self { endpoints })
        }
    }
}

#[derive(Default)]
pub struct ControlServer {
    pub(crate) _handles: Vec<JoinHandle<()>>,
    // Keep guards alive for cleanup on drop
    #[cfg(unix)]
    _unix_guards: Vec<UnixSocketGuard>,
}

impl ControlServer {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            _handles: Vec::new(),
            #[cfg(unix)]
            _unix_guards: Vec::new(),
        }
    }
}

#[cfg(unix)]
#[derive(Debug)]
struct UnixSocketGuard {
    path: PathBuf,
}

#[cfg(unix)]
impl Drop for UnixSocketGuard {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.path)
            && e.kind() != io::ErrorKind::NotFound
        {
            log::error!(
                "Failed to cleanup unix socket {}: {}",
                self.path.display(),
                e
            );
        }
    }
}

pub async fn start_control_server_from_env(cx: Handles) -> Result<ControlServer, ServerError> {
    let Some(cfg) = ControlConfig::from_env() else {
        return Ok(ControlServer::empty());
    };
    start_control_server(cfg, cx).await
}

pub async fn start_control_server(
    cfg: ControlConfig,
    cx: Handles,
) -> Result<ControlServer, ServerError> {
    let mut handles = Vec::new();
    #[cfg(unix)]
    let mut unix_guards = Vec::new();

    for endpoint in cfg.endpoints {
        match endpoint {
            ControlEndpoint::Tcp(addr) => {
                handles.push(spawn_tcp_server(&addr, cx.clone()).await?);
            }
            #[cfg(unix)]
            ControlEndpoint::Unix(path) => {
                let (handle, guard) = spawn_unix_server(path, cx.clone())?;
                handles.push(handle);
                unix_guards.push(guard);
            }
            #[cfg(windows)]
            ControlEndpoint::Npipe(path) => {
                handles.push(spawn_npipe_server(&path, cx.clone()));
            }
        }
    }

    Ok(ControlServer {
        _handles: handles,
        #[cfg(unix)]
        _unix_guards: unix_guards,
    })
}

async fn spawn_tcp_server(addr: &str, cx: Handles) -> Result<JoinHandle<()>, ServerError> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|source| ServerError::BindTcp {
            addr: addr.to_owned(),
            source,
        })?;
    log::info!("Control API listening on tcp {addr}");

    Ok(tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let cx = cx.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_stream(stream, cx).await {
                            log::warn!("Control connection error: {e}");
                        }
                    });
                }
                Err(e) => {
                    log::error!("Control TCP accept error: {e}");
                    break;
                }
            }
        }
    }))
}

#[cfg(windows)]
fn spawn_npipe_server(path: &str, cx: Handles) -> JoinHandle<()> {
    let pipe_path = normalize_npipe_path(path);
    log::info!("Control API listening on npipe {pipe_path}");

    tokio::spawn(async move {
        let mut first_instance = true;

        loop {
            let server = ServerOptions::new()
                .first_pipe_instance(first_instance)
                .create(&pipe_path);

            let server = match server {
                Ok(server) => server,
                Err(e) => {
                    log::error!("Failed to create npipe {pipe_path}: {e}");
                    break;
                }
            };

            first_instance = false;

            match server.connect().await {
                Ok(()) => {
                    let cx = cx.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_stream(server, cx).await {
                            log::warn!("Control connection error: {e}");
                        }
                    });
                }
                Err(e) => {
                    log::error!("Control npipe accept error: {e}");
                    break;
                }
            }
        }
    })
}

#[cfg(unix)]
fn spawn_unix_server(
    path: PathBuf,
    cx: Handles,
) -> Result<(JoinHandle<()>, UnixSocketGuard), ServerError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| ServerError::CreateSocketDir {
            path: parent.display().to_string(),
            source,
        })?;
    }
    if path.exists() {
        fs::remove_file(&path).map_err(|source| ServerError::RemoveStaleSocket {
            path: path.display().to_string(),
            source,
        })?;
    }

    let listener = UnixListener::bind(&path).map_err(|source| ServerError::BindUnix {
        path: path.display().to_string(),
        source,
    })?;
    log::info!("Control API listening on unix {}", path.display());
    let guard = UnixSocketGuard { path };

    let handle = tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let cx = cx.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_stream(stream, cx).await {
                            log::warn!("Control connection error: {e}");
                        }
                    });
                }
                Err(e) => {
                    log::error!("Control unix accept error: {e}");
                    break;
                }
            }
        }
    });

    Ok((handle, guard))
}

async fn handle_stream<T>(stream: T, cx: Handles) -> Result<(), ServerError>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let (reader, mut writer) = split(stream);
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let outcome = requests::handle_line(&cx, line).await;

        let payload =
            serde_json::to_string(outcome.response()).map_err(ServerError::SerializeResponse)?;
        writer.write_all(payload.as_bytes()).await?;
        writer.write_all(b"\n").await?;

        if let requests::HandleOutcome::EnterSubscription { filter, .. } = outcome {
            handle_subscription_mode(&cx, &mut lines, &mut writer, filter).await?;
            return Ok(());
        }
    }

    Ok(())
}

async fn event_writer<W>(event: DaemonEvent, writer: &mut W) -> Result<(), ServerError>
where
    W: AsyncWrite + Unpin,
{
    let msg = EventMessage::from(event);
    let payload = serde_json::to_string(&msg).map_err(ServerError::SerializeEvent)?;
    writer.write_all(payload.as_bytes()).await?;
    writer.write_all(b"\n").await?;

    Ok(())
}

async fn handle_subscription_mode<R, W>(
    cx: &Handles,
    lines: &mut Lines<BufReader<R>>,
    writer: &mut W,
    filter: EventFilter,
) -> Result<(), ServerError>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut receiver = cx.events.subscribe();

    loop {
        tokio::select! {
            // Handle incoming events from broadcast channel
            event_result = receiver.recv() => {
                match event_result {
                    Ok(event) => {
                        if filter.matches(&event)
                            && let Err(e) = event_writer(event, writer).await  {
                                log::error!("Error publishing event {e:?}");
                            }
                    }
                    Err(RecvError::Lagged(count)) => {
                        log::warn!("Subscription client lagged, missed {count} events");
                    }
                    Err(RecvError::Closed) => {
                        log::debug!("Broadcast channel closed");
                        break;
                    }
                }
            }

            // Handle incoming client messages (ping, disconnect)
            line_result = lines.next_line() => {
                match line_result {
                    Ok(Some(line)) => {
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }

                        // Handle regular requests while subscribed (e.g., ping)
                        let outcome = requests::handle_line(cx, line).await;
                        let payload = serde_json::to_string(outcome.response())
                            .map_err(ServerError::SerializeResponse)?;
                        writer.write_all(payload.as_bytes()).await?;
                        writer.write_all(b"\n").await?;
                    }
                    Ok(None) => {
                        // Client disconnected
                        log::debug!("Subscribed client disconnected");
                        break;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(unix)]
fn default_unix_socket_path() -> Option<PathBuf> {
    let base = dirs::runtime_dir()
        .or_else(dirs::cache_dir)
        .or_else(dirs::data_dir)?;
    Some(base.join("wf-info-2").join("control.sock"))
}

#[cfg(windows)]
fn default_control_endpoints() -> Vec<ControlEndpoint> {
    vec![ControlEndpoint::Npipe(default_npipe_path())]
}

#[cfg(unix)]
fn default_control_endpoints() -> Vec<ControlEndpoint> {
    default_unix_socket_path()
        .into_iter()
        .map(ControlEndpoint::Unix)
        .collect()
}

#[cfg(all(not(unix), not(windows)))]
fn default_control_endpoints() -> Vec<ControlEndpoint> {
    vec![ControlEndpoint::Tcp(default_tcp_addr())]
}

#[cfg(all(not(unix), not(windows)))]
fn default_tcp_addr() -> String {
    "127.0.0.1:47410".to_string()
}

#[cfg(windows)]
fn default_npipe_path() -> String {
    "wf-info-2-control".to_owned()
}

#[cfg(windows)]
fn normalize_npipe_path(pipe: impl AsRef<str>) -> String {
    let raw = pipe.as_ref();
    let lower = raw.to_ascii_lowercase();
    if lower.starts_with(r"\\.\pipe\") {
        raw.to_owned()
    } else {
        format!(r"\\.\pipe\{}", raw.trim_start_matches(['\\', '/']))
    }
}
