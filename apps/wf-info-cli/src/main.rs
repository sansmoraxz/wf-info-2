use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use serde_json::Value;
use serde_json::value::{RawValue, to_raw_value};
#[cfg(unix)]
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt as _, AsyncRead, AsyncWrite, AsyncWriteExt as _, BufReader};
use tokio::net::TcpStream;
use wf_control::control_ops::{ControlOp, InventoryOp, ScreenshotOp, WfmOp};
use wf_control::{
    ControlConfig, ControlEndpoint, FilterParams, LoadInventoryParams, MarketPriceParams,
    RefreshParams, Request, ResponseEnvelope, ScreenshotParams, SigninParams, SignstatusParams,
    StaleParams, SubscribeParams,
};

#[cfg(unix)]
use tokio::net::UnixStream;
#[cfg(windows)]
use tokio::net::windows::named_pipe::ClientOptions;

/// CLI client for the wf-info-2 daemon
#[derive(Parser, Debug)]
#[command(name = "wf-info-cli")]
#[command(about = "CLI client for the wf-info-2 daemon")]
struct Cli {
    #[command(flatten)]
    connection: ConnectionArgs,

    /// Pretty-print JSON output
    #[arg(long, global = true)]
    pretty: bool,

    /// Request ID to use
    #[arg(long, global = true)]
    id: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug, Clone)]
struct ConnectionArgs {
    /// TCP address to connect to
    #[arg(long, env = "WF_INFO_API_TCP")]
    tcp: Option<String>,

    /// Unix socket path
    #[cfg(unix)]
    #[arg(long, env = "WF_INFO_API_UNIX")]
    unix: Option<PathBuf>,

    /// Named pipe name
    #[cfg(windows)]
    #[arg(long, env = "WF_INFO_API_NPIPE")]
    npipe: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Ping the daemon
    Ping,

    /// Subscribe to events (streaming mode)
    Watch(SubscribeParams),

    /// Load inventory data
    #[command(name = "inventory-load")]
    InventoryLoad(LoadInventoryParams),

    /// Filter inventory items
    #[command(name = "inventory-filter")]
    InventoryFilter(FilterParams),

    /// Get inventory metadata
    #[command(name = "inventory-meta")]
    InventoryMeta,

    /// Update inventory stale status
    #[command(name = "inventory-stale")]
    InventoryStale(StaleParams),

    /// Refresh inventory from game
    #[command(name = "inventory-refresh")]
    InventoryRefresh(RefreshParams),

    /// Trigger a screenshot capture
    Screenshot(ScreenshotParams),

    /// Query market prices for an item
    #[command(name = "wfm-price")]
    WFMarketPrice(MarketPriceParams),

    /// Refresh warframe.market item cache
    #[command(name = "wfm-refresh")]
    WFMarketRefresh,

    /// Sign in to warframe.market
    #[command(name = "wfm-signin")]
    WfmSignin(SigninParams),

    /// Sign out from warframe.market
    #[command(name = "wfm-signout")]
    WfmSignout,

    /// Check warframe.market auth status
    #[command(name = "wfm-status")]
    WfmStatus(SignstatusParams),

    /// Call a generic operation by name
    Call(CallArgs),
}

#[derive(Args, Debug, Clone)]
struct CallArgs {
    /// Operation name to call
    op: String,

    /// Parameters as JSON
    #[arg(long, value_parser = parse_json_value)]
    params: Option<Box<RawValue>>,
}

// Internal types for request handling

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Compact,
    Pretty,
}

impl OutputFormat {
    fn print_json_line(self, raw: &str) {
        match self {
            // Stream-transcode text -> pretty text: payloads can embed
            // megabytes of base64 (screenshots), so never materialize a
            // Value tree just to re-indent.
            Self::Pretty => match pretty_print_json(raw) {
                Ok(pretty) => println!("{pretty}"),
                Err(_) => println!("{raw}"),
            },
            Self::Compact => println!("{raw}"),
        }
    }
}

#[derive(Debug)]
struct CliConfig {
    tcp_addr: Option<String>,
    #[cfg(unix)]
    unix_path: Option<PathBuf>,
    #[cfg(windows)]
    npipe: Option<String>,
    output: OutputFormat,
    id: Option<String>,
}

#[derive(Debug)]
enum CliMode {
    Request {
        op: String,
        params: Option<Box<RawValue>>,
    },
    Watch(SubscribeParams),
}

// Conversion implementations

impl ConnectionArgs {
    fn into_cli_config(
        self,
        output: OutputFormat,
        id: Option<String>,
    ) -> anyhow::Result<CliConfig> {
        let mut cfg = CliConfig {
            tcp_addr: self.tcp,
            #[cfg(unix)]
            unix_path: self.unix,
            #[cfg(windows)]
            npipe: self.npipe,
            output,
            id,
        };

        let should_load_defaults = {
            #[cfg(windows)]
            {
                cfg.tcp_addr.is_none() && cfg.npipe.is_none()
            }
            #[cfg(unix)]
            {
                cfg.tcp_addr.is_none() && cfg.unix_path.is_none()
            }
            #[cfg(all(not(unix), not(windows)))]
            {
                cfg.tcp_addr.is_none()
            }
        };

        if should_load_defaults && let Some(default_cfg) = ControlConfig::from_env() {
            for endpoint in default_cfg.endpoints {
                match endpoint {
                    ControlEndpoint::Tcp(addr) if cfg.tcp_addr.is_none() => {
                        cfg.tcp_addr = Some(addr);
                    }
                    #[cfg(unix)]
                    ControlEndpoint::Unix(path) if cfg.unix_path.is_none() => {
                        cfg.unix_path = Some(path);
                    }
                    #[cfg(windows)]
                    ControlEndpoint::Npipe(pipe) if cfg.npipe.is_none() => {
                        cfg.npipe = Some(pipe);
                    }
                    _ => {}
                }
                #[cfg(windows)]
                {
                    if cfg.tcp_addr.is_some() && cfg.npipe.is_some() {
                        break;
                    }
                }
                #[cfg(unix)]
                {
                    if cfg.tcp_addr.is_some() && cfg.unix_path.is_some() {
                        break;
                    }
                }
                #[cfg(all(not(unix), not(windows)))]
                {
                    if cfg.tcp_addr.is_some() {
                        break;
                    }
                }
            }
        }

        let missing_target = {
            #[cfg(windows)]
            {
                cfg.tcp_addr.is_none() && cfg.npipe.is_none()
            }
            #[cfg(unix)]
            {
                cfg.tcp_addr.is_none() && cfg.unix_path.is_none()
            }
            #[cfg(all(not(unix), not(windows)))]
            {
                cfg.tcp_addr.is_none()
            }
        };

        if missing_target {
            let missing_msg = if cfg!(windows) {
                "Missing connection target: set WF_INFO_API_TCP/WF_INFO_API_NPIPE or rely on defaults"
            } else if cfg!(unix) {
                "Missing connection target: set WF_INFO_API_TCP/WF_INFO_API_UNIX or rely on defaults"
            } else {
                "Missing connection target: set WF_INFO_API_TCP or rely on defaults"
            };
            anyhow::bail!(missing_msg);
        }

        Ok(cfg)
    }
}

impl Commands {
    fn into_cli_mode(self) -> anyhow::Result<CliMode> {
        const NO_PARAMS: Option<Value> = None;
        match self {
            Self::Watch(params) => Ok(CliMode::Watch(params)),
            Self::Ping => request_mode(ControlOp::Ping, NO_PARAMS),
            Self::InventoryLoad(params) => {
                request_mode(ControlOp::Inventory(InventoryOp::Load), Some(params))
            }
            Self::InventoryFilter(params) => {
                request_mode(ControlOp::Inventory(InventoryOp::Filter), Some(params))
            }
            Self::InventoryMeta => {
                request_mode(ControlOp::Inventory(InventoryOp::MetaGet), NO_PARAMS)
            }
            Self::InventoryStale(params) => {
                request_mode(ControlOp::Inventory(InventoryOp::StaleUpdate), Some(params))
            }
            Self::InventoryRefresh(params) => {
                request_mode(ControlOp::Inventory(InventoryOp::Refresh), Some(params))
            }
            Self::Screenshot(params) => {
                request_mode(ControlOp::Screenshot(ScreenshotOp::Trigger), Some(params))
            }
            Self::WFMarketPrice(params) => request_mode(ControlOp::Wfm(WfmOp::Price), Some(params)),
            Self::WFMarketRefresh => request_mode(ControlOp::Wfm(WfmOp::Refresh), NO_PARAMS),
            Self::WfmSignin(params) => request_mode(ControlOp::Wfm(WfmOp::Signin), Some(params)),
            Self::WfmSignout => request_mode(ControlOp::Wfm(WfmOp::Signout), NO_PARAMS),
            Self::WfmStatus(params) => {
                request_mode(ControlOp::Wfm(WfmOp::Signstatus), Some(params))
            }
            Self::Call(args) => Ok(CliMode::Request {
                op: args.op,
                params: Some(match args.params {
                    Some(params) => params,
                    None => to_raw_value(&serde_json::Map::new())?,
                }),
            }),
        }
    }
}

// Value parsers for clap

/// Validates the argument is JSON but keeps it as unparsed text: it is only
/// ever re-embedded verbatim into the outgoing request.
fn parse_json_value(raw: &str) -> Result<Box<RawValue>, String> {
    RawValue::from_string(raw.to_owned()).map_err(|e| format!("Invalid JSON: {e}"))
}

fn pretty_print_json(raw: &str) -> Result<String, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(raw);
    let mut out = Vec::with_capacity(raw.len());
    let mut serializer = serde_json::Serializer::pretty(&mut out);
    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
    deserializer.end()?;
    // serde_json never emits invalid UTF-8
    Ok(String::from_utf8_lossy(&out).into_owned())
}

fn request_mode(op: ControlOp, params: Option<impl Serialize>) -> anyhow::Result<CliMode> {
    Ok(CliMode::Request {
        op: op.to_string(),
        params: params.map(|params| to_raw_value(&params)).transpose()?,
    })
}

// Main

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let output = if cli.pretty {
        OutputFormat::Pretty
    } else {
        OutputFormat::Compact
    };
    let cfg = cli.connection.into_cli_config(output, cli.id)?;

    match cli.command.into_cli_mode()? {
        CliMode::Request { op, params } => {
            let response = send_request(&cfg, op, params).await?;
            cfg.output.print_json_line(response.trim_end());
        }
        CliMode::Watch(params) => {
            run_watch(&cfg, params).await?;
        }
    }

    Ok(())
}

// Network functions

async fn send_request(
    cfg: &CliConfig,
    op: String,
    params: Option<Box<RawValue>>,
) -> anyhow::Result<String> {
    let request = Request {
        id: cfg.id.clone(),
        op,
        params,
    };
    let payload = serde_json::to_string(&request)?;

    if let Some(addr) = cfg.tcp_addr.as_ref() {
        let mut stream = TcpStream::connect(&addr).await?;
        stream.write_all(payload.as_bytes()).await?;
        stream.write_all(b"\n").await?;
        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        return Ok(line);
    }

    #[cfg(windows)]
    {
        if let Some(pipe) = cfg.npipe.as_ref() {
            let pipe = normalize_npipe_path(pipe);
            let mut stream = ClientOptions::new().open(&pipe)?;
            stream.write_all(payload.as_bytes()).await?;
            stream.write_all(b"\n").await?;
            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            reader.read_line(&mut line).await?;
            return Ok(line);
        }
    }

    #[cfg(unix)]
    {
        if let Some(path) = cfg.unix_path.as_ref() {
            let mut stream = UnixStream::connect(&path).await?;
            stream.write_all(payload.as_bytes()).await?;
            stream.write_all(b"\n").await?;
            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            reader.read_line(&mut line).await?;
            return Ok(line);
        }
    }

    anyhow::bail!("No valid connection target")
}

async fn run_watch(cfg: &CliConfig, params: SubscribeParams) -> anyhow::Result<()> {
    let request = Request {
        id: None,
        op: ControlOp::Subscribe.to_string(),
        params: Some(to_raw_value(&params)?),
    };
    let payload = serde_json::to_string(&request)?;

    if let Some(addr) = cfg.tcp_addr.as_ref() {
        let stream = TcpStream::connect(&addr).await?;
        return watch_stream(stream, &payload, cfg.output).await;
    }

    #[cfg(windows)]
    {
        if let Some(pipe) = cfg.npipe.as_ref() {
            let pipe = normalize_npipe_path(pipe);
            let stream = ClientOptions::new().open(&pipe)?;
            return watch_stream(stream, &payload, cfg.output).await;
        }
    }

    #[cfg(unix)]
    {
        if let Some(path) = cfg.unix_path.as_ref() {
            let stream = UnixStream::connect(&path).await?;
            return watch_stream(stream, &payload, cfg.output).await;
        }
    }

    anyhow::bail!("No valid connection target")
}

async fn watch_stream<S>(
    mut stream: S,
    subscribe_payload: &str,
    output: OutputFormat,
) -> anyhow::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    // Send subscribe request
    stream.write_all(subscribe_payload.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    // Read subscribe response
    reader.read_line(&mut line).await?;
    if let ResponseEnvelope::Err { error, .. } = serde_json::from_str(&line)? {
        anyhow::bail!("Subscribe failed: {error}");
    }

    if output == OutputFormat::Pretty {
        eprintln!("Subscribed. Waiting for events... (Ctrl+C to exit)");
    }

    // Stream events
    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            // Connection closed
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        output.print_json_line(trimmed);
    }

    Ok(())
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
