use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use tantivy::Term;
use tantivy::query::{Occur, QueryParser, TermQuery};
use tantivy::schema::IndexRecordOption;

use crate::inventory::Inventory;
use crate::{inventory_refresh, process, storage};

use super::item_data::lookup_item_info;
use super::search::{
    build_tantivy_index, collect_inventory_items, get_or_build_inventory_index, search_inventory,
};
use super::state::current_account;
use super::utils::parse_params;

#[derive(Debug, Deserialize, Default)]
pub(crate) struct LoadInventoryParams {
    pub path: Option<String>,
    pub json: Option<Value>,
    pub raw: Option<String>,
    pub save: Option<bool>,
    pub source: Option<String>,
}

pub(crate) async fn handle_inventory_load(params: Option<Value>) -> Result<Value> {
    let params: LoadInventoryParams = parse_params(params)?;
    let sources =
        params.path.is_some() as u8 + params.json.is_some() as u8 + params.raw.is_some() as u8;
    if sources != 1 {
        return Err(anyhow!(
            "inventory.load expects exactly one of 'path', 'json', or 'raw'"
        ));
    }

    let inventory = if let Some(path) = params.path {
        let raw = tokio::fs::read_to_string(&path)
            .await
            .with_context(|| format!("Failed to read inventory file {}", path))?;
        serde_json::from_str(&raw).context("Failed to parse inventory JSON")?
    } else if let Some(raw) = params.raw {
        serde_json::from_str(&raw).context("Failed to parse inventory JSON")?
    } else if let Some(json) = params.json {
        serde_json::from_value(json).context("Failed to parse inventory JSON")?
    } else {
        return Err(anyhow!("Missing inventory source"));
    };

    let save = params.save.unwrap_or(true);
    if save {
        storage::save_inventory(&inventory)?;
        let source = params.source.as_deref().unwrap_or("manual");
        let _ = storage::touch_inventory_updated(Some(source));
    }

    let meta = storage::read_inventory_meta().unwrap_or_default();

    Ok(json!({
        "saved": save,
        "summary": inventory_summary(&inventory),
        "meta": meta,
    }))
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct FilterParams {
    pub category: Option<String>,
    pub item_type: Option<String>,
    pub contains: Option<String>,
    pub tradable: Option<bool>,
    pub item_count: Option<CountFilter>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub include_details: Option<bool>,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CountOp {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Ne,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub(crate) struct CountFilter {
    pub op: CountOp,
    pub value: i64,
}

pub(crate) async fn handle_inventory_filter(params: Option<Value>) -> Result<Value> {
    let params: FilterParams = parse_params(params)?;

    let (inventory, used_custom_path) = if let Some(path) = params.path.clone() {
        let raw = tokio::fs::read_to_string(&path)
            .await
            .with_context(|| format!("Failed to read inventory file {}", path))?;
        (
            serde_json::from_str(&raw).context("Failed to parse inventory JSON")?,
            true,
        )
    } else {
        (storage::read_inventory()?, false)
    };

    let category = params.category.as_deref().and_then(normalize_category);
    if matches!(category, Some("unknown")) {
        return Err(anyhow!(
            "Unknown category '{}'",
            params.category.as_deref().unwrap_or_default()
        ));
    }
    let include_details = params.include_details.unwrap_or(false);

    let meta = storage::read_inventory_meta().unwrap_or_default();

    // Count items in selected category for reporting; index uses full inventory for reuse
    let items_for_counts = collect_inventory_items(&inventory, category, false);
    let total = items_for_counts.len();

    // Build or reuse cached index unless a custom inventory path was provided
    let search_index = if used_custom_path {
        let all_items = collect_inventory_items(&inventory, None, false);
        build_tantivy_index(&all_items)?
    } else {
        get_or_build_inventory_index(&inventory, &meta)?
    };

    let mut clauses: Vec<(Occur, Box<dyn tantivy::query::Query>)> = Vec::new();

    if let Some(cat) = category {
        let term = Term::from_field_text(search_index.category, cat);
        clauses.push((
            Occur::Must,
            Box::new(TermQuery::new(term, IndexRecordOption::Basic)),
        ));
    }

    if let Some(exact) = params.item_type.as_deref() {
        let term = Term::from_field_text(search_index.item_type_exact, exact);
        clauses.push((
            Occur::Must,
            Box::new(TermQuery::new(term, IndexRecordOption::Basic)),
        ));
    }

    if let Some(text) = params.contains.as_deref() {
        let parser = QueryParser::for_index(
            &search_index.index,
            vec![
                search_index.item_type_text,
                search_index.details_name,
                search_index.details_desc,
            ],
        );
        let query = parser.parse_query(text)?;
        clauses.push((Occur::Must, query));
    }

    let (_total_matches, mut values) = search_inventory(&search_index, clauses)?;
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(usize::MAX);

    // Apply non-indexable filters and optional detail expansion
    let mut filtered_values = Vec::new();
    for value in values.drain(..) {
        let mut value = value;
        let mut matches = true;
        if let Value::Object(ref map) = value {
            if let Some(tradable) = params.tradable {
                let item_type = map.get("item_type").and_then(Value::as_str);
                let category = map.get("category").and_then(Value::as_str);
                let details = item_type.and_then(|t| lookup_item_info(t, category));
                let detail_tradable = details.as_ref().and_then(|d| {
                    d.details
                        .get("tradable")
                        .or_else(|| d.details.get("Tradable"))
                        .and_then(Value::as_bool)
                });
                matches = matches && detail_tradable == Some(tradable);
            }

            if let Some(filter) = params.item_count {
                if let Some(count) = extract_item_count(map) {
                    let ok = match filter.op {
                        CountOp::Gt => count > filter.value,
                        CountOp::Gte => count >= filter.value,
                        CountOp::Lt => count < filter.value,
                        CountOp::Lte => count <= filter.value,
                        CountOp::Eq => count == filter.value,
                        CountOp::Ne => count != filter.value,
                    };
                    matches = matches && ok;
                } else {
                    matches = false;
                }
            }
        }

        if !matches {
            continue;
        }

        if include_details {
            if let Value::Object(ref mut map) = value {
                if let Some(item_type) = map.get("item_type").and_then(Value::as_str) {
                    let category = map.get("category").and_then(Value::as_str);
                    if let Some(details) = lookup_item_info(item_type, category) {
                        map.insert("details".to_string(), details.details.clone());
                    }
                }
            }
        }

        filtered_values.push(value);
    }

    let filtered = filtered_values.len();
    let results: Vec<Value> = filtered_values
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    Ok(json!({
        "total": total,
        "filtered": filtered,
        "offset": offset,
        "limit": limit,
        "items": results,
        "meta": meta,
    }))
}

pub(crate) fn handle_inventory_meta_get() -> Result<Value> {
    let meta = storage::read_inventory_meta().unwrap_or_default();
    Ok(serde_json::to_value(meta).context("Failed to serialize inventory metadata")?)
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct RefreshParams {
    pub scan_retries: Option<u32>,
    pub scan_delay_ms: Option<u64>,
    pub save: Option<bool>,
    pub source: Option<String>,
}

pub(crate) async fn handle_inventory_refresh(params: Option<Value>) -> Result<Value> {
    let params: RefreshParams = parse_params(params)?;
    let account_id = current_account()
        .ok_or_else(|| anyhow!("No active account detected. Log in to Warframe first."))?;

    let pid = process::get_warframe_pid()
        .ok_or_else(|| anyhow!("Warframe process not detected; launch the game and try again"))?;

    let scan_retries = params.scan_retries.unwrap_or(5);
    let scan_delay = Duration::from_millis(params.scan_delay_ms.unwrap_or(1500));

    let inventory =
        inventory_refresh::fetch_inventory_from_process(&account_id, pid, scan_retries, scan_delay)
            .await?
            .ok_or_else(|| anyhow!("Could not locate auth data in Warframe memory"))?
            .inventory;

    let save = params.save.unwrap_or(true);
    if save {
        storage::save_inventory(&inventory)?;
        let source = params.source.as_deref().unwrap_or("live-refresh");
        let _ = storage::touch_inventory_updated(Some(source));
    }

    let meta = storage::read_inventory_meta().unwrap_or_default();

    Ok(json!({
        "saved": save,
        "summary": inventory_summary(&inventory),
        "meta": meta,
    }))
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct StaleParams {
    pub timestamp: Option<Value>,
    pub reason: Option<String>,
}

pub(crate) fn handle_inventory_stale_update(params: Option<Value>) -> Result<Value> {
    let params: StaleParams = parse_params(params)?;
    let timestamp = if let Some(value) = params.timestamp {
        parse_timestamp(value)?
    } else {
        Utc::now()
    };

    let meta = storage::mark_inventory_stale_at(timestamp, params.reason)?;
    Ok(serde_json::to_value(meta).context("Failed to serialize inventory metadata")?)
}

pub(crate) fn inventory_summary(inventory: &Inventory) -> Value {
    json!({
        "suits": inventory.suits.len(),
        "long_guns": inventory.long_guns.len(),
        "pistols": inventory.pistols.len(),
        "melee": inventory.melee.len(),
        "space_suits": inventory.space_suits.len(),
        "space_guns": inventory.space_guns.len(),
        "space_melee": inventory.space_melee.len(),
        "raw_upgrades": inventory.raw_upgrades.len(),
        "upgrades": inventory.upgrades.len(),
        "recipes": inventory.recipes.len(),
        "pending_recipes": inventory.pending_recipes.len(),
        "trades_remaining": inventory.trades_remaining,
        "supported_syndicates": inventory.supported_syndicates,
    })
}

// TODO: this looks sus
pub(crate) fn normalize_category(category: &str) -> Option<&'static str> {
    match category.to_lowercase().as_str() {
        "all" => None,
        "suit" | "suits" | "warframe" | "warframes" => Some("suits"),
        "long_gun" | "long_guns" | "primary" | "primaries" | "rifles" => Some("long_guns"),
        "pistol" | "pistols" | "secondary" | "secondaries" => Some("pistols"),
        "melee" => Some("melee"),
        "archwing" | "space_suit" | "space_suits" => Some("space_suits"),
        "archgun" | "arch_gun" | "space_gun" | "space_guns" => Some("space_guns"),
        "archmelee" | "arch_melee" | "space_melee" | "space_melees" => Some("space_melee"),
        "raw_upgrades" | "rawmods" | "raw_mods" => Some("raw_upgrades"),
        "upgrades" | "mods" | "arcanes" => Some("upgrades"),
        "recipes" | "blueprints" => Some("recipes"),
        "pending_recipes" | "pending" => Some("pending_recipes"),
        _ => Some("unknown"),
    }
}

fn parse_timestamp(value: Value) -> Result<DateTime<Utc>> {
    match value {
        Value::String(s) => {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
                return Ok(dt.with_timezone(&Utc));
            }
            if let Ok(num) = s.parse::<i64>() {
                return Ok(epoch_to_datetime(num));
            }
            Err(anyhow!("Unsupported timestamp string format"))
        }
        Value::Number(num) => {
            let Some(num) = num.as_i64() else {
                return Err(anyhow!("Invalid timestamp number"));
            };
            Ok(epoch_to_datetime(num))
        }
        _ => Err(anyhow!("Unsupported timestamp format")),
    }
}

fn epoch_to_datetime(value: i64) -> DateTime<Utc> {
    let (secs, nsec) = if value > 1_000_000_000_000 {
        let secs = value / 1000;
        let nsec = ((value % 1000).abs() as u32) * 1_000_000;
        (secs, nsec)
    } else {
        (value, 0)
    };
    Utc.timestamp_opt(secs, nsec)
        .single()
        .unwrap_or_else(Utc::now)
}

fn extract_item_count(map: &serde_json::Map<String, Value>) -> Option<i64> {
    map.get("ItemCount")
        .or_else(|| map.get("item_count"))
        .and_then(|v| v.as_i64())
}
