use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ItemType, ObjectId};

/// Represents a Railjack (crew ship) in the inventory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrewShip {
    #[serde(rename = "ItemType")]
    pub item_type: ItemType,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Value>>,

    #[serde(rename = "CrewMembers")]
    pub crew_members: Option<Value>,

    #[serde(rename = "Customization")]
    pub customization: Option<Value>,

    #[serde(rename = "Weapon")]
    pub weapon: Option<Value>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_crewship() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_crew_ship_test.json"
        ));

        let item: CrewShip = from_str(json_data).unwrap();

        assert_eq!(item.item_type, "/Lotus/Types/Game/CrewShip/Ships/RailJack");
    }
}
