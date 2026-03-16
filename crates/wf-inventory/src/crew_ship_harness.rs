use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ObjectId, Polarity};

/// Represents a Railjack reactor/harness in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrewShipHarness {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(rename = "Features")]
    pub features: Option<i64>,

    #[serde(rename = "Polarity")]
    pub polarity: Option<Vec<Polarity>>,

    #[serde(rename = "Polarized")]
    pub polarized: Option<i64>,

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
    fn test_deserialize_crewshipharness() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_crew_ship_harness_test.json"
        ));

        let item: CrewShipHarness = from_str(json_data).unwrap();

        assert_eq!(
            item.item_type,
            "/Lotus/Types/Game/CrewShip/RailJack/DefaultHarness"
        );
        assert_eq!(item.xp.unwrap(), 21375974);
    }
}
