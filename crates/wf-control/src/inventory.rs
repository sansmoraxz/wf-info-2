use std::convert::Infallible;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
#[cfg(feature = "memory")]
use std::time::Duration;

use chrono::{DateTime, TimeZone as _, Utc};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use tantivy::Term;
use tantivy::query::{Occur, Query, QueryParser, QueryParserError, TermQuery};
use tantivy::schema::IndexRecordOption;
use tokio::fs;

use wf_core::storage;
use wf_inventory::Inventory;
use wf_itemdata::item_data::ItemIndex;

use super::events::{
    DaemonEvent, EventBus, InventoryFetchedEvent, InventoryStaleEvent, InventorySummary, Source,
};
use super::market::{MarketCache, fetch_market_summary};
use super::requests::{ControlError, HandleOp, Handles};
use super::search::{
    Category, EnvelopeAccess as _, IndexedInventory, InventoryIndexCache, InventoryItemEnvelope,
    SearchError, count_inventory_items, search_inventory,
};
use wf_itemdata::traits::Item as _;

#[cfg(feature = "memory")]
use wf_core::{inventory_refresh, process};

#[derive(Debug, thiserror::Error)]
pub(super) enum InventoryError {
    #[error("inventory.load expects exactly one of 'path', 'json', or 'raw'")]
    AmbiguousSource,
    #[error("Failed to read inventory file {}", path.display())]
    ReadFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("Failed to parse inventory JSON")]
    ParseJson(#[source] serde_json::Error),
    #[error("Unknown category '{0}'")]
    UnknownCategory(String),
    #[error("Unsupported timestamp string format")]
    UnsupportedTimestamp,
    #[cfg(not(feature = "memory"))]
    #[error("inventory.refresh requires the 'memory' feature to be enabled")]
    MemoryFeatureDisabled,
    #[cfg(feature = "memory")]
    #[error("Warframe process not detected; launch the game and try again")]
    ProcessNotDetected,
    #[cfg(feature = "memory")]
    #[error("Could not locate auth data in Warframe memory")]
    AuthNotFound,
    #[cfg(feature = "memory")]
    #[error(transparent)]
    Refresh(#[from] inventory_refresh::RefreshError),
    #[error(transparent)]
    Storage(#[from] storage::StorageError),
    #[error(transparent)]
    Search(#[from] SearchError),
    #[error(transparent)]
    Tantivy(#[from] tantivy::TantivyError),
    #[error(transparent)]
    QueryParser(#[from] QueryParserError),
}

/// Wire mirror for inventory.load params; converted to [`LoadInventoryRequest`]
/// so the exactly-one-source rule is enforced before the handler runs.
#[derive(Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct LoadInventoryParams {
    /// Path to inventory JSON file
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    /// JSON value to load, kept unparsed until it deserializes directly into
    /// [`Inventory`] — the (potentially huge) payload is never materialized
    /// as a `serde_json::Value` tree.
    #[cfg_attr(feature = "cli", arg(long, value_parser = crate::utils::parse_json_value))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<Box<RawValue>>,
    /// Raw JSON string
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    /// Save inventory to disk
    #[cfg_attr(
        feature = "cli",
        arg(long, num_args = 0..=1, default_missing_value = "true")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<bool>,
    /// Source identifier
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Treat the file as AES-128-CBC encrypted
    #[cfg_attr(
        feature = "cli",
        arg(long, num_args = 0..=1, default_missing_value = "true")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
}

/// Exactly one place an inventory can be loaded from. `encrypted` only makes
/// sense for file reads, so it lives inside `Path`.
#[derive(Debug)]
pub(super) enum InventoryInput {
    Path { path: PathBuf, encrypted: bool },
    Json(Box<RawValue>),
    Raw(String),
}

impl InventoryInput {
    async fn load(self) -> Result<Inventory, InventoryError> {
        match self {
            Self::Path { path, encrypted } => {
                if encrypted {
                    let data =
                        fs::read(&path)
                            .await
                            .map_err(|source| InventoryError::ReadFile {
                                path: path.clone(),
                                source,
                            })?;
                    Ok(storage::decrypt_inventory_bytes(&data)?)
                } else {
                    let raw = fs::read_to_string(&path).await.map_err(|source| {
                        InventoryError::ReadFile {
                            path: path.clone(),
                            source,
                        }
                    })?;
                    serde_json::from_str(&raw).map_err(InventoryError::ParseJson)
                }
            }
            Self::Raw(raw) => serde_json::from_str(&raw).map_err(InventoryError::ParseJson),
            Self::Json(json) => serde_json::from_str(json.get()).map_err(InventoryError::ParseJson),
        }
    }
}

#[derive(Debug)]
pub(super) struct LoadInventoryRequest {
    pub input: InventoryInput,
    pub save: bool,
    pub source: Source,
}

impl TryFrom<LoadInventoryParams> for LoadInventoryRequest {
    type Error = InventoryError;

    fn try_from(params: LoadInventoryParams) -> Result<Self, Self::Error> {
        let encrypted = params.encrypted.unwrap_or(false);
        let input = match (params.path, params.json, params.raw) {
            (Some(path), None, None) => InventoryInput::Path { path, encrypted },
            (None, Some(json), None) => InventoryInput::Json(json),
            (None, None, Some(raw)) => InventoryInput::Raw(raw),
            _ => return Err(InventoryError::AmbiguousSource),
        };
        Ok(Self {
            input,
            save: params.save.unwrap_or(true),
            source: params.source.unwrap_or_default(),
        })
    }
}

#[derive(Debug, Serialize)]
pub(super) struct InventoryLoadResponse {
    pub saved: bool,
    pub summary: InventorySummary,
    pub meta: storage::InventoryMeta,
}

#[derive(Debug, Serialize)]
pub(super) struct InventoryFilterResponse {
    pub total: usize,
    pub filtered: usize,
    pub offset: usize,
    pub limit: usize,
    pub items: Vec<InventoryItemEnvelope>,
    pub meta: storage::InventoryMeta,
}

impl HandleOp for LoadInventoryParams {
    type Response = InventoryLoadResponse;

    async fn handle(self, cx: &Handles) -> Result<Self::Response, ControlError> {
        Ok(handle_inventory_load(&cx.events, self).await?)
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct FilterParams {
    /// Filter by category
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Filter by item type
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    /// Filter items containing text
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    /// Filter by tradability
    #[cfg_attr(
        feature = "cli",
        arg(long, num_args = 0..=1, default_missing_value = "true")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradable: Option<bool>,
    /// Filter by item count, e.g. `gt:5`, `eq:1`
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<CountFilter>,
    /// Limit number of results
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset for pagination
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Include detailed item information
    #[cfg_attr(
        feature = "cli",
        arg(long, num_args = 0..=1, default_missing_value = "true")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_details: Option<bool>,
    /// Include warframe.market price data
    #[cfg_attr(
        feature = "cli",
        arg(long, num_args = 0..=1, default_missing_value = "true")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_market: Option<bool>,
    /// Path to inventory JSON file
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    /// Treat the file as AES-128-CBC encrypted
    #[cfg_attr(
        feature = "cli",
        arg(long, num_args = 0..=1, default_missing_value = "true")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, strum::EnumString, strum::Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CountOp {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Ne,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct CountFilter {
    pub op: CountOp,
    pub value: i64,
}

/// CLI shorthand: `op:value`, e.g. `gt:5`.
impl FromStr for CountFilter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, value) = s
            .split_once(':')
            .ok_or_else(|| format!("expected `op:value`, got `{s}`"))?;
        Ok(Self {
            op: op.parse().map_err(|_| format!("unknown count op `{op}`"))?,
            value: value
                .parse()
                .map_err(|_| format!("invalid count value `{value}`"))?,
        })
    }
}

impl HandleOp for FilterParams {
    type Response = InventoryFilterResponse;

    async fn handle(self, cx: &Handles) -> Result<Self::Response, ControlError> {
        Ok(handle_inventory_filter(&cx.inventory_index, &cx.market, &cx.item_index, self).await?)
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[cfg_attr(
    not(feature = "memory"),
    allow(
        dead_code,
        reason = "fields are only read by the memory-feature refresh handler"
    )
)]
pub struct RefreshParams {
    /// Number of scan retries
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_retries: Option<u32>,
    /// Delay between scans in milliseconds
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_delay_ms: Option<u64>,
    /// Save inventory to disk after refresh
    #[cfg_attr(
        feature = "cli",
        arg(long, num_args = 0..=1, default_missing_value = "true")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<bool>,
    /// Source identifier
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

impl HandleOp for RefreshParams {
    type Response = InventoryLoadResponse;

    #[cfg(feature = "memory")]
    async fn handle(self, cx: &Handles) -> Result<Self::Response, ControlError> {
        Ok(handle_inventory_refresh(&cx.http, &cx.events, self).await?)
    }

    #[cfg(not(feature = "memory"))]
    async fn handle(self, _cx: &Handles) -> Result<Self::Response, ControlError> {
        Err(InventoryError::MemoryFeatureDisabled.into())
    }
}

/// Timestamp accepted as a numeric epoch (seconds or milliseconds) or a
/// string holding RFC3339 or a stringified epoch.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TimestampParam {
    Epoch(i64),
    Text(String),
}

impl FromStr for TimestampParam {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<i64>()
            .map_or_else(|_| Self::Text(s.to_owned()), Self::Epoch))
    }
}

impl TimestampParam {
    fn to_datetime(&self) -> Result<DateTime<Utc>, InventoryError> {
        match self {
            Self::Text(s) => {
                if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
                    return Ok(dt.with_timezone(&Utc));
                }
                if let Ok(num) = s.parse::<i64>() {
                    return Ok(epoch_to_datetime(num));
                }
                Err(InventoryError::UnsupportedTimestamp)
            }
            Self::Epoch(num) => Ok(epoch_to_datetime(*num)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct StaleParams {
    /// Timestamp for stale marker (epoch seconds/millis or RFC3339)
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimestampParam>,
    /// Reason for marking stale
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl HandleOp for StaleParams {
    type Response = storage::InventoryMeta;

    async fn handle(self, cx: &Handles) -> Result<Self::Response, ControlError> {
        Ok(handle_inventory_stale_update(&cx.events, self)?)
    }
}

pub(super) async fn handle_inventory_load(
    events: &EventBus,
    params: LoadInventoryParams,
) -> Result<InventoryLoadResponse, InventoryError> {
    let LoadInventoryRequest {
        input,
        save,
        source,
    } = LoadInventoryRequest::try_from(params)?;
    let inventory = input.load().await?;
    let summary = inventory_summary(&inventory);

    if save {
        storage::save_inventory(&inventory)?;
        if let Err(e) = storage::touch_inventory_updated(Some(&source.to_string())) {
            log::warn!("Failed to update inventory meta timestamp: {e}");
        }

        // Emit inventory fetched event
        events.emit(DaemonEvent::InventoryFetched(InventoryFetchedEvent {
            timestamp: Utc::now(),
            source,
            summary: summary.clone(),
        }));
    }

    let meta = storage::read_inventory_meta().unwrap_or_default();

    Ok(InventoryLoadResponse {
        saved: save,
        summary,
        meta,
    })
}

pub(super) async fn handle_inventory_filter(
    index: &InventoryIndexCache,
    market: &MarketCache,
    item_index: &ItemIndex,
    mut params: FilterParams,
) -> Result<InventoryFilterResponse, InventoryError> {
    let custom_path = params.path.take().map(|path| InventoryInput::Path {
        path,
        encrypted: params.encrypted.unwrap_or(false),
    });

    let category = match params.category.as_deref() {
        None => None,
        Some(raw) if raw.eq_ignore_ascii_case("all") => None,
        Some(raw) => Some(
            raw.parse::<Category>()
                .map_err(|_| InventoryError::UnknownCategory(raw.to_owned()))?,
        ),
    };
    let include_details = params.include_details.unwrap_or(false);

    let meta = storage::read_inventory_meta().unwrap_or_default();

    // Inventory and its index travel together: a custom path gets a fresh
    // uncached pair; the stored inventory reuses the cached pair.
    let indexed = match custom_path {
        Some(input) => Arc::new(IndexedInventory::build(
            input.load().await?,
            None,
            item_index,
        )?),
        None => index.get_or_build(&meta, item_index)?,
    };
    let search_index = &indexed.index;

    // Count items in selected category for reporting
    let total = count_inventory_items(&indexed.inventory, category);

    let mut clauses: Vec<(Occur, Box<dyn Query>)> = Vec::new();

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

    let (_total_matches, envelopes) = search_inventory(search_index, clauses)?;
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(usize::MAX);

    // Apply non-indexable filters and optional detail expansion
    let mut filtered_items = Vec::new();
    for mut envelope in envelopes {
        if let Some(tradable) = params.tradable {
            let details =
                item_index.lookup(envelope.item_type(), Some(envelope.category().as_ref()));
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
            && let Some(info) =
                item_index.lookup(envelope.item_type(), Some(envelope.category().as_ref()))
        {
            envelope.set_details(Arc::clone(&info.details));
        }

        filtered_items.push(envelope);
    }

    let include_market = params.include_market.unwrap_or(false);
    if include_market {
        for envelope in &mut filtered_items {
            if let Some(summary) = fetch_market_summary(market, &envelope.item_type().into()).await
            {
                envelope.set_market(summary);
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

pub(super) fn handle_inventory_meta_get() -> storage::InventoryMeta {
    storage::read_inventory_meta().unwrap_or_default()
}

#[cfg(feature = "memory")]
pub(crate) async fn handle_inventory_refresh(
    client: &reqwest::Client,
    events: &EventBus,
    params: RefreshParams,
) -> Result<InventoryLoadResponse, InventoryError> {
    let pid = process::get_warframe_pid().ok_or(InventoryError::ProcessNotDetected)?;

    let scan_retries = params.scan_retries.unwrap_or(5);
    let scan_delay = Duration::from_millis(params.scan_delay_ms.unwrap_or(1500));

    let inventory =
        inventory_refresh::fetch_inventory_from_process(client, pid, scan_retries, scan_delay)
            .await?
            .ok_or(InventoryError::AuthNotFound)?
            .inventory;

    let save = params.save.unwrap_or(true);
    let source = params.source.unwrap_or(Source::LiveRefresh);
    let summary = inventory_summary(&inventory);
    if save {
        storage::save_inventory(&inventory)?;
        if let Err(e) = storage::touch_inventory_updated(Some(&source.to_string())) {
            log::warn!("Failed to update inventory meta timestamp: {e}");
        }

        // Emit inventory fetched event
        events.emit(DaemonEvent::InventoryFetched(InventoryFetchedEvent {
            timestamp: Utc::now(),
            source,
            summary: summary.clone(),
        }));
    }

    let meta = storage::read_inventory_meta().unwrap_or_default();

    Ok(InventoryLoadResponse {
        saved: save,
        summary,
        meta,
    })
}

pub(super) fn handle_inventory_stale_update(
    events: &EventBus,
    params: StaleParams,
) -> Result<storage::InventoryMeta, InventoryError> {
    let timestamp = if let Some(value) = params.timestamp {
        value.to_datetime()?
    } else {
        Utc::now()
    };

    let meta = storage::mark_inventory_stale_at(timestamp, params.reason.clone())?;

    events.emit(DaemonEvent::InventoryStale(InventoryStaleEvent {
        timestamp: Utc::now(),
        stale_since: timestamp,
        reason: params.reason,
    }));

    Ok(meta)
}

pub(super) fn inventory_summary(inventory: &Inventory) -> InventorySummary {
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
    let result = if value > 1_000_000_000_000 {
        Utc.timestamp_millis_opt(value)
    } else {
        Utc.timestamp_opt(value, 0)
    };
    result.single().unwrap_or_else(Utc::now)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::collect_inventory_items;
    use serde_json::json;

    #[test]
    fn load_request_requires_exactly_one_source() {
        let none = LoadInventoryRequest::try_from(LoadInventoryParams::default());
        none.unwrap_err();

        let two = LoadInventoryRequest::try_from(LoadInventoryParams {
            path: Some("a.json".into()),
            raw: Some("{}".into()),
            ..Default::default()
        });
        two.unwrap_err();

        let one = LoadInventoryRequest::try_from(LoadInventoryParams {
            path: Some("a.json".into()),
            encrypted: Some(true),
            ..Default::default()
        })
        .unwrap();
        assert!(matches!(
            one.input,
            InventoryInput::Path {
                encrypted: true,
                ..
            }
        ));
    }

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

        let item_index = ItemIndex::default();
        let items = collect_inventory_items(&inventory, Some(Category::Suits), &item_index);
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
                "total": 48_i64,
                "filtered": value["filtered"],
                "offset": 0_i64,
                "limit": 10_i64,
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
