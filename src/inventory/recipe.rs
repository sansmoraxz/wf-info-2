use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::{DateWrapper, ObjectId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemCount")]
    pub item_count: i64,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingRecipe {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "TargetItemId")]
    pub target_item_id: Option<String>,

    #[serde(rename = "CompletionDate")]
    pub completion_date: DateWrapper,

    #[serde(flatten)]
    pub other: Option<Value>,
}
