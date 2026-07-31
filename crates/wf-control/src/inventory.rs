#[cfg(feature = "memory")]
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tantivy::Term;
use tantivy::query::{Occur, QueryParser, TermQuery};
use tantivy::schema::IndexRecordOption;

use wf_core::storage;
use wf_inventory::Inventory;

use super::broadcaster;
use super::events::{
    DaemonEvent, InventoryFetchedEvent, InventoryStaleEvent, InventorySummary, Source,
};
use super::market::fetch_market_summary;
use super::search::{
    Category, InventoryItemEnvelope, build_tantivy_index, collect_inventory_items,
    get_or_build_inventory_index, search_inventory,
};
use wf_itemdata::item_data::lookup_item_info;

#[cfg(feature = "memory")]
use wf_core::{inventory_refresh, process};

#[derive(Debug, Deserialize, Default)]
pub(crate) struct LoadInventoryParams {
    pub path: Option<String>,
    pub json: Option<Value>,
    pub raw: Option<String>,
    pub save: Option<bool>,
    pub source: Option<Source>,
    pub encrypted: Option<bool>,
}

#[derive(Debug, Serialize)]
pub(crate) struct InventoryLoadResponse {
    pub saved: bool,
    pub summary: InventorySummary,
    pub meta: storage::InventoryMeta,
}

#[derive(Debug, Serialize)]
pub(crate) struct InventoryFilterResponse {
    pub total: usize,
    pub filtered: usize,
    pub offset: usize,
    pub limit: usize,
    pub items: Vec<InventoryItemEnvelope>,
    pub meta: storage::InventoryMeta,
}

pub(crate) async fn handle_inventory_load(
    params: LoadInventoryParams,
) -> Result<InventoryLoadResponse> {
    let sources =
        params.path.is_some() as u8 + params.json.is_some() as u8 + params.raw.is_some() as u8;
    if sources != 1 {
        return Err(anyhow!(
            "inventory.load expects exactly one of 'path', 'json', or 'raw'"
        ));
    }

    let encrypted = params.encrypted.unwrap_or(false);
    let inventory = if let Some(path) = params.path {
        if encrypted {
            let data = tokio::fs::read(&path)
                .await
                .with_context(|| format!("Failed to read inventory file {}", path))?;
            storage::decrypt_inventory_bytes(&data)?
        } else {
            let raw = tokio::fs::read_to_string(&path)
                .await
                .with_context(|| format!("Failed to read inventory file {}", path))?;
            serde_json::from_str(&raw).context("Failed to parse inventory JSON")?
        }
    } else if let Some(raw) = params.raw {
        serde_json::from_str(&raw).context("Failed to parse inventory JSON")?
    } else if let Some(json) = params.json {
        serde_json::from_value(json).context("Failed to parse inventory JSON")?
    } else {
        return Err(anyhow!("Missing inventory source"));
    };

    let save = params.save.unwrap_or(true);
    let source = params.source.unwrap_or_default();
    if save {
        storage::save_inventory(&inventory)?;
        let _ = storage::touch_inventory_updated(Some(&source.to_string()));

        // Emit inventory fetched event
        broadcaster::emit(DaemonEvent::InventoryFetched(InventoryFetchedEvent {
            timestamp: Utc::now(),
            source,
            summary: inventory_summary(&inventory),
        }));
    }

    let meta = storage::read_inventory_meta().unwrap_or_default();

    Ok(InventoryLoadResponse {
        saved: save,
        summary: inventory_summary(&inventory),
        meta,
    })
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
    pub include_market: Option<bool>,
    pub path: Option<String>,
    pub encrypted: Option<bool>,
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

pub(crate) async fn handle_inventory_filter(
    params: FilterParams,
) -> Result<InventoryFilterResponse> {
    let encrypted = params.encrypted.unwrap_or(false);
    let (inventory, used_custom_path) = if let Some(path) = params.path.clone() {
        let inv = if encrypted {
            let data = tokio::fs::read(&path)
                .await
                .with_context(|| format!("Failed to read inventory file {}", path))?;
            storage::decrypt_inventory_bytes(&data)?
        } else {
            let raw = tokio::fs::read_to_string(&path)
                .await
                .with_context(|| format!("Failed to read inventory file {}", path))?;
            serde_json::from_str(&raw).context("Failed to parse inventory JSON")?
        };
        (inv, true)
    } else {
        (storage::read_inventory()?, false)
    };

    let category = match params.category.as_deref() {
        None => None,
        Some(raw) if raw.eq_ignore_ascii_case("all") => None,
        Some(raw) => Some(
            raw.parse::<Category>()
                .map_err(|_| anyhow!("Unknown category '{}'", raw))?,
        ),
    };
    let include_details = params.include_details.unwrap_or(false);

    let meta = storage::read_inventory_meta().unwrap_or_default();

    // Count items in selected category for reporting; index uses full inventory for reuse
    let items_for_counts = collect_inventory_items(&inventory, category);
    let total = items_for_counts.len();

    // Build or reuse cached index unless a custom inventory path was provided
    let search_index = if used_custom_path {
        let all_items = collect_inventory_items(&inventory, None);
        build_tantivy_index(&all_items)?
    } else {
        get_or_build_inventory_index(&inventory, &meta)?
    };

    let mut clauses: Vec<(Occur, Box<dyn tantivy::query::Query>)> = Vec::new();

    if let Some(cat) = category {
        let term = Term::from_field_text(search_index.category, cat.as_ref());
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

    let (_total_matches, mut envelopes) = search_inventory(&search_index, clauses)?;
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(usize::MAX);

    // Apply non-indexable filters and optional detail expansion
    let mut filtered_items = Vec::new();
    for mut envelope in envelopes.drain(..) {
        if let Some(tradable) = params.tradable {
            let details = lookup_item_info(envelope.item_type(), Some(envelope.category().as_ref()));
            let detail_tradable = details.as_ref().map(|d| d.details.tradable());
            if detail_tradable != Some(tradable) {
                continue;
            }
        }

        if let Some(filter) = params.item_count {
            let Some(count) = envelope.item_count() else {
                continue;
            };
            let ok = match filter.op {
                CountOp::Gt => count > filter.value,
                CountOp::Gte => count >= filter.value,
                CountOp::Lt => count < filter.value,
                CountOp::Lte => count <= filter.value,
                CountOp::Eq => count == filter.value,
                CountOp::Ne => count != filter.value,
            };
            if !ok {
                continue;
            }
        }

        if include_details
            && let Some(details) =
                lookup_item_info(envelope.item_type(), Some(envelope.category().as_ref()))
        {
            envelope.set_details(details.details);
        }

        filtered_items.push(envelope);
    }

    let include_market = params.include_market.unwrap_or(false);
    if include_market {
        for envelope in &mut filtered_items {
            if let Some(market) = fetch_market_summary(envelope.item_type()).await {
                envelope.set_market(market);
            }
        }
    }

    let filtered = filtered_items.len();
    let items: Vec<_> = filtered_items
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    Ok(InventoryFilterResponse {
        total,
        filtered,
        offset,
        limit,
        items,
        meta,
    })
}

pub(crate) fn handle_inventory_meta_get() -> Result<storage::InventoryMeta> {
    Ok(storage::read_inventory_meta().unwrap_or_default())
}

#[cfg(feature = "memory")]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct RefreshParams {
    pub scan_retries: Option<u32>,
    pub scan_delay_ms: Option<u64>,
    pub save: Option<bool>,
    pub source: Option<Source>,
}

#[cfg(feature = "memory")]
pub(crate) async fn handle_inventory_refresh(
    params: RefreshParams,
) -> Result<InventoryLoadResponse> {
    let pid = process::get_warframe_pid()
        .ok_or_else(|| anyhow!("Warframe process not detected; launch the game and try again"))?;

    let scan_retries = params.scan_retries.unwrap_or(5);
    let scan_delay = Duration::from_millis(params.scan_delay_ms.unwrap_or(1500));

    let inventory = inventory_refresh::fetch_inventory_from_process(pid, scan_retries, scan_delay)
        .await?
        .ok_or_else(|| anyhow!("Could not locate auth data in Warframe memory"))?
        .inventory;

    let save = params.save.unwrap_or(true);
    let source = params.source.unwrap_or(Source::LiveRefresh);
    if save {
        storage::save_inventory(&inventory)?;
        let _ = storage::touch_inventory_updated(Some(&source.to_string()));

        // Emit inventory fetched event
        broadcaster::emit(DaemonEvent::InventoryFetched(InventoryFetchedEvent {
            timestamp: Utc::now(),
            source,
            summary: inventory_summary(&inventory),
        }));
    }

    let meta = storage::read_inventory_meta().unwrap_or_default();

    Ok(InventoryLoadResponse {
        saved: save,
        summary: inventory_summary(&inventory),
        meta,
    })
}

#[cfg(not(feature = "memory"))]
pub(crate) async fn handle_inventory_refresh() -> Result<InventoryLoadResponse> {
    anyhow::bail!("inventory.refresh requires the 'memory' feature to be enabled")
}

/// Timestamp accepted as a numeric epoch (seconds or milliseconds) or a
/// string holding RFC3339 or a stringified epoch.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum TimestampParam {
    Epoch(i64),
    Text(String),
}

impl TimestampParam {
    fn to_datetime(&self) -> Result<DateTime<Utc>> {
        match self {
            Self::Text(s) => {
                if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
                    return Ok(dt.with_timezone(&Utc));
                }
                if let Ok(num) = s.parse::<i64>() {
                    return Ok(epoch_to_datetime(num));
                }
                Err(anyhow!("Unsupported timestamp string format"))
            }
            Self::Epoch(num) => Ok(epoch_to_datetime(*num)),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct StaleParams {
    pub timestamp: Option<TimestampParam>,
    pub reason: Option<String>,
}

pub(crate) fn handle_inventory_stale_update(params: StaleParams) -> Result<storage::InventoryMeta> {
    let timestamp = if let Some(value) = params.timestamp {
        value.to_datetime()?
    } else {
        Utc::now()
    };

    let reason = params.reason.clone();
    let meta = storage::mark_inventory_stale_at(timestamp, reason.clone())?;

    // Emit inventory stale event
    broadcaster::emit(DaemonEvent::InventoryStale(InventoryStaleEvent {
        timestamp: Utc::now(),
        stale_since: timestamp,
        reason,
    }));

    Ok(meta)
}

pub(crate) fn inventory_summary(inventory: &Inventory) -> InventorySummary {
    InventorySummary {
        suits: inventory.suits.len(),
        long_guns: inventory.long_guns.len(),
        pistols: inventory.pistols.len(),
        melee: inventory.melee.len(),
        space_suits: inventory.space_suits.len(),
        space_guns: inventory.space_guns.len(),
        space_melee: inventory.space_melee.len(),
        raw_upgrades: inventory.raw_upgrades.len(),
        upgrades: inventory.upgrades.len(),
        recipes: inventory.recipes.len(),
        pending_recipes: inventory.pending_recipes.len(),
        trades_remaining: inventory.trades_remaining,
        supported_syndicates: inventory.supported_syndicates.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Pins the wire shape of the inventory.load/refresh and inventory.filter
    /// response payloads against the old json! literals.
    #[test]
    fn load_and_filter_responses_match_legacy_shape() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../wf-inventory/testdata/inventory/sample_inventory.json"
        ));
        let inventory: Inventory = serde_json::from_str(raw).unwrap();
        let meta = storage::InventoryMeta::default();
        let meta_value = serde_json::to_value(&meta).unwrap();

        let load = InventoryLoadResponse {
            saved: true,
            summary: inventory_summary(&inventory),
            meta: meta.clone(),
        };
        assert_eq!(
            serde_json::to_value(&load).unwrap(),
            json!({
                "saved": true,
                "summary": serde_json::to_value(inventory_summary(&inventory)).unwrap(),
                "meta": meta_value,
            })
        );

        let items = crate::search::collect_inventory_items(&inventory, Some(Category::Suits));
        let envelopes: Vec<_> = items.into_iter().map(|v| v.envelope).collect();
        let filter = InventoryFilterResponse {
            total: 48,
            filtered: envelopes.len(),
            offset: 0,
            limit: 10,
            items: envelopes,
            meta,
        };
        let value = serde_json::to_value(&filter).unwrap();
        assert_eq!(
            value,
            json!({
                "total": 48,
                "filtered": value["filtered"],
                "offset": 0,
                "limit": 10,
                "items": value["items"],
                "meta": meta_value,
            })
        );
        assert!(value["items"].as_array().is_some_and(|a| !a.is_empty()));
        assert_eq!(value["items"][0]["category"], "suits");
    }

    #[test]
    fn inventory_summary_matches_legacy_shape() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../wf-inventory/testdata/inventory/sample_inventory.json"
        ));
        let inventory: Inventory = serde_json::from_str(raw).unwrap();
        let summary = inventory_summary(&inventory);
        assert_eq!(
            serde_json::to_value(&summary).unwrap(),
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
        );
    }
}
