use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::control_ops::ControlOp;

use super::inventory::{
    FilterParams, InventoryFilterResponse, InventoryLoadResponse, LoadInventoryParams, StaleParams,
    handle_inventory_filter, handle_inventory_load, handle_inventory_meta_get,
    handle_inventory_refresh, handle_inventory_stale_update,
};
use super::market::{
    MarketPriceParams, MarketPriceResponse, MarketRefreshResponse, handle_market_price,
    handle_market_refresh,
};
use super::screenshot::{ScreenshotEvent, ScreenshotParams, handle_screenshot_trigger};
use super::subscription::{self, EventFilter, SubscribeParams, SubscribeResponse};
use super::utils::parse_params;
use super::wfm_auth::{
    SignstatusParams, SignstatusResponse, handle_wfm_signin, handle_wfm_signout,
    handle_wfm_signstatus, parse_signin_params,
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
    Ping(PingResponse),
    InventoryLoad(InventoryLoadResponse),
    InventoryFilter(InventoryFilterResponse),
    InventoryMeta(wf_core::storage::InventoryMeta),
    Screenshot(ScreenshotEvent),
    MarketPrice(MarketPriceResponse),
    MarketRefresh(MarketRefreshResponse),
    Signstatus(SignstatusResponse),
    Subscribe(SubscribeResponse),
    Empty(EmptyResponse),
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
        let result = parse_params::<SubscribeParams>(req.params)
            .and_then(subscription::handle_subscribe);
        return match result {
            Ok(result) => HandleResult {
                response: Response::ok(id, ResponseData::Subscribe(result.response)),
                subscription_filter: Some(result.filter),
            },
            Err(e) => HandleResult {
                response: Response::error(id, e.to_string()),
                subscription_filter: None,
            },
        };
    }

    let result = match ControlOp::parse(&req.op) {
        Ok(ControlOp::Ping) => Ok(ResponseData::Ping(PingResponse { pong: true })),
        Ok(ControlOp::InventoryLoad) => match parse::<LoadInventoryParams>(req.params) {
            Ok(params) => handle_inventory_load(params)
                .await
                .map(ResponseData::InventoryLoad),
            Err(e) => Err(e),
        },
        Ok(ControlOp::InventoryFilter) => match parse::<FilterParams>(req.params) {
            Ok(params) => handle_inventory_filter(params)
                .await
                .map(ResponseData::InventoryFilter),
            Err(e) => Err(e),
        },
        Ok(ControlOp::InventoryMetaGet) => {
            handle_inventory_meta_get().map(ResponseData::InventoryMeta)
        }
        Ok(ControlOp::InventoryStaleUpdate) => match parse::<StaleParams>(req.params) {
            Ok(params) => handle_inventory_stale_update(params).map(ResponseData::InventoryMeta),
            Err(e) => Err(e),
        },
        Ok(ControlOp::ScreenshotTrigger) => match parse::<ScreenshotParams>(req.params) {
            Ok(params) => handle_screenshot_trigger(params)
                .await
                .map(ResponseData::Screenshot),
            Err(e) => Err(e),
        },
        Ok(ControlOp::InventoryRefresh) => dispatch_refresh(req.params).await,
        Ok(ControlOp::WFMarketPrice) => match parse::<MarketPriceParams>(req.params) {
            Ok(params) => handle_market_price(params)
                .await
                .map(ResponseData::MarketPrice),
            Err(e) => Err(e),
        },
        Ok(ControlOp::WFMarketRefresh) => {
            handle_market_refresh().await.map(ResponseData::MarketRefresh)
        }
        Ok(ControlOp::WfmSignstatus) => match parse::<SignstatusParams>(req.params) {
            Ok(params) => handle_wfm_signstatus(params)
                .await
                .map(ResponseData::Signstatus),
            Err(e) => Err(e),
        },
        Ok(ControlOp::WfmSignin) => match parse_signin_params(req.params) {
            Ok(params) => handle_wfm_signin(params)
                .await
                .map(|()| ResponseData::Empty(EmptyResponse {})),
            Err(e) => Err(e),
        },
        Ok(ControlOp::WfmSignout) => handle_wfm_signout()
            .await
            .map(|()| ResponseData::Empty(EmptyResponse {})),
        Ok(ControlOp::Subscribe) => Err(anyhow::anyhow!("Unexpected subscribe operation")),
        Err(e) => Err(e),
    };

    HandleResult {
        response: match result {
            Ok(data) => Response::ok(id, data),
            Err(e) => Response::error(id, e.to_string()),
        },
        subscription_filter: None,
    }
}

fn parse<T: for<'de> Deserialize<'de> + Default>(params: Option<Value>) -> anyhow::Result<T> {
    parse_params(params)
}

#[cfg(feature = "memory")]
async fn dispatch_refresh(params: Option<Value>) -> anyhow::Result<ResponseData> {
    let params = parse::<super::inventory::RefreshParams>(params)?;
    handle_inventory_refresh(params)
        .await
        .map(ResponseData::InventoryLoad)
}

#[cfg(not(feature = "memory"))]
async fn dispatch_refresh(_params: Option<Value>) -> anyhow::Result<ResponseData> {
    handle_inventory_refresh()
        .await
        .map(ResponseData::InventoryLoad)
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
        assert!(value["error"].as_str().unwrap().starts_with("Invalid request:"));
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
        let result = handle_line(r#"{"id":"3","op":"subscribe","params":{"events":["game_start"]}}"#).await;
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
        assert_eq!(
            serde_json::to_value(EmptyResponse {}).unwrap(),
            json!({})
        );
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
            status: Some("online".into()),
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
            status: "ingame".into(),
        };
        assert_eq!(
            serde_json::to_value(&set).unwrap(),
            json!({ "ok": true, "status": "ingame" })
        );
    }
}
