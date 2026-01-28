use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::control_ops::ControlOp;

use super::inventory::{
    handle_inventory_filter, handle_inventory_load, handle_inventory_meta_get,
    handle_inventory_refresh, handle_inventory_stale_update,
};
use super::screenshot::handle_screenshot_trigger;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Option<Value>,
    pub op: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Option<Value>,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Response {
    fn ok(id: Option<Value>, data: Value) -> Self {
        Self {
            id,
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(id: Option<Value>, message: String) -> Self {
        Self {
            id,
            ok: false,
            data: None,
            error: Some(message),
        }
    }
}

pub(crate) async fn handle_line(line: &str) -> Response {
    match serde_json::from_str::<Request>(line) {
        Ok(req) => handle_request(req).await,
        Err(e) => Response::error(None, format!("Invalid request: {}", e)),
    }
}

async fn handle_request(req: Request) -> Response {
    let id = req.id.clone();

    let result = match ControlOp::parse(&req.op) {
        Ok(ControlOp::Ping) => Ok(json!({ "pong": true })),
        Ok(ControlOp::InventoryLoad) => handle_inventory_load(req.params).await,
        Ok(ControlOp::InventoryFilter) => handle_inventory_filter(req.params).await,
        Ok(ControlOp::InventoryMetaGet) => handle_inventory_meta_get(),
        Ok(ControlOp::InventoryStaleUpdate) => handle_inventory_stale_update(req.params),
        Ok(ControlOp::ScreenshotTrigger) => handle_screenshot_trigger(req.params),
        Ok(ControlOp::InventoryRefresh) => handle_inventory_refresh(req.params).await,
        Err(e) => Err(e),
    };

    match result {
        Ok(data) => Response::ok(id, data),
        Err(e) => Response::error(id, e.to_string()),
    }
}
