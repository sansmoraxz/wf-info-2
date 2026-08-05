use serde::{Deserialize, Serialize};

use crate::common::WeaponEntry;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Melee(pub WeaponEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_melee() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_melee_test.json"
        ));

        let melee: Melee = from_str(json_data).unwrap();

        assert_eq!(
            melee.0.item_type,
            "/Lotus/Weapons/Tenno/Melee/Swords/HeatSword/HeatLongSword"
        );
        assert_eq!(melee.0.xp.unwrap(), 4_940_862);
    }
}
