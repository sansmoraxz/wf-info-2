use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ItemType, ObjectId};

/// Represents a Railjack weapon in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrewShipWeapon {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "UpgradeFingerprint")]
    pub upgrade_fingerprint: Option<String>,

    #[serde(rename = "UpgradeType")]
    pub upgrade_type: Option<String>,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(flatten)]
    pub other: Option<serde_json::Map<String, Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_crewshipweapon() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_crew_ship_weapon_test.json"
        ));

        let item: CrewShipWeapon = from_str(json_data).unwrap();

        assert_eq!(
            item.item_type,
            "/Lotus/Weapons/CrewShip/ElectricTether/ZektiElectricTetherCannonTierC"
        );
    }
}
