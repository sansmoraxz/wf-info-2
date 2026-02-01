use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Arcane>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arcane {
    pub category: String,
    #[serde(default)]
    pub drops: Vec<Drop>,
    pub image_name: String,
    #[serde(default)]
    pub level_stats: Vec<LevelStat>,
    pub masterable: bool,
    pub name: String,
    pub rarity: Option<String>,
    pub tradable: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub exclude_from_codex: Option<bool>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub components: Option<Vec<Component>>,
    pub consume_on_build: Option<bool>,
    pub skip_build_time_price: Option<i64>,
}

impl ProductCategory for Arcane {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Upgrades".to_string(), "RawUpgrades".to_string()]
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop {
    pub chance: f64,
    pub location: String,
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelStat {
    pub stats: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Patchlog {
    pub name: String,
    pub date: String,
    pub url: String,
    pub additions: String,
    pub changes: String,
    pub fixes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub unique_name: String,
    pub name: String,
    pub item_count: i64,
    pub image_name: String,
    pub tradable: bool,
    pub masterable: bool,
    pub drops: Vec<Drop>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_arcane() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/arcane_test.json"
        ));

        let rec: Arcane = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Upgrades/CosmeticEnhancers/Defensive/SpeedOnDamage"
        );
    }
}
