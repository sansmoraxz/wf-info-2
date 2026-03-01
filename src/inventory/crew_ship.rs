use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::ObjectId;

/// Represents a Railjack (crew ship) in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrewShip {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Value>>,

    #[serde(rename = "CrewMembers")]
    pub crew_members: Option<Value>,

    #[serde(rename = "Customization")]
    pub customization: Option<Value>,

    #[serde(rename = "Weapon")]
    pub weapon: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}
