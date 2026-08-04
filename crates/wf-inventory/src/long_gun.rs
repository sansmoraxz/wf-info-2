use serde::{Deserialize, Serialize};

use crate::common::WeaponEntry;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LongGun(pub WeaponEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_long_gun() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_long_gun_test.json"
        ));

        let long_gun: LongGun = from_str(json_data).unwrap();

        assert_eq!(
            long_gun.0.item_type,
            "/Lotus/Weapons/Grineer/LongGuns/GrineerSniperRifle/GrnSniperRifle"
        );
        assert_eq!(long_gun.0.xp.unwrap(), 524_343);
    }
}
