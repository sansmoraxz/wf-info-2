//! Archwing item data.

use serde::{Deserialize, Serialize};

use crate::itemdata::ProductCategory;
use crate::itemdata::common::{Ability, Patchlog};
use crate::itemdata::components::Component;
use crate::itemdata::enums::{ArchwingProductCategory, ArchwingType, Polarity, Slot};
use crate::itemdata::props::{
    CharacterStats, ItemDetailProps, ItemIdentityProps, PrimeProps, TradableProps, WikiaProps,
};
use crate::itemdata::traits::{
    Buildable, Character, Equippable, HasAbilities, Item, Prime, WikiaLinked,
};

pub type Root = Vec<Archwing>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Archwing {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: ArchwingType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

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

    pub product_category: ArchwingProductCategory,

    // Droppable
    #[serde(default)]
    pub patchlogs: Vec<Patchlog>,

    // Grouped props
    #[serde(flatten)]
    pub stats: CharacterStats,
    #[serde(flatten)]
    pub prime: PrimeProps,
    #[serde(flatten)]
    pub wikia: WikiaProps,
}

impl ProductCategory for Archwing {
    fn get_product_categories(&self) -> Vec<String> {
        vec![self.product_category.as_str().to_string()]
    }
}

impl Item for Archwing {
    fn unique_name(&self) -> &str {
        &self.identity.unique_name
    }
    fn name(&self) -> &str {
        &self.identity.name
    }
    fn category(&self) -> &str {
        &self.identity.category
    }
    fn type_field(&self) -> &str {
        self.type_field.as_str()
    }
    fn image_name(&self) -> Option<&str> {
        self.detail.image_name.as_deref()
    }
    fn tradable(&self) -> bool {
        self.trade.tradable
    }
    fn masterable(&self) -> bool {
        self.trade.masterable
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
        self.prime.is_prime
    }
    fn vaulted(&self) -> Option<bool> {
        self.prime.vaulted
    }
    fn vault_date(&self) -> Option<&str> {
        self.prime.vault_date.as_deref()
    }
    fn estimated_vault_date(&self) -> Option<&str> {
        self.prime.estimated_vault_date.as_deref()
    }
}

impl WikiaLinked for Archwing {
    fn wiki_available(&self) -> Option<bool> {
        self.wikia.wiki_available
    }
    fn wikia_url(&self) -> Option<&str> {
        self.wikia.wikia_url.as_deref()
    }
    fn wikia_thumbnail(&self) -> Option<&str> {
        self.wikia.wikia_thumbnail.as_deref()
    }
    fn introduced(&self) -> Option<&crate::itemdata::common::Introduced> {
        self.wikia.introduced.as_ref()
    }
    fn release_date(&self) -> Option<&str> {
        self.wikia.release_date.as_deref()
    }
}

impl Character for Archwing {
    fn health(&self) -> i64 {
        self.stats.health
    }
    fn shield(&self) -> i64 {
        self.stats.shield
    }
    fn armor(&self) -> i64 {
        self.stats.armor
    }
    fn power(&self) -> i64 {
        self.stats.power
    }
    fn stamina(&self) -> i64 {
        self.stats.stamina
    }
    fn sprint_speed(&self) -> Option<f64> {
        self.stats.sprint_speed
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
    fn slot(&self) -> Option<&Slot> {
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
            rec.identity.unique_name,
            "/Lotus/Powersuits/Archwing/StealthJetPack/StealthJetPack"
        );
        assert_eq!(rec.identity.name, "Itzal");
        assert_eq!(rec.identity.category, "Archwing");
        assert!(!rec.trade.tradable);
        assert!(rec.trade.masterable);

        // Character stats
        assert_eq!(rec.stats.health, 200);
        assert_eq!(rec.stats.shield, 220);
        assert_eq!(rec.stats.armor, 50);
        assert_eq!(rec.stats.power, 220);

        // Abilities
        assert_eq!(rec.abilities.len(), 4);

        // Buildable
        assert_eq!(rec.build_price, 25000);
        assert_eq!(rec.components.len(), 5);

        assert!(!rec.prime.is_prime);
    }

    #[test]
    fn test_deserialize_archwing_odonata() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/archwing_test_2.json"
        ));
        let rec: Archwing = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Odonata");
        assert_eq!(rec.stats.health, 425);
        assert_eq!(rec.stats.armor, 100);
        assert_eq!(rec.stats.shield, 430);
        assert_eq!(rec.build_price, 7000);
        assert!(!rec.prime.is_prime);
    }

    #[test]
    fn test_deserialize_archwing_prime() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/archwing_test_3.json"
        ));
        let rec: Archwing = from_str(json_data).unwrap();

        assert_eq!(rec.identity.name, "Odonata Prime");
        assert_eq!(rec.stats.health, 650);
        assert_eq!(rec.stats.shield, 640);
        assert!(rec.prime.is_prime);
        assert_eq!(rec.prime.vaulted, Some(true));
    }
}
