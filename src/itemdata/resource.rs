use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;

pub type Root = Vec<Resource>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
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
    pub item_count: Option<i64>,
    #[serde(default)]
    pub parents: Vec<String>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
    pub skip_build_time_price: Option<i64>,
    #[serde(default)]
    pub drops: Vec<Drop2>,
    pub exclude_from_codex: Option<bool>,
}

impl ProductCategory for Resource {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["MiscItems".to_string()]
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
    pub drops: Vec<Drop>,
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
    fn test_deserialize_resource() {
        let json_data = r#"
{
  "category": "Resources",
  "description": "Sinewy and metallic, possessed of great elastic strength.\n\nLocation: Cambion Drift (Deimos) from Yellow Mining Lesions, Spitia Infested Cysts, and Rolizor Infested Cysts.",
  "imageName": "adramalium-03bd190fdb.png",
  "itemCount": 20,
  "masterable": false,
  "name": "Adramalium",
  "parents": [
    "Adramal Alloy",
    "Xaku Kintsu Helmet"
  ],
  "patchlogs": [
    {
      "name": "Heart of Deimos: Hotfix 29.0.2",
      "date": "2020-08-26T19:56:10Z",
      "url": "https://forums.warframe.com/topic/1217239-heart-of-deimos-hotfix-2902/",
      "additions": "",
      "changes": "",
      "fixes": "Fixed Refined Bapholite Blueprint requiring Adramalium instead of raw Bapholite."
    }
  ],
  "tradable": false,
  "type": "Gem",
  "uniqueName": "/Lotus/Types/Items/Gems/Deimos/DeimosCommonOreAItem"
}
"#;

        let rec: Resource = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Items/Gems/Deimos/DeimosCommonOreAItem"
        );
    }
}
