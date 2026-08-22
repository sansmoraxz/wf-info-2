use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::sync::Arc;

use tantivy::query::QueryParserError;
use wf_core::storage::{self, InventoryMeta};
use wf_itemdata::item_data::ItemIndex;

use crate::control_ops::{ControlOp, InventoryOp, ScreenshotOp, WfmOp};

use super::events::EventBus;
use super::inventory::{
    FilterParams, InventoryError, InventoryFilterResponse, InventoryLoadResponse,
    LoadInventoryParams, RefreshParams, StaleParams, handle_inventory_meta_get,
};
use super::market::{
    MarketCache, MarketError, MarketPriceParams, MarketPriceResponse, MarketRefreshResponse,
    handle_market_refresh,
};
use super::screenshot::{
    ScreenshotConfig, ScreenshotError, ScreenshotEvent, ScreenshotParams, ScreenshotState,
};
use super::search::{InventoryIndexCache, SearchError};
use super::subscription::{self, EventFilter, SubscribeParams, SubscribeResponse};
use super::utils::{ParamsError, parse_params, parse_required_params};
use super::wfm_auth::{
    SigninParams, SignstatusParams, SignstatusResponse, WfmError, WfmHandle, handle_wfm_signout,
};

/// Cheaply-cloneable bundle of every per-module handle, assembled once at the
/// composition root and threaded through the control server.
#[derive(Clone)]
pub struct Handles {
    pub events: EventBus,
    pub wfm: WfmHandle,
    /// Process-wide HTTP client; clones share the same connection pool.
    pub http: reqwest::Client,
    pub market: Arc<MarketCache>,
    pub(crate) inventory_index: Arc<InventoryIndexCache>,
    pub(crate) item_index: Arc<ItemIndex>,
    pub screenshot: Arc<ScreenshotState>,
}

impl Handles {
    /// Build all handles, spawning the WFM actor. Must run inside a tokio
    /// runtime.
    #[must_use]
    pub fn new(screenshot: ScreenshotConfig) -> Self {
        let http = reqwest::Client::new();
        Self {
            events: EventBus::new(),
            wfm: WfmHandle::spawn(),
            market: Arc::new(MarketCache::from(http.clone())),
            http,
            inventory_index: Arc::default(),
            item_index: Arc::default(),
            screenshot: Arc::new(screenshot.into()),
        }
    }
}

/// Any error a control operation can produce. Stringified via `Display` at
/// the wire boundary in [`handle_request`], so each variant's message (or its
/// transparent inner message) IS the wire error string.
#[derive(Debug, thiserror::Error)]
pub(super) enum ControlError {
    #[error("Unknown operation '{0}'")]
    UnknownOperation(String),
    #[error("Unexpected subscribe operation")]
    UnexpectedSubscribe,
    #[error(transparent)]
    Params(#[from] ParamsError),
    #[error(transparent)]
    Inventory(#[from] InventoryError),
    #[error(transparent)]
    Search(#[from] SearchError),
    #[error(transparent)]
    Tantivy(#[from] tantivy::TantivyError),
    #[error(transparent)]
    QueryParser(#[from] QueryParserError),
    #[error(transparent)]
    Market(#[from] MarketError),
    #[error(transparent)]
    Wfm(#[from] WfmError),
    #[error(transparent)]
    Screenshot(#[from] ScreenshotError),
    #[error(transparent)]
    Storage(#[from] storage::StorageError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

/// A control operation's params type: handling consumes the params and yields
/// a typed response that converts into [`ResponseData`].
pub(super) trait HandleOp {
    type Response: Into<ResponseData>;
    async fn handle(self, cx: &Handles) -> Result<Self::Response, ControlError>;
}

/// Wire request envelope, shared by the daemon (deserialize) and CLI (serialize).
/// `params` stays unparsed JSON text ([`RawValue`]) so the payload is
/// deserialized exactly once, directly into the op's typed params struct.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub op: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Box<RawValue>>,
}

#[derive(Debug, Serialize)]
pub(super) struct PingResponse {
    pub pong: monostate::MustBe!(true),
}

/// Serializes to `{}`; a unit struct would serialize as `null`.
#[derive(Debug, Serialize)]
pub(super) struct EmptyResponse {}

/// Typed payload of a successful response. Serialize-only untagged: each
/// variant serializes as its inner response object. `#[from]` also accepts
/// the unboxed type, forwarding through `Box: From<T>`.
#[derive(Debug, Serialize, derive_more::From)]
#[serde(untagged)]
pub(super) enum ResponseData {
    #[from(PingResponse, Box<PingResponse>)]
    Ping(Box<PingResponse>),
    #[from(InventoryLoadResponse, Box<InventoryLoadResponse>)]
    InventoryLoad(Box<InventoryLoadResponse>),
    #[from(InventoryFilterResponse, Box<InventoryFilterResponse>)]
    InventoryFilter(Box<InventoryFilterResponse>),
    #[from(InventoryMeta, Box<InventoryMeta>)]
    InventoryMeta(Box<InventoryMeta>),
    #[from(ScreenshotEvent, Box<ScreenshotEvent>)]
    Screenshot(Box<ScreenshotEvent>),
    #[from(MarketPriceResponse, Box<MarketPriceResponse>)]
    MarketPrice(Box<MarketPriceResponse>),
    #[from(MarketRefreshResponse, Box<MarketRefreshResponse>)]
    MarketRefresh(Box<MarketRefreshResponse>),
    #[from(SignstatusResponse, Box<SignstatusResponse>)]
    Signstatus(Box<SignstatusResponse>),
    #[from(SubscribeResponse, Box<SubscribeResponse>)]
    Subscribe(Box<SubscribeResponse>),
    #[from(EmptyResponse, Box<EmptyResponse>)]
    Empty(Box<EmptyResponse>),
}

#[derive(Debug, Serialize)]
pub(super) struct Response {
    pub(crate) id: Option<String>,
    #[serde(flatten)]
    body: ResponseBody,
}

/// Success and failure are mutually exclusive; the `ok` wire marker is fixed
/// per variant via `monostate` (legacy shape:
/// `{"ok":true,"data":..}` / `{"ok":false,"error":..}`).
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ResponseBody {
    Ok {
        ok: monostate::MustBe!(true),
        data: ResponseData,
    },
    Err {
        ok: monostate::MustBe!(false),
        error: String,
    },
}

impl Response {
    fn ok(id: Option<String>, data: ResponseData) -> Self {
        Self {
            id,
            body: ResponseBody::Ok {
                ok: monostate::MustBe!(true),
                data,
            },
        }
    }

    fn error(id: Option<String>, message: String) -> Self {
        Self {
            id,
            body: ResponseBody::Err {
                ok: monostate::MustBe!(false),
                error: message,
            },
        }
    }
}

/// Client-side view of a [`Response`] line, mirroring [`ResponseBody`]:
/// success and failure are mutually exclusive, discriminated by the `ok`
/// wire marker. `data` stays unparsed JSON for printing or forwarding.
/// The daemon serializes [`Response`]; clients deserialize this.
///
/// Deserialized via [`ResponseEnvelopeWire`] rather than `untagged`:
/// untagged enums buffer into serde's internal representation, which
/// [`RawValue`] cannot be recovered from.
#[derive(Debug, Deserialize)]
#[serde(from = "ResponseEnvelopeWire")]
pub enum ResponseEnvelope {
    Ok {
        id: Option<String>,
        data: Option<Box<RawValue>>,
    },
    Err {
        id: Option<String>,
        error: String,
    },
}

/// Flat wire shape [`ResponseEnvelope`] derives its `Deserialize` through.
#[derive(Deserialize)]
struct ResponseEnvelopeWire {
    id: Option<String>,
    ok: bool,
    error: Option<String>,
    data: Option<Box<RawValue>>,
}

impl From<ResponseEnvelopeWire> for ResponseEnvelope {
    fn from(wire: ResponseEnvelopeWire) -> Self {
        if wire.ok {
            Self::Ok {
                id: wire.id,
                data: wire.data,
            }
        } else {
            Self::Err {
                id: wire.id,
                error: wire.error.unwrap_or_else(|| "unknown error".to_owned()),
            }
        }
    }
}

/// Outcome of handling a request line: either a plain reply, or a reply that
/// transitions the connection into subscription mode.
pub(super) enum HandleOutcome {
    Reply(Response),
    EnterSubscription {
        response: Response,
        filter: EventFilter,
    },
}

impl HandleOutcome {
    pub(crate) fn response(&self) -> &Response {
        match self {
            Self::Reply(response) | Self::EnterSubscription { response, .. } => response,
        }
    }
}

async fn run<P>(params: P, cx: &Handles) -> Result<ResponseData, ControlError>
where
    P: HandleOp,
{
    Ok(params.handle(cx).await?.into())
}

pub(super) async fn handle_line(cx: &Handles, line: &str) -> HandleOutcome {
    match serde_json::from_str::<Request>(line) {
        Ok(req) => handle_request(cx, req).await,
        Err(e) => HandleOutcome::Reply(Response::error(None, format!("Invalid request: {e}"))),
    }
}

async fn handle_request(cx: &Handles, req: Request) -> HandleOutcome {
    let id = req.id.clone();

    // Handle subscribe separately since it needs to return the filter
    if let Ok(ControlOp::Subscribe) = req.op.parse() {
        let result =
            parse_params::<SubscribeParams>(req.params).map(subscription::handle_subscribe);
        return match result {
            Ok(result) => HandleOutcome::EnterSubscription {
                response: Response::ok(id, result.response.into()),
                filter: result.filter,
            },
            Err(e) => HandleOutcome::Reply(Response::error(id, e.to_string())),
        };
    }

    HandleOutcome::Reply(match dispatch(cx, &req.op, req.params).await {
        Ok(data) => Response::ok(id, data),
        Err(e) => Response::error(id, e.to_string()),
    })
}

async fn dispatch(
    cx: &Handles,
    op: &str,
    params: Option<Box<RawValue>>,
) -> Result<ResponseData, ControlError> {
    let op: ControlOp = op
        .parse()
        .map_err(|_| ControlError::UnknownOperation(op.to_owned()))?;
    Ok(match op {
        ControlOp::Ping => PingResponse {
            pong: monostate::MustBe!(true),
        }
        .into(),
        ControlOp::Inventory(InventoryOp::Load) => {
            run(parse_params::<LoadInventoryParams>(params)?, cx).await?
        }
        ControlOp::Inventory(InventoryOp::Filter) => {
            run(parse_params::<FilterParams>(params)?, cx).await?
        }
        ControlOp::Inventory(InventoryOp::MetaGet) => handle_inventory_meta_get().into(),
        ControlOp::Inventory(InventoryOp::StaleUpdate) => {
            run(parse_params::<StaleParams>(params)?, cx).await?
        }
        ControlOp::Inventory(InventoryOp::Refresh) => {
            run(parse_params::<RefreshParams>(params)?, cx).await?
        }
        ControlOp::Screenshot(ScreenshotOp::Trigger) => {
            run(parse_params::<ScreenshotParams>(params)?, cx).await?
        }
        ControlOp::Wfm(WfmOp::Price) => run(parse_params::<MarketPriceParams>(params)?, cx).await?,
        ControlOp::Wfm(WfmOp::Refresh) => handle_market_refresh(&cx.market).await?.into(),
        ControlOp::Wfm(WfmOp::Signstatus) => {
            run(parse_params::<SignstatusParams>(params)?, cx).await?
        }
        ControlOp::Wfm(WfmOp::Signin) => {
            run(parse_required_params::<SigninParams>(params)?, cx).await?
        }
        ControlOp::Wfm(WfmOp::Signout) => {
            handle_wfm_signout(&cx.wfm).await?;
            EmptyResponse {}.into()
        }
        ControlOp::Subscribe => return Err(ControlError::UnexpectedSubscribe),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wfm_auth::Status;
    use serde_json::json;

    #[tokio::test]
    async fn ping_response_matches_legacy_wire_shape() {
        let cx = Handles::new(ScreenshotConfig::default());
        let outcome = handle_line(&cx, r#"{"id":"1","op":"ping"}"#).await;
        let value = serde_json::to_value(outcome.response()).unwrap();
        assert_eq!(
            value,
            json!({ "id": "1", "ok": true, "data": { "pong": true } })
        );
        assert!(matches!(outcome, HandleOutcome::Reply(_)));
    }

    #[tokio::test]
    async fn malformed_request_returns_error_shape() {
        let cx = Handles::new(ScreenshotConfig::default());
        let outcome = handle_line(&cx, "not json").await;
        let value = serde_json::to_value(outcome.response()).unwrap();
        assert_eq!(value["ok"], json!(false));
        assert!(
            value["error"]
                .as_str()
                .unwrap()
                .starts_with("Invalid request:")
        );
        assert!(value.get("data").is_none());
        assert_eq!(value["id"], json!(null));
    }

    #[tokio::test]
    async fn unknown_op_returns_error() {
        let cx = Handles::new(ScreenshotConfig::default());
        let outcome = handle_line(&cx, r#"{"id":"2","op":"nope"}"#).await;
        let value = serde_json::to_value(outcome.response()).unwrap();
        assert_eq!(value["ok"], json!(false));
        assert_eq!(value["error"], json!("Unknown operation 'nope'"));
    }

    #[tokio::test]
    async fn subscribe_returns_filter_and_typed_response() {
        let cx = Handles::new(ScreenshotConfig::default());
        let outcome = handle_line(
            &cx,
            r#"{"id":"3","op":"subscribe","params":{"events":["game_start"]}}"#,
        )
        .await;
        let value = serde_json::to_value(outcome.response()).unwrap();
        assert_eq!(value["ok"], json!(true));
        assert_eq!(value["data"]["subscribed"], json!(true));
        assert_eq!(
            value["data"]["filter"]["allowed_events"],
            json!(["game_start"])
        );
        assert!(matches!(outcome, HandleOutcome::EnterSubscription { .. }));
    }

    /// [`ResponseEnvelope`] must keep parsing whatever [`Response`]
    /// serializes — ok and error shapes both.
    #[test]
    fn response_envelope_round_trips_both_response_shapes() {
        let ok = Response::ok(
            Some("7".into()),
            PingResponse {
                pong: monostate::MustBe!(true),
            }
            .into(),
        );
        let raw = serde_json::to_string(&ok).unwrap();
        let envelope: ResponseEnvelope = serde_json::from_str(&raw).unwrap();
        let ResponseEnvelope::Ok { id, data, .. } = envelope else {
            panic!("ok response parsed as Err variant");
        };
        assert_eq!(id.as_deref(), Some("7"));
        assert_eq!(data.unwrap().get(), r#"{"pong":true}"#);

        let err = Response::error(None, "boom".into());
        let raw = serde_json::to_string(&err).unwrap();
        let envelope: ResponseEnvelope = serde_json::from_str(&raw).unwrap();
        let ResponseEnvelope::Err { id, error, .. } = envelope else {
            panic!("error response parsed as Ok variant");
        };
        assert_eq!(id, None);
        assert_eq!(error, "boom");
    }

    #[test]
    fn empty_response_serializes_to_empty_object() {
        assert_eq!(serde_json::to_value(EmptyResponse {}).unwrap(), json!({}));
    }

    #[test]
    fn signstatus_response_wire_shapes() {
        let unauth = SignstatusResponse::Unauthenticated;
        assert_eq!(
            serde_json::to_value(&unauth).unwrap(),
            json!({ "state": "unauthenticated" })
        );

        let auth = SignstatusResponse::Authenticated {
            status: Some(Status::Online),
            expires_at: "2026-07-27T00:00:00Z".parse().unwrap(),
            expired: false,
        };
        assert_eq!(
            serde_json::to_value(&auth).unwrap(),
            json!({
                "state": "authenticated",
                "status": "online",
                "expires_at": "2026-07-27T00:00:00Z",
                "expired": false,
            })
        );

        let set = SignstatusResponse::Set {
            status: Status::Ingame,
        };
        assert_eq!(
            serde_json::to_value(&set).unwrap(),
            json!({ "state": "set", "status": "ingame" })
        );
    }
}
