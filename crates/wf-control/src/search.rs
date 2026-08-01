use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tantivy::Index;
use tantivy::collector::{Count, TopDocs};
use tantivy::doc;
use tantivy::query::{AllQuery, BooleanQuery, Occur};
use tantivy::schema::{
    Field, IndexRecordOption, STORED, STRING, SchemaBuilder, TextFieldIndexing, TextOptions,
    Value as TantivyValue,
};
use tantivy::tokenizer::NgramTokenizer;

use wf_core::storage;
use wf_inventory::Inventory;

use wf_itemdata::item_data::lookup_item_info;

#[derive(Clone)]
pub(crate) struct InventorySearchIndex {
    pub index: Index,
    pub item_type_exact: Field,
    pub item_type_text: Field,
    pub details_name: Field,
    pub details_desc: Field,
    pub category: Field,
    pub raw_json: Field,
}

/// An inventory snapshot together with the search index built from it. The
/// two are only ever created (and cached) as a unit, so an index can never
/// be paired with a different inventory than the one it was built over.
pub(crate) struct IndexedInventory {
    pub inventory: Inventory,
    pub index: InventorySearchIndex,
    /// `InventoryMeta::last_updated` at build time; the cache key.
    last_updated: Option<DateTime<Utc>>,
}

impl IndexedInventory {
    pub(crate) fn build(inventory: Inventory, last_updated: Option<DateTime<Utc>>) -> Result<Self> {
        let items = collect_inventory_items(&inventory, None);
        let index = build_tantivy_index(&items)?;
        Ok(Self {
            inventory,
            index,
            last_updated,
        })
    }
}

/// Shared cache of the current [`IndexedInventory`] pair, keyed by
/// `InventoryMeta::last_updated`. Owned by the composition root as
/// `Arc<InventoryIndexCache>`.
#[derive(Default)]
pub(crate) struct InventoryIndexCache(arc_swap::ArcSwapOption<IndexedInventory>);

impl InventoryIndexCache {
    /// Return the cached inventory+index pair if it matches
    /// `meta.last_updated`, otherwise load the stored inventory and build a
    /// fresh pair.
    pub(crate) fn get_or_build(
        &self,
        meta: &storage::InventoryMeta,
    ) -> Result<Arc<IndexedInventory>> {
        if let Some(cached) = self.0.load().as_ref()
            && cached.last_updated == meta.last_updated
        {
            return Ok(Arc::clone(cached));
        }

        let inventory = storage::read_inventory()?;
        let indexed = Arc::new(IndexedInventory::build(inventory, meta.last_updated)?);
        self.0.store(Some(Arc::clone(&indexed)));

        Ok(indexed)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ItemEnvelope<T> {
    pub item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    // details/market are injected after tantivy retrieval and are never part
    // of the stored raw_json; skip_deserializing keeps a stray same-named key
    // in the item's catch-all from being parsed as these types.
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub details: Option<wf_itemdata::item_data::ItemDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub market: Option<crate::market::MarketSummary>,
    #[serde(flatten)]
    pub item: T,
}

/// Keys the envelope itself emits. A game item whose catch-all carried one of
/// these would serialize as a duplicate JSON key (breaking re-parse) or shadow
/// the injected value, so they are stripped at envelope construction — the
/// same overwrite semantics the old map-insertion code had.
const RESERVED_ENVELOPE_KEYS: [&str; 5] = ["category", "item_type", "item_id", "details", "market"];

/// An inventory category. `Display` emits the wire names used as serde tags
/// on [`InventoryItemEnvelope`]; `FromStr` additionally accepts the user-facing
/// aliases previously handled by `normalize_category`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString, strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub(crate) enum Category {
    #[strum(
        to_string = "suits",
        serialize = "suit",
        serialize = "warframe",
        serialize = "warframes"
    )]
    Suits,
    #[strum(
        to_string = "long_guns",
        serialize = "long_gun",
        serialize = "primary",
        serialize = "primaries",
        serialize = "rifles"
    )]
    LongGuns,
    #[strum(
        to_string = "pistols",
        serialize = "pistol",
        serialize = "secondary",
        serialize = "secondaries"
    )]
    Pistols,
    Melee,
    #[strum(
        to_string = "space_suits",
        serialize = "space_suit",
        serialize = "archwing"
    )]
    SpaceSuits,
    #[strum(
        to_string = "space_guns",
        serialize = "space_gun",
        serialize = "archgun",
        serialize = "arch_gun"
    )]
    SpaceGuns,
    #[strum(
        to_string = "space_melee",
        serialize = "space_melees",
        serialize = "archmelee",
        serialize = "arch_melee"
    )]
    SpaceMelee,
    #[strum(
        to_string = "raw_upgrades",
        serialize = "rawmods",
        serialize = "raw_mods"
    )]
    RawUpgrades,
    #[strum(to_string = "upgrades", serialize = "mods", serialize = "arcanes")]
    Upgrades,
    #[strum(to_string = "recipes", serialize = "blueprints")]
    Recipes,
    #[strum(to_string = "pending_recipes", serialize = "pending")]
    PendingRecipes,
}

/// A searchable inventory item tagged with its category.
#[enum_dispatch::enum_dispatch(EnvelopeAccess)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "category", rename_all = "snake_case")]
pub(crate) enum InventoryItemEnvelope {
    Suits(ItemEnvelope<wf_inventory::suit::Suit>),
    LongGuns(ItemEnvelope<wf_inventory::long_gun::LongGun>),
    Pistols(ItemEnvelope<wf_inventory::pistol::Pistol>),
    Melee(ItemEnvelope<wf_inventory::melee::Melee>),
    SpaceSuits(ItemEnvelope<wf_inventory::space_suit::SpaceSuit>),
    SpaceGuns(ItemEnvelope<wf_inventory::space_gun::SpaceGun>),
    SpaceMelee(ItemEnvelope<wf_inventory::space_melee::SpaceMelee>),
    RawUpgrades(ItemEnvelope<wf_inventory::upgrades::RawUpgrade>),
    Upgrades(ItemEnvelope<wf_inventory::upgrades::Upgrade>),
    Recipes(ItemEnvelope<wf_inventory::recipe::Recipe>),
    PendingRecipes(ItemEnvelope<wf_inventory::recipe::PendingRecipe>),
}

/// Category-erased view over the `ItemEnvelope<T>` inside each variant —
/// everything the envelope offers that doesn't depend on the item type.
/// `enum_dispatch` generates the delegating impl on [`InventoryItemEnvelope`].
#[enum_dispatch::enum_dispatch]
pub(crate) trait EnvelopeAccess {
    fn item_type(&self) -> &str;
    fn item_other(&self) -> Option<&serde_json::Value>;
    fn set_details(&mut self, details: wf_itemdata::item_data::ItemDetails);
    fn set_market(&mut self, market: crate::market::MarketSummary);
}

impl<T: HasOther> EnvelopeAccess for ItemEnvelope<T> {
    fn item_type(&self) -> &str {
        &self.item_type
    }
    fn item_other(&self) -> Option<&serde_json::Value> {
        self.item.other()
    }
    fn set_details(&mut self, details: wf_itemdata::item_data::ItemDetails) {
        self.details = Some(details);
    }
    fn set_market(&mut self, market: crate::market::MarketSummary) {
        self.market = Some(market);
    }
}

impl InventoryItemEnvelope {
    pub fn category(&self) -> Category {
        match self {
            Self::Suits(_) => Category::Suits,
            Self::LongGuns(_) => Category::LongGuns,
            Self::Pistols(_) => Category::Pistols,
            Self::Melee(_) => Category::Melee,
            Self::SpaceSuits(_) => Category::SpaceSuits,
            Self::SpaceGuns(_) => Category::SpaceGuns,
            Self::SpaceMelee(_) => Category::SpaceMelee,
            Self::RawUpgrades(_) => Category::RawUpgrades,
            Self::Upgrades(_) => Category::Upgrades,
            Self::Recipes(_) => Category::Recipes,
            Self::PendingRecipes(_) => Category::PendingRecipes,
        }
    }

    pub fn item_count(&self) -> Option<i64> {
        // Only recipes and raw upgrades model ItemCount; preserve the old
        // behavior of also finding it in the flattened catch-all elsewhere.
        match self {
            Self::Recipes(env) => Some(env.item.item_count),
            Self::RawUpgrades(env) => Some(env.item.item_count),
            _ => extract_other_item_count(self.item_other()),
        }
    }
}

fn extract_other_item_count(other: Option<&serde_json::Value>) -> Option<i64> {
    other
        .and_then(|v| v.get("ItemCount").or_else(|| v.get("item_count")))
        .and_then(|v| v.as_i64())
}

trait HasOther {
    fn other(&self) -> Option<&serde_json::Value>;
    fn other_mut(&mut self) -> Option<&mut serde_json::Value>;
}

// The wf-inventory item types are unrelated structs that each happen to have
// a `pub other: Option<Value>` catch-all; Rust has no structural typing, so a
// blanket impl is impossible and the alternatives (a trait+impls in
// wf-inventory, or a derive crate) are strictly more code for the same 11
// impls. A field-accessor macro is the cheapest correct form here.
macro_rules! impl_has_other {
    ($($ty:ty),+ $(,)?) => {
        $(impl HasOther for $ty {
            fn other(&self) -> Option<&serde_json::Value> {
                self.other.as_ref()
            }
            fn other_mut(&mut self) -> Option<&mut serde_json::Value> {
                self.other.as_mut()
            }
        })+
    };
}

impl_has_other!(
    wf_inventory::suit::Suit,
    wf_inventory::long_gun::LongGun,
    wf_inventory::pistol::Pistol,
    wf_inventory::melee::Melee,
    wf_inventory::space_suit::SpaceSuit,
    wf_inventory::space_gun::SpaceGun,
    wf_inventory::space_melee::SpaceMelee,
    wf_inventory::upgrades::RawUpgrade,
    wf_inventory::upgrades::Upgrade,
    wf_inventory::recipe::Recipe,
    wf_inventory::recipe::PendingRecipe,
);

pub(crate) struct ItemView {
    pub details_name: Option<String>,
    pub details_desc: Option<String>,
    pub envelope: InventoryItemEnvelope,
}

pub(crate) fn collect_inventory_items(
    inventory: &Inventory,
    category: Option<Category>,
) -> Vec<ItemView> {
    let mut items = Vec::new();

    let mut push_item = |envelope: InventoryItemEnvelope| {
        let info = lookup_item_info(envelope.item_type(), Some(envelope.category().as_ref()));
        items.push(ItemView {
            details_name: info.as_ref().and_then(|item| item.name.clone()),
            details_desc: info.as_ref().and_then(|item| item.description.clone()),
            envelope,
        });
    };

    fn envelope<T: Clone + HasOther>(
        item: &T,
        item_type: &str,
        item_id: Option<String>,
    ) -> ItemEnvelope<T> {
        let mut item = item.clone();
        if let Some(serde_json::Value::Object(map)) = item.other_mut() {
            for key in RESERVED_ENVELOPE_KEYS {
                map.remove(key);
            }
        }
        ItemEnvelope {
            item_type: item_type.to_string(),
            item_id,
            details: None,
            market: None,
            item,
        }
    }

    // One arm per category: inventory field, envelope variant, and how the
    // item id is derived (`item_id.oid`, `last_added_id.oid`, or absent).
    macro_rules! collect {
        ($cat:ident, $field:ident, |$item:ident| $id:expr) => {
            if category.is_none_or(|sel| sel == Category::$cat) {
                for $item in &inventory.$field {
                    push_item(InventoryItemEnvelope::$cat(envelope(
                        $item,
                        &$item.item_type,
                        $id,
                    )));
                }
            }
        };
    }

    collect!(Suits, suits, |item| Some(item.item_id.oid.clone()));
    collect!(LongGuns, long_guns, |item| Some(item.item_id.oid.clone()));
    collect!(Pistols, pistols, |item| Some(item.item_id.oid.clone()));
    collect!(Melee, melee, |item| Some(item.item_id.oid.clone()));
    collect!(SpaceSuits, space_suits, |item| Some(
        item.item_id.oid.clone()
    ));
    collect!(SpaceGuns, space_guns, |item| Some(item.item_id.oid.clone()));
    collect!(SpaceMelee, space_melee, |item| Some(
        item.item_id.oid.clone()
    ));
    collect!(RawUpgrades, raw_upgrades, |item| Some(
        item.last_added_id.oid.clone()
    ));
    collect!(Upgrades, upgrades, |item| Some(item.item_id.oid.clone()));
    collect!(Recipes, recipes, |item| None);
    collect!(PendingRecipes, pending_recipes, |item| Some(
        item.item_id.oid.clone()
    ));

    items
}

pub(crate) fn build_tantivy_index(items: &[ItemView]) -> Result<InventorySearchIndex> {
    let mut schema_builder = SchemaBuilder::default();

    let item_type_exact = schema_builder.add_text_field("item_type_exact", STRING | STORED);
    let category = schema_builder.add_text_field("category", STRING | STORED);

    let ngram_indexing = TextFieldIndexing::default()
        .set_tokenizer("ngram3")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let ngram_opts = TextOptions::default()
        .set_indexing_options(ngram_indexing)
        .set_stored();

    let item_type_text = schema_builder.add_text_field("item_type", ngram_opts.clone());
    let details_name = schema_builder.add_text_field("details_name", ngram_opts.clone());
    let details_desc = schema_builder.add_text_field("details_desc", ngram_opts);

    let raw_json = schema_builder.add_text_field("raw_json", STORED);

    let schema = schema_builder.build();
    let index = Index::create_in_ram(schema);
    index
        .tokenizers()
        .register("ngram3", NgramTokenizer::new(2, 6, true)?);

    let mut writer = index.writer(20_000_000)?; // ~20MB buffer, tiny dataset

    for item in items {
        // One unserializable item must not fail the whole index build
        let raw = match serde_json::to_string(&item.envelope) {
            Ok(raw) => raw,
            Err(e) => {
                log::warn!(
                    "Skipping unserializable item {}: {}",
                    item.envelope.item_type(),
                    e
                );
                continue;
            }
        };
        let mut doc = doc! {
            item_type_exact => item.envelope.item_type().to_string(),
            category => item.envelope.category().to_string(),
            item_type_text => item.envelope.item_type().to_string(),
            raw_json => raw,
        };

        if let Some(name) = &item.details_name {
            doc.add_text(details_name, name);
        }
        if let Some(desc) = &item.details_desc {
            doc.add_text(details_desc, desc);
        }

        writer.add_document(doc)?;
    }

    writer.commit()?;

    Ok(InventorySearchIndex {
        index,
        item_type_exact,
        item_type_text,
        details_name,
        details_desc,
        category,
        raw_json,
    })
}

pub(crate) fn search_inventory(
    search_index: &InventorySearchIndex,
    clauses: Vec<(Occur, Box<dyn tantivy::query::Query>)>,
) -> Result<(usize, Vec<InventoryItemEnvelope>)> {
    let reader = search_index.index.reader()?;
    let searcher = reader.searcher();

    let query: Box<dyn tantivy::query::Query> = if clauses.is_empty() {
        Box::new(AllQuery)
    } else {
        Box::new(BooleanQuery::new(clauses))
    };

    let total_matches = searcher.search(&query, &Count)?;

    let top_docs = if total_matches == 0 {
        Vec::new()
    } else {
        searcher.search(&query, &TopDocs::with_limit(total_matches))?
    };

    let mut results = Vec::new();
    for (_score, addr) in top_docs {
        let raw = searcher
            .doc::<tantivy::TantivyDocument>(addr)?
            .get_first(search_index.raw_json)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default();
        match serde_json::from_str::<InventoryItemEnvelope>(&raw) {
            Ok(envelope) => results.push(envelope),
            Err(e) => log::warn!("Skipping unparseable indexed item: {}", e),
        }
    }

    Ok((total_matches, results))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn sample_inventory() -> Inventory {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../wf-inventory/testdata/inventory/sample_inventory.json"
        ));
        serde_json::from_str(raw).unwrap()
    }

    const ALL_CATEGORIES: [(Category, &str); 11] = [
        (Category::Suits, "suits"),
        (Category::LongGuns, "long_guns"),
        (Category::Pistols, "pistols"),
        (Category::Melee, "melee"),
        (Category::SpaceSuits, "space_suits"),
        (Category::SpaceGuns, "space_guns"),
        (Category::SpaceMelee, "space_melee"),
        (Category::RawUpgrades, "raw_upgrades"),
        (Category::Upgrades, "upgrades"),
        (Category::Recipes, "recipes"),
        (Category::PendingRecipes, "pending_recipes"),
    ];

    /// The envelope must survive the tantivy raw_json round-trip
    /// (serialize -> parse -> serialize must be a fixed point), and the
    /// serialized "category" tag must agree with the category() accessor,
    /// for every one of the 11 categories.
    #[test]
    fn envelope_round_trips_through_json_for_all_categories() {
        let inventory = sample_inventory();
        let items = collect_inventory_items(&inventory, None);
        assert!(!items.is_empty());

        let mut seen = std::collections::HashSet::new();
        for item in &items {
            seen.insert(item.envelope.category());
            let raw = serde_json::to_string(&item.envelope).unwrap();
            let parsed: InventoryItemEnvelope = serde_json::from_str(&raw).unwrap();
            let value = serde_json::to_value(&parsed).unwrap();
            assert_eq!(
                value,
                serde_json::to_value(&item.envelope).unwrap(),
                "round-trip mismatch for category {}",
                item.envelope.category()
            );
            // The serde tag and the hand-written category() must never drift
            assert_eq!(value["category"], item.envelope.category().to_string());
        }
        for (cat, wire) in ALL_CATEGORIES {
            assert!(seen.contains(&cat), "fixture missing category {}", cat);
            // Category's Display must byte-match the serde tag wire names
            assert_eq!(cat.to_string(), wire);
            // ...and FromStr must round-trip the canonical name
            assert_eq!(wire.parse::<Category>().unwrap(), cat);
        }
    }

    /// The production collection path must produce the same JSON objects the
    /// old push_item logic did: to_value(item) + injected
    /// category/item_type/item_id, for every category and every item.
    #[test]
    fn collected_envelopes_match_legacy_injected_shape() {
        let inventory = sample_inventory();

        fn legacy<T: serde::Serialize>(
            item: &T,
            category: &str,
            item_type: &str,
            item_id: Option<&str>,
        ) -> Value {
            let mut value = serde_json::to_value(item).unwrap();
            let map = value.as_object_mut().unwrap();
            map.insert("category".into(), Value::String(category.to_string()));
            map.insert("item_type".into(), Value::String(item_type.to_string()));
            if let Some(id) = item_id {
                map.insert("item_id".into(), Value::String(id.to_string()));
            }
            value
        }

        macro_rules! check_category {
            ($field:ident, $cat:literal, $id:expr) => {
                let views = collect_inventory_items(&inventory, Some($cat.parse().unwrap()));
                assert_eq!(views.len(), inventory.$field.len(), $cat);
                for (view, item) in views.iter().zip(&inventory.$field) {
                    let id: Option<&str> = $id(item);
                    let expected = legacy(item, $cat, &item.item_type, id);
                    assert_eq!(
                        serde_json::to_value(&view.envelope).unwrap(),
                        expected,
                        "legacy shape mismatch in {}",
                        $cat
                    );
                }
            };
        }

        type IdOf<T> = fn(&T) -> Option<&str>;
        check_category!(
            suits,
            "suits",
            (|i| Some(i.item_id.oid.as_str())) as IdOf<wf_inventory::suit::Suit>
        );
        check_category!(
            long_guns,
            "long_guns",
            (|i| Some(i.item_id.oid.as_str())) as IdOf<wf_inventory::long_gun::LongGun>
        );
        check_category!(
            pistols,
            "pistols",
            (|i| Some(i.item_id.oid.as_str())) as IdOf<wf_inventory::pistol::Pistol>
        );
        check_category!(
            melee,
            "melee",
            (|i| Some(i.item_id.oid.as_str())) as IdOf<wf_inventory::melee::Melee>
        );
        check_category!(
            space_suits,
            "space_suits",
            (|i| Some(i.item_id.oid.as_str())) as IdOf<wf_inventory::space_suit::SpaceSuit>
        );
        check_category!(
            space_guns,
            "space_guns",
            (|i| Some(i.item_id.oid.as_str())) as IdOf<wf_inventory::space_gun::SpaceGun>
        );
        check_category!(
            space_melee,
            "space_melee",
            (|i| Some(i.item_id.oid.as_str())) as IdOf<wf_inventory::space_melee::SpaceMelee>
        );
        check_category!(
            raw_upgrades,
            "raw_upgrades",
            (|i| Some(i.last_added_id.oid.as_str())) as IdOf<wf_inventory::upgrades::RawUpgrade>
        );
        check_category!(
            upgrades,
            "upgrades",
            (|i| Some(i.item_id.oid.as_str())) as IdOf<wf_inventory::upgrades::Upgrade>
        );
        check_category!(
            recipes,
            "recipes",
            (|_| None) as IdOf<wf_inventory::recipe::Recipe>
        );
        check_category!(
            pending_recipes,
            "pending_recipes",
            (|i| Some(i.item_id.oid.as_str())) as IdOf<wf_inventory::recipe::PendingRecipe>
        );
    }

    /// Reserved envelope keys in an item's catch-all must be stripped at
    /// construction — otherwise serialization emits duplicate JSON keys and
    /// retrieval silently drops the item.
    #[test]
    fn reserved_keys_in_catch_all_are_stripped() {
        let inventory = sample_inventory();
        let mut suit = inventory.suits[0].clone();
        let mut extra = serde_json::Map::new();
        extra.insert("category".into(), Value::String("evil".into()));
        extra.insert("market".into(), Value::String("evil".into()));
        extra.insert("details".into(), Value::String("evil".into()));
        extra.insert("KeptKey".into(), Value::String("kept".into()));
        suit.other = Some(Value::Object(extra));

        let mut poisoned = inventory.clone();
        poisoned.suits = vec![suit];
        let items = collect_inventory_items(&poisoned, Some(Category::Suits));
        assert_eq!(items.len(), 1);

        let raw = serde_json::to_string(&items[0].envelope).unwrap();
        let parsed: InventoryItemEnvelope = serde_json::from_str(&raw).unwrap();
        let value = serde_json::to_value(&parsed).unwrap();
        assert_eq!(value["category"], "suits");
        assert_eq!(value["KeptKey"], "kept");
        assert!(value.get("market").is_none());
        assert!(value.get("details").is_none());
    }

    #[test]
    fn envelope_item_count_reads_typed_and_flattened_fields() {
        let inventory = sample_inventory();

        let recipe = &inventory.recipes[0];
        let envelope = InventoryItemEnvelope::Recipes(ItemEnvelope {
            item_type: recipe.item_type.clone(),
            item_id: None,
            details: None,
            market: None,
            item: recipe.clone(),
        });
        assert_eq!(envelope.item_count(), Some(recipe.item_count));

        let suit = &inventory.suits[0];
        let envelope = InventoryItemEnvelope::Suits(ItemEnvelope {
            item_type: suit.item_type.clone(),
            item_id: Some(suit.item_id.oid.clone()),
            details: None,
            market: None,
            item: suit.clone(),
        });
        assert_eq!(envelope.item_count(), None);
    }
}
