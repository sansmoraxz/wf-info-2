use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::control_ops::ControlOp;

use super::inventory::{
    handle_inventory_filter, handle_inventory_load, handle_inventory_meta_get,
    handle_inventory_refresh, handle_inventory_stale_update,
};
use super::screenshot::handle_screenshot_trigger;
use super::subscription::{self, EventFilter};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Option<String>,
    pub op: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Response {
    fn ok(id: Option<String>, data: Value) -> Self {
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
        return match subscription::handle_subscribe(req.params) {
            Ok(result) => HandleResult {
                response: Response::ok(id, result.response),
                subscription_filter: Some(result.filter),
            },
            Err(e) => HandleResult {
                response: Response::error(id, e.to_string()),
                subscription_filter: None,
            },
        };
    }

    let result = match ControlOp::parse(&req.op) {
        Ok(ControlOp::Ping) => Ok(json!({ "pong": true })),
        Ok(ControlOp::InventoryLoad) => handle_inventory_load(req.params).await,
        Ok(ControlOp::InventoryFilter) => handle_inventory_filter(req.params).await,
        Ok(ControlOp::InventoryMetaGet) => handle_inventory_meta_get(),
        Ok(ControlOp::InventoryStaleUpdate) => handle_inventory_stale_update(req.params),
        Ok(ControlOp::ScreenshotTrigger) => handle_screenshot_trigger(req.params).await,
        Ok(ControlOp::InventoryRefresh) => handle_inventory_refresh(req.params).await,
        Ok(ControlOp::Subscribe) => unreachable!(), // Handled above
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
