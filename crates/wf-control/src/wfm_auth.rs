use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use anyhow::{Result, anyhow};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock, oneshot};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::header;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite};

use crate::utils::{WFM_AUTH_BASE, WFM_SUB_PROTOCOL, WFM_WS_URL, parse_params};
use wf_core::storage::{self, AuthTokenData};

// ── WS message types ──

#[derive(Debug, Deserialize)]
struct WsMessage {
    route: Option<String>,
    payload: Option<Value>,
    #[allow(dead_code)]
    id: Option<String>,
    #[serde(rename = "refId")]
    ref_id: Option<String>,
}

// ── Session state ──

struct WfmSession {
    ws_tx: futures_util::stream::SplitSink<
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        tungstenite::Message,
    >,
    #[allow(dead_code)]
    tokens: AuthTokenData,
    current_status: Option<String>,
    /// Pending requests waiting for a response, keyed by route
    pending: Arc<Mutex<HashMap<String, oneshot::Sender<WsMessage>>>>,
}

static SESSION: OnceLock<RwLock<Option<WfmSession>>> = OnceLock::new();

fn session_lock() -> &'static RwLock<Option<WfmSession>> {
    SESSION.get_or_init(|| RwLock::new(None))
}

// ── REST auth calls (v1 API) ──
//
// v1 signin: POST /auth/signin with snake_case body, JWT returned in
// Authorization response header as "JWT <token>".

/// Sign in via v1 API. Returns the JWT access token.
async fn rest_signin(email: &str, password: &str, device_id: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let raw_resp = client
        .post(format!("{}/auth/signin", WFM_AUTH_BASE))
        .header("Authorization", "JWT")
        .json(&json!({
            "auth_type": "header",
            "email": email,
            "password": password,
            "device_id": device_id,
        }))
        .send()
        .await?;

    let status = raw_resp.status();
    if !status.is_success() {
        let body = raw_resp.text().await.unwrap_or_default();
        return Err(anyhow!("WFM signin failed ({}): {}", status, body));
    }

    // Extract JWT from Authorization header ("JWT <token>")
    let auth_header = raw_resp
        .headers()
        .get("Authorization")
        .ok_or_else(|| anyhow!("No Authorization header in signin response"))?
        .to_str()
        .map_err(|_| anyhow!("Invalid Authorization header encoding"))?
        .to_string();

    if !auth_header.starts_with("JWT ") {
        return Err(anyhow!(
            "Unexpected Authorization header format: {}",
            auth_header
        ));
    }

    let jwt = auth_header[4..].to_string();
    Ok(jwt)
}

// ── WebSocket connection ──

async fn connect_and_auth(tokens: AuthTokenData) -> Result<()> {
    let mut request = WFM_WS_URL.into_client_request()?;
    request
        .headers_mut()
        .insert(header::SEC_WEBSOCKET_PROTOCOL, WFM_SUB_PROTOCOL.parse()?);

    let (ws_stream, _response) = tokio_tungstenite::connect_async(request).await?;

    let pending: Arc<Mutex<HashMap<String, oneshot::Sender<WsMessage>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let (ws_tx, ws_rx) = ws_stream.split();

    // Spawn reader task
    let pending_clone = pending.clone();
    tokio::spawn(ws_recv_loop(ws_rx, pending_clone));

    let session = WfmSession {
        ws_tx,
        tokens: tokens.clone(),
        current_status: None,
        pending,
    };

    {
        let mut guard = session_lock().write().await;
        *guard = Some(session);
    }

    // Authenticate on the WebSocket
    let resp = ws_command(
        "@wfm|cmd/auth/signIn",
        json!({ "token": tokens.access_token }),
    )
    .await?;

    if let Some(route) = &resp.route {
        if route.ends_with(":error") {
            let err_msg = resp
                .payload
                .as_ref()
                .and_then(|p| p.as_str())
                .unwrap_or("unknown error");
            // Clear session on auth failure
            let mut guard = session_lock().write().await;
            *guard = None;
            return Err(anyhow!("WS auth failed: {}", err_msg));
        }
    }

    log::info!("WFM WebSocket authenticated");
    Ok(())
}

async fn ws_recv_loop(
    mut ws_rx: futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pending: Arc<Mutex<HashMap<String, oneshot::Sender<WsMessage>>>>,
) {
    while let Some(msg_result) = ws_rx.next().await {
        let msg = match msg_result {
            Ok(tungstenite::Message::Text(text)) => text,
            Ok(tungstenite::Message::Close(_)) => {
                log::info!("WFM WebSocket closed by server");
                break;
            }
            Err(e) => {
                log::warn!("WFM WebSocket error: {}", e);
                break;
            }
            _ => continue,
        };

        log::debug!("WFM WS raw message: {}", msg);

        let parsed: WsMessage = match serde_json::from_str(&msg) {
            Ok(m) => m,
            Err(e) => {
                log::debug!("WFM WS unparseable message: {}", e);
                continue;
            }
        };

        log::debug!(
            "WFM WS parsed: route={:?}, id={:?}, refId={:?}",
            parsed.route,
            parsed.id,
            parsed.ref_id
        );

        // If this is a response to a pending command, deliver it.
        // Server responds with route like "cmd/foo:ok" or "cmd/foo:error"
        // for a command sent as "cmd/foo". Match by stripping the suffix.
        if let Some(ref route) = parsed.route {
            if let Some(base_route) = route
                .strip_suffix(":ok")
                .or_else(|| route.strip_suffix(":error"))
            {
                let mut map = pending.lock().await;
                if let Some(tx) = map.remove(base_route) {
                    let _ = tx.send(parsed);
                    continue;
                }
            }
        }

        // Handle events
        if let Some(ref route) = parsed.route {
            if route == "@wfm|event/status/set" {
                if let Some(ref payload) = parsed.payload {
                    if let Some(status) = payload.get("status").and_then(|s| s.as_str()) {
                        let mut guard = session_lock().write().await;
                        if let Some(ref mut session) = *guard {
                            session.current_status = Some(status.to_string());
                        }
                    }
                }
            } else if route == "@wfm|event/auth/revoked" {
                log::warn!("WFM auth token revoked by server");
                let mut guard = session_lock().write().await;
                *guard = None;
            }
        }
    }

    // Connection dropped — clear session
    let mut guard = session_lock().write().await;
    *guard = None;
    log::info!("WFM WebSocket disconnected");
}

/// Send a command and wait for the response.
async fn ws_command(route: &str, payload: Value) -> Result<WsMessage> {
    let (tx, rx) = oneshot::channel();

    let json_msg = json!({
        "route": route,
        "payload": payload,
    });

    {
        let mut guard = session_lock().write().await;
        let session = guard
            .as_mut()
            .ok_or_else(|| anyhow!("Not connected to WFM"))?;

        session.pending.lock().await.insert(route.to_string(), tx);

        session
            .ws_tx
            .send(tungstenite::Message::Text(json_msg.to_string().into()))
            .await
            .map_err(|e| anyhow!("WS send failed: {}", e))?;
    }

    let resp = tokio::time::timeout(std::time::Duration::from_secs(15), rx)
        .await
        .map_err(|_| anyhow!("WFM command timed out"))?
        .map_err(|_| anyhow!("WFM response channel closed"))?;

    Ok(resp)
}

// ── Handlers ──

#[derive(Debug, Deserialize, Default)]
struct SignstatusParams {
    status: Option<String>,
    /// Nullable optional: absent = don't change, null = remove, number = set
    duration: Option<Value>,
}

pub(crate) async fn handle_wfm_signstatus(params: Option<Value>) -> Result<Value> {
    let p: SignstatusParams = parse_params(params)?;

    // If no status provided, return current state
    if p.status.is_none() {
        let guard = session_lock().read().await;
        return match guard.as_ref() {
            Some(session) => {
                let expires_at = session.tokens.expires_at;
                let expired = expires_at < Utc::now();
                Ok(json!({
                    "authenticated": true,
                    "status": session.current_status,
                    "expires_at": expires_at.to_rfc3339(),
                    "expired": expired,
                }))
            }
            None => Ok(json!({
                "authenticated": false,
                "status": null,
            })),
        };
    }

    let status = p.status.unwrap();
    match status.as_str() {
        "online" | "invisible" | "ingame" => {}
        other => {
            return Err(anyhow!(
                "Invalid status '{}'. Must be: online, invisible, ingame",
                other
            ));
        }
    }

    // Build payload with PATCH semantics for duration
    let mut payload = json!({ "status": status });
    if let Some(duration_val) = p.duration {
        // duration present in params (could be null or a number)
        payload["duration"] = duration_val;
    }
    // If duration not in params at all, we omit it (PATCH: no change)

    let resp = ws_command("@wfm|cmd/status/set", payload).await?;

    if let Some(route) = &resp.route {
        if route.ends_with(":error") {
            let err_msg = resp
                .payload
                .as_ref()
                .map(|p| serde_json::to_string(p).unwrap_or_default())
                .unwrap_or_else(|| "unknown error".into());
            return Err(anyhow!("Status update failed: {}", err_msg));
        }
    }

    // Update local state
    {
        let mut guard = session_lock().write().await;
        if let Some(ref mut session) = *guard {
            session.current_status = Some(status.clone());
        }
    }

    Ok(json!({
        "ok": true,
        "status": status,
    }))
}

// ── Sign in handler ──

#[derive(Debug, Deserialize)]
struct SigninParams {
    email: String,
    password: String,
    #[serde(default = "default_client_id")]
    client_id: String,
    #[serde(default = "default_device_name")]
    device_name: String,
}

fn default_client_id() -> String {
    "wf-info-2".to_string()
}

fn default_device_name() -> String {
    "wf-info-2".to_string()
}

pub(crate) async fn handle_wfm_signin(params: Option<Value>) -> Result<Value> {
    let p: SigninParams = match params {
        Some(value) => {
            serde_json::from_value(value).map_err(|e| anyhow!("Invalid signin params: {}", e))?
        }
        None => return Err(anyhow!("Missing signin params (email, password required)")),
    };

    // Load existing device_id or generate a new stable one
    let device_id = match storage::read_auth_token() {
        Ok(existing) => existing.device_id,
        Err(_) => uuid::Uuid::new_v4().to_string(),
    };

    let jwt = rest_signin(&p.email, &p.password, &device_id).await?;

    let token_data = AuthTokenData {
        access_token: jwt,
        refresh_token: String::new(), // v1 API doesn't provide refresh tokens
        device_id,
        client_id: p.client_id,
        device_name: p.device_name,
        expires_at: Utc::now() + chrono::Duration::hours(24), // conservative estimate
    };

    storage::save_auth_token(&token_data)?;

    // Connect WebSocket and authenticate
    connect_and_auth(token_data).await?;

    Ok(json!({}))
}

// ── Sign out handler ──

pub(crate) async fn handle_wfm_signout(_params: Option<Value>) -> Result<Value> {
    // Clear WS session
    {
        let mut guard = session_lock().write().await;
        *guard = None;
    }

    // Delete stored tokens
    storage::delete_auth_token()?;

    log::info!("Signed out from WFM");
    Ok(json!({}))
}

// ── Session restore (called on daemon start) ──

pub async fn try_restore_session() {
    let token_data = match storage::read_auth_token() {
        Ok(t) => t,
        Err(_) => return, // No cached token, nothing to restore
    };

    // If token is expired, we can't refresh with v1 — user must re-login
    if token_data.expires_at < Utc::now() {
        log::warn!("WFM token expired, please sign in again");
        return;
    }

    match connect_and_auth(token_data).await {
        Ok(()) => log::info!("WFM session restored from cached token"),
        Err(e) => log::warn!("Failed to restore WFM session: {}", e),
    }
}

/// Set the WFM profile status if authenticated. Used by daemon for auto-status.
pub async fn set_status_if_connected(status: &str) {
    let is_connected = {
        let guard = session_lock().read().await;
        guard.is_some()
    };

    if !is_connected {
        return;
    }

    match ws_command("@wfm|cmd/status/set", json!({ "status": status })).await {
        Ok(_) => {
            let mut guard = session_lock().write().await;
            if let Some(ref mut session) = *guard {
                session.current_status = Some(status.to_string());
            }
            log::info!("WFM status set to '{}'", status);
        }
        Err(e) => log::warn!("Failed to set WFM status: {}", e),
    }
}
