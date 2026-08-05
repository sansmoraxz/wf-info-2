use serde::{Deserialize, Serialize};

use crate::common::WeaponEntry;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SpaceSuit(pub WeaponEntry);

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_inventory_space_suit() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_space_suit_test.json"
        ));

        let space_suit: SpaceSuit = from_str(json_data).unwrap();

        assert_eq!(
            space_suit.0.item_type,
            "/Lotus/Powersuits/Archwing/SupportJetPack/SupportJetPack"
        );
        assert_eq!(space_suit.0.xp.unwrap(), 4_376_023);
    }
}
