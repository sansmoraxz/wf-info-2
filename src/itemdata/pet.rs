use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Pet>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pet {
    pub armor: Option<i64>,
    pub category: String,
    pub description: String,
    pub health: Option<i64>,
    pub image_name: String,
    pub introduced: Option<Introduced>,
    pub masterable: bool,
    pub mastery_req: i64,
    pub name: String,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    #[serde(default)]
    pub polarities: Vec<String>,
    pub power: Option<i64>,
    pub product_category: String,
    pub release_date: Option<String>,
    pub shield: Option<i64>,
    pub stamina: Option<i64>,
    pub tradable: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub unique_name: String,
    pub wiki_available: Option<bool>,
    pub wikia_thumbnail: Option<String>,
    pub wikia_url: Option<String>,
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub components: Option<Vec<Component>>,
    pub consume_on_build: Option<bool>,
    pub critical_chance: Option<i64>,
    pub critical_multiplier: Option<i64>,
    #[serde(default)]
    pub damage_per_shot: Vec<i64>,
    pub fire_rate: Option<i64>,
    pub omega_attenuation: Option<i64>,
    pub proc_chance: Option<i64>,
    pub skip_build_time_price: Option<i64>,
    pub total_damage: Option<i64>,
    pub exclude_from_codex: Option<bool>,
    pub drops: Option<Vec<Drop2>>,
}

impl ProductCategory for Pet {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.clone()]
    }
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
pub struct Component {
    pub unique_name: String,
    pub name: String,
    pub description: String,
    pub item_count: i64,
    pub image_name: String,
    pub tradable: bool,
    pub drops: Vec<Drop>,
    pub masterable: bool,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drop {
    pub chance: i64,
    pub location: String,
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: String,
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
    fn test_deserialize_pet() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/pet_test.json"
        ));

        let rec: Pet = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Game/CatbrowPet/MirrorCatbrowPetPowerSuit"
        );
    }
}
