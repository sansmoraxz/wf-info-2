use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::ObjectId;

/// Represents a crew member (on-call or assigned) in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrewMember {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "AssignedRole")]
    pub assigned_role: Option<i64>,

    #[serde(rename = "NemesisFingerprint")]
    pub nemesis_fingerprint: Option<i64>,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Value>>,

    #[serde(rename = "PowersuitType")]
    pub powersuit_type: Option<String>,

    #[serde(flatten)]
    pub other: Option<Value>,
}
