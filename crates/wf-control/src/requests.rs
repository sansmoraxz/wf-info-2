use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::control_ops::{ControlOp, InventoryOp, ScreenshotOp, WfmOp};

use super::events::EventBus;
use super::inventory::{
    FilterParams, InventoryFilterResponse, InventoryLoadResponse, LoadInventoryParams,
    RefreshParams, StaleParams, handle_inventory_meta_get,
};
use super::market::{
    MarketCache, MarketPriceParams, MarketPriceResponse, MarketRefreshResponse,
    handle_market_refresh,
};
use super::screenshot::{ScreenshotConfig, ScreenshotEvent, ScreenshotParams, ScreenshotState};
use super::search::InventoryIndexCache;
use super::subscription::{self, EventFilter, SubscribeParams, SubscribeResponse};
use super::utils::{parse_params, parse_required_params};
use super::wfm_auth::{
    SigninParams, SignstatusParams, SignstatusResponse, WfmHandle, handle_wfm_signout,
};

/// Cheaply-cloneable bundle of every per-module handle, assembled once at the
/// composition root and threaded through the control server.
#[derive(Clone)]
pub struct Handles {
    pub events: EventBus,
    pub wfm: WfmHandle,
    /// Process-wide HTTP client; clones share the same connection pool.
    pub http: reqwest::Client,
    pub(crate) market: Arc<MarketCache>,
    pub(crate) inventory_index: Arc<InventoryIndexCache>,
    pub(crate) item_index: Arc<wf_itemdata::item_data::ItemIndex>,
    pub screenshot: Arc<ScreenshotState>,
}

impl Handles {
    /// Build all handles, spawning the WFM actor. Must run inside a tokio
    /// runtime.
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

/// A control operation's params type: handling consumes the params and yields
/// a typed response that converts into [`ResponseData`].
pub(crate) trait HandleOp {
    type Response: Into<ResponseData>;
    async fn handle(self, cx: &Handles) -> anyhow::Result<Self::Response>;
}

async fn run<P: HandleOp>(params: P, cx: &Handles) -> anyhow::Result<ResponseData> {
    Ok(params.handle(cx).await?.into())
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Option<String>,
    pub op: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize)]
pub(crate) struct PingResponse {
    pub pong: bool,
}

/// Serializes to `{}`.
#[derive(Debug, Serialize)]
pub(crate) struct EmptyResponse {}

/// Typed payload of a successful response. Serialize-only untagged: each
/// variant serializes as its inner response object. `#[from]` also accepts
/// the unboxed type, forwarding through `Box: From<T>`.
#[derive(Debug, Serialize, derive_more::From)]
#[serde(untagged)]
pub(crate) enum ResponseData {
    #[from(PingResponse, Box<PingResponse>)]
    Ping(Box<PingResponse>),
    #[from(InventoryLoadResponse, Box<InventoryLoadResponse>)]
    InventoryLoad(Box<InventoryLoadResponse>),
    #[from(InventoryFilterResponse, Box<InventoryFilterResponse>)]
    InventoryFilter(Box<InventoryFilterResponse>),
    #[from(wf_core::storage::InventoryMeta, Box<wf_core::storage::InventoryMeta>)]
    InventoryMeta(Box<wf_core::storage::InventoryMeta>),
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
pub struct Response {
    pub id: Option<String>,
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

/// Outcome of handling a request line: either a plain reply, or a reply that
/// transitions the connection into subscription mode.
pub(crate) enum HandleOutcome {
    Reply(Response),
    EnterSubscription {
        response: Response,
        filter: EventFilter,
    },
}

impl HandleOutcome {
    pub(crate) fn response(&self) -> &Response {
        match self {
            Self::Reply(response) => response,
            Self::EnterSubscription { response, .. } => response,
        }
    }
}

pub(crate) async fn handle_line(cx: &Handles, line: &str) -> HandleOutcome {
    match serde_json::from_str::<Request>(line) {
        Ok(req) => handle_request(cx, req).await,
        Err(e) => HandleOutcome::Reply(Response::error(None, format!("Invalid request: {}", e))),
    }
}

async fn handle_request(cx: &Handles, req: Request) -> HandleOutcome {
    let id = req.id.clone();

    // Handle subscribe separately since it needs to return the filter
    if let Ok(ControlOp::Subscribe) = req.op.parse() {
        let result =
            parse_params::<SubscribeParams>(req.params).and_then(subscription::handle_subscribe);
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

async fn dispatch(cx: &Handles, op: &str, params: Option<Value>) -> anyhow::Result<ResponseData> {
    let op: ControlOp = op
        .parse()
        .map_err(|_| anyhow::anyhow!("Unknown operation '{}'", op))?;
    Ok(match op {
        ControlOp::Ping => PingResponse { pong: true }.into(),
        ControlOp::Inventory(InventoryOp::Load) => {
            run(parse_params::<LoadInventoryParams>(params)?, cx).await?
        }
        ControlOp::Inventory(InventoryOp::Filter) => {
            run(parse_params::<FilterParams>(params)?, cx).await?
        }
        ControlOp::Inventory(InventoryOp::MetaGet) => handle_inventory_meta_get()?.into(),
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
        ControlOp::Wfm(WfmOp::Signin) => run(parse_required_params::<SigninParams>(params)?, cx).await?,
        ControlOp::Wfm(WfmOp::Signout) => {
            handle_wfm_signout(&cx.wfm).await?;
            EmptyResponse {}.into()
        }
        ControlOp::Subscribe => return Err(anyhow::anyhow!("Unexpected subscribe operation")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
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
            status: Some(crate::wfm_auth::Status::Online),
            expires_at: "2026-07-27T00:00:00+00:00".into(),
            expired: false,
        };
        assert_eq!(
            serde_json::to_value(&auth).unwrap(),
            json!({
                "state": "authenticated",
                "status": "online",
                "expires_at": "2026-07-27T00:00:00+00:00",
                "expired": false,
            })
        );

        let set = SignstatusResponse::Set {
            status: crate::wfm_auth::Status::Ingame,
        };
        assert_eq!(
            serde_json::to_value(&set).unwrap(),
            json!({ "state": "set", "status": "ingame" })
        );
    }
}
