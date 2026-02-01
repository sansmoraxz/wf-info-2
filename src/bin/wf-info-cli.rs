use serde_json::{Value, json};
use std::env;
#[cfg(unix)]
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use wf_info_2::control::{ControlConfig, ControlEndpoint};
use wf_info_2::control_ops::ControlOp;

#[cfg(unix)]
use tokio::net::UnixStream;
#[cfg(windows)]
use tokio::net::windows::named_pipe::ClientOptions;

#[derive(Debug)]
struct CliConfig {
    tcp_addr: Option<String>,
    #[cfg(unix)]
    unix_path: Option<PathBuf>,
    #[cfg(windows)]
    npipe: Option<String>,
    pretty: bool,
    id: Option<Value>,
}

#[derive(Debug, Clone)]
enum CliOp {
    Known(ControlOp),
    Call(String),
}

impl CliOp {
    fn op_string(&self) -> String {
        match self {
            Self::Known(op) => op.as_str().to_string(),
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        print_usage();
        std::process::exit(2);
    }

    let (cfg, mode) = parse_args(&mut args)?;

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

fn parse_args(args: &mut Vec<String>) -> anyhow::Result<(CliConfig, CliMode)> {
    let mut cfg = CliConfig {
        tcp_addr: None,
        #[cfg(unix)]
        unix_path: None,
        #[cfg(windows)]
        npipe: None,
        pretty: false,
        id: None,
    };

    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--tcp" => {
                idx += 1;
                cfg.tcp_addr = args.get(idx).cloned();
            }
            #[cfg(unix)]
            "--unix" => {
                idx += 1;
                cfg.unix_path = args.get(idx).map(|v| PathBuf::from(v));
            }
            #[cfg(windows)]
            "--npipe" => {
                idx += 1;
                cfg.npipe = args.get(idx).cloned();
            }
            "--pretty" => {
                cfg.pretty = true;
            }
            "--id" => {
                idx += 1;
                if let Some(raw) = args.get(idx) {
                    cfg.id = Some(parse_jsonish(raw));
                }
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            _ => break,
        }
        idx += 1;
    }

    let cmd_args = args[idx..].to_vec();
    if cmd_args.is_empty() {
        print_usage();
        std::process::exit(2);
    }

    let mode = parse_mode(cmd_args)?;

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

    if should_load_defaults {
        if let Some(default_cfg) = ControlConfig::from_env() {
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

    Ok((cfg, mode))
}

fn parse_mode(args: Vec<String>) -> anyhow::Result<CliMode> {
    if args.first().map(|s| s.as_str()) == Some("watch") {
        let mut events = None;
        let rest: Vec<_> = args.into_iter().skip(1).collect();
        let mut idx = 0;
        while idx < rest.len() {
            match rest[idx].as_str() {
                "--events" => {
                    idx += 1;
                    if let Some(v) = rest.get(idx) {
                        events = Some(v.split(',').map(|s| s.trim().to_string()).collect());
                    }
                }
                _ => {}
            }
            idx += 1;
        }
        Ok(CliMode::Watch(WatchConfig { events }))
    } else {
        Ok(CliMode::Request(parse_command(args)?))
    }
}

fn parse_command(args: Vec<String>) -> anyhow::Result<Command> {
    let mut iter = args.into_iter();
    let cmd = iter.next().unwrap();
    let mut params = json!({});

    match cmd.as_str() {
        "ping" => Ok(Command {
            op: CliOp::Known(ControlOp::Ping),
            params: None,
        }),
        "inventory-load" => {
            let mut path = None;
            let mut raw = None;
            let mut json_value = None;
            let mut save = None;
            let mut source = None;

            let rest = iter.collect::<Vec<_>>();
            let mut idx = 0;
            while idx < rest.len() {
                match rest[idx].as_str() {
                    "--path" => {
                        idx += 1;
                        path = rest.get(idx).cloned();
                    }
                    "--raw" => {
                        idx += 1;
                        raw = rest.get(idx).cloned();
                    }
                    "--json" => {
                        idx += 1;
                        json_value = rest.get(idx).map(|v| parse_jsonish(v));
                    }
                    "--save" => {
                        idx += 1;
                        save = rest.get(idx).and_then(|v| v.parse::<bool>().ok());
                    }
                    "--source" => {
                        idx += 1;
                        source = rest.get(idx).cloned();
                    }
                    _ => {}
                }
                idx += 1;
            }

            if let Some(v) = path {
                params["path"] = Value::String(v);
            }
            if let Some(v) = raw {
                params["raw"] = Value::String(v);
            }
            if let Some(v) = json_value {
                params["json"] = v;
            }
            if let Some(v) = save {
                params["save"] = Value::Bool(v);
            }
            if let Some(v) = source {
                params["source"] = Value::String(v);
            }

            Ok(Command {
                op: CliOp::Known(ControlOp::InventoryLoad),
                params: Some(params),
            })
        }
        "inventory-filter" => {
            let rest = iter.collect::<Vec<_>>();
            let mut idx = 0;
            while idx < rest.len() {
                match rest[idx].as_str() {
                    "--category" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            params["category"] = Value::String(v.to_string());
                        }
                    }
                    "--item-type" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            params["item_type"] = Value::String(v.to_string());
                        }
                    }
                    "--contains" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            params["contains"] = Value::String(v.to_string());
                        }
                    }
                    "--limit" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            if let Ok(n) = v.parse::<u64>() {
                                params["limit"] = Value::Number(n.into());
                            }
                        }
                    }
                    "--offset" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            if let Ok(n) = v.parse::<u64>() {
                                params["offset"] = Value::Number(n.into());
                            }
                        }
                    }
                    "--include-details" => {
                        params["include_details"] = Value::Bool(true);
                    }
                    "--path" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            params["path"] = Value::String(v.to_string());
                        }
                    }
                    _ => {}
                }
                idx += 1;
            }

            Ok(Command {
                op: CliOp::Known(ControlOp::InventoryFilter),
                params: Some(params),
            })
        }
        "inventory-meta" => Ok(Command {
            op: CliOp::Known(ControlOp::InventoryMetaGet),
            params: None,
        }),
        "inventory-stale" => {
            let rest = iter.collect::<Vec<_>>();
            let mut idx = 0;
            while idx < rest.len() {
                match rest[idx].as_str() {
                    "--timestamp" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            params["timestamp"] = parse_jsonish(v);
                        }
                    }
                    "--reason" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            params["reason"] = Value::String(v.to_string());
                        }
                    }
                    _ => {}
                }
                idx += 1;
            }
            Ok(Command {
                op: CliOp::Known(ControlOp::InventoryStaleUpdate),
                params: Some(params),
            })
        }
        "inventory-refresh" => {
            let rest = iter.collect::<Vec<_>>();
            let mut idx = 0;
            while idx < rest.len() {
                match rest[idx].as_str() {
                    "--scan-retries" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            if let Ok(n) = v.parse::<u64>() {
                                params["scan_retries"] = Value::Number(n.into());
                            }
                        }
                    }
                    "--scan-delay-ms" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            if let Ok(n) = v.parse::<u64>() {
                                params["scan_delay_ms"] = Value::Number(n.into());
                            }
                        }
                    }
                    "--no-save" => {
                        params["save"] = Value::Bool(false);
                    }
                    "--source" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            params["source"] = Value::String(v.to_string());
                        }
                    }
                    flag if flag.starts_with("--account") => {
                        eprintln!(
                            "warning: --account-id no longer needed; daemon uses current login"
                        );
                    }
                    _ => {}
                }
                idx += 1;
            }
            Ok(Command {
                op: CliOp::Known(ControlOp::InventoryRefresh),
                params: Some(params),
            })
        }
        "screenshot" => {
            let rest = iter.collect::<Vec<_>>();
            let mut idx = 0;
            while idx < rest.len() {
                match rest[idx].as_str() {
                    "--action" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            params["action"] = Value::String(v.to_string());
                        }
                    }
                    "--metadata" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            params["metadata"] = parse_jsonish(v);
                        }
                    }
                    _ => {}
                }
                idx += 1;
            }
            Ok(Command {
                op: CliOp::Known(ControlOp::ScreenshotTrigger),
                params: Some(params),
            })
        }
        "call" => {
            let op = iter
                .next()
                .ok_or_else(|| anyhow::anyhow!("call requires op name"))?;
            let rest = iter.collect::<Vec<_>>();
            let mut idx = 0;
            while idx < rest.len() {
                match rest[idx].as_str() {
                    "--params" => {
                        idx += 1;
                        if let Some(v) = rest.get(idx) {
                            params = parse_jsonish(v);
                        }
                    }
                    _ => {}
                }
                idx += 1;
            }

            Ok(Command {
                op: CliOp::Call(op),
                params: Some(params),
            })
        }
        _ => anyhow::bail!("Unknown command '{}'", cmd),
    }
}

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

fn print_usage() {
    let unix_flag = if cfg!(unix) { " | --unix PATH" } else { "" };
    let npipe_flag = if cfg!(windows) { " | --npipe NAME" } else { "" };
    let unix_env = if cfg!(unix) {
        " / WF_INFO_API_UNIX"
    } else {
        ""
    };
    let npipe_env = if cfg!(windows) {
        " / WF_INFO_API_NPIPE"
    } else {
        ""
    };
    eprintln!(
        "Usage: wf-info-cli [--tcp ADDR{unix_flag}{npipe_flag}] [--pretty] [--id ID] <command> [options]\n\
Commands:\n\
  ping\n\
  watch [--events EVENT1,EVENT2,...]\n\
  inventory-load --path PATH | --raw JSON | --json JSON [--save true|false] [--source NAME]\n\
  inventory-filter [--category NAME] [--item-type TYPE] [--contains TEXT] [--limit N] [--offset N] [--include-details] [--path PATH]\n\
  inventory-meta\n\
  inventory-stale [--timestamp TS] [--reason TEXT]\n\
  inventory-refresh [--scan-retries N] [--scan-delay-ms N] [--no-save] [--source NAME]\n\
  screenshot [--action NAME] [--metadata JSON]\n\
  call OP [--params JSON]\n\
\n\
Event types: account_login, account_logout, inventory_fetched, inventory_stale, profile_updated, screenshot_triggered\n\
\n\
Connection defaults to WF_INFO_API_TCP{unix_env}{npipe_env} or the built-in platform default if flags are omitted.",
        unix_flag = unix_flag,
        npipe_flag = npipe_flag,
        unix_env = unix_env,
        npipe_env = npipe_env,
    );
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
