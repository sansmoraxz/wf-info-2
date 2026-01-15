use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represent unupgraded mods
#[derive(Debug, Serialize, Deserialize)]
pub struct RawUpgrade {
    #[serde(rename = "ItemCount")]
    pub item_count: Option<i64>,

    #[serde(rename = "ItemType")]
    pub item_type: Option<String>,

    #[serde(rename = "LastAdded")]
    pub last_added: Option<ObjectId>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

/// Represent upgraded mods
#[derive(Debug, Serialize, Deserialize)]
pub struct Upgrade {
    #[serde(rename = "ItemId")]
    pub item_id: Option<ObjectId>,

    #[serde(rename = "ItemType")]
    pub item_type: Option<String>,

    #[serde(rename = "UpgradeFingerprint")]
    pub upgrade_fingerprint: Option<UpgradeFingerprint>,

    #[serde(flatten)]
    pub other: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeFingerprint {
    pub lvl: i64,
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
    fn test_deserialize_raw_upgrade() {
        let js = r#"{ "ItemCount": 32, "ItemType": "/Lotus/Upgrades/Mods/Warframe/AvatarShieldMaxMod", "LastAdded": {"$oid":"695a535a4e4a24181e0bf2b8"} }"#;
        let u: RawUpgrade = from_str(js).unwrap();
        assert_eq!(u.item_count.unwrap(), 32);
    }

    fn test_deserialize_upgrade() {
        let js = r#"
{
    "ItemId": {
    "$oid": "5bc1067d57904a037245c7b0"
    },
    "ItemType": "/Lotus/Upgrades/Mods/Warframe/AvatarShieldMaxMod",
    "UpgradeFingerprint": "{\"lvl\":10}"
}
"#;
        let u: Upgrade = from_str(js).unwrap();
        assert_eq!(
            u.item_type.unwrap(),
            "/Lotus/Upgrades/Mods/Warframe/AvatarShieldMaxMod"
        );

        assert_eq!(u.upgrade_fingerprint.unwrap().lvl, 10);
    }
}
