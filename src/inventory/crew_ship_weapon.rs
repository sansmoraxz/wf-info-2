use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::ObjectId;

/// Represents a Railjack weapon in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrewShipWeapon {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "UpgradeFingerprint")]
    pub upgrade_fingerprint: Option<String>,

    #[serde(rename = "UpgradeType")]
    pub upgrade_type: Option<String>,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(flatten)]
    pub other: Option<Value>,
}
