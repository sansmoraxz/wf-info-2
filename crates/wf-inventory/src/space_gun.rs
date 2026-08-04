use serde::{Deserialize, Serialize};

use crate::common::WeaponEntry;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SpaceGun(pub WeaponEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_space_gun() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_space_gun_test.json"
        ));

        let space_gun: SpaceGun = from_str(json_data).unwrap();

        assert_eq!(
            space_gun.0.item_type,
            "/Lotus/Weapons/Tenno/Archwing/Primary/FoldingMachineGun/ArchMachineGun"
        );
        assert_eq!(space_gun.0.xp.unwrap(), 2_284_379);
    }
}
