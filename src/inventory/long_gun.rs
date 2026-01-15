use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LongGun {
    #[serde(rename = "Configs")]
    pub configs: Option<Vec<Config>>,

    #[serde(rename = "ItemId")]
    pub item_id: Option<ObjectId>,

    #[serde(rename = "ItemType")]
    pub item_type: Option<String>,

    #[serde(rename = "UpgradeVer")]
    pub upgrade_ver: Option<i64>,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "Skins")]
    pub skins: Option<Vec<String>>,

    #[serde(rename = "Upgrades")]
    pub upgrades: Option<Vec<String>>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectId {
    #[serde(rename = "$oid")]
    pub oid: String,
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
            long_gun.item_type.as_ref().unwrap(),
            "/Lotus/Weapons/Grineer/LongGuns/GrineerSniperRifle/GrnSniperRifle"
        );
        assert_eq!(long_gun.upgrade_ver.unwrap(), 101);
        assert_eq!(long_gun.xp.unwrap(), 524343);
        assert_eq!(long_gun.configs.as_ref().unwrap().len(), 3);
    }
}
