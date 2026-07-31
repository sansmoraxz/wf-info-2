use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use anyhow::{Result, anyhow};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock, oneshot};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::header;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite};

use crate::utils::{WFM_AUTH_BASE, WFM_SUB_PROTOCOL, WFM_WS_URL};
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

// ── WS routes ──

/// Outbound command routes on the WFM socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
enum WsRoute {
    #[strum(serialize = "@wfm|cmd/auth/signIn")]
    AuthSignIn,
    #[strum(serialize = "@wfm|cmd/status/set")]
    StatusSet,
}

/// Server-pushed event routes on the WFM socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString)]
enum WsEvent {
    #[strum(serialize = "@wfm|event/status/set")]
    StatusSet,
    #[strum(serialize = "@wfm|event/auth/revoked")]
    AuthRevoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReplyOutcome {
    Ok,
    Error,
}

/// A classified incoming route: the server replies to a command sent as
/// `cmd/foo` with `cmd/foo:ok` or `cmd/foo:error`.
enum IncomingRoute {
    Reply {
        command: WsRoute,
        outcome: ReplyOutcome,
    },
    Event(WsEvent),
    Other,
}

impl From<&str> for IncomingRoute {
    fn from(route: &str) -> Self {
        let (base, outcome) = if let Some(base) = route.strip_suffix(":ok") {
            (base, ReplyOutcome::Ok)
        } else if let Some(base) = route.strip_suffix(":error") {
            (base, ReplyOutcome::Error)
        } else {
            return route.parse::<WsEvent>().map_or(Self::Other, Self::Event);
        };
        base.parse::<WsRoute>()
            .map_or(Self::Other, |command| Self::Reply { command, outcome })
    }
}

/// Reply to a command, as delivered by the recv loop.
#[derive(Debug)]
struct WsReply {
    outcome: ReplyOutcome,
    payload: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct StatusEventPayload {
    status: Status,
}

// ── Session state ──

/// A warframe.market profile status. Serialized lowercase on the WFM wire.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::Display,
    strum::EnumString,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum Status {
    Online,
    Invisible,
    Ingame,
}

type PendingMap = Arc<Mutex<HashMap<WsRoute, oneshot::Sender<WsReply>>>>;

/// An open WFM WebSocket that has NOT been authenticated yet. The only way to
/// obtain a [`WfmSession`] is to consume this via [`WsConnection::authenticate`],
/// so an unauthenticated connection can never be stored in the global session.
struct WsConnection {
    ws_tx: futures_util::stream::SplitSink<
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        tungstenite::Message,
    >,
    /// Pending requests waiting for a response, keyed by command route
    pending: PendingMap,
}

struct WfmSession {
    conn: WsConnection,
    #[allow(dead_code)]
    tokens: AuthTokenData,
    current_status: Option<Status>,
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

impl WsConnection {
    /// Open the WFM WebSocket and spawn its recv loop. The connection is not
    /// authenticated yet — call [`Self::authenticate`] to obtain a session.
    async fn connect() -> Result<Self> {
        let mut request = WFM_WS_URL.into_client_request()?;
        request
            .headers_mut()
            .insert(header::SEC_WEBSOCKET_PROTOCOL, WFM_SUB_PROTOCOL.parse()?);

        let (ws_stream, _response) = tokio_tungstenite::connect_async(request).await?;

        let pending: PendingMap = Arc::new(Mutex::new(HashMap::new()));
        let (ws_tx, ws_rx) = ws_stream.split();

        tokio::spawn(ws_recv_loop(ws_rx, pending.clone()));

        Ok(Self { ws_tx, pending })
    }

    /// Authenticate on the socket, consuming the connection. On failure the
    /// connection is dropped — an unauthenticated session can never leak out.
    async fn authenticate(mut self, tokens: AuthTokenData) -> Result<WfmSession> {
        let rx = self
            .send_command(WsRoute::AuthSignIn, json!({ "token": tokens.access_token }))
            .await?;
        let reply = await_reply(rx).await?;

        if reply.outcome == ReplyOutcome::Error {
            let err_msg = reply
                .payload
                .as_ref()
                .and_then(|p| p.as_str())
                .unwrap_or("unknown error");
            return Err(anyhow!("WS auth failed: {}", err_msg));
        }

        log::info!("WFM WebSocket authenticated");
        Ok(WfmSession {
            conn: self,
            tokens,
            current_status: None,
        })
    }

    /// Register a pending reply slot and send the command. The returned
    /// receiver resolves when the recv loop delivers the matching reply.
    async fn send_command(
        &mut self,
        route: WsRoute,
        payload: Value,
    ) -> Result<oneshot::Receiver<WsReply>> {
        let (tx, rx) = oneshot::channel();
        let json_msg = json!({
            "route": route.to_string(),
            "payload": payload,
        });

        self.pending.lock().await.insert(route, tx);
        self.ws_tx
            .send(tungstenite::Message::Text(json_msg.to_string().into()))
            .await
            .map_err(|e| anyhow!("WS send failed: {}", e))?;
        Ok(rx)
    }
}

async fn await_reply(rx: oneshot::Receiver<WsReply>) -> Result<WsReply> {
    tokio::time::timeout(std::time::Duration::from_secs(15), rx)
        .await
        .map_err(|_| anyhow!("WFM command timed out"))?
        .map_err(|_| anyhow!("WFM response channel closed"))
}

async fn connect_and_auth(tokens: AuthTokenData) -> Result<()> {
    let session = WsConnection::connect().await?.authenticate(tokens).await?;
    *session_lock().write().await = Some(session);
    Ok(())
}

async fn ws_recv_loop(
    mut ws_rx: futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pending: PendingMap,
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

        let Some(ref route) = parsed.route else {
            continue;
        };

        match IncomingRoute::from(route.as_str()) {
            IncomingRoute::Reply { command, outcome } => {
                let mut map = pending.lock().await;
                if let Some(tx) = map.remove(&command) {
                    let _ = tx.send(WsReply {
                        outcome,
                        payload: parsed.payload,
                    });
                }
            }
            IncomingRoute::Event(WsEvent::StatusSet) => {
                let payload = parsed
                    .payload
                    .map(serde_json::from_value::<StatusEventPayload>);
                match payload {
                    Some(Ok(StatusEventPayload { status })) => {
                        let mut guard = session_lock().write().await;
                        if let Some(ref mut session) = *guard {
                            session.current_status = Some(status);
                        }
                    }
                    Some(Err(e)) => log::debug!("Ignoring unparseable WFM status event: {}", e),
                    None => {}
                }
            }
            IncomingRoute::Event(WsEvent::AuthRevoked) => {
                log::warn!("WFM auth token revoked by server");
                let mut guard = session_lock().write().await;
                *guard = None;
            }
            IncomingRoute::Other => {}
        }
    }

    // Connection dropped — clear session
    let mut guard = session_lock().write().await;
    *guard = None;
    log::info!("WFM WebSocket disconnected");
}

/// Send a command on the globally stored (authenticated) session and wait for
/// the reply. The session lock is released while waiting.
async fn ws_command(route: WsRoute, payload: Value) -> Result<WsReply> {
    let rx = {
        let mut guard = session_lock().write().await;
        let session = guard
            .as_mut()
            .ok_or_else(|| anyhow!("Not connected to WFM"))?;
        session.conn.send_command(route, payload).await?
    };

    await_reply(rx).await
}

// ── Handlers ──

#[derive(Debug, Deserialize, Default)]
pub(crate) struct SignstatusParams {
    status: Option<String>,
    /// PATCH semantics: absent = don't change, null = remove, number = set
    #[serde(default, with = "serde_with::rust::double_option")]
    duration: Option<Option<u64>>,
}

#[derive(Debug, Serialize)]
struct StatusSetPayload {
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Option<u64>>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub(crate) enum SignstatusResponse {
    Authenticated {
        status: Option<Status>,
        expires_at: String,
        expired: bool,
    },
    Unauthenticated,
    Set {
        status: Status,
    },
}

pub(crate) async fn handle_wfm_signstatus(p: SignstatusParams) -> Result<SignstatusResponse> {
    // If no status provided, return current state
    if p.status.is_none() {
        let guard = session_lock().read().await;
        return match guard.as_ref() {
            Some(session) => {
                let expires_at = session.tokens.expires_at;
                let expired = expires_at < Utc::now();
                Ok(SignstatusResponse::Authenticated {
                    status: session.current_status,
                    expires_at: expires_at.to_rfc3339(),
                    expired,
                })
            }
            None => Ok(SignstatusResponse::Unauthenticated),
        };
    }

    let raw_status = p.status.unwrap();
    let status = raw_status.parse::<Status>().map_err(|_| {
        anyhow!(
            "Invalid status '{}'. Must be: online, invisible, ingame",
            raw_status
        )
    })?;

    // Build payload with PATCH semantics for duration:
    // None serializes as omitted, Some(None) as null, Some(Some(n)) as n
    let payload = serde_json::to_value(StatusSetPayload {
        status,
        duration: p.duration,
    })?;

    let reply = ws_command(WsRoute::StatusSet, payload).await?;

    if reply.outcome == ReplyOutcome::Error {
        let err_msg = reply
            .payload
            .as_ref()
            .map(|p| serde_json::to_string(p).unwrap_or_default())
            .unwrap_or_else(|| "unknown error".into());
        return Err(anyhow!("Status update failed: {}", err_msg));
    }

    // Update local state
    {
        let mut guard = session_lock().write().await;
        if let Some(ref mut session) = *guard {
            session.current_status = Some(status);
        }
    }

    Ok(SignstatusResponse::Set { status })
}

// ── Sign in handler ──

#[derive(Debug, Deserialize)]
pub(crate) struct SigninParams {
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

pub(crate) fn parse_signin_params(params: Option<Value>) -> Result<SigninParams> {
    match params {
        Some(value) => {
            serde_json::from_value(value).map_err(|e| anyhow!("Invalid signin params: {}", e))
        }
        None => Err(anyhow!("Missing signin params (email, password required)")),
    }
}

pub(crate) async fn handle_wfm_signin(p: SigninParams) -> Result<()> {
    // Load existing device_id or generate a new stable one
    let device_id = match storage::read_auth_token() {
        Ok(existing) => existing.device_id,
        Err(_) => uuid::Uuid::new_v4().to_string(),
    };

    let jwt = rest_signin(&p.email, &p.password, &device_id).await?;

    let token_data = AuthTokenData {
        access_token: jwt,
        refresh_token: None, // v1 API doesn't provide refresh tokens
        device_id,
        client_id: p.client_id,
        device_name: p.device_name,
        expires_at: Utc::now() + chrono::Duration::hours(24), // conservative estimate
    };

    storage::save_auth_token(&token_data)?;

    // Connect WebSocket and authenticate
    connect_and_auth(token_data).await?;

    Ok(())
}

// ── Sign out handler ──

pub(crate) async fn handle_wfm_signout() -> Result<()> {
    // Clear WS session
    {
        let mut guard = session_lock().write().await;
        *guard = None;
    }

    // Delete stored tokens
    storage::delete_auth_token()?;

    log::info!("Signed out from WFM");
    Ok(())
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
pub async fn set_status_if_connected(status: Status) {
    let is_connected = {
        let guard = session_lock().read().await;
        guard.is_some()
    };

    if !is_connected {
        return;
    }

    match ws_command(WsRoute::StatusSet, json!({ "status": status })).await {
        Ok(_) => {
            let mut guard = session_lock().write().await;
            if let Some(ref mut session) = *guard {
                session.current_status = Some(status);
            }
            log::info!("WFM status set to '{}'", status);
        }
        Err(e) => log::warn!("Failed to set WFM status: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signstatus_duration_patch_semantics() {
        let p: SignstatusParams = serde_json::from_str(r#"{"status":"online"}"#).unwrap();
        assert_eq!(p.duration, None);

        let p: SignstatusParams =
            serde_json::from_str(r#"{"status":"online","duration":null}"#).unwrap();
        assert_eq!(p.duration, Some(None));

        let p: SignstatusParams =
            serde_json::from_str(r#"{"status":"online","duration":3600}"#).unwrap();
        assert_eq!(p.duration, Some(Some(3600)));
    }

    #[test]
    fn status_set_payload_matches_patch_wire_shape() {
        let omitted = serde_json::to_value(StatusSetPayload {
            status: Status::Online,
            duration: None,
        })
        .unwrap();
        assert_eq!(omitted, serde_json::json!({ "status": "online" }));

        let null = serde_json::to_value(StatusSetPayload {
            status: Status::Online,
            duration: Some(None),
        })
        .unwrap();
        assert_eq!(
            null,
            serde_json::json!({ "status": "online", "duration": null })
        );

        let set = serde_json::to_value(StatusSetPayload {
            status: Status::Online,
            duration: Some(Some(60)),
        })
        .unwrap();
        assert_eq!(
            set,
            serde_json::json!({ "status": "online", "duration": 60 })
        );
    }
}
