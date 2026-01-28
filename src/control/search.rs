use std::sync::{OnceLock, RwLock};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use tantivy::Index;
use tantivy::collector::{Count, TopDocs};
use tantivy::doc;
use tantivy::query::{AllQuery, BooleanQuery, Occur};
use tantivy::schema::{
    Field, IndexRecordOption, STORED, STRING, SchemaBuilder, TextFieldIndexing, TextOptions,
};
use tantivy::tokenizer::NgramTokenizer;

use crate::inventory::Inventory;
use crate::storage;

use super::item_data::lookup_item_info;

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

pub(crate) struct ItemView {
    pub item_type: String,
    pub details_name: Option<String>,
    pub details_desc: Option<String>,
    pub value: Value,
}

pub(crate) fn collect_inventory_items(
    inventory: &Inventory,
    category: Option<&str>,
    include_details: bool,
) -> Vec<ItemView> {
    let mut items = Vec::new();

    let mut push_item =
        |category_name: &str, item_type: &str, item_id: Option<&str>, item_value: Value| {
            let mut value = item_value;
            let mut details_name = None;
            let mut details_desc = None;

            if let Value::Object(ref mut map) = value {
                map.insert(
                    "category".to_string(),
                    Value::String(category_name.to_string()),
                );
                map.insert(
                    "item_type".to_string(),
                    Value::String(item_type.to_string()),
                );
                if let Some(id) = item_id {
                    map.insert("item_id".to_string(), Value::String(id.to_string()));
                }

                let info = lookup_item_info(item_type, Some(category_name));
                details_name = info.as_ref().and_then(|item| item.name.clone());
                details_desc = info.as_ref().and_then(|item| item.description.clone());
                if include_details {
                    if let Some(info) = info {
                        map.insert("details".to_string(), info.details.clone());
                    }
                }
            }

            items.push(ItemView {
                item_type: item_type.to_string(),
                details_name,
                details_desc,
                value,
            });
        };

    let include = |name: &str, selected: Option<&str>| match selected {
        None => true,
        Some("unknown") => false,
        Some(sel) => name == sel,
    };

    if include("suits", category) {
        for item in &inventory.suits {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item("suits", &item.item_type, Some(&item.item_id.oid), value);
        }
    }

    if include("long_guns", category) {
        for item in &inventory.long_guns {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item("long_guns", &item.item_type, Some(&item.item_id.oid), value);
        }
    }

    if include("pistols", category) {
        for item in &inventory.pistols {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item("pistols", &item.item_type, Some(&item.item_id.oid), value);
        }
    }

    if include("melee", category) {
        for item in &inventory.melee {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item("melee", &item.item_type, Some(&item.item_id.oid), value);
        }
    }

    if include("space_suits", category) {
        for item in &inventory.space_suits {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item(
                "space_suits",
                &item.item_type,
                Some(&item.item_id.oid),
                value,
            );
        }
    }

    if include("space_guns", category) {
        for item in &inventory.space_guns {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item(
                "space_guns",
                &item.item_type,
                Some(&item.item_id.oid),
                value,
            );
        }
    }

    if include("space_melee", category) {
        for item in &inventory.space_melee {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item(
                "space_melee",
                &item.item_type,
                Some(&item.item_id.oid),
                value,
            );
        }
    }

    if include("raw_upgrades", category) {
        for item in &inventory.raw_upgrades {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item(
                "raw_upgrades",
                &item.item_type,
                Some(&item.last_added_id.oid),
                value,
            );
        }
    }

    if include("upgrades", category) {
        for item in &inventory.upgrades {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item("upgrades", &item.item_type, Some(&item.item_id.oid), value);
        }
    }

    if include("recipes", category) {
        for item in &inventory.recipes {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item("recipes", &item.item_type, None, value);
        }
    }

    if include("pending_recipes", category) {
        for item in &inventory.pending_recipes {
            let value = serde_json::to_value(item).unwrap_or(Value::Null);
            push_item(
                "pending_recipes",
                &item.item_type,
                Some(&item.item_id.oid),
                value,
            );
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
    let items = collect_inventory_items(inventory, None, false);
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
        let raw = serde_json::to_string(&item.value)?;
        let mut doc = doc! {
            item_type_exact => item.item_type.clone(),
            category => item.value.get("category").and_then(Value::as_str).unwrap_or("").to_string(),
            item_type_text => item.item_type.clone(),
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
) -> Result<(usize, Vec<Value>)> {
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
        let raw = match searcher
            .doc::<tantivy::TantivyDocument>(addr)?
            .get_first(search_index.raw_json)
        {
            Some(tantivy::schema::OwnedValue::Str(s)) => s.to_string(),
            _ => String::new(),
        };
        let value: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);
        results.push(value);
    }

    Ok((total_matches, results))
}
