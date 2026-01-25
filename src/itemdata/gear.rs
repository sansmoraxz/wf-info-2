use serde::{Deserialize, Serialize};

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
        let json_data = r#"
{
  "buildPrice": 3500,
  "buildQuantity": 1,
  "buildTime": 3600,
  "category": "Gear",
  "components": [
    {
      "uniqueName": "/Lotus/Types/Recipes/EidolonRecipes/Prospecting/MiningLaserCBlueprint",
      "name": "Blueprint",
      "description": "Enhanced with cybernetics, this tool is able to locate nearby gems and ore veins and has chance to retrieve Rare Gems on any Landscape.\n\nTravel far from Hub settlements for the best chance of mining rare minerals.",
      "itemCount": 1,
      "imageName": "blueprint.png",
      "tradable": false,
      "masterable": false,
      "drops": []
    },
    {
      "uniqueName": "/Lotus/Types/Items/MiscItems/Circuits",
      "name": "Circuits",
      "description": "Various electronic components.\n\nLocation: Venus, Ceres, and the Kuva Fortress",
      "itemCount": 500,
      "imageName": "circuits-5e77d8b169.png",
      "tradable": false,
      "drops": [],
      "masterable": false,
      "type": "Resource"
    },
    {
      "uniqueName": "/Lotus/Types/Items/Gems/Eidolon/UncommonOreAAlloyAItem",
      "name": "Fersteel Alloy",
      "description": "Ferros that has been forged into a stronger metal.\n\nBlueprint sold by Old Man Suumbaat in Cetus on Earth.",
      "itemCount": 40,
      "imageName": "fersteel-alloy-c11c0e2d3c.png",
      "tradable": false,
      "drops": [],
      "masterable": false
    },
    {
      "uniqueName": "/Lotus/Types/Items/Gems/Eidolon/UncommonGemACutAItem",
      "name": "Marquise Veridos",
      "description": "Polished and cut to perfection.\n\nBlueprint sold by Old Man Suumbaat in Cetus on Earth.",
      "itemCount": 2,
      "imageName": "marquise-veridos-217877dacf.png",
      "tradable": false,
      "drops": [],
      "masterable": false
    }
  ],
  "consumeOnBuild": true,
  "description": "Enhanced with cybernetics, this tool is able to locate nearby gems and ore veins and has chance to retrieve Rare Gems on any Landscape.\n\nTravel far from Hub settlements for the best chance of mining rare minerals.",
  "imageName": "advanced-nosam-cutter-e654a1fc03.png",
  "masterable": false,
  "name": "Advanced Nosam Cutter",
  "skipBuildTimePrice": 10,
  "tradable": false,
  "type": "Gear",
  "uniqueName": "/Lotus/Types/Restoratives/Consumable/MiningLaserC"
}
"#;

        let rec: Gear = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Restoratives/Consumable/MiningLaserC"
        );
    }
}
