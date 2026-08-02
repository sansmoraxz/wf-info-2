//! Enemy item data.

use serde::{Deserialize, Serialize};

use crate::ProductCategory;
use crate::common::{Drop, Patchlog};
use crate::enums::EnemyType;
#[cfg(test)]
use crate::enums::ResistanceType;
use crate::props::{EnemyCombatStats, ItemDetailProps, ItemIdentityProps, TradableProps};
use crate::traits::{Droppable, Item};

pub use crate::props::{Affector, Resistance};

pub type Root = Vec<Enemy>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enemy {
    #[serde(flatten)]
    pub identity: ItemIdentityProps,
    #[serde(rename = "type")]
    pub type_field: EnemyType,
    #[serde(flatten)]
    pub detail: ItemDetailProps,
    #[serde(flatten)]
    pub trade: TradableProps,

    #[serde(flatten)]
    pub combat: EnemyCombatStats,

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
        &self.identity.unique_name
    }
    fn name(&self) -> &str {
        &self.identity.name
    }
    fn category(&self) -> &str {
        &self.identity.category
    }
    fn type_field(&self) -> &str {
        self.type_field.as_ref()
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
            rec.identity.unique_name,
            "/Lotus/Types/Enemies/CorpusChampions/TeamD/CCTeamDOspreyAvatar"
        );
        assert_eq!(rec.identity.name, "002-Er");
        assert_eq!(rec.identity.category, "Enemy");
        assert_eq!(rec.type_field, EnemyType::Corpus);
        assert!(!rec.trade.tradable);

        // Combat stats
        assert_eq!(rec.combat.health, 150);
        assert_eq!(rec.combat.shield, 100);
        assert_eq!(rec.combat.armor, 200);

        // Resistances
        assert_eq!(rec.combat.resistances.len(), 3);
        assert_ne!(
            rec.combat.resistances[0].type_field,
            ResistanceType::Unknown(String::new())
        );
        assert!(!rec.combat.resistances[0].affectors.is_empty());

        // Drops with null chance handling
        assert!(!rec.drops.is_empty());
        assert!(rec.drops.iter().any(|d| d.chance.is_none()));
    }

    #[test]
    fn test_deserialize_enemy_grineer() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/enemy_test_2.json"
        ));
        let rec: Enemy = from_str(json_data).unwrap();

        assert_eq!(rec.type_field, EnemyType::Grineer);
        assert_eq!(rec.combat.health, 800);
        assert_eq!(rec.combat.armor, 250);
        assert_eq!(rec.combat.shield, 0);
        assert_eq!(rec.combat.resistances.len(), 3);
    }

    #[test]
    fn test_deserialize_enemy_corpus() {
        let json_data = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/testdata/itemdata/enemy_test_3.json"
        ));
        let rec: Enemy = from_str(json_data).unwrap();

        assert_eq!(rec.type_field, EnemyType::Corpus);
        assert!(rec.combat.health > 0);
        assert_eq!(rec.combat.resistances.len(), 3);
    }
}
