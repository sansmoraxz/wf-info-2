//! Enemy item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Patchlog};
use crate::itemdata::traits::{Droppable, Item};

pub type Root = Vec<Enemy>;

/// Damage type affector within a resistance entry.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Affector {
    pub element: String,
    pub modifier: f64,
}

/// Resistance entry (e.g., Shield, Alloy Armor, Robotic).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resistance {
    pub amount: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub affectors: Vec<Affector>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enemy {
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image_name: String,
    pub description: String,
    pub tradable: bool,

    // Combat stats
    pub health: f64,
    pub shield: f64,
    pub armor: f64,
    pub region_bits: i64,
    #[serde(default)]
    pub resistances: Vec<Resistance>,

    // Optional
    pub faction: Option<String>,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Enemy {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Enemy".to_string()]
    }
}

impl Item for Enemy {
    fn unique_name(&self) -> &str {
        &self.unique_name
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn category(&self) -> &str {
        &self.category
    }
    fn type_field(&self) -> &str {
        &self.type_field
    }
    fn image_name(&self) -> Option<&str> {
        Some(&self.image_name)
    }
    fn tradable(&self) -> bool {
        self.tradable
    }
    fn masterable(&self) -> bool {
        false
    }
    fn patchlogs(&self) -> &[Patchlog] {
        &self.patchlogs
    }
}

impl Droppable for Enemy {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_enemy() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/enemy_test.json"
        ));

        let rec: Enemy = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Enemies/CorpusChampions/TeamD/CCTeamDOspreyAvatar"
        );
        assert_eq!(rec.resistances.len(), 3);
        // Verify null chance drop is handled
        assert!(rec.drops.iter().any(|d| d.chance.is_none()));
    }
}
