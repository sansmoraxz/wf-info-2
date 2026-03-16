use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ObjectId;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_crewmember() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_crew_member_test.json"
        ));

        let item: CrewMember = from_str(json_data).unwrap();

        assert_eq!(
            item.item_type,
            "/Lotus/Types/Game/CrewShip/CrewMember/NewLokaCrewMemberGenerator"
        );
        assert_eq!(item.xp.unwrap(), 0);
    }
}
