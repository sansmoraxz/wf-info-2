use serde::{Deserialize, Serialize};

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
        let json_data = r#"
{
  "armor": 300,
  "category": "Pets",
  "description": "A vicious and deadly Kavat prized for its combat prowess, offering increased critical chance and punishing those that dare attack it.",
  "health": 310,
  "imageName": "adarza-kavat-9155649686.png",
  "introduced": {
    "name": "Update: Specters of the Rail 0.0",
    "url": "https://wiki.warframe.com/w/Update_19%23Update%3A_Specters_of_the_Rail",
    "aliases": [
      "Specters of the Rail",
      "Specters of the Rail 0.0",
      "SotR"
    ],
    "parent": "18.15",
    "date": "2016-07-08"
  },
  "masterable": true,
  "masteryReq": 0,
  "name": "Adarza Kavat",
  "patchlogs": [
    {
      "name": "Shrine of the Eidolon: Hotfix 22.13.3",
      "date": "2018-02-21T20:18:33Z",
      "url": "https://forums.warframe.com/topic/924885-shrine-of-the-eidolon-hotfix-22133/",
      "additions": "",
      "changes": "Adarza Kavat’s Cat’s Eye Critical Chance Buff no longer apply to Operators.",
      "fixes": ""
    },
    {
      "name": "Update: Specters of the Rail",
      "date": "2016-07-08T14:12:48Z",
      "url": "https://forums.warframe.com/topic/668132-update-specters-of-the-rail/",
      "additions": "",
      "changes": "Adarza Kavat grants increased critical chance to all nearby Tenno for a short duration.",
      "fixes": ""
    }
  ],
  "polarities": [
    "penjaga",
    "penjaga"
  ],
  "power": 100,
  "productCategory": "KubrowPets",
  "releaseDate": "2016-07-08",
  "shield": 270,
  "stamina": 8,
  "tradable": false,
  "type": "Pets",
  "uniqueName": "/Lotus/Types/Game/CatbrowPet/MirrorCatbrowPetPowerSuit",
  "wikiAvailable": true,
  "wikiaThumbnail": "https://wiki.warframe.com/images/AdarzaKavat.png?c023b",
  "wikiaUrl": "https://wiki.warframe.com/w/Adarza_Kavat"
}
"#;

        let rec: Pet = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Game/CatbrowPet/MirrorCatbrowPetPowerSuit"
        );
    }
}
