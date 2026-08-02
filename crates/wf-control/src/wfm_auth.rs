use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt as _, StreamExt as _};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::time::error::Elapsed;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::client::IntoClientRequest as _;
use tokio_tungstenite::tungstenite::http::header;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite};

use crate::requests::{ControlError, EmptyResponse, HandleOp, Handles};
use crate::utils::{WFM_AUTH_BASE, WFM_SUB_PROTOCOL, WFM_WS_URL};
use wf_core::storage::{self, AuthTokenData};

#[derive(Debug, thiserror::Error)]
pub(crate) enum WfmError {
    #[error("WFM actor unavailable")]
    ActorUnavailable,
    #[error("Not connected to WFM")]
    NotConnected,
    #[error("WFM command timed out")]
    Timeout(#[from] Elapsed),
    #[error("WFM response channel closed")]
    ChannelClosed(#[from] oneshot::error::RecvError),
    /// Error message reported by the WFM server in a `:error` reply.
    #[error("{0}")]
    Server(String),
    #[error("WS auth failed: {0}")]
    WsAuthFailed(String),
    #[error("WS send failed: {0}")]
    WsSend(String),
    // tungstenite::Error is 136 bytes; store its message to keep this enum small.
    #[error("{0}")]
    Ws(String),
    #[error("WFM signin failed ({status}): {body}")]
    SigninFailed {
        status: reqwest::StatusCode,
        body: String,
    },
    #[error("No Authorization header in signin response")]
    NoAuthHeader,
    #[error("Invalid Authorization header encoding: {0}")]
    AuthHeaderEncoding(String),
    #[error("Unexpected Authorization header format: {0}")]
    AuthHeaderFormat(String),
    #[error("Invalid status '{0}'. Must be: online, invisible, ingame")]
    InvalidStatus(String),
    #[error("Status update failed: {0}")]
    StatusUpdateFailed(String),
    #[error(transparent)]
    Header(#[from] header::InvalidHeaderValue),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Storage(#[from] storage::StorageError),
}

// ── WS message types ──

#[derive(Debug, Deserialize)]
struct WsMessage {
    route: Option<String>,
    payload: Option<Value>,
    id: Option<String>,
    #[serde(rename = "refId")]
    ref_id: Option<String>,
}

// ── WS routes ──

/// Outbound command routes on the WFM socket.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    strum::Display,
    strum::EnumString,
    serde_with::SerializeDisplay,
)]
enum WsRoute {
    #[strum(serialize = "@wfm|cmd/auth/signIn")]
    AuthSignIn,
    #[strum(serialize = "@wfm|cmd/status/set")]
    StatusSet,
}

/// Outbound message envelope on the WFM socket:
/// `{"route": ..., "payload": ...}`.
#[derive(Serialize)]
struct WsCommand<'a, T> {
    route: WsRoute,
    payload: &'a T,
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
/// `cmd/foo` with `cmd/foo:ok` or `cmd/foo:error`. Unrecognized routes are a
/// parse error (and ignored by the recv loop).
enum IncomingRoute {
    Reply {
        command: WsRoute,
        outcome: ReplyOutcome,
    },
    Event(WsEvent),
}

impl FromStr for IncomingRoute {
    type Err = strum::ParseError;

    fn from_str(route: &str) -> Result<Self, Self::Err> {
        let (base, outcome) = if let Some(base) = route.strip_suffix(":ok") {
            (base, ReplyOutcome::Ok)
        } else if let Some(base) = route.strip_suffix(":error") {
            (base, ReplyOutcome::Error)
        } else {
            return route.parse::<WsEvent>().map(Self::Event);
        };
        base.parse::<WsRoute>()
            .map(|command| Self::Reply { command, outcome })
    }
}

/// Reply to a command, as delivered by the recv loop.
#[derive(Debug)]
struct WsReply {
    outcome: ReplyOutcome,
    payload: Option<Value>,
}

impl WsReply {
    /// The payload on success, or the server's error message on failure.
    fn into_result(self) -> Result<Option<Value>, WfmError> {
        match self.outcome {
            ReplyOutcome::Ok => Ok(self.payload),
            ReplyOutcome::Error => {
                let msg = self.payload.map_or_else(
                    || "unknown error".into(),
                    |p| match p {
                        Value::String(s) => s,
                        other => other.to_string(),
                    },
                );
                Err(WfmError::Server(msg))
            }
        }
    }
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
    ws_tx: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>,
    /// Pending requests waiting for a response, keyed by command route
    pending: PendingMap,
}

impl WsConnection {
    /// Open the WFM WebSocket and spawn its recv loop. The connection is not
    /// authenticated yet — call [`Self::authenticate`] to obtain a session.
    async fn connect(wfm: WfmHandle) -> Result<Self, WfmError> {
        let mut request = WFM_WS_URL
            .into_client_request()
            .map_err(|e| WfmError::Ws(e.to_string()))?;
        request
            .headers_mut()
            .insert(header::SEC_WEBSOCKET_PROTOCOL, WFM_SUB_PROTOCOL.parse()?);

        let (ws_stream, _response) = tokio_tungstenite::connect_async(request)
            .await
            .map_err(|e| WfmError::Ws(e.to_string()))?;

        let pending: PendingMap = Arc::new(Mutex::new(HashMap::new()));
        let (ws_tx, ws_rx) = ws_stream.split();

        tokio::spawn(ws_recv_loop(wfm, ws_rx, Arc::clone(&pending)));

        Ok(Self { ws_tx, pending })
    }

    /// Authenticate on the socket, consuming the connection. On failure the
    /// connection is dropped — an unauthenticated session can never leak out.
    async fn authenticate(mut self, tokens: AuthTokenData) -> Result<WfmSession, WfmError> {
        #[derive(Serialize)]
        struct SignInPayload<'a> {
            token: &'a str,
        }

        let rx = self
            .send_command(
                WsRoute::AuthSignIn,
                &SignInPayload {
                    token: &tokens.access_token,
                },
            )
            .await?;
        await_reply(rx)
            .await?
            .into_result()
            .map_err(|e| WfmError::WsAuthFailed(e.to_string()))?;

        log::info!("WFM WebSocket authenticated");
        Ok(WfmSession {
            conn: self,
            tokens,
            current_status: None,
        })
    }

    /// Register a pending reply slot and send the command. The returned
    /// receiver resolves when the recv loop delivers the matching reply.
    async fn send_command<T>(
        &mut self,
        route: WsRoute,
        payload: &T,
    ) -> Result<oneshot::Receiver<WsReply>, WfmError>
    where
        T: Serialize,
    {
        let (tx, rx) = oneshot::channel();
        let json_msg = serde_json::to_string(&WsCommand { route, payload })?;

        self.pending.lock().await.insert(route, tx);
        self.ws_tx
            .send(tungstenite::Message::Text(json_msg.into()))
            .await
            .map_err(|e| WfmError::WsSend(e.to_string()))?;
        Ok(rx)
    }
}

pub(crate) struct WfmSession {
    conn: WsConnection,
    tokens: AuthTokenData,
    current_status: Option<Status>,
}

/// WFM sign-in state machine, owned exclusively by the actor task spawned in
/// [`WfmHandle::spawn`]. The only transition into `SignedIn` is via
/// [`WfmState::sign_in`] with a [`WfmSession`], which itself can only be
/// produced by [`WsConnection::authenticate`].
#[derive(Default)]
enum WfmState {
    #[default]
    SignedOut,
    SignedIn(Box<WfmSession>),
}

impl WfmState {
    fn sign_in(&mut self, session: WfmSession) {
        *self = Self::SignedIn(Box::new(session));
    }

    fn sign_out(&mut self) {
        *self = Self::SignedOut;
    }

    fn session(&self) -> Option<&WfmSession> {
        match self {
            Self::SignedIn(session) => Some(session),
            Self::SignedOut => None,
        }
    }

    fn session_mut(&mut self) -> Option<&mut WfmSession> {
        match self {
            Self::SignedIn(session) => Some(session),
            Self::SignedOut => None,
        }
    }

    /// Record the server-confirmed status; no-op when signed out.
    fn record_status(&mut self, status: Status) {
        if let Self::SignedIn(session) = self {
            session.current_status = Some(status);
        }
    }
}

// ── Actor ──

/// Snapshot of the signed-in session, as reported by the actor.
pub(crate) struct SessionInfo {
    pub(crate) status: Option<Status>,
    pub(crate) expires_at: DateTime<Utc>,
}

/// Commands processed by the WFM actor task. Long WS round-trips never block
/// the actor: `SetStatus` replies with the pending-reply receiver, which the
/// caller awaits with a timeout.
enum WfmCmd {
    SignIn {
        tokens: AuthTokenData,
        reply: oneshot::Sender<Result<(), WfmError>>,
    },
    SignOut,
    SetStatus {
        payload: StatusSetPayload,
        reply: oneshot::Sender<Result<oneshot::Receiver<WsReply>, WfmError>>,
    },
    GetSession {
        reply: oneshot::Sender<Option<SessionInfo>>,
    },
    /// Server-confirmed status, sent by the recv loop or after a successful set.
    RecordStatus(Status),
    /// The WS recv loop ended or auth was revoked; drop the session.
    Disconnected,
}

/// Cheaply-cloneable handle to the WFM actor, the only way to interact with
/// the sign-in state machine.
#[derive(Clone)]
pub struct WfmHandle(mpsc::Sender<WfmCmd>);

impl WfmHandle {
    /// Spawn the actor task owning the [`WfmState`] machine.
    #[must_use]
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::channel(16);
        let handle = Self(tx);
        tokio::spawn(actor_loop(handle.clone(), rx));
        handle
    }

    async fn send(&self, cmd: WfmCmd) -> Result<(), WfmError> {
        self.0
            .send(cmd)
            .await
            .map_err(|_| WfmError::ActorUnavailable)
    }

    /// Connect the WS, authenticate, and store the session.
    async fn sign_in(&self, tokens: AuthTokenData) -> Result<(), WfmError> {
        let (tx, rx) = oneshot::channel();
        self.send(WfmCmd::SignIn { tokens, reply: tx }).await?;
        rx.await?
    }

    async fn sign_out(&self) -> Result<(), WfmError> {
        self.send(WfmCmd::SignOut).await
    }

    async fn session(&self) -> Option<SessionInfo> {
        let (tx, rx) = oneshot::channel();
        self.send(WfmCmd::GetSession { reply: tx }).await.ok()?;
        rx.await.ok().flatten()
    }

    /// Send a status/set command and wait for the server reply. The actor
    /// only performs the (fast) WS write; this method awaits the reply.
    async fn set_status(&self, payload: StatusSetPayload) -> Result<WsReply, WfmError> {
        let (tx, rx) = oneshot::channel();
        self.send(WfmCmd::SetStatus { payload, reply: tx }).await?;
        let reply_rx = rx.await??;
        await_reply(reply_rx).await
    }

    async fn record_status(&self, status: Status) {
        self.send(WfmCmd::RecordStatus(status)).await.ok();
    }
}

// ── Handlers ──

#[derive(Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct SignstatusParams {
    /// Set status (online, invisible, ingame)
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    /// PATCH semantics: absent = don't change, null = remove, number = set
    #[cfg_attr(
        feature = "cli",
        arg(long, value_parser = parse_nullable_u64, help = "Set status duration (seconds), use \"null\" to clear")
    )]
    #[serde(
        default,
        with = "serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    #[allow(
        clippy::option_option,
        reason = "PATCH semantics: absent (don't change) vs null (remove) vs number (set)"
    )]
    duration: Option<Option<u64>>,
}

#[derive(Debug, Serialize)]
struct StatusSetPayload {
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(
        clippy::option_option,
        reason = "PATCH semantics: absent (don't change) vs null (remove) vs number (set)"
    )]
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

impl HandleOp for SignstatusParams {
    type Response = SignstatusResponse;

    async fn handle(self, cx: &Handles) -> Result<Self::Response, ControlError> {
        Ok(handle_wfm_signstatus(&cx.wfm, self).await?)
    }
}

// ── Sign in handler ──

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct SigninParams {
    /// Account email
    #[cfg_attr(feature = "cli", arg(long))]
    email: String,
    /// Account password
    #[cfg_attr(feature = "cli", arg(long))]
    password: String,
    /// Client ID
    #[cfg_attr(feature = "cli", arg(long, default_value = "wf-info-2"))]
    #[serde(default = "default_app_name")]
    client_id: String,
    /// Device name
    #[cfg_attr(feature = "cli", arg(long, default_value = "wf-info-2"))]
    #[serde(default = "default_app_name")]
    device_name: String,
}

impl HandleOp for SigninParams {
    type Response = EmptyResponse;

    async fn handle(self, cx: &Handles) -> Result<Self::Response, ControlError> {
        handle_wfm_signin(&cx.http, &cx.wfm, self).await?;
        Ok(EmptyResponse {})
    }
}

async fn actor_loop(handle: WfmHandle, mut rx: mpsc::Receiver<WfmCmd>) {
    let mut state = WfmState::default();
    while let Some(cmd) = rx.recv().await {
        match cmd {
            WfmCmd::SignIn { tokens, reply } => {
                let result = async {
                    let session = WsConnection::connect(handle.clone())
                        .await?
                        .authenticate(tokens)
                        .await?;
                    state.sign_in(session);
                    Ok(())
                }
                .await;
                // Reply failures mean the requester gave up; nothing to do.
                reply.send(result).ok();
            }
            WfmCmd::SignOut | WfmCmd::Disconnected => state.sign_out(),
            WfmCmd::SetStatus { payload, reply } => {
                let result = match state.session_mut() {
                    Some(session) => {
                        session
                            .conn
                            .send_command(WsRoute::StatusSet, &payload)
                            .await
                    }
                    None => Err(WfmError::NotConnected),
                };
                reply.send(result).ok();
            }
            WfmCmd::GetSession { reply } => {
                reply
                    .send(state.session().map(|session| SessionInfo {
                        status: session.current_status,
                        expires_at: session.tokens.expires_at,
                    }))
                    .ok();
            }
            WfmCmd::RecordStatus(status) => state.record_status(status),
        }
    }
}

// ── REST auth calls (v1 API) ──
//
// v1 signin: POST /auth/signin with snake_case body, JWT returned in
// Authorization response header as "JWT <token>".

/// Sign in via v1 API. Returns the JWT access token.
async fn rest_signin(
    client: &reqwest::Client,
    email: &str,
    password: &str,
    device_id: &str,
) -> Result<String, WfmError> {
    let raw_resp = client
        .post(format!("{WFM_AUTH_BASE}/auth/signin"))
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
        return Err(WfmError::SigninFailed { status, body });
    }

    // Extract JWT from Authorization header ("JWT <token>")
    let auth_header = raw_resp
        .headers()
        .get("Authorization")
        .ok_or(WfmError::NoAuthHeader)?
        .to_str()
        .map_err(|e| WfmError::AuthHeaderEncoding(e.to_string()))?
        .to_owned();

    let Some(jwt) = auth_header.strip_prefix("JWT ") else {
        return Err(WfmError::AuthHeaderFormat(auth_header));
    };

    Ok(jwt.to_owned())
}

// ── WebSocket recv loop ──

async fn await_reply(rx: oneshot::Receiver<WsReply>) -> Result<WsReply, WfmError> {
    Ok(timeout(Duration::from_secs(15), rx).await??)
}

async fn ws_recv_loop(
    wfm: WfmHandle,
    mut ws_rx: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
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
                log::warn!("WFM WebSocket error: {e}");
                break;
            }
            _ => continue,
        };

        log::debug!("WFM WS raw message: {msg}");

        let parsed: WsMessage = match serde_json::from_str(&msg) {
            Ok(m) => m,
            Err(e) => {
                log::debug!("WFM WS unparseable message: {e}");
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

        match route.parse::<IncomingRoute>() {
            Ok(IncomingRoute::Reply { command, outcome }) => {
                let mut map = pending.lock().await;
                if let Some(tx) = map.remove(&command) {
                    // A dropped waiter (timed out) is fine; discard the reply.
                    tx.send(WsReply {
                        outcome,
                        payload: parsed.payload,
                    })
                    .ok();
                }
            }
            Ok(IncomingRoute::Event(WsEvent::StatusSet)) => {
                let payload = parsed
                    .payload
                    .map(serde_json::from_value::<StatusEventPayload>);
                match payload {
                    Some(Ok(StatusEventPayload { status })) => {
                        wfm.record_status(status).await;
                    }
                    Some(Err(e)) => log::debug!("Ignoring unparseable WFM status event: {e}"),
                    None => {}
                }
            }
            Ok(IncomingRoute::Event(WsEvent::AuthRevoked)) => {
                log::warn!("WFM auth token revoked by server");
                wfm.send(WfmCmd::Disconnected).await.ok();
            }
            Err(_) => {}
        }
    }

    // Connection dropped — clear session
    wfm.send(WfmCmd::Disconnected).await.ok();
    log::info!("WFM WebSocket disconnected");
}

pub(crate) async fn handle_wfm_signstatus(
    wfm: &WfmHandle,
    p: SignstatusParams,
) -> Result<SignstatusResponse, WfmError> {
    // If no status provided, return current state
    let Some(raw_status) = p.status else {
        return match wfm.session().await {
            Some(session) => {
                let expired = session.expires_at < Utc::now();
                Ok(SignstatusResponse::Authenticated {
                    status: session.status,
                    expires_at: session.expires_at.to_rfc3339(),
                    expired,
                })
            }
            None => Ok(SignstatusResponse::Unauthenticated),
        };
    };
    let status = raw_status
        .parse::<Status>()
        .map_err(|_| WfmError::InvalidStatus(raw_status))?;

    // PATCH semantics for duration:
    // None serializes as omitted, Some(None) as null, Some(Some(n)) as n
    wfm.set_status(StatusSetPayload {
        status,
        duration: p.duration,
    })
    .await?
    .into_result()
    .map_err(|e| WfmError::StatusUpdateFailed(e.to_string()))?;

    wfm.record_status(status).await;

    Ok(SignstatusResponse::Set { status })
}

fn default_app_name() -> String {
    "wf-info-2".to_owned()
}

#[cfg(feature = "cli")]
fn parse_nullable_u64(raw: &str) -> Result<Option<u64>, String> {
    if raw.eq_ignore_ascii_case("null") {
        Ok(None)
    } else {
        raw.parse::<u64>().map(Some).map_err(|e| e.to_string())
    }
}

pub(crate) async fn handle_wfm_signin(
    client: &reqwest::Client,
    wfm: &WfmHandle,
    p: SigninParams,
) -> Result<(), WfmError> {
    // Load existing device_id or generate a new stable one
    let device_id = match storage::read_auth_token() {
        Ok(existing) => existing.device_id,
        Err(_) => uuid::Uuid::new_v4().to_string(),
    };

    let jwt = rest_signin(client, &p.email, &p.password, &device_id).await?;

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
    wfm.sign_in(token_data).await?;

    Ok(())
}

// ── Sign out handler ──

pub(crate) async fn handle_wfm_signout(wfm: &WfmHandle) -> Result<(), WfmError> {
    wfm.sign_out().await?;

    // Delete stored tokens
    storage::delete_auth_token()?;

    log::info!("Signed out from WFM");
    Ok(())
}

// ── Session restore (called on daemon start) ──

pub async fn try_restore_session(wfm: &WfmHandle) {
    // No cached token means nothing to restore
    let Ok(token_data) = storage::read_auth_token() else {
        return;
    };

    // If token is expired, we can't refresh with v1 — user must re-login
    if token_data.expires_at < Utc::now() {
        log::warn!("WFM token expired, please sign in again");
        return;
    }

    match wfm.sign_in(token_data).await {
        Ok(()) => log::info!("WFM session restored from cached token"),
        Err(e) => log::warn!("Failed to restore WFM session: {e}"),
    }
}

/// Set the WFM profile status if authenticated. Used by daemon for auto-status.
pub async fn set_status_if_connected(wfm: &WfmHandle, status: Status) {
    if wfm.session().await.is_none() {
        return;
    }

    let payload = StatusSetPayload {
        status,
        duration: None,
    };
    match wfm.set_status(payload).await {
        Ok(_) => {
            wfm.record_status(status).await;
            log::info!("WFM status set to '{status}'");
        }
        Err(e) => log::warn!("Failed to set WFM status: {e}"),
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
            serde_json::json!({ "status": "online", "duration": 60_u64 })
        );
    }
}
