use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ObjectId;

/// Represents a resource drone in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drone {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "CurrentHP")]
    pub current_hp: Option<i64>,

    #[serde(rename = "RepairStart")]
    pub repair_start: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_drone() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_drone_test.json"
        ));

        let item: Drone = from_str(json_data).unwrap();

        assert_eq!(item.item_type, "/Lotus/Types/Ship/BasicResourceDrone");
    }
}
