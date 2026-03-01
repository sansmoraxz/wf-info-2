//! Skin/cosmetic item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Drop, Patchlog};
use crate::itemdata::props::BuildableProps;
use crate::itemdata::traits::{Buildable, Droppable, Item};

pub type Root = Vec<Skin>;

/// A single hex colour entry in a Color Palette.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HexColour {
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image_name: String,
    pub description: Option<String>,
    pub tradable: bool,
    pub masterable: bool,
    pub exclude_from_codex: Option<bool>,
    pub show_in_inventory: Option<bool>,

    // Grouped props
    #[serde(flatten)]
    pub build: BuildableProps,

    // Color Palette specific
    #[serde(default)]
    pub hex_colours: Vec<HexColour>,

    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Skin {
    fn get_product_categories(&self) -> Vec<String> {
        vec!["Skins".to_string()]
    }
}

impl Item for Skin {
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

impl Droppable for Skin {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Skin {
    fn build_price(&self) -> Option<i64> {
        self.build.build_price
    }
    fn build_quantity(&self) -> Option<i64> {
        self.build.build_quantity
    }
    fn build_time(&self) -> Option<i64> {
        self.build.build_time
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        self.build.skip_build_time_price
    }
    fn consume_on_build(&self) -> Option<bool> {
        self.build.consume_on_build
    }
    fn mastery_req(&self) -> Option<i64> {
        self.build.mastery_req
    }
    fn market_cost(&self) -> Option<i64> {
        self.build.market_cost
    }
    fn bp_cost(&self) -> Option<i64> {
        self.build.bp_cost
    }
    fn components(&self) -> &[crate::itemdata::components::Component] {
        &self.build.components
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_skin() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/skin_test.json"
        ));

        let rec: Skin = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/StoreItems/SuitCustomizations/ColourPickerAccessibilityItemA"
        );
        assert!(!rec.hex_colours.is_empty());
    }
}
