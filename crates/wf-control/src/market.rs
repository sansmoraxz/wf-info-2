use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use wf_core::storage;
use wf_inventory::Inventory;

use super::state::AppState;
use super::utils::WFM_API_BASE;
use wf_itemdata::item_data::lookup_item_info;

const CACHE_TTL: Duration = Duration::from_secs(3600); // 1 hour

// ── Domain newtypes ──

/// In-game item path like `/Lotus/Types/...`. A string on the wire, but must
/// never be interchanged with [`WfmId`]: each indexes a different cache map.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Display,
    derive_more::From,
    derive_more::AsRef,
)]
#[serde(transparent)]
#[from(forward)]
#[as_ref(str)]
pub(crate) struct GameRef(String);

/// warframe.market object id. A string on the wire, but must never be
/// interchanged with [`GameRef`]: each indexes a different cache map.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Deserialize,
    derive_more::Display,
    derive_more::From,
    derive_more::AsRef,
)]
#[serde(transparent)]
#[as_ref(str)]
pub(crate) struct WfmId(String);

// ── Types for WFM API responses ──

#[derive(Debug, Clone, Deserialize)]
struct WfmItemsResponse {
    data: Vec<WfmItemRaw>,
}

#[derive(Debug, Clone, Deserialize)]
struct WfmItemRaw {
    id: WfmId,
    slug: String,
    #[serde(rename = "gameRef")]
    game_ref: Option<GameRef>,
    #[serde(default)]
    tags: Vec<String>,
    ducats: Option<i64>,
    i18n: Option<WfmI18n>,
}

#[derive(Debug, Clone, Deserialize)]
struct WfmI18n {
    en: Option<WfmLocale>,
}

#[derive(Debug, Clone, Deserialize)]
struct WfmLocale {
    name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct WfmOrdersResponse {
    data: Vec<WfmOrder>,
}

#[derive(Debug, Clone, Deserialize)]
struct WfmOrder {
    #[serde(rename = "type")]
    order_type: String,
    platinum: f64,
    #[allow(dead_code)]
    quantity: i64,
    user: WfmOrderUser,
    visible: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
struct WfmOrderUser {
    status: Option<String>,
    #[serde(rename = "ingameName")]
    #[allow(dead_code)]
    ingame_name: Option<String>,
}

// ── Cached item data ──

#[derive(Debug, Clone)]
pub(crate) struct WfmItem {
    pub id: WfmId,
    pub slug: String,
    pub game_ref: Option<GameRef>,
    pub name: String,
    pub tags: Vec<String>,
    pub ducats: Option<i64>,
}

pub(crate) struct WfmCache {
    // Positions into `items`; the cache is only ever replaced wholesale,
    // so indices stay valid for its lifetime.
    game_ref_index: HashMap<GameRef, usize>,
    id_index: HashMap<WfmId, usize>,
    items: Vec<WfmItem>,
    last_refreshed_at: DateTime<Utc>,
    #[allow(dead_code)]
    item_count: usize,
}

fn is_cache_stale(state: &AppState) -> bool {
    let guard = state
        .market_cache
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    match guard.as_ref() {
        None => true,
        Some(cache) => {
            Utc::now().signed_duration_since(cache.last_refreshed_at)
                > chrono::Duration::from_std(CACHE_TTL).unwrap_or(chrono::Duration::hours(1))
        }
    }
}

fn cache_age_secs(state: &AppState) -> Option<i64> {
    let guard = state
        .market_cache
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    guard.as_ref().map(|c| {
        Utc::now()
            .signed_duration_since(c.last_refreshed_at)
            .num_seconds()
    })
}

async fn ensure_cache(state: &AppState) -> Result<()> {
    if is_cache_stale(state) {
        refresh_cache(state).await?;
    }
    Ok(())
}

async fn refresh_cache(state: &AppState) -> Result<(usize, DateTime<Utc>)> {
    let url = format!("{}/items", WFM_API_BASE);
    let client = reqwest::Client::new();
    let resp: WfmItemsResponse = client.get(&url).send().await?.json().await?;

    let mut game_ref_index = HashMap::new();
    let mut id_index = HashMap::new();
    let mut items = Vec::with_capacity(resp.data.len());

    for raw in resp.data {
        let name = raw
            .i18n
            .as_ref()
            .and_then(|i| i.en.as_ref())
            .and_then(|l| l.name.clone())
            .unwrap_or_else(|| raw.slug.replace('_', " "));

        let item = WfmItem {
            id: raw.id,
            slug: raw.slug,
            game_ref: raw.game_ref,
            name,
            tags: raw.tags,
            ducats: raw.ducats,
        };

        let pos = items.len();
        if let Some(ref gr) = item.game_ref {
            game_ref_index.insert(gr.clone(), pos);
        }
        id_index.insert(item.id.clone(), pos);
        items.push(item);
    }

    let now = Utc::now();
    let count = items.len();

    // Poisoning is recoverable: the cache is only ever replaced wholesale.
    let mut guard = state
        .market_cache
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    *guard = Some(WfmCache {
        game_ref_index,
        id_index,
        items,
        last_refreshed_at: now,
        item_count: count,
    });

    Ok((count, now))
}

// ── Cache lookups ──

fn lookup_by_game_ref(state: &AppState, game_ref: &GameRef) -> Option<WfmItem> {
    let guard = state
        .market_cache
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    guard
        .as_ref()
        .and_then(|c| Some(c.items[*c.game_ref_index.get(game_ref)?].clone()))
}

fn lookup_by_id(state: &AppState, id: &WfmId) -> Option<WfmItem> {
    let guard = state
        .market_cache
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    guard
        .as_ref()
        .and_then(|c| Some(c.items[*c.id_index.get(id)?].clone()))
}

fn search_items(state: &AppState, query: &str) -> Vec<WfmItem> {
    let guard = state
        .market_cache
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(cache) = guard.as_ref() else {
        return Vec::new();
    };

    let query_lower = query.to_lowercase();
    let query_set = format!("{} set", query_lower);
    let query_slug = query_lower.replace(' ', "_");
    let mut results: Vec<(usize, &WfmItem)> = cache
        .items
        .iter()
        .filter_map(|item| {
            let name_lower = item.name.to_lowercase();
            if name_lower == query_lower || name_lower == query_set {
                Some((0, item))
            } else if name_lower.starts_with(&query_lower) {
                Some((1, item))
            } else if name_lower.contains(&query_lower) {
                Some((2, item))
            } else if item.slug.contains(&query_slug) {
                Some((3, item))
            } else {
                None
            }
        })
        .collect();

    // Sort by: score first, then prefer set items (tagged "set") over components
    let is_set = |item: &WfmItem| item.tags.iter().any(|t| t == "set");
    results.sort_by(|(score_a, item_a), (score_b, item_b)| {
        score_a
            .cmp(score_b)
            .then_with(|| is_set(item_b).cmp(&is_set(item_a)))
    });
    results.into_iter().map(|(_, item)| item.clone()).collect()
}

// ── Item detail fetching (for set parts) ──

#[derive(Debug, Clone, Deserialize)]
struct WfmItemDetailResponse {
    data: WfmItemDetail,
}

#[derive(Debug, Clone, Deserialize)]
struct WfmItemDetail {
    #[serde(rename = "setParts")]
    set_parts: Option<Vec<WfmId>>,
}

async fn fetch_item_detail(slug: &str) -> Result<WfmItemDetail> {
    let url = format!("{}/items/{}", WFM_API_BASE, slug);
    let client = reqwest::Client::new();
    let resp: WfmItemDetailResponse = client.get(&url).send().await?.json().await?;
    Ok(resp.data)
}

// ── Order fetching ──

async fn fetch_orders(slug: &str) -> Result<Vec<WfmOrder>> {
    let url = format!("{}/orders/item/{}", WFM_API_BASE, slug);
    let client = reqwest::Client::new();
    let resp: WfmOrdersResponse = client.get(&url).send().await?.json().await?;
    Ok(resp.data)
}

// ── Typed order/price summaries ──

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct PriceStats {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub median: Option<f64>,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct OrderSummary {
    pub sell: PriceStats,
    pub buy: PriceStats,
    pub total_listings: usize,
}

/// Market price summary for a single item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct MarketSummary {
    pub slug: String,
    pub ducats: Option<i64>,
    pub prices: OrderSummary,
    pub cache_age_secs: Option<i64>,
}

fn summarize_orders(orders: &[WfmOrder]) -> OrderSummary {
    let sell_prices: Vec<f64> = orders
        .iter()
        .filter(|o| {
            o.order_type == "sell"
                && o.visible.unwrap_or(true)
                && o.user.status.as_deref() == Some("ingame")
        })
        .map(|o| o.platinum)
        .collect();

    let buy_prices: Vec<f64> = orders
        .iter()
        .filter(|o| {
            o.order_type == "buy"
                && o.visible.unwrap_or(true)
                && o.user.status.as_deref() == Some("ingame")
        })
        .map(|o| o.platinum)
        .collect();

    OrderSummary {
        sell: price_stats(&sell_prices),
        buy: price_stats(&buy_prices),
        total_listings: orders.len(),
    }
}

fn price_stats(prices: &[f64]) -> PriceStats {
    if prices.is_empty() {
        return PriceStats {
            min: None,
            max: None,
            median: None,
            count: 0,
        };
    }

    let mut sorted = prices.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let min = sorted.first().copied();
    let max = sorted.last().copied();
    let median = if sorted.len().is_multiple_of(2) {
        let mid = sorted.len() / 2;
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[sorted.len() / 2])
    };

    PriceStats {
        min,
        max,
        median,
        count: sorted.len(),
    }
}

// ── Inventory count lookup ──

pub(crate) fn count_in_inventory(inventory: &Inventory, item_type: &str) -> i64 {
    // Equipment categories: each entry is 1 owned
    macro_rules! count_vec {
        ($field:expr) => {
            $field.iter().filter(|i| i.item_type == item_type).count() as i64
        };
    }
    macro_rules! count_opt_vec {
        ($field:expr) => {
            $field
                .as_ref()
                .map(|v| v.iter().filter(|i| i.item_type == item_type).count() as i64)
                .unwrap_or(0)
        };
    }

    let equip_count = count_vec!(inventory.suits)
        + count_vec!(inventory.long_guns)
        + count_vec!(inventory.pistols)
        + count_vec!(inventory.melee)
        + count_vec!(inventory.space_suits)
        + count_vec!(inventory.space_guns)
        + count_vec!(inventory.space_melee)
        + count_opt_vec!(inventory.mech_suits)
        + count_opt_vec!(inventory.sentinels)
        + count_opt_vec!(inventory.sentinel_weapons)
        + count_opt_vec!(inventory.operator_amps)
        + count_opt_vec!(inventory.hoverboards)
        + count_opt_vec!(inventory.horses)
        + count_opt_vec!(inventory.motorcycles)
        + count_opt_vec!(inventory.crew_ships)
        + count_opt_vec!(inventory.crew_ship_weapons)
        + count_opt_vec!(inventory.drifter_melee);

    if equip_count > 0 {
        return equip_count;
    }

    // Recipes (blueprints/components) have ItemCount
    for recipe in &inventory.recipes {
        if recipe.item_type == item_type {
            return recipe.item_count;
        }
    }

    // Raw upgrades (unranked mods/arcanes)
    for upgrade in &inventory.raw_upgrades {
        if upgrade.item_type == item_type {
            return upgrade.item_count;
        }
    }

    // Ranked upgrades
    for upgrade in &inventory.upgrades {
        if upgrade.item_type == item_type {
            return 1;
        }
    }

    // Misc items / consumables
    if let Some(ref misc) = inventory.misc_items {
        for item in misc {
            if item.item_type == item_type {
                return item.item_count;
            }
        }
    }

    0
}

// ── Handlers ──

#[derive(Debug, Deserialize, Default)]
pub(crate) struct MarketPriceParams {
    pub item_type: Option<String>,
    pub search: Option<String>,
    pub include_parts: Option<bool>,
}

#[derive(Debug, Serialize)]
pub(crate) struct MarketItemInfo {
    pub name: String,
    pub slug: String,
    pub game_ref: Option<GameRef>,
    pub ducats: Option<i64>,
    pub tags: Vec<String>,
    pub is_set: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct OwnedCount {
    pub owned: Option<i64>,
}

#[derive(Debug, Serialize)]
pub(crate) struct SetPartInfo {
    pub name: String,
    pub slug: String,
    pub game_ref: Option<GameRef>,
    pub ducats: Option<i64>,
    pub prices: OrderSummary,
    pub inventory: OwnedCount,
}

#[derive(Debug, Serialize)]
pub(crate) struct MarketPriceResponse {
    pub item: MarketItemInfo,
    pub prices: OrderSummary,
    pub inventory: OwnedCount,
    pub cache_age_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<wf_itemdata::item_data::ItemDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_parts: Option<Vec<SetPartInfo>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct MarketRefreshResponse {
    pub items_count: usize,
    pub refreshed_at: String,
}

pub(crate) async fn handle_market_price(
    state: &AppState,
    params: MarketPriceParams,
) -> Result<MarketPriceResponse> {
    if params.item_type.is_none() && params.search.is_none() {
        return Err(anyhow!(
            "wfm.price requires 'item_type' or 'search' parameter"
        ));
    }

    ensure_cache(state).await?;

    // Resolve the WFM item
    let wfm_item = if let Some(ref item_type) = params.item_type {
        // Try direct gameRef lookup
        let direct = lookup_by_game_ref(state, &GameRef::from(item_type.as_str()));
        if direct.is_some() {
            direct
        } else {
            // Fallback: search by name
            search_items(state, item_type).into_iter().next()
        }
    } else if let Some(ref query) = params.search {
        search_items(state, query).into_iter().next()
    } else {
        None
    };

    let wfm_item = wfm_item.ok_or_else(|| anyhow!("Item not found on warframe.market"))?;

    // Fetch orders
    let orders = fetch_orders(&wfm_item.slug).await?;
    let prices = summarize_orders(&orders);

    // Inventory count (graceful)
    let inventory = storage::read_inventory().ok();
    let owned = inventory.as_ref().and_then(|inv| {
        wfm_item
            .game_ref
            .as_ref()
            .map(|gr| count_in_inventory(inv, gr.as_ref()))
    });

    // Item details from itemdata
    let details = wfm_item
        .game_ref
        .as_ref()
        .and_then(|gr| lookup_item_info(gr.as_ref(), None))
        .map(|info| info.details);

    // Set parts: detect set items by "set" tag, then fetch detail for setParts
    let include_parts = params.include_parts.unwrap_or(true);
    let is_set = wfm_item.tags.contains(&"set".to_string());
    let set_parts = if include_parts && is_set {
        match fetch_item_detail(&wfm_item.slug).await {
            Ok(detail) => {
                if let Some(ref part_ids) = detail.set_parts {
                    let mut parts = Vec::new();
                    for part_id in part_ids {
                        // Skip the set item itself
                        if *part_id == wfm_item.id {
                            continue;
                        }
                        if let Some(part) = lookup_by_id(state, part_id) {
                            let part_orders = fetch_orders(&part.slug).await?;
                            let part_prices = summarize_orders(&part_orders);

                            let part_owned = inventory.as_ref().and_then(|inv| {
                                part.game_ref
                                    .as_ref()
                                    .map(|gr| count_in_inventory(inv, gr.as_ref()))
                            });

                            parts.push(SetPartInfo {
                                name: part.name,
                                slug: part.slug,
                                game_ref: part.game_ref,
                                ducats: part.ducats,
                                prices: part_prices,
                                inventory: OwnedCount { owned: part_owned },
                            });
                        }
                    }
                    Some(parts)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    } else {
        None
    };

    Ok(MarketPriceResponse {
        item: MarketItemInfo {
            name: wfm_item.name,
            slug: wfm_item.slug,
            game_ref: wfm_item.game_ref,
            ducats: wfm_item.ducats,
            tags: wfm_item.tags,
            is_set,
        },
        prices,
        inventory: OwnedCount { owned },
        cache_age_secs: cache_age_secs(state),
        details,
        set_parts,
    })
}

pub(crate) async fn handle_market_refresh(state: &AppState) -> Result<MarketRefreshResponse> {
    let (count, refreshed_at) = refresh_cache(state).await?;

    Ok(MarketRefreshResponse {
        items_count: count,
        refreshed_at: refreshed_at.to_rfc3339(),
    })
}

/// Fetch market price summary for a single item by game_ref.
/// Used by inventory-filter enrichment.
pub(crate) async fn fetch_market_summary(
    state: &AppState,
    game_ref: &GameRef,
) -> Option<MarketSummary> {
    if ensure_cache(state).await.is_err() {
        return None;
    }

    let wfm_item = lookup_by_game_ref(state, game_ref)?;
    let orders = fetch_orders(&wfm_item.slug).await.ok()?;
    let prices = summarize_orders(&orders);

    Some(MarketSummary {
        slug: wfm_item.slug,
        ducats: wfm_item.ducats,
        prices,
        cache_age_secs: cache_age_secs(state),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn price_stats_matches_legacy_shape() {
        let empty = price_stats(&[]);
        assert_eq!(
            serde_json::to_value(&empty).unwrap(),
            json!({ "min": null, "max": null, "median": null, "count": 0 })
        );

        let stats = price_stats(&[10.0, 20.0, 30.0, 40.0]);
        assert_eq!(
            serde_json::to_value(&stats).unwrap(),
            json!({ "min": 10.0, "max": 40.0, "median": 25.0, "count": 4 })
        );
    }

    #[test]
    fn market_summary_matches_legacy_shape() {
        let summary = MarketSummary {
            slug: "harrow_prime_set".to_string(),
            ducats: Some(100),
            prices: OrderSummary {
                sell: price_stats(&[15.0]),
                buy: price_stats(&[]),
                total_listings: 1,
            },
            cache_age_secs: Some(42),
        };
        assert_eq!(
            serde_json::to_value(&summary).unwrap(),
            json!({
                "slug": "harrow_prime_set",
                "ducats": 100,
                "prices": {
                    "sell": { "min": 15.0, "max": 15.0, "median": 15.0, "count": 1 },
                    "buy": { "min": null, "max": null, "median": null, "count": 0 },
                    "total_listings": 1,
                },
                "cache_age_secs": 42,
            })
        );
    }

    #[test]
    fn market_price_response_omits_absent_optionals() {
        let response = MarketPriceResponse {
            item: MarketItemInfo {
                name: "Harrow Prime Set".to_string(),
                slug: "harrow_prime_set".to_string(),
                game_ref: None,
                ducats: None,
                tags: vec!["set".to_string()],
                is_set: true,
            },
            prices: OrderSummary {
                sell: price_stats(&[]),
                buy: price_stats(&[]),
                total_listings: 0,
            },
            inventory: OwnedCount { owned: None },
            cache_age_secs: None,
            details: None,
            set_parts: None,
        };
        let value = serde_json::to_value(&response).unwrap();
        assert_eq!(
            value,
            json!({
                "item": {
                    "name": "Harrow Prime Set",
                    "slug": "harrow_prime_set",
                    "game_ref": null,
                    "ducats": null,
                    "tags": ["set"],
                    "is_set": true,
                },
                "prices": {
                    "sell": { "min": null, "max": null, "median": null, "count": 0 },
                    "buy": { "min": null, "max": null, "median": null, "count": 0 },
                    "total_listings": 0,
                },
                "inventory": { "owned": null },
                "cache_age_secs": null,
            })
        );
        assert!(value.get("details").is_none());
        assert!(value.get("set_parts").is_none());
    }
}
