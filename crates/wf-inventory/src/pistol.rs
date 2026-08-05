use serde::{Deserialize, Serialize};

use crate::common::WeaponEntry;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Pistol(pub WeaponEntry);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_pistol() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/inventory/inventory_pistol_test.json"
        ));

        let pistol: Pistol = from_str(json_data).unwrap();

        assert_eq!(
            pistol.0.item_type,
            "/Lotus/Weapons/Corpus/Pistols/CorpusMinigun/CorpusMinigun"
        );
        assert_eq!(pistol.0.xp.unwrap(), 3_744_243);
    }
}
