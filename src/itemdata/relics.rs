//! Void Relic item data.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::itemdata::common::{Drop, Patchlog};
use crate::itemdata::traits::{Droppable, Item};
use crate::itemdata::ProductCategory;

pub type Root = Vec<Relic>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    // Core identity
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image_name: String,
    pub description: String,

    // Tradable
    pub tradable: bool,
    pub masterable: bool,

    // Relic-specific
    #[serde(default)]
    pub locations: Vec<Value>, // observed to be empty array
    #[serde(default)]
    pub rewards: Vec<Value>, // observed to be empty array
    pub exclude_from_codex: Option<bool>,

    // Droppable
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Relic {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["MiscItems".to_string()]
    }
}

impl Item for Relic {
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
        self.masterable
    }
    fn patchlogs(&self) -> &[Patchlog] {
        &self.patchlogs
    }
}

impl Droppable for Relic {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_relic() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/relics_test.json"
        ));

        let rec: Relic = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Game/Projections/T4VoidProjectionLavosPrimeASilver"
        );
    }
}
