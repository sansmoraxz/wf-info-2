use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::control_ops::ControlOp;

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
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Response {
    fn ok(id: Option<String>, data: ResponseData) -> Self {
        Self {
            id,
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(id: Option<String>, message: String) -> Self {
        Self {
            id,
            ok: false,
            data: None,
            error: Some(message),
        }
    }
}

/// Result of handling a request, including optional subscription filter.
pub(crate) struct HandleResult {
    pub response: Response,
    /// If set, the connection should transition to subscription mode with this filter.
    pub subscription_filter: Option<EventFilter>,
}

pub(crate) async fn handle_line(line: &str) -> HandleResult {
    match serde_json::from_str::<Request>(line) {
        Ok(req) => handle_request(req).await,
        Err(e) => HandleResult {
            response: Response::error(None, format!("Invalid request: {}", e)),
            subscription_filter: None,
        },
    }
}

async fn handle_request(req: Request) -> HandleResult {
    let id = req.id.clone();

    // Handle subscribe separately since it needs to return the filter
    if let Ok(ControlOp::Subscribe) = ControlOp::parse(&req.op) {
        let result =
            parse_params::<SubscribeParams>(req.params).and_then(subscription::handle_subscribe);
        return match result {
            Ok(result) => HandleResult {
                response: Response::ok(id, ResponseData::Subscribe(Box::new(result.response))),
                subscription_filter: Some(result.filter),
            },
            Err(e) => HandleResult {
                response: Response::error(id, e.to_string()),
                subscription_filter: None,
            },
        };
    }

    let result = dispatch(&req.op, req.params).await;

    HandleResult {
        response: match result {
            Ok(data) => Response::ok(id, data),
            Err(e) => Response::error(id, e.to_string()),
        },
        subscription_filter: None,
    }
}

async fn dispatch(op: &str, params: Option<Value>) -> anyhow::Result<ResponseData> {
    Ok(match ControlOp::parse(op)? {
        ControlOp::Ping => ResponseData::Ping(Box::new(PingResponse { pong: true })),
        ControlOp::InventoryLoad => ResponseData::InventoryLoad(Box::new(
            handle_inventory_load(parse_params(params)?).await?,
        )),
        ControlOp::InventoryFilter => ResponseData::InventoryFilter(Box::new(
            handle_inventory_filter(parse_params(params)?).await?,
        )),
        ControlOp::InventoryMetaGet => {
            ResponseData::InventoryMeta(Box::new(handle_inventory_meta_get()?))
        }
        ControlOp::InventoryStaleUpdate => ResponseData::InventoryMeta(Box::new(
            handle_inventory_stale_update(parse_params(params)?)?,
        )),
        ControlOp::ScreenshotTrigger => ResponseData::Screenshot(Box::new(
            handle_screenshot_trigger(parse_params(params)?).await?,
        )),
        ControlOp::InventoryRefresh => {
            ResponseData::InventoryLoad(Box::new(dispatch_refresh(params).await?))
        }
        ControlOp::WFMarketPrice => {
            ResponseData::MarketPrice(Box::new(handle_market_price(parse_params(params)?).await?))
        }
        ControlOp::WFMarketRefresh => {
            ResponseData::MarketRefresh(Box::new(handle_market_refresh().await?))
        }
        ControlOp::WfmSignstatus => ResponseData::Signstatus(Box::new(
            handle_wfm_signstatus(parse_params(params)?).await?,
        )),
        ControlOp::WfmSignin => {
            handle_wfm_signin(parse_signin_params(params)?).await?;
            ResponseData::Empty(Box::new(EmptyResponse {}))
        }
        ControlOp::WfmSignout => {
            handle_wfm_signout().await?;
            ResponseData::Empty(Box::new(EmptyResponse {}))
        }
        ControlOp::Subscribe => return Err(anyhow::anyhow!("Unexpected subscribe operation")),
    })
}

#[cfg(feature = "memory")]
async fn dispatch_refresh(params: Option<Value>) -> anyhow::Result<InventoryLoadResponse> {
    handle_inventory_refresh(parse_params(params)?).await
}

#[cfg(not(feature = "memory"))]
async fn dispatch_refresh(_params: Option<Value>) -> anyhow::Result<InventoryLoadResponse> {
    handle_inventory_refresh().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn ping_response_matches_legacy_wire_shape() {
        let result = handle_line(r#"{"id":"1","op":"ping"}"#).await;
        let value = serde_json::to_value(&result.response).unwrap();
        assert_eq!(
            value,
            json!({ "id": "1", "ok": true, "data": { "pong": true } })
        );
        assert!(result.subscription_filter.is_none());
    }

    #[tokio::test]
    async fn malformed_request_returns_error_shape() {
        let result = handle_line("not json").await;
        let value = serde_json::to_value(&result.response).unwrap();
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
        let result = handle_line(r#"{"id":"2","op":"nope"}"#).await;
        let value = serde_json::to_value(&result.response).unwrap();
        assert_eq!(value["ok"], json!(false));
        assert_eq!(value["error"], json!("Unknown operation 'nope'"));
    }

    #[tokio::test]
    async fn subscribe_returns_filter_and_typed_response() {
        let result =
            handle_line(r#"{"id":"3","op":"subscribe","params":{"events":["game_start"]}}"#).await;
        let value = serde_json::to_value(&result.response).unwrap();
        assert_eq!(value["ok"], json!(true));
        assert_eq!(value["data"]["subscribed"], json!(true));
        assert_eq!(
            value["data"]["filter"]["allowed_events"],
            json!(["game_start"])
        );
        assert!(result.subscription_filter.is_some());
    }

    #[test]
    fn empty_response_serializes_to_empty_object() {
        assert_eq!(serde_json::to_value(EmptyResponse {}).unwrap(), json!({}));
    }

    #[test]
    fn signstatus_response_matches_legacy_shapes() {
        let unauth = SignstatusResponse::Unauthenticated {
            authenticated: false,
            status: None,
        };
        assert_eq!(
            serde_json::to_value(&unauth).unwrap(),
            json!({ "authenticated": false, "status": null })
        );

        let auth = SignstatusResponse::Authenticated {
            authenticated: true,
            status: Some(crate::wfm_auth::Status::Online),
            expires_at: "2026-07-27T00:00:00+00:00".into(),
            expired: false,
        };
        assert_eq!(
            serde_json::to_value(&auth).unwrap(),
            json!({
                "authenticated": true,
                "status": "online",
                "expires_at": "2026-07-27T00:00:00+00:00",
                "expired": false,
            })
        );

        let set = SignstatusResponse::Set {
            ok: true,
            status: crate::wfm_auth::Status::Ingame,
        };
        assert_eq!(
            serde_json::to_value(&set).unwrap(),
            json!({ "ok": true, "status": "ingame" })
        );
    }
}
