use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ItemType, ObjectId};

/// Represents a sentinel weapon in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentinelWeapon {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Value>>,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_sentinelweapon() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_sentinel_weapon_test.json"
        ));

        let item: SentinelWeapon = from_str(json_data).unwrap();

        assert_eq!(
            item.item_type,
            "/Lotus/Types/Sentinels/SentinelWeapons/Gremlin"
        );
        assert_eq!(item.xp.unwrap(), 20_545_526);
    }
}
