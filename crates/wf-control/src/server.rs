#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::path::PathBuf;

use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

#[cfg(unix)]
use tokio::net::UnixListener;
#[cfg(windows)]
use tokio::net::windows::named_pipe::ServerOptions;

use super::broadcaster;
use super::events::EventMessage;
use super::requests;
use super::subscription::EventFilter;

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
    pub fn from_env() -> Option<Self> {
        let mut endpoints = Vec::new();

        if let Ok(addr) = std::env::var("WF_INFO_API_TCP") {
            endpoints.push(ControlEndpoint::Tcp(addr));
        }

        #[cfg(unix)]
        {
            if let Ok(path) = std::env::var("WF_INFO_API_UNIX") {
                endpoints.push(ControlEndpoint::Unix(PathBuf::from(path)));
            }
        }

        #[cfg(windows)]
        {
            if let Ok(pipe) = std::env::var("WF_INFO_API_NPIPE") {
                endpoints.push(ControlEndpoint::Npipe(normalize_npipe_path(pipe)));
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
    pub handles: Vec<JoinHandle<()>>,
    // Keep guards alive for cleanup on drop
    #[cfg(unix)]
    _unix_guards: Vec<UnixSocketGuard>,
}

impl ControlServer {
    pub fn empty() -> Self {
        Self {
            handles: Vec::new(),
            #[cfg(unix)]
            _unix_guards: Vec::new(),
        }
    }
}

pub async fn start_control_server_from_env() -> Result<ControlServer> {
    let Some(cfg) = ControlConfig::from_env() else {
        return Ok(ControlServer::empty());
    };
    start_control_server(cfg).await
}

pub async fn start_control_server(cfg: ControlConfig) -> Result<ControlServer> {
    let mut handles = Vec::new();
    #[cfg(unix)]
    let mut unix_guards = Vec::new();

    for endpoint in cfg.endpoints {
        match endpoint {
            ControlEndpoint::Tcp(addr) => {
                handles.push(spawn_tcp_server(addr).await?);
            }
            #[cfg(unix)]
            ControlEndpoint::Unix(path) => {
                let (handle, guard) = spawn_unix_server(path).await?;
                handles.push(handle);
                unix_guards.push(guard);
            }
            #[cfg(windows)]
            ControlEndpoint::Npipe(path) => {
                handles.push(spawn_npipe_server(path).await?);
            }
        }
    }

    Ok(ControlServer {
        handles,
        #[cfg(unix)]
        _unix_guards: unix_guards,
    })
}

async fn spawn_tcp_server(addr: String) -> Result<JoinHandle<()>> {
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind TCP control socket at {}", addr))?;
    log::info!("Control API listening on tcp {}", addr);

    Ok(tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(async move {
                        if let Err(e) = handle_stream(stream).await {
                            log::warn!("Control connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Control TCP accept error: {}", e);
                    break;
                }
            }
        }
    }))
}

#[cfg(windows)]
async fn spawn_npipe_server(path: String) -> Result<JoinHandle<()>> {
    let pipe_path = normalize_npipe_path(path);
    log::info!("Control API listening on npipe {}", pipe_path);

    Ok(tokio::spawn(async move {
        let mut first_instance = true;

        loop {
            let server = ServerOptions::new()
                .first_pipe_instance(first_instance)
                .create(&pipe_path);

            let server = match server {
                Ok(server) => server,
                Err(e) => {
                    log::error!("Failed to create npipe {}: {}", pipe_path, e);
                    break;
                }
            };

            first_instance = false;

            match server.connect().await {
                Ok(()) => {
                    tokio::spawn(async move {
                        if let Err(e) = handle_stream(server).await {
                            log::warn!("Control connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Control npipe accept error: {}", e);
                    break;
                }
            }
        }
    }))
}

#[cfg(unix)]
async fn spawn_unix_server(path: PathBuf) -> Result<(JoinHandle<()>, UnixSocketGuard)> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create unix socket dir {}", parent.display()))?;
    }
    if path.exists() {
        fs::remove_file(&path)
            .with_context(|| format!("Failed to remove existing unix socket {}", path.display()))?;
    }

    let listener = UnixListener::bind(&path)
        .with_context(|| format!("Failed to bind unix control socket {}", path.display()))?;
    log::info!("Control API listening on unix {}", path.display());
    let guard = UnixSocketGuard { path: path.clone() };

    let handle = tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(async move {
                        if let Err(e) = handle_stream(stream).await {
                            log::warn!("Control connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Control unix accept error: {}", e);
                    break;
                }
            }
        }
    });

    Ok((handle, guard))
}

async fn handle_stream<T>(stream: T) -> Result<()>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let (reader, mut writer) = tokio::io::split(stream);
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let response = requests::handle_line(line).await;

        // Check if this is a subscribe response that transitions to subscription mode
        if let Some(filter) = response.subscription_filter.clone() {
            // Send the success response first
            let payload = serde_json::to_string(&response.response)
                .context("Failed to serialize response")?;
            writer.write_all(payload.as_bytes()).await?;
            writer.write_all(b"\n").await?;

            // Enter subscription mode
            handle_subscription_mode(&mut lines, &mut writer, filter).await?;
            return Ok(());
        }

        let payload =
            serde_json::to_string(&response.response).context("Failed to serialize response")?;
        writer.write_all(payload.as_bytes()).await?;
        writer.write_all(b"\n").await?;
    }

    Ok(())
}

async fn event_writer<W>(event: crate::DaemonEvent, writer: &mut W) -> Result<()>
where
    W: AsyncWrite + Unpin,
{
    let msg = EventMessage::from(event);
    let payload =
        serde_json::to_string(&msg).context(format!("Failed to serialize event {:?}", &msg))?;
    writer.write_all(payload.as_bytes()).await?;
    writer.write_all(b"\n").await?;

    Ok(())
}

async fn handle_subscription_mode<R, W>(
    lines: &mut tokio::io::Lines<BufReader<R>>,
    writer: &mut W,
    filter: EventFilter,
) -> Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut receiver = broadcaster::subscribe();

    loop {
        tokio::select! {
            // Handle incoming events from broadcast channel
            event_result = receiver.recv() => {
                match event_result {
                    Ok(event) => {
                        if filter.matches(&event) {
                            if let Err(e) = event_writer(event, writer).await  {
                                log::error!("Error publishing event {:?}", e);
                            }
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(count)) => {
                        log::warn!("Subscription client lagged, missed {} events", count);
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
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
                        let response = requests::handle_line(line).await;
                        let payload = serde_json::to_string(&response.response)
                            .context("Failed to serialize response")?;
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
#[derive(Debug)]
struct UnixSocketGuard {
    path: PathBuf,
}

#[cfg(unix)]
impl Drop for UnixSocketGuard {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.path) {
            if e.kind() != std::io::ErrorKind::NotFound {
                log::debug!(
                    "Failed to cleanup unix socket {}: {}",
                    self.path.display(),
                    e
                );
            }
        }
    }
}

#[cfg(unix)]
fn default_unix_socket_path() -> Option<PathBuf> {
    let base = dirs::runtime_dir()
        .or_else(dirs::cache_dir)
        .or_else(|| dirs::data_dir())?;
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
    normalize_npipe_path("wf-info-2-control")
}

#[cfg(windows)]
fn normalize_npipe_path(pipe: impl AsRef<str>) -> String {
    let raw = pipe.as_ref();
    let lower = raw.to_ascii_lowercase();
    if lower.starts_with(r"\\.\pipe\") {
        raw.to_string()
    } else {
        format!(r"\\.\pipe\{}", raw.trim_start_matches(['\\', '/']))
    }
}
