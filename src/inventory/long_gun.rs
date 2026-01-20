use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::{ObjectId, Polarity};

#[derive(Debug, Serialize, Deserialize)]
pub struct LongGun {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(rename = "FocusLens")]
    pub focus_lens: Option<String>,

    #[serde(rename = "Polarity")]
    pub polarity: Option<Vec<Polarity>>,

    #[serde(rename = "Polarized")]
    pub polarized: Option<i64>,

    #[serde(rename = "ModSlotPurchases")]
    pub mod_slot_purchases: Option<i64>,

    #[serde(rename = "IsNew")]
    pub is_new: Option<bool>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_long_gun() {
        let json_data = r#"
{
  "Configs": [
    {
      "Skins": [
        "",
        "",
        "/Lotus/Upgrades/Skins/HolsterCustomizations/RifleUpperBack"
      ],
      "Upgrades": [
        "",
        "",
        "",
        "",
        "",
        "",
        "5bf2b2c1a38e4a0d6a3aaf32",
        "5bfbd4c93f8d4a31036fc6fd"
      ]
    },
    {
      "Skins": [
        "",
        "",
        "/Lotus/Upgrades/Skins/HolsterCustomizations/RifleUpperBack"
      ]
    },
    {
      "Skins": [
        "",
        "",
        "/Lotus/Upgrades/Skins/HolsterCustomizations/RifleUpperBack"
      ]
    }
  ],
  "ItemId": {
    "$oid": "5be8327857904a1a1471982f"
  },
  "ItemType": "/Lotus/Weapons/Grineer/LongGuns/GrineerSniperRifle/GrnSniperRifle",
  "UpgradeVer": 101,
  "XP": 524343
}
  "#;

        let long_gun: LongGun = from_str(json_data).unwrap();

        assert_eq!(
            long_gun.item_type,
            "/Lotus/Weapons/Grineer/LongGuns/GrineerSniperRifle/GrnSniperRifle"
        );
        assert_eq!(long_gun.xp.unwrap(), 524343);
    }
}
