use std::sync::{OnceLock, RwLock};

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

#[derive(Clone)]
struct CachedInventoryIndex {
    meta_last_updated: Option<DateTime<Utc>>,
    index: InventorySearchIndex,
}

static INVENTORY_INDEX_CACHE: OnceLock<RwLock<Option<CachedInventoryIndex>>> = OnceLock::new();

fn inventory_index_cache() -> &'static RwLock<Option<CachedInventoryIndex>> {
    INVENTORY_INDEX_CACHE.get_or_init(|| RwLock::new(None))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ItemEnvelope<T> {
    pub item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    // Injected after tantivy retrieval, never part of the stored raw_json
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub details: Option<wf_itemdata::item_data::ItemDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<crate::market::MarketSummary>,
    #[serde(flatten)]
    pub item: T,
}

/// A searchable inventory item tagged with its category.
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

macro_rules! for_each_envelope {
    ($self:expr, $env:ident => $body:expr) => {
        match $self {
            InventoryItemEnvelope::Suits($env) => $body,
            InventoryItemEnvelope::LongGuns($env) => $body,
            InventoryItemEnvelope::Pistols($env) => $body,
            InventoryItemEnvelope::Melee($env) => $body,
            InventoryItemEnvelope::SpaceSuits($env) => $body,
            InventoryItemEnvelope::SpaceGuns($env) => $body,
            InventoryItemEnvelope::SpaceMelee($env) => $body,
            InventoryItemEnvelope::RawUpgrades($env) => $body,
            InventoryItemEnvelope::Upgrades($env) => $body,
            InventoryItemEnvelope::Recipes($env) => $body,
            InventoryItemEnvelope::PendingRecipes($env) => $body,
        }
    };
}

impl InventoryItemEnvelope {
    pub fn category(&self) -> &'static str {
        match self {
            Self::Suits(_) => "suits",
            Self::LongGuns(_) => "long_guns",
            Self::Pistols(_) => "pistols",
            Self::Melee(_) => "melee",
            Self::SpaceSuits(_) => "space_suits",
            Self::SpaceGuns(_) => "space_guns",
            Self::SpaceMelee(_) => "space_melee",
            Self::RawUpgrades(_) => "raw_upgrades",
            Self::Upgrades(_) => "upgrades",
            Self::Recipes(_) => "recipes",
            Self::PendingRecipes(_) => "pending_recipes",
        }
    }

    pub fn item_type(&self) -> &str {
        for_each_envelope!(self, env => &env.item_type)
    }

    pub fn item_count(&self) -> Option<i64> {
        // Only recipes and raw upgrades model ItemCount; preserve the old
        // behavior of also finding it in the flattened catch-all elsewhere.
        match self {
            Self::Recipes(env) => Some(env.item.item_count),
            Self::RawUpgrades(env) => Some(env.item.item_count),
            _ => for_each_envelope!(self, env => extract_other_item_count(env.item.other())),
        }
    }

    pub fn set_details(&mut self, details: wf_itemdata::item_data::ItemDetails) {
        for_each_envelope!(self, env => env.details = Some(details))
    }

    pub fn set_market(&mut self, market: crate::market::MarketSummary) {
        for_each_envelope!(self, env => env.market = Some(market))
    }
}

fn extract_other_item_count(other: Option<&serde_json::Value>) -> Option<i64> {
    other
        .and_then(|v| v.get("ItemCount").or_else(|| v.get("item_count")))
        .and_then(|v| v.as_i64())
}

trait HasOther {
    fn other(&self) -> Option<&serde_json::Value>;
}

macro_rules! impl_has_other {
    ($($ty:ty),+ $(,)?) => {
        $(impl HasOther for $ty {
            fn other(&self) -> Option<&serde_json::Value> {
                self.other.as_ref()
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
    category: Option<&str>,
) -> Vec<ItemView> {
    let mut items = Vec::new();

    let mut push_item = |envelope: InventoryItemEnvelope| {
        let info = lookup_item_info(envelope.item_type(), Some(envelope.category()));
        items.push(ItemView {
            details_name: info.as_ref().and_then(|item| item.name.clone()),
            details_desc: info.as_ref().and_then(|item| item.description.clone()),
            envelope,
        });
    };

    let include = |name: &str, selected: Option<&str>| match selected {
        None => true,
        Some("unknown") => false,
        Some(sel) => name == sel,
    };

    fn envelope<T: Clone>(item: &T, item_type: &str, item_id: Option<String>) -> ItemEnvelope<T> {
        ItemEnvelope {
            item_type: item_type.to_string(),
            item_id,
            details: None,
            market: None,
            item: item.clone(),
        }
    }

    if include("suits", category) {
        for item in &inventory.suits {
            push_item(InventoryItemEnvelope::Suits(envelope(
                item,
                &item.item_type,
                Some(item.item_id.oid.clone()),
            )));
        }
    }

    if include("long_guns", category) {
        for item in &inventory.long_guns {
            push_item(InventoryItemEnvelope::LongGuns(envelope(
                item,
                &item.item_type,
                Some(item.item_id.oid.clone()),
            )));
        }
    }

    if include("pistols", category) {
        for item in &inventory.pistols {
            push_item(InventoryItemEnvelope::Pistols(envelope(
                item,
                &item.item_type,
                Some(item.item_id.oid.clone()),
            )));
        }
    }

    if include("melee", category) {
        for item in &inventory.melee {
            push_item(InventoryItemEnvelope::Melee(envelope(
                item,
                &item.item_type,
                Some(item.item_id.oid.clone()),
            )));
        }
    }

    if include("space_suits", category) {
        for item in &inventory.space_suits {
            push_item(InventoryItemEnvelope::SpaceSuits(envelope(
                item,
                &item.item_type,
                Some(item.item_id.oid.clone()),
            )));
        }
    }

    if include("space_guns", category) {
        for item in &inventory.space_guns {
            push_item(InventoryItemEnvelope::SpaceGuns(envelope(
                item,
                &item.item_type,
                Some(item.item_id.oid.clone()),
            )));
        }
    }

    if include("space_melee", category) {
        for item in &inventory.space_melee {
            push_item(InventoryItemEnvelope::SpaceMelee(envelope(
                item,
                &item.item_type,
                Some(item.item_id.oid.clone()),
            )));
        }
    }

    if include("raw_upgrades", category) {
        for item in &inventory.raw_upgrades {
            push_item(InventoryItemEnvelope::RawUpgrades(envelope(
                item,
                &item.item_type,
                Some(item.last_added_id.oid.clone()),
            )));
        }
    }

    if include("upgrades", category) {
        for item in &inventory.upgrades {
            push_item(InventoryItemEnvelope::Upgrades(envelope(
                item,
                &item.item_type,
                Some(item.item_id.oid.clone()),
            )));
        }
    }

    if include("recipes", category) {
        for item in &inventory.recipes {
            push_item(InventoryItemEnvelope::Recipes(envelope(
                item,
                &item.item_type,
                None,
            )));
        }
    }

    if include("pending_recipes", category) {
        for item in &inventory.pending_recipes {
            push_item(InventoryItemEnvelope::PendingRecipes(envelope(
                item,
                &item.item_type,
                Some(item.item_id.oid.clone()),
            )));
        }
    }

    items
}

pub(crate) fn get_or_build_inventory_index(
    inventory: &Inventory,
    meta: &storage::InventoryMeta,
) -> Result<InventorySearchIndex> {
    // Fast path: reuse cached index if metadata matches last update timestamp
    if let Ok(guard) = inventory_index_cache().read() {
        if let Some(cached) = guard.as_ref() {
            if cached.meta_last_updated == meta.last_updated {
                return Ok(cached.index.clone());
            }
        }
    }

    // Build fresh index over the entire inventory
    let items = collect_inventory_items(inventory, None);
    let index = build_tantivy_index(&items)?;

    // Update cache (best effort; ignore lock poisoning)
    if let Ok(mut guard) = inventory_index_cache().write() {
        *guard = Some(CachedInventoryIndex {
            meta_last_updated: meta.last_updated,
            index: index.clone(),
        });
    }

    Ok(index)
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
        .register("ngram3", NgramTokenizer::new(2, 6, true).unwrap());

    let mut writer = index.writer(20_000_000)?; // ~20MB buffer, tiny dataset

    for item in items {
        let raw = serde_json::to_string(&item.envelope)?;
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

    let total_matches = searcher.search(&query, &Count)? as usize;

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

    /// The envelope must survive the tantivy raw_json round-trip:
    /// serialize -> parse -> serialize must be a fixed point.
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
            assert_eq!(
                serde_json::to_value(&parsed).unwrap(),
                serde_json::to_value(&item.envelope).unwrap(),
                "round-trip mismatch for category {}",
                item.envelope.category()
            );
        }
        // Exercise every category present in the fixture
        for cat in ["suits", "long_guns", "pistols", "melee", "recipes"] {
            assert!(seen.contains(cat), "fixture missing category {}", cat);
        }
    }

    /// The envelope serialization must produce the same JSON object the old
    /// push_item logic did: to_value(item) + injected category/item_type/item_id.
    #[test]
    fn envelope_matches_legacy_injected_shape() {
        let inventory = sample_inventory();

        let legacy = |item_value: Value, category: &str, item_type: &str, item_id: Option<&str>| {
            let mut value = item_value;
            if let Value::Object(ref mut map) = value {
                map.insert("category".into(), Value::String(category.to_string()));
                map.insert("item_type".into(), Value::String(item_type.to_string()));
                if let Some(id) = item_id {
                    map.insert("item_id".into(), Value::String(id.to_string()));
                }
            }
            value
        };

        for item in &inventory.suits {
            let expected = legacy(
                serde_json::to_value(item).unwrap(),
                "suits",
                &item.item_type,
                Some(&item.item_id.oid),
            );
            let envelope = InventoryItemEnvelope::Suits(ItemEnvelope {
                item_type: item.item_type.clone(),
                item_id: Some(item.item_id.oid.clone()),
                details: None,
                market: None,
                item: item.clone(),
            });
            assert_eq!(serde_json::to_value(&envelope).unwrap(), expected);
        }

        for item in &inventory.recipes {
            let expected = legacy(
                serde_json::to_value(item).unwrap(),
                "recipes",
                &item.item_type,
                None,
            );
            let envelope = InventoryItemEnvelope::Recipes(ItemEnvelope {
                item_type: item.item_type.clone(),
                item_id: None,
                details: None,
                market: None,
                item: item.clone(),
            });
            assert_eq!(serde_json::to_value(&envelope).unwrap(), expected);
        }

        for item in &inventory.raw_upgrades {
            let expected = legacy(
                serde_json::to_value(item).unwrap(),
                "raw_upgrades",
                &item.item_type,
                Some(&item.last_added_id.oid),
            );
            let envelope = InventoryItemEnvelope::RawUpgrades(ItemEnvelope {
                item_type: item.item_type.clone(),
                item_id: Some(item.last_added_id.oid.clone()),
                details: None,
                market: None,
                item: item.clone(),
            });
            assert_eq!(serde_json::to_value(&envelope).unwrap(), expected);
        }
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
