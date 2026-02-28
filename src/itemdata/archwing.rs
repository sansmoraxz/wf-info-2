//! Archwing item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Ability, Introduced, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::enums::Polarity;
use crate::itemdata::traits::{
    Buildable, Character, Equippable, HasAbilities, Item, Prime, WikiaLinked,
};

pub type Root = Vec<Archwing>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Archwing {
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

    // Character stats
    pub health: i64,
    pub shield: i64,
    pub armor: i64,
    pub power: i64,
    pub stamina: i64,
    pub sprint: Option<f64>,
    pub sprint_speed: f64,

    // Archwing-specific
    #[serde(default)]
    pub abilities: Vec<Ability>,
    pub color: Option<i64>,
    pub conclave: Option<bool>,

    // Buildable
    pub build_price: i64,
    pub build_quantity: i64,
    pub build_time: i64,
    pub skip_build_time_price: i64,
    pub consume_on_build: bool,
    pub mastery_req: i64,
    #[serde(default)]
    pub components: Vec<Component>,

    // Equippable
    pub polarities: Option<Vec<Polarity>>,

    // Prime/vault
    #[serde(default)]
    pub is_prime: bool,
    pub vaulted: Option<bool>,
    pub vault_date: Option<String>,
    pub estimated_vault_date: Option<String>,

    // Wikia
    pub wiki_available: Option<bool>,
    pub wikia_url: Option<String>,
    pub wikia_thumbnail: Option<String>,
    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,
    pub product_category: String,

    // Droppable
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Archwing {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.clone()]
    }
}

impl Item for Archwing {
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

impl Buildable for Archwing {
    fn build_price(&self) -> Option<i64> {
        Some(self.build_price)
    }
    fn build_quantity(&self) -> Option<i64> {
        Some(self.build_quantity)
    }
    fn build_time(&self) -> Option<i64> {
        Some(self.build_time)
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        Some(self.skip_build_time_price)
    }
    fn consume_on_build(&self) -> Option<bool> {
        Some(self.consume_on_build)
    }
    fn mastery_req(&self) -> Option<i64> {
        Some(self.mastery_req)
    }
    fn market_cost(&self) -> Option<i64> {
        None
    }
    fn bp_cost(&self) -> Option<i64> {
        None
    }
    fn components(&self) -> &[Component] {
        &self.components
    }
}

impl Prime for Archwing {
    fn is_prime(&self) -> bool {
        self.is_prime
    }
    fn vaulted(&self) -> Option<bool> {
        self.vaulted
    }
    fn vault_date(&self) -> Option<&str> {
        self.vault_date.as_deref()
    }
    fn estimated_vault_date(&self) -> Option<&str> {
        self.estimated_vault_date.as_deref()
    }
}

impl WikiaLinked for Archwing {
    fn wiki_available(&self) -> Option<bool> {
        self.wiki_available
    }
    fn wikia_url(&self) -> Option<&str> {
        self.wikia_url.as_deref()
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        self.wikia_thumbnail.as_deref()
    }
    fn introduced(&self) -> Option<&Introduced> {
        self.introduced.as_ref()
    }
    fn release_date(&self) -> Option<&str> {
        self.release_date.as_deref()
    }
}

impl Character for Archwing {
    fn health(&self) -> i64 {
        self.health
    }
    fn shield(&self) -> i64 {
        self.shield
    }
    fn armor(&self) -> i64 {
        self.armor
    }
    fn power(&self) -> i64 {
        self.power
    }
    fn stamina(&self) -> i64 {
        self.stamina
    }
    fn sprint_speed(&self) -> Option<f64> {
        Some(self.sprint_speed)
    }
}

impl HasAbilities for Archwing {
    fn abilities(&self) -> &[Ability] {
        &self.abilities
    }
}

impl Equippable for Archwing {
    fn polarities(&self) -> &[Polarity] {
        match &self.polarities {
            Some(p) => p,
            None => &[],
        }
    }
    fn slot(&self) -> Option<i64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_archwing() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/archwing_test.json"
        ));

        let rec: Archwing = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Powersuits/Archwing/StealthJetPack/StealthJetPack"
        );
    }
}
