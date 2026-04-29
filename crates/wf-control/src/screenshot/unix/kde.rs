use std::fs::{self, File};
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::{Mutex, mpsc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;
use zbus::{
    blocking::{Proxy, connection::Builder as ConnectionBuilder},
    interface,
};

use super::common::{ensure_command_available, ensure_png_bytes, temp_path, window_matches_hint};

pub(super) fn capture_active_window(warframe_pid: u32) -> Result<Vec<u8>> {
    ensure_command_available("spectacle")?;
    let info = get_active_window_info()?;

    if !window_matches_warframe(&info, warframe_pid) {
        bail!("Warframe window is not active; focus it and try again");
    }

    let output_path = temp_path("wf-info-kde", "png");
    let output_path_str = output_path
        .to_str()
        .ok_or_else(|| anyhow!("Temporary screenshot path is not valid UTF-8"))?;

    let result = Command::new("spectacle")
        .args([
            "--background",
            "--activewindow",
            "--nonotify",
            "--output",
            output_path_str,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to run Spectacle for KDE window capture")?;

    let stderr = String::from_utf8_lossy(&result.stderr).trim().to_string();
    if !stderr.is_empty() {
        log::warn!("Spectacle stderr: {}", stderr);
    }
    if !result.status.success() {
        let _ = fs::remove_file(&output_path);
        if stderr.is_empty() {
            bail!("Spectacle failed to capture the active Warframe window on KDE");
        }
        bail!(
            "Spectacle failed to capture the active Warframe window on KDE: {}",
            stderr
        );
    }

    let _metadata = fs::metadata(&output_path).with_context(|| {
        format!(
            "Spectacle did not produce a screenshot file at {}",
            output_path.display()
        )
    })?;
    let bytes = fs::read(&output_path).with_context(|| {
        format!(
            "Spectacle did not produce a screenshot file at {}",
            output_path.display()
        )
    })?;
    let _ = fs::remove_file(&output_path);

    if bytes.is_empty() {
        bail!("Spectacle returned an empty screenshot");
    }
    ensure_png_bytes(&bytes, "Spectacle KDE capture")?;

    Ok(bytes)
}

#[derive(Debug, Deserialize)]
struct ActiveWindowInfo {
    title: String,
    class_name: String,
    pid: u32,
}

fn get_active_window_info() -> Result<ActiveWindowInfo> {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("failed to read system time")?
        .as_millis();
    let marker = format!("wf-info-kde-{unique_suffix}");

    let kw_script = format!(
        r#"
function output_error(message) {{
    callDBus("{{dbus_addr}}", "/", "io.github.wf_info.KWinScriptCallback", "error", message.toString());
}}
function output_result(message) {{
    callDBus("{{dbus_addr}}", "/", "io.github.wf_info.KWinScriptCallback", "result", message.toString());
}}
function run() {{
    let w = workspace.activeWindow;
    if (w == null) {{
        output_error("No active window");
    }} else {{
        output_result(JSON.stringify({{
            title: w.caption,
            class_name: w.resourceClass,
            pid: w.pid
        }}));
    }}
}}
run();
"#
    );

    run_kwin_script(&kw_script, &marker)
}

fn window_matches_warframe(info: &ActiveWindowInfo, warframe_pid: u32) -> bool {
    info.pid == warframe_pid || window_matches_hint(&info.title, &info.class_name)
}

fn run_kwin_script(script_contents: &str, script_name: &str) -> Result<ActiveWindowInfo> {
    let (tx, rx) = mpsc::channel();
    let conn = ConnectionBuilder::session()
        .context("Failed to create DBus listener for KWin window query")?
        .serve_at("/", ScriptCallback::new(tx))
        .context("Failed to register KWin callback object")?
        .build()
        .context("Failed to connect to KWin session DBus")?;
    let dbus_addr = conn
        .unique_name()
        .ok_or_else(|| anyhow!("Failed to determine DBus unique name for KWin callback"))?
        .to_string();
    let kwin_proxy = Proxy::new(
        &conn,
        "org.kde.KWin",
        "/Scripting",
        "org.kde.kwin.Scripting",
    )
    .context("Failed to connect to KWin /Scripting DBus interface")?;

    let script_path = temp_path("wf-info-kde-script", "js");
    let kw_script = script_contents.replace("{dbus_addr}", &dbus_addr);
    let mut script_file =
        File::create(&script_path).context("Failed to create temporary KWin script file")?;
    script_file
        .write_all(kw_script.as_bytes())
        .context("Failed to write temporary KWin script")?;
    script_file
        .flush()
        .context("Failed to flush temporary KWin script")?;

    let script_id: i32 = kwin_proxy
        .call("loadScript", &(script_path.to_str().unwrap(), script_name))
        .context("Failed to load temporary KWin script")?;
    if script_id < 0 {
        let _ = fs::remove_file(&script_path);
        bail!("Failed to load temporary KWin script");
    }

    let script_proxy = Proxy::new(
        &conn,
        "org.kde.KWin",
        format!("/Scripting/Script{script_id}"),
        "org.kde.kwin.Script",
    )
    .context("Failed to connect to temporary KWin script DBus interface")?;

    script_proxy
        .call::<_, _, ()>("run", &())
        .context("Failed to run temporary KWin script")?;
    script_proxy
        .call::<_, _, ()>("stop", &())
        .context("Failed to stop temporary KWin script")?;

    let payload_result = match rx.recv_timeout(Duration::from_secs(5)) {
        Ok(ScriptMessage::Result(payload)) => Ok(payload),
        Ok(ScriptMessage::Error(message)) => {
            log::error!("Temporary KWin script returned error: {}", message);
            Err(anyhow!("KWin script error: {}", message))
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            Err(anyhow!("Timed out waiting for KWin script response"))
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err(anyhow!("KWin response channel disconnected"))
        }
    };

    let _: Result<(), _> = kwin_proxy.call("unloadScript", &(script_name,));
    let _ = fs::remove_file(&script_path);

    let payload = payload_result?;
    parse_active_window_info(&payload)
}

enum ScriptMessage {
    Result(String),
    Error(String),
}

struct ScriptCallback {
    tx: Mutex<mpsc::Sender<ScriptMessage>>,
}

impl ScriptCallback {
    fn new(tx: mpsc::Sender<ScriptMessage>) -> Self {
        Self { tx: Mutex::new(tx) }
    }
}

#[interface(name = "io.github.wf_info.KWinScriptCallback")]
impl ScriptCallback {
    #[zbus(name = "result")]
    fn result(&self, message: &str) {
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(ScriptMessage::Result(message.to_string()));
        }
    }

    #[zbus(name = "error")]
    fn error(&self, message: &str) {
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(ScriptMessage::Error(message.to_string()));
        }
    }
}

fn parse_active_window_info(payload: &str) -> Result<ActiveWindowInfo> {
    serde_json::from_str(payload)
        .or_else(|_| {
            let unescaped: String =
                serde_json::from_str(payload).context("failed to unescape KWin payload")?;
            serde_json::from_str(&unescaped).context("failed to parse unescaped KWin payload")
        })
        .context("failed to parse KWin active window info")
}
