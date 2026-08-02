use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ItemType, ObjectId};

/// Represents a MOA companion in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoaPet {
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

    #[serde(rename = "ModularParts")]
    pub modular_parts: Option<Vec<String>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_moapet() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_moa_pet_test.json"
        ));

        let item: MoaPet = from_str(json_data).unwrap();

        assert_eq!(
            item.item_type,
            "/Lotus/Types/Friendly/Pets/MoaPets/MoaPetPowerSuit"
        );
        assert_eq!(item.xp.unwrap(), 904_219);
    }
}
