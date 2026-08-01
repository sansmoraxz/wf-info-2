use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::control_ops::ControlOp;
use crate::state::AppState;

use super::inventory::{
    InventoryFilterResponse, InventoryLoadResponse, handle_inventory_filter, handle_inventory_load,
    handle_inventory_meta_get, handle_inventory_refresh, handle_inventory_stale_update,
};
use super::market::{
    MarketPriceResponse, MarketRefreshResponse, handle_market_price, handle_market_refresh,
};
use super::screenshot::{ScreenshotEvent, handle_screenshot_trigger};
use super::subscription::{self, EventFilter, SubscribeParams, SubscribeResponse};
use super::utils::parse_params;
use super::wfm_auth::{
    SignstatusResponse, handle_wfm_signin, handle_wfm_signout, handle_wfm_signstatus,
    parse_signin_params,
};

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
/// variant serializes as its inner response object.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub(crate) enum ResponseData {
    Ping(Box<PingResponse>),
    InventoryLoad(Box<InventoryLoadResponse>),
    InventoryFilter(Box<InventoryFilterResponse>),
    InventoryMeta(Box<wf_core::storage::InventoryMeta>),
    Screenshot(Box<ScreenshotEvent>),
    MarketPrice(Box<MarketPriceResponse>),
    MarketRefresh(Box<MarketRefreshResponse>),
    Signstatus(Box<SignstatusResponse>),
    Subscribe(Box<SubscribeResponse>),
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

pub(crate) async fn handle_line(state: &Arc<AppState>, line: &str) -> HandleOutcome {
    match serde_json::from_str::<Request>(line) {
        Ok(req) => handle_request(state, req).await,
        Err(e) => HandleOutcome::Reply(Response::error(None, format!("Invalid request: {}", e))),
    }
}

async fn handle_request(state: &Arc<AppState>, req: Request) -> HandleOutcome {
    let id = req.id.clone();

    // Handle subscribe separately since it needs to return the filter
    if let Ok(ControlOp::Subscribe) = req.op.parse() {
        let result =
            parse_params::<SubscribeParams>(req.params).and_then(subscription::handle_subscribe);
        return match result {
            Ok(result) => HandleOutcome::EnterSubscription {
                response: Response::ok(id, ResponseData::Subscribe(Box::new(result.response))),
                filter: result.filter,
            },
            Err(e) => HandleOutcome::Reply(Response::error(id, e.to_string())),
        };
    }

    HandleOutcome::Reply(match dispatch(state, &req.op, req.params).await {
        Ok(data) => Response::ok(id, data),
        Err(e) => Response::error(id, e.to_string()),
    })
}

async fn dispatch(
    state: &Arc<AppState>,
    op: &str,
    params: Option<Value>,
) -> anyhow::Result<ResponseData> {
    let op: ControlOp = op
        .parse()
        .map_err(|_| anyhow::anyhow!("Unknown operation '{}'", op))?;
    Ok(match op {
        ControlOp::Ping => ResponseData::Ping(Box::new(PingResponse { pong: true })),
        ControlOp::InventoryLoad => ResponseData::InventoryLoad(Box::new(
            handle_inventory_load(state, parse_params(params)?).await?,
        )),
        ControlOp::InventoryFilter => ResponseData::InventoryFilter(Box::new(
            handle_inventory_filter(state, parse_params(params)?).await?,
        )),
        ControlOp::InventoryMetaGet => {
            ResponseData::InventoryMeta(Box::new(handle_inventory_meta_get()?))
        }
        ControlOp::InventoryStaleUpdate => ResponseData::InventoryMeta(Box::new(
            handle_inventory_stale_update(state, parse_params(params)?)?,
        )),
        ControlOp::ScreenshotTrigger => ResponseData::Screenshot(Box::new(
            handle_screenshot_trigger(state, parse_params(params)?).await?,
        )),
        ControlOp::InventoryRefresh => {
            ResponseData::InventoryLoad(Box::new(dispatch_refresh(state, params).await?))
        }
        ControlOp::WFMarketPrice => ResponseData::MarketPrice(Box::new(
            handle_market_price(state, parse_params(params)?).await?,
        )),
        ControlOp::WFMarketRefresh => {
            ResponseData::MarketRefresh(Box::new(handle_market_refresh(state).await?))
        }
        ControlOp::WfmSignstatus => ResponseData::Signstatus(Box::new(
            handle_wfm_signstatus(state, parse_params(params)?).await?,
        )),
        ControlOp::WfmSignin => {
            handle_wfm_signin(state, parse_signin_params(params)?).await?;
            ResponseData::Empty(Box::new(EmptyResponse {}))
        }
        ControlOp::WfmSignout => {
            handle_wfm_signout(state).await?;
            ResponseData::Empty(Box::new(EmptyResponse {}))
        }
        ControlOp::Subscribe => return Err(anyhow::anyhow!("Unexpected subscribe operation")),
    })
}

#[cfg(feature = "memory")]
async fn dispatch_refresh(
    state: &AppState,
    params: Option<Value>,
) -> anyhow::Result<InventoryLoadResponse> {
    handle_inventory_refresh(state, parse_params(params)?).await
}

#[cfg(not(feature = "memory"))]
async fn dispatch_refresh(
    state: &AppState,
    _params: Option<Value>,
) -> anyhow::Result<InventoryLoadResponse> {
    handle_inventory_refresh(state).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn ping_response_matches_legacy_wire_shape() {
        let state = Arc::new(AppState::default());
        let outcome = handle_line(&state, r#"{"id":"1","op":"ping"}"#).await;
        let value = serde_json::to_value(outcome.response()).unwrap();
        assert_eq!(
            value,
            json!({ "id": "1", "ok": true, "data": { "pong": true } })
        );
        assert!(matches!(outcome, HandleOutcome::Reply(_)));
    }

    #[tokio::test]
    async fn malformed_request_returns_error_shape() {
        let state = Arc::new(AppState::default());
        let outcome = handle_line(&state, "not json").await;
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
        let state = Arc::new(AppState::default());
        let outcome = handle_line(&state, r#"{"id":"2","op":"nope"}"#).await;
        let value = serde_json::to_value(outcome.response()).unwrap();
        assert_eq!(value["ok"], json!(false));
        assert_eq!(value["error"], json!("Unknown operation 'nope'"));
    }

    #[tokio::test]
    async fn subscribe_returns_filter_and_typed_response() {
        let state = Arc::new(AppState::default());
        let outcome = handle_line(
            &state,
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
