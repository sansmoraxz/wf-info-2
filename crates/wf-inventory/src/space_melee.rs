use serde::{Deserialize, Serialize};

use crate::common::WeaponEntry;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SpaceMelee(pub WeaponEntry);

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
            space_melee.0.item_type,
            "/Lotus/Weapons/Tenno/Archwing/Melee/GrnArchHand/GrnArchHandWeapon"
        );
        assert_eq!(space_melee.0.xp.unwrap(), 561_680);
    }
}
