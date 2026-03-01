use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::ObjectId;

/// Represents a landing craft (ship) in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ship {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "AirSupportPower")]
    pub air_support_power: Option<String>,

    #[serde(rename = "ShipExterior")]
    pub ship_exterior: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}
