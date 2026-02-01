use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats, serde_as};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Warframe>;

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Warframe {
    pub abilities: Vec<Ability>,
    pub armor: i64,
    #[serde_as(as = "Option<OneOrMany<_, formats::PreferOne>>")]
    pub aura: Option<Vec<String>>,
    pub bp_cost: Option<i64>,
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub category: String,
    pub color: Option<i64>,
    #[serde(default)]
    pub components: Vec<Component>,
    pub conclave: Option<bool>,
    pub consume_on_build: Option<bool>,
    pub description: Option<String>,
    #[serde(default)]
    pub exalted: Vec<String>,
    pub health: i64,
    pub image_name: Option<String>,
    pub introduced: Option<Introduced>,
    pub is_prime: bool,
    pub market_cost: Option<i64>,
    pub masterable: bool,
    pub mastery_req: Option<i64>,
    pub name: String,
    pub passive_description: Option<String>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    #[serde(default)]
    pub polarities: Vec<String>,
    pub power: i64,
    pub product_category: Option<String>,
    pub release_date: Option<String>,
    pub sex: Option<String>,
    pub shield: i64,
    pub skip_build_time_price: Option<i64>,
    pub sprint: Option<f64>,
    pub sprint_speed: Option<f64>,
    pub stamina: i64,
    pub tradable: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub wiki_available: Option<bool>,
    pub wikia_url: Option<String>,
    pub estimated_vault_date: Option<String>,
    pub vault_date: Option<String>,
    pub vaulted: Option<bool>,
    pub drops: Option<Vec<Drop2>>,
}

impl ProductCategory for Warframe {
    fn get_product_categories(&self) -> Vec<String> {
        match &self.product_category {
            Some(v) => vec![v.to_string()],
            None => vec![],
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub unique_name: String,
    pub name: String,
    pub description: String,
    pub image_name: String,
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
    pub drops: Vec<Drop>,
    pub prime_selling_price: Option<i64>,
    pub ducats: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub exclude_from_codex: Option<bool>,
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
pub struct Introduced {
    pub name: String,
    pub url: String,
    pub aliases: Vec<String>,
    pub parent: String,
    pub date: String,
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
    pub chance: i64,
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
    fn test_deserialize_warframe() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/warframe_test.json"
        ));

        let rec: Warframe = from_str(json_data).unwrap();

        assert_eq!(rec.unique_name, "/Lotus/Powersuits/Priest/HarrowPrime");
    }
}
