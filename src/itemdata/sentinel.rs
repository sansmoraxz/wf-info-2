//! Sentinel companion item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Introduced, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::enums::{Polarity, SentinelProductCategory, Slot};
use crate::itemdata::traits::{Buildable, Character, Equippable, Item, Prime, WikiaLinked};

pub type Root = Vec<Sentinel>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sentinel {
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

    // Buildable
    pub build_price: Option<i64>,
    pub build_quantity: Option<i64>,
    pub build_time: Option<i64>,
    pub skip_build_time_price: Option<i64>,
    pub consume_on_build: Option<bool>,
    pub mastery_req: Option<i64>,
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
    pub wikia_thumbnail: Option<String>,
    pub introduced: Option<Introduced>,
    pub release_date: String,
    pub product_category: SentinelProductCategory,

    // Droppable
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,
}

impl ProductCategory for Sentinel {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.as_str().to_string()]
    }
}

impl Item for Sentinel {
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

impl Buildable for Sentinel {
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
        None
    }
    fn bp_cost(&self) -> Option<i64> {
        None
    }
    fn components(&self) -> &[Component] {
        &self.components
    }
}

impl Prime for Sentinel {
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

impl WikiaLinked for Sentinel {
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
        Some(&self.release_date)
    }
}

impl Character for Sentinel {
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
        None
    }
}

impl Equippable for Sentinel {
    fn polarities(&self) -> &[Polarity] {
        &self.polarities
    }
    fn slot(&self) -> Option<&Slot> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_deserialize_pet() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/sentinel_test.json"
        ));

        let rec: Sentinel = from_str(json_data).unwrap();

        assert_eq!(
            rec.unique_name,
            "/Lotus/Types/Sentinels/SentinelPowersuits/CarrierPowerSuit"
        );
    }
}
