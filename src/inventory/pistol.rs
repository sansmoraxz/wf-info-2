use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inventory::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pistol {
    #[serde(rename = "ItemType")]
    pub item_type: String,

    #[serde(rename = "ItemId")]
    pub item_id: ObjectId,

    #[serde(rename = "XP")]
    pub xp: Option<i64>,

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
        "/Lotus/Upgrades/Skins/HolsterCustomizations/PistolHipsR"
        ],
        "Upgrades": [
        "",
        "",
        "5bc212eea38e4a8b6d15e89b",
        "5bcf33a4a38e4a43f22f7b4a",
        "",
        "5bc24e033f8d4a32bf3d5f9b",
        "",
        "5bc3631257904a5c9b799fd5"
        ]
    },
    {
        "Skins": [
        "",
        "",
        "/Lotus/Upgrades/Skins/HolsterCustomizations/PistolHipsR"
        ]
    },
    {
        "Skins": [
        "",
        "",
        "/Lotus/Upgrades/Skins/HolsterCustomizations/PistolHipsR"
        ]
    }
    ],
    "ItemId": {
    "$oid": "5be899d757904a4ff96b9c30"
    },
    "ItemType": "/Lotus/Weapons/Corpus/Pistols/CorpusMinigun/CorpusMinigun",
    "UpgradeVer": 101,
    "XP": 3744243
}
"#;

        let pistol: Pistol = from_str(json_data).unwrap();

        assert_eq!(
            pistol.item_type,
            "/Lotus/Weapons/Corpus/Pistols/CorpusMinigun/CorpusMinigun"
        );

        assert_eq!(pistol.xp.unwrap(), 3744243);
    }
}
