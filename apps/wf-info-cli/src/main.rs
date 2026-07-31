use clap::{Args, Parser, Subcommand};
use serde_json::{Value, json};
#[cfg(unix)]
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use wf_control::control_ops::ControlOp;
use wf_control::{ControlConfig, ControlEndpoint};

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

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Ping the daemon
    Ping,

    /// Subscribe to events (streaming mode)
    Watch(WatchArgs),

    /// Load inventory data
    #[command(name = "inventory-load")]
    InventoryLoad(InventoryLoadArgs),

    /// Filter inventory items
    #[command(name = "inventory-filter")]
    InventoryFilter(InventoryFilterArgs),

    /// Get inventory metadata
    #[command(name = "inventory-meta")]
    InventoryMeta,

    /// Update inventory stale status
    #[command(name = "inventory-stale")]
    InventoryStale(InventoryStaleArgs),

    /// Refresh inventory from game
    #[command(name = "inventory-refresh")]
    InventoryRefresh(InventoryRefreshArgs),

    /// Trigger a screenshot capture
    Screenshot(ScreenshotArgs),

    /// Query market prices for an item
    #[command(name = "wfm-price")]
    WFMarketPrice(WFMarketPriceArgs),

    /// Refresh warframe.market item cache
    #[command(name = "wfm-refresh")]
    WFMarketRefresh,

    /// Sign in to warframe.market
    #[command(name = "wfm-signin")]
    WfmSignin(WfmSigninArgs),

    /// Sign out from warframe.market
    #[command(name = "wfm-signout")]
    WfmSignout,

    /// Check warframe.market auth status
    #[command(name = "wfm-status")]
    WfmStatus(WfmStatusArgs),

    /// Call a generic operation by name
    Call(CallArgs),
}

#[derive(Args, Debug, Clone)]
struct WatchArgs {
    /// Comma-separated list of events to subscribe to
    /// (game_start, account_login, account_logout, system_quit, inventory_fetched,
    /// inventory_stale, profile_updated, screenshot_triggered)
    #[arg(long, value_delimiter = ',')]
    events: Option<Vec<String>>,
}

#[derive(Args, Debug, Clone)]
struct InventoryLoadArgs {
    /// Path to inventory JSON file
    #[arg(long)]
    path: Option<String>,

    /// Raw JSON string
    #[arg(long)]
    raw: Option<String>,

    /// JSON value to load
    #[arg(long, value_parser = parse_json_value)]
    json: Option<Value>,

    /// Save inventory to disk
    #[arg(long)]
    save: Option<bool>,

    /// Source identifier
    #[arg(long)]
    source: Option<String>,

    /// Treat the file as AES-128-CBC encrypted
    #[arg(long)]
    encrypted: bool,
}

#[derive(Args, Debug, Clone)]
struct InventoryFilterArgs {
    /// Filter by category
    #[arg(long)]
    category: Option<String>,

    /// Filter by item type
    #[arg(long)]
    item_type: Option<String>,

    /// Filter items containing text
    #[arg(long)]
    contains: Option<String>,

    /// Limit number of results
    #[arg(long)]
    limit: Option<u64>,

    /// Offset for pagination
    #[arg(long)]
    offset: Option<u64>,

    /// Include detailed item information
    #[arg(long)]
    include_details: bool,

    /// Include warframe.market price data
    #[arg(long)]
    include_market: bool,

    /// Path filter
    #[arg(long)]
    path: Option<String>,

    /// Treat the file as AES-128-CBC encrypted
    #[arg(long)]
    encrypted: bool,
}

#[derive(Args, Debug, Clone)]
struct InventoryStaleArgs {
    /// Timestamp for stale marker
    #[arg(long, value_parser = parse_jsonish_clap)]
    timestamp: Option<Value>,

    /// Reason for marking stale
    #[arg(long)]
    reason: Option<String>,
}

#[derive(Args, Debug, Clone)]
struct InventoryRefreshArgs {
    /// Number of scan retries
    #[arg(long)]
    scan_retries: Option<u64>,

    /// Delay between scans in milliseconds
    #[arg(long)]
    scan_delay_ms: Option<u64>,

    /// Disable saving after refresh
    #[arg(long)]
    no_save: bool,

    /// Source identifier
    #[arg(long)]
    source: Option<String>,
}

#[derive(Args, Debug, Clone)]
struct ScreenshotArgs {
    /// Additional metadata (JSON)
    #[arg(long, value_parser = parse_jsonish_clap)]
    metadata: Option<Value>,
}

#[derive(Args, Debug, Clone)]
struct WFMarketPriceArgs {
    /// Item type (gameRef / unique_name path)
    #[arg(long)]
    item_type: Option<String>,

    /// Text search against warframe.market item names
    #[arg(long)]
    search: Option<String>,

    /// Include set component prices and inventory counts
    #[arg(long)]
    include_parts: Option<bool>,
}

#[derive(Args, Debug, Clone)]
struct WfmSigninArgs {
    /// Account email
    #[arg(long)]
    email: String,

    /// Account password
    #[arg(long)]
    password: String,

    /// Client ID
    #[arg(long, default_value = "wf-info-2")]
    client_id: String,

    /// Device name
    #[arg(long)]
    device_name: Option<String>,
}

#[derive(Args, Debug, Clone)]
struct WfmStatusArgs {
    /// Set status (online, invisible, ingame)
    #[arg(long)]
    status: Option<String>,

    /// Set status duration (seconds), use "null" to clear
    #[arg(long, value_parser = parse_jsonish_clap)]
    duration: Option<Value>,
}

#[derive(Args, Debug, Clone)]
struct CallArgs {
    /// Operation name to call
    op: String,

    /// Parameters as JSON
    #[arg(long, value_parser = parse_jsonish_clap)]
    params: Option<Value>,
}

// Internal types for request handling

#[derive(Debug)]
struct CliConfig {
    tcp_addr: Option<String>,
    #[cfg(unix)]
    unix_path: Option<PathBuf>,
    #[cfg(windows)]
    npipe: Option<String>,
    pretty: bool,
    id: Option<String>,
}

#[derive(Debug, Clone)]
enum CliOp {
    Known(ControlOp),
    Call(String),
}

impl CliOp {
    fn op_string(&self) -> String {
        match self {
            Self::Known(op) => op.to_string(),
            Self::Call(op) => op.clone(),
        }
    }
}

#[derive(Debug)]
struct Command {
    op: CliOp,
    params: Option<Value>,
}

#[derive(Debug)]
struct WatchConfig {
    events: Option<Vec<String>>,
}

#[derive(Debug)]
enum CliMode {
    Request(Command),
    Watch(WatchConfig),
}

// Value parsers for clap

fn parse_jsonish_clap(raw: &str) -> Result<Value, String> {
    Ok(parse_jsonish(raw))
}

fn parse_json_value(raw: &str) -> Result<Value, String> {
    serde_json::from_str(raw).map_err(|e| format!("Invalid JSON: {}", e))
}

fn parse_jsonish(raw: &str) -> Value {
    if let Ok(v) = serde_json::from_str(raw) {
        v
    } else if let Ok(num) = raw.parse::<i64>() {
        Value::Number(num.into())
    } else if let Ok(b) = raw.parse::<bool>() {
        Value::Bool(b)
    } else {
        Value::String(raw.to_string())
    }
}

// Conversion implementations

impl ConnectionArgs {
    fn into_cli_config(self, pretty: bool, id: Option<String>) -> anyhow::Result<CliConfig> {
        let mut cfg = CliConfig {
            tcp_addr: self.tcp,
            #[cfg(unix)]
            unix_path: self.unix,
            #[cfg(windows)]
            npipe: self.npipe,
            pretty,
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

        if should_load_defaults
            && let Some(default_cfg) = ControlConfig::from_env() {
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
    fn into_cli_mode(self) -> CliMode {
        match self {
            Commands::Watch(args) => CliMode::Watch(WatchConfig {
                events: args.events,
            }),
            _ => CliMode::Request(self.into_command()),
        }
    }

    fn into_command(self) -> Command {
        match self {
            Commands::Ping => Command {
                op: CliOp::Known(ControlOp::Ping),
                params: None,
            },
            Commands::InventoryLoad(args) => Command {
                op: CliOp::Known(ControlOp::InventoryLoad),
                params: Some(args.into_params()),
            },
            Commands::InventoryFilter(args) => Command {
                op: CliOp::Known(ControlOp::InventoryFilter),
                params: Some(args.into_params()),
            },
            Commands::InventoryMeta => Command {
                op: CliOp::Known(ControlOp::InventoryMetaGet),
                params: None,
            },
            Commands::InventoryStale(args) => Command {
                op: CliOp::Known(ControlOp::InventoryStaleUpdate),
                params: Some(args.into_params()),
            },
            Commands::InventoryRefresh(args) => Command {
                op: CliOp::Known(ControlOp::InventoryRefresh),
                params: Some(args.into_params()),
            },
            Commands::Screenshot(args) => Command {
                op: CliOp::Known(ControlOp::ScreenshotTrigger),
                params: Some(args.into_params()),
            },
            Commands::WFMarketPrice(args) => Command {
                op: CliOp::Known(ControlOp::WFMarketPrice),
                params: Some(args.into_params()),
            },
            Commands::WFMarketRefresh => Command {
                op: CliOp::Known(ControlOp::WFMarketRefresh),
                params: None,
            },
            Commands::WfmSignin(args) => {
                let mut params = json!({
                    "email": args.email,
                    "password": args.password,
                    "client_id": args.client_id,
                });
                if let Some(name) = args.device_name {
                    params["device_name"] = Value::String(name);
                }
                Command {
                    op: CliOp::Known(ControlOp::WfmSignin),
                    params: Some(params),
                }
            }
            Commands::WfmSignout => Command {
                op: CliOp::Known(ControlOp::WfmSignout),
                params: None,
            },
            Commands::WfmStatus(args) => {
                let mut params = json!({});
                if let Some(s) = args.status {
                    params["status"] = Value::String(s);
                }
                if let Some(d) = args.duration {
                    params["duration"] = d;
                }
                Command {
                    op: CliOp::Known(ControlOp::WfmSignstatus),
                    params: Some(params),
                }
            }
            Commands::Call(args) => Command {
                op: CliOp::Call(args.op),
                params: Some(args.params.unwrap_or_else(|| json!({}))),
            },
            Commands::Watch(_) => unreachable!("Watch handled separately in into_cli_mode"),
        }
    }
}

impl InventoryLoadArgs {
    fn into_params(self) -> Value {
        let mut params = json!({});
        if let Some(v) = self.path {
            params["path"] = Value::String(v);
        }
        if let Some(v) = self.raw {
            params["raw"] = Value::String(v);
        }
        if let Some(v) = self.json {
            params["json"] = v;
        }
        if let Some(v) = self.save {
            params["save"] = Value::Bool(v);
        }
        if let Some(v) = self.source {
            params["source"] = Value::String(v);
        }
        if self.encrypted {
            params["encrypted"] = Value::Bool(true);
        }
        params
    }
}

impl InventoryFilterArgs {
    fn into_params(self) -> Value {
        let mut params = json!({});
        if let Some(v) = self.category {
            params["category"] = Value::String(v);
        }
        if let Some(v) = self.item_type {
            params["item_type"] = Value::String(v);
        }
        if let Some(v) = self.contains {
            params["contains"] = Value::String(v);
        }
        if let Some(v) = self.limit {
            params["limit"] = Value::Number(v.into());
        }
        if let Some(v) = self.offset {
            params["offset"] = Value::Number(v.into());
        }
        if self.include_details {
            params["include_details"] = Value::Bool(true);
        }
        if self.include_market {
            params["include_market"] = Value::Bool(true);
        }
        if let Some(v) = self.path {
            params["path"] = Value::String(v);
        }
        if self.encrypted {
            params["encrypted"] = Value::Bool(true);
        }
        params
    }
}

impl InventoryStaleArgs {
    fn into_params(self) -> Value {
        let mut params = json!({});
        if let Some(v) = self.timestamp {
            params["timestamp"] = v;
        }
        if let Some(v) = self.reason {
            params["reason"] = Value::String(v);
        }
        params
    }
}

impl InventoryRefreshArgs {
    fn into_params(self) -> Value {
        let mut params = json!({});
        if let Some(v) = self.scan_retries {
            params["scan_retries"] = Value::Number(v.into());
        }
        if let Some(v) = self.scan_delay_ms {
            params["scan_delay_ms"] = Value::Number(v.into());
        }
        if self.no_save {
            params["save"] = Value::Bool(false);
        }
        if let Some(v) = self.source {
            params["source"] = Value::String(v);
        }
        params
    }
}

impl ScreenshotArgs {
    fn into_params(self) -> Value {
        let mut params = json!({});
        if let Some(v) = self.metadata {
            params["metadata"] = v;
        }
        params
    }
}

impl WFMarketPriceArgs {
    fn into_params(self) -> Value {
        let mut params = json!({});
        if let Some(v) = self.item_type {
            params["item_type"] = Value::String(v);
        }
        if let Some(v) = self.search {
            params["search"] = Value::String(v);
        }
        if let Some(v) = self.include_parts {
            params["include_parts"] = Value::Bool(v);
        }
        params
    }
}

// Main

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let cfg = cli.connection.into_cli_config(cli.pretty, cli.id)?;
    let mode = cli.command.into_cli_mode();

    match mode {
        CliMode::Request(cmd) => {
            let response = send_request(&cfg, cmd).await?;

            if cfg.pretty {
                let value: Value = serde_json::from_str(&response)?;
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!("{}", response.trim_end());
            }
        }
        CliMode::Watch(watch_cfg) => {
            run_watch(&cfg, watch_cfg).await?;
        }
    }

    Ok(())
}

// Network functions

async fn send_request(cfg: &CliConfig, cmd: Command) -> anyhow::Result<String> {
    let request = json!({
        "id": cfg.id,
        "op": cmd.op.op_string(),
        "params": cmd.params,
    });
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

async fn run_watch(cfg: &CliConfig, watch_cfg: WatchConfig) -> anyhow::Result<()> {
    let request = json!({
        "op": "subscribe",
        "params": {
            "events": watch_cfg.events,
        },
    });
    let payload = serde_json::to_string(&request)?;

    if let Some(addr) = cfg.tcp_addr.as_ref() {
        let stream = TcpStream::connect(&addr).await?;
        return watch_stream(stream, &payload, cfg.pretty).await;
    }

    #[cfg(windows)]
    {
        if let Some(pipe) = cfg.npipe.as_ref() {
            let pipe = normalize_npipe_path(pipe);
            let stream = ClientOptions::new().open(&pipe)?;
            return watch_stream(stream, &payload, cfg.pretty).await;
        }
    }

    #[cfg(unix)]
    {
        if let Some(path) = cfg.unix_path.as_ref() {
            let stream = UnixStream::connect(&path).await?;
            return watch_stream(stream, &payload, cfg.pretty).await;
        }
    }

    anyhow::bail!("No valid connection target")
}

async fn watch_stream<S>(mut stream: S, subscribe_payload: &str, pretty: bool) -> anyhow::Result<()>
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
    let response: Value = serde_json::from_str(&line)?;

    if response.get("ok") != Some(&Value::Bool(true)) {
        let error = response
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or("Unknown error");
        anyhow::bail!("Subscribe failed: {}", error);
    }

    if pretty {
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

        if pretty {
            if let Ok(value) = serde_json::from_str::<Value>(trimmed) {
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else {
                println!("{}", trimmed);
            }
        } else {
            println!("{}", trimmed);
        }
    }

    Ok(())
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
