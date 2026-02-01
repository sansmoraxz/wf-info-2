use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Gear>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gear {
    pub category: String,
    pub description: String,
    pub image_name: String,
    pub masterable: bool,
    pub name: String,
    pub tradable: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    #[serde(default)]
    pub components: Vec<Component>,
    pub consume_on_build: Option<bool>,
    pub skip_build_time_price: Option<i64>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    #[serde(default)]
    pub drops: Vec<Drop2>,
    pub item_count: Option<i64>,
    pub parents: Option<Vec<String>>,
}

impl ProductCategory for Gear {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Consumables".to_string()]
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub unique_name: String,
    pub name: String,
    pub description: String,
    pub item_count: i64,
    pub image_name: String,
    pub tradable: bool,
    pub masterable: bool,
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drop {
    pub chance: f64,
    pub location: String,
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub struct Drop2 {
    pub chance: f64,
    pub location: String,
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_gear() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/gear_test.json"
        ));

        let rec: Gear = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Restoratives/Consumable/MiningLaserC"
        );
    }
}
