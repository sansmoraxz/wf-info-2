use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::ObjectId;

/// Represents a Drifter melee weapon in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrifterMelee {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Value>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_driftermelee() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_drifter_melee_test.json"
        ));

        let item: DrifterMelee = from_str(json_data).unwrap();

        assert_eq!(
            item.item_type,
            "/Lotus/Types/Friendly/PlayerControllable/Weapons/DuviriDualSwords"
        );
    }
}
