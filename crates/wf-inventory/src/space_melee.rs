use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ItemType, ObjectId, Polarity};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpaceMelee {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "ItemCount", skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(rename = "FocusLens")]
    pub focus_lens: Option<String>,

    #[serde(rename = "Polarity")]
    pub polarity: Option<Vec<Polarity>>,

    #[serde(rename = "Polarized")]
    pub polarized: Option<i64>,

    #[serde(rename = "ModSlotPurchases")]
    pub mod_slot_purchases: Option<i64>,

    #[serde(rename = "IsNew")]
    pub is_new: Option<bool>,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_space_melee() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_space_melee_test.json"
        ));

        let space_melee: SpaceMelee = from_str(json_data).unwrap();

        assert_eq!(
            space_melee.item_type,
            "/Lotus/Weapons/Tenno/Archwing/Melee/GrnArchHand/GrnArchHandWeapon"
        );

        assert_eq!(space_melee.xp.unwrap(), 561_680);
    }
}
