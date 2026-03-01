//! Arcane enhancement item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, LevelStat, Patchlog};
use crate::itemdata::enums::{ArcaneType, Rarity};
use crate::itemdata::props::BuildableProps;
use crate::itemdata::traits::{Droppable, Item};

pub type Root = Vec<Arcane>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arcane {
    // Core identity
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: ArcaneType,
    pub image_name: String,

    // Tradable
    pub tradable: bool,
    pub masterable: bool,

    // Arcane-specific
    #[serde(default)]
    pub rarity: Option<Rarity>,
    #[serde(default)]
    pub level_stats: Vec<LevelStat>,
    pub exclude_from_codex: Option<bool>,

    // Grouped props
    #[serde(flatten)]
    pub build: BuildableProps,

    // Droppable
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Arcane {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Upgrades".to_string(), "RawUpgrades".to_string()]
    }
}

impl Item for Arcane {
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
        self.type_field.as_str()
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

impl Droppable for Arcane {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_arcane() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/arcane_test.json"
        ));

        let rec: Arcane = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Upgrades/CosmeticEnhancers/Defensive/SpeedOnDamage"
        );
    }
}
