//! Warframe character suit item data.

use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats, serde_as};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Ability, Drop, Introduced, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::enums::Polarity;
use crate::itemdata::traits::{
    Buildable, Character, Droppable, Equippable, HasAbilities, Item, Prime, WikiaLinked,
};

pub type Root = Vec<Warframe>;

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Warframe {
    // Core identity
    pub unique_name: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub image_name: Option<String>,
    pub description: Option<String>,

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
    pub sprint_speed: Option<f64>,

    // Warframe-specific
    #[serde(default)]
    pub abilities: Vec<Ability>,
    #[serde_as(as = "Option<OneOrMany<_, formats::PreferOne>>")]
    pub aura: Option<Vec<String>>,
    pub passive_description: Option<String>,
    pub sex: Option<String>,
    #[serde(default)]
    pub exalted: Vec<String>,
    pub color: Option<i64>,
    pub conclave: Option<bool>,

    // Buildable
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub skip_build_time_price: Option<i64>,
    pub consume_on_build: Option<bool>,
    pub mastery_req: Option<i64>,
    pub market_cost: Option<i64>,
    pub bp_cost: Option<i64>,
    #[serde(default)]
    pub components: Vec<Component>,

    // Equippable
    #[serde(default)]
    pub polarities: Vec<Polarity>,

    // Prime/vault
    #[serde(default)]
    pub is_prime: bool,
    pub vaulted: Option<bool>,
    pub vault_date: Option<String>,
    pub estimated_vault_date: Option<String>,

    // Wikia
    pub wiki_available: Option<bool>,
    pub wikia_url: Option<String>,
    pub introduced: Option<Introduced>,
    pub release_date: Option<String>,
    pub product_category: Option<String>,

    // Droppable
    #[serde(default)]
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Warframe {
    fn get_product_categories(&self) -> Vec<String> {
        match &self.product_category {
            Some(v) => vec![v.to_string()],
            None => vec![],
        }
    }
}

impl Item for Warframe {
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
        self.image_name.as_deref()
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

impl Droppable for Warframe {
    fn drops(&self) -> &[Drop] {
        &self.drops
    }
}

impl Buildable for Warframe {
    fn build_price(&self) -> Option<i64> {
        self.build_price
    }
    fn build_quantity(&self) -> Option<i64> {
        self.build_quantity
    }
    fn build_time(&self) -> Option<i64> {
        self.build_time
    }
    fn skip_build_time_price(&self) -> Option<i64> {
        self.skip_build_time_price
    }
    fn consume_on_build(&self) -> Option<bool> {
        self.consume_on_build
    }
    fn mastery_req(&self) -> Option<i64> {
        self.mastery_req
    }
    fn market_cost(&self) -> Option<i64> {
        self.market_cost
    }
    fn bp_cost(&self) -> Option<i64> {
        self.bp_cost
    }
    fn components(&self) -> &[Component] {
        &self.components
    }
}

impl Prime for Warframe {
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

impl WikiaLinked for Warframe {
    fn wiki_available(&self) -> Option<bool> {
        self.wiki_available
    }
    fn wikia_url(&self) -> Option<&str> {
        self.wikia_url.as_deref()
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        None
    }
    fn introduced(&self) -> Option<&Introduced> {
        self.introduced.as_ref()
    }
    fn release_date(&self) -> Option<&str> {
        self.release_date.as_deref()
    }
}

impl Character for Warframe {
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
        self.sprint_speed
    }
}

impl HasAbilities for Warframe {
    fn abilities(&self) -> &[Ability] {
        &self.abilities
    }
}

impl Equippable for Warframe {
    fn polarities(&self) -> &[Polarity] {
        &self.polarities
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
    fn test_deserialize_warframe() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/warframe_test.json"
        ));

        let rec: Warframe = from_str(json_data).unwrap();

        assert_eq!(rec.unique_name, "/Lotus/Powersuits/Priest/HarrowPrime");
    }
}
