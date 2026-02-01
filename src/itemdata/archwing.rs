use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Archwing>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Archwing {
    pub abilities: Vec<Ability>,
    pub armor: i64,
    pub build_price: i64,
    pub build_quantity: i64,
    pub build_time: i64,
    pub category: String,
    pub color: Option<i64>,
    pub components: Vec<Component>,
    pub conclave: Option<bool>,
    pub consume_on_build: bool,
    pub description: String,
    pub health: i64,
    pub image_name: String,
    pub introduced: Option<Introduced>,
    pub is_prime: bool,
    pub masterable: bool,
    pub mastery_req: i64,
    pub name: String,
    pub patchlogs: Vec<Patchlog>,
    pub polarities: Option<Vec<String>>,
    pub power: i64,
    pub product_category: String,
    pub release_date: Option<String>,
    pub shield: i64,
    pub skip_build_time_price: i64,
    pub sprint: Option<f64>,
    pub sprint_speed: f64,
    pub stamina: i64,
    pub tradable: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub wiki_available: Option<bool>,
    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,
    pub estimated_vault_date: Option<String>,
    pub vault_date: Option<String>,
    pub vaulted: Option<bool>,
}

impl ProductCategory for Archwing {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.clone()]
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
pub struct Introduced {
    pub name: String,
    pub url: String,
    pub aliases: Vec<String>,
    pub parent: String,
    pub date: String,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_archwing() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/archwing_test.json"
        ));

        let rec: Archwing = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Powersuits/Archwing/StealthJetPack/StealthJetPack"
        );
    }
}
