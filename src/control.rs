use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};
use std::time::Duration;
use tantivy::collector::{Count, TopDocs};
use tantivy::doc;
use tantivy::query::{AllQuery, BooleanQuery, Occur, QueryParser, TermQuery};
use tantivy::schema::{
    Field, IndexRecordOption, STORED, STRING, SchemaBuilder, TextFieldIndexing, TextOptions,
};
use tantivy::tokenizer::NgramTokenizer;
use tantivy::{Index, Term};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use crate::control_ops::ControlOp;
use crate::inventory::Inventory;
use crate::inventory_refresh;
use crate::itemdata;
use crate::itemdata::ProductCategory;
use crate::process;
use crate::storage;

#[cfg(unix)]
use tokio::net::UnixListener;

#[derive(Debug, Clone)]
pub enum ControlEndpoint {
    Tcp(String),
    Unix(PathBuf),
}

#[derive(Debug, Clone)]
pub struct ControlConfig {
    pub endpoints: Vec<ControlEndpoint>,
}

static CURRENT_ACCOUNT_ID: OnceLock<RwLock<Option<String>>> = OnceLock::new();

fn account_id_store() -> &'static RwLock<Option<String>> {
    CURRENT_ACCOUNT_ID.get_or_init(|| RwLock::new(None))
}

pub fn set_current_account(account_id: Option<String>) {
    if let Ok(mut guard) = account_id_store().write() {
        *guard = account_id;
    }
}

fn current_account() -> Option<String> {
    account_id_store().read().ok().and_then(|g| g.clone())
}

impl ControlConfig {
    pub fn from_env() -> Option<Self> {
        let mut endpoints = Vec::new();

        if let Ok(addr) = std::env::var("WF_INFO_API_TCP") {
            endpoints.push(ControlEndpoint::Tcp(addr));
        }

        let unix_socket = std::env::var("WF_INFO_API_UNIX")
            .ok()
            .map(PathBuf::from)
            .or_else(default_unix_socket_path);

        if let Some(path) = unix_socket {
            endpoints.push(ControlEndpoint::Unix(path));
        }

        if endpoints.is_empty() {
            None
        } else {
            Some(Self { endpoints })
        }
    }
}

#[derive(Default)]
pub struct ControlServer {
    pub handles: Vec<JoinHandle<()>>,
    // Keep guards alive for cleanup on drop
    _unix_guards: Vec<UnixSocketGuard>,
}

impl ControlServer {
    pub fn empty() -> Self {
        Self {
            handles: Vec::new(),
            _unix_guards: Vec::new(),
        }
    }
}

pub async fn start_control_server_from_env() -> Result<ControlServer> {
    let Some(cfg) = ControlConfig::from_env() else {
        return Ok(ControlServer::empty());
    };
    start_control_server(cfg).await
}

pub async fn start_control_server(cfg: ControlConfig) -> Result<ControlServer> {
    let mut handles = Vec::new();
    let mut unix_guards = Vec::new();

    for endpoint in cfg.endpoints {
        match endpoint {
            ControlEndpoint::Tcp(addr) => {
                handles.push(spawn_tcp_server(addr).await?);
            }
            ControlEndpoint::Unix(path) => {
                #[cfg(unix)]
                {
                    let (handle, guard) = spawn_unix_server(path).await?;
                    handles.push(handle);
                    unix_guards.push(guard);
                }
                #[cfg(not(unix))]
                {
                    let _ = path;
                    return Err(anyhow!("Unix sockets are not supported on this platform"));
                }
            }
        }
    }

    Ok(ControlServer {
        handles,
        _unix_guards: unix_guards,
    })
}

async fn spawn_tcp_server(addr: String) -> Result<JoinHandle<()>> {
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind TCP control socket at {}", addr))?;
    log::info!("Control API listening on tcp {}", addr);

    Ok(tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(async move {
                        if let Err(e) = handle_stream(stream).await {
                            log::warn!("Control connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Control TCP accept error: {}", e);
                    break;
                }
            }
        }
    }))
}

#[cfg(unix)]
async fn spawn_unix_server(path: PathBuf) -> Result<(JoinHandle<()>, UnixSocketGuard)> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create unix socket dir {}", parent.display()))?;
    }
    if path.exists() {
        fs::remove_file(&path)
            .with_context(|| format!("Failed to remove existing unix socket {}", path.display()))?;
    }

    let listener = UnixListener::bind(&path)
        .with_context(|| format!("Failed to bind unix control socket {}", path.display()))?;
    log::info!("Control API listening on unix {}", path.display());
    let guard = UnixSocketGuard { path: path.clone() };

    let handle = tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(async move {
                        if let Err(e) = handle_stream(stream).await {
                            log::warn!("Control connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Control unix accept error: {}", e);
                    break;
                }
            }
        }
    });

    Ok((handle, guard))
}

async fn handle_stream<T>(stream: T) -> Result<()>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let (reader, mut writer) = tokio::io::split(stream);
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<Request>(line) {
            Ok(req) => handle_request(req).await,
            Err(e) => Response::error(None, format!("Invalid request: {}", e)),
        };

        let payload = serde_json::to_string(&response).context("Failed to serialize response")?;
        writer.write_all(payload.as_bytes()).await?;
        writer.write_all(b"\n").await?;
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Request {
    pub id: Option<Value>,
    pub op: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize)]
struct Response {
    pub id: Option<Value>,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Response {
    fn ok(id: Option<Value>, data: Value) -> Self {
        Self {
            id,
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(id: Option<Value>, message: String) -> Self {
        Self {
            id,
            ok: false,
            data: None,
            error: Some(message),
        }
    }
}

async fn handle_request(req: Request) -> Response {
    let id = req.id.clone();

    let result = match ControlOp::parse(&req.op) {
        Ok(ControlOp::Ping) => Ok(json!({ "pong": true })),
        Ok(ControlOp::InventoryLoad) => handle_inventory_load(req.params).await,
        Ok(ControlOp::InventoryFilter) => handle_inventory_filter(req.params).await,
        Ok(ControlOp::InventoryMetaGet) => handle_inventory_meta_get(),
        Ok(ControlOp::InventoryStaleUpdate) => handle_inventory_stale_update(req.params),
        Ok(ControlOp::ScreenshotTrigger) => handle_screenshot_trigger(req.params),
        Ok(ControlOp::InventoryRefresh) => handle_inventory_refresh(req.params).await,
        Err(e) => Err(e),
    };

    match result {
        Ok(data) => Response::ok(id, data),
        Err(e) => Response::error(id, e.to_string()),
    }
}

#[derive(Debug, Deserialize, Default)]
struct LoadInventoryParams {
    path: Option<String>,
    json: Option<Value>,
    raw: Option<String>,
    save: Option<bool>,
    source: Option<String>,
}

async fn handle_inventory_load(params: Option<Value>) -> Result<Value> {
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
struct FilterParams {
    category: Option<String>,
    item_type: Option<String>,
    contains: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    include_details: Option<bool>,
    path: Option<String>,
}

async fn handle_inventory_filter(params: Option<Value>) -> Result<Value> {
    let params: FilterParams = parse_params(params)?;

    let inventory = if let Some(path) = params.path {
        let raw = tokio::fs::read_to_string(&path)
            .await
            .with_context(|| format!("Failed to read inventory file {}", path))?;
        serde_json::from_str(&raw).context("Failed to parse inventory JSON")?
    } else {
        storage::read_inventory()?
    };

    let category = params.category.as_deref().and_then(normalize_category);
    if matches!(category, Some("unknown")) {
        return Err(anyhow!(
            "Unknown category '{}'",
            params.category.as_deref().unwrap_or_default()
        ));
    }
    let include_details = params.include_details.unwrap_or(false);

    // Build search index (in-RAM) and execute query via tantivy
    let items = collect_inventory_items(&inventory, category, false);
    let total = items.len();

    let search_index = build_tantivy_index(&items)?;
    let reader = search_index.index.reader()?;
    let searcher = reader.searcher();

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

    let query: Box<dyn tantivy::query::Query> = if clauses.is_empty() {
        Box::new(AllQuery)
    } else {
        Box::new(BooleanQuery::new(clauses))
    };

    let filtered = searcher.search(&query, &Count)? as usize;
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(usize::MAX);
    let fetch = offset.saturating_add(limit).min(filtered.max(offset));

    let top_docs = if fetch == 0 {
        Vec::new()
    } else {
        searcher.search(&query, &TopDocs::with_limit(fetch))?
    };

    let mut results = Vec::new();
    for (_score, addr) in top_docs.into_iter().skip(offset) {
        let raw = match searcher
            .doc::<tantivy::TantivyDocument>(addr)?
            .get_first(search_index.raw_json)
        {
            Some(tantivy::schema::OwnedValue::Str(s)) => s.to_string(),
            _ => String::new(),
        };
        let mut value: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);

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

        results.push(value);
        if results.len() >= limit {
            break;
        }
    }

    let meta = storage::read_inventory_meta().unwrap_or_default();

    Ok(json!({
        "total": total,
        "filtered": filtered,
        "offset": offset,
        "limit": limit,
        "items": results,
        "meta": meta,
    }))
}

fn handle_inventory_meta_get() -> Result<Value> {
    let meta = storage::read_inventory_meta().unwrap_or_default();
    Ok(serde_json::to_value(meta).context("Failed to serialize inventory metadata")?)
}

#[derive(Debug, Deserialize, Default)]
struct RefreshParams {
    scan_retries: Option<u32>,
    scan_delay_ms: Option<u64>,
    save: Option<bool>,
    source: Option<String>,
}

async fn handle_inventory_refresh(params: Option<Value>) -> Result<Value> {
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
struct StaleParams {
    timestamp: Option<Value>,
    reason: Option<String>,
}

fn handle_inventory_stale_update(params: Option<Value>) -> Result<Value> {
    let params: StaleParams = parse_params(params)?;
    let timestamp = if let Some(value) = params.timestamp {
        parse_timestamp(value)?
    } else {
        Utc::now()
    };

    let meta = storage::mark_inventory_stale_at(timestamp, params.reason)?;
    Ok(serde_json::to_value(meta).context("Failed to serialize inventory metadata")?)
}

#[derive(Debug, Deserialize, Default)]
struct ScreenshotParams {
    action: Option<String>,
    metadata: Option<Value>,
}

fn handle_screenshot_trigger(params: Option<Value>) -> Result<Value> {
    let params: ScreenshotParams = parse_params(params)?;
    let event = record_screenshot_event(params.action, params.metadata)?;
    Ok(serde_json::to_value(event).context("Failed to serialize screenshot event")?)
}

fn parse_params<T>(params: Option<Value>) -> Result<T>
where
    T: for<'de> Deserialize<'de> + Default,
{
    match params {
        Some(value) => Ok(serde_json::from_value(value).context("Invalid params")?),
        None => Ok(T::default()),
    }
}

fn inventory_summary(inventory: &Inventory) -> Value {
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
fn normalize_category(category: &str) -> Option<&'static str> {
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

struct ItemView {
    item_type: String,
    details_name: Option<String>,
    details_desc: Option<String>,
    value: Value,
}

struct InventorySearchIndex {
    index: Index,
    item_type_exact: Field,
    item_type_text: Field,
    details_name: Field,
    details_desc: Field,
    category: Field,
    raw_json: Field,
}

fn collect_inventory_items(
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

fn build_tantivy_index(items: &[ItemView]) -> Result<InventorySearchIndex> {
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

#[derive(Debug, Clone, Serialize)]
struct ItemInfo {
    name: Option<String>,
    unique_name: String,
    product_category: Option<String>,
    description: Option<String>,
    details: Value,
}

// Maps uniqueName/item_type -> all matching ItemInfo variants (multiple productCategory variants may exist)
static ITEM_INDEX: OnceLock<HashMap<String, Vec<ItemInfo>>> = OnceLock::new();

#[derive(Debug)]
struct UnixSocketGuard {
    path: PathBuf,
}

impl Drop for UnixSocketGuard {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.path) {
            if e.kind() != std::io::ErrorKind::NotFound {
                log::debug!(
                    "Failed to cleanup unix socket {}: {}",
                    self.path.display(),
                    e
                );
            }
        }
    }
}

fn default_unix_socket_path() -> Option<PathBuf> {
    let base = dirs::runtime_dir()
        .or_else(dirs::cache_dir)
        .or_else(|| dirs::data_dir())?;
    Some(base.join("wf-info-2").join("control.sock"))
}

fn category_to_product_category(cat: &str) -> Option<&'static str> {
    match cat {
        "suits" => Some("Suits"),
        "long_guns" => Some("LongGuns"),
        "pistols" => Some("Pistols"),
        "melee" => Some("Melee"),
        "space_suits" => Some("SpaceSuits"),
        "space_guns" => Some("SpaceGuns"),
        "space_melee" => Some("SpaceMelee"),
        "raw_upgrades" => Some("RawUpgrades"),
        "upgrades" => Some("Upgrades"),
        "recipes" | "pending_recipes" => Some("Recipes"),
        _ => None,
    }
}

fn lookup_item_info(item_type: &str, category: Option<&str>) -> Option<ItemInfo> {
    let index = ITEM_INDEX.get_or_init(build_item_index);
    let entries = index.get(item_type)?;
    if let Some(cat) = category.and_then(category_to_product_category) {
        if let Some(found) = entries
            .iter()
            .find(|info| info.product_category.as_deref() == Some(cat))
        {
            return Some(found.clone());
        }
    }
    // fallback: first entry
    entries.first().cloned()
}

fn build_item_index() -> HashMap<String, Vec<ItemInfo>> {
    let mut index: HashMap<String, Vec<ItemInfo>> = HashMap::new();

    let Some(data_dir) = find_itemdata_dir() else {
        log::warn!("Item data directory not found; item details disabled");
        return index;
    };

    let mut push_info = |v: &Value, product_category: Option<String>| {
        let Some(unique_name) = v.get("uniqueName").and_then(Value::as_str) else {
            return;
        };
        let name = v.get("name").and_then(Value::as_str).map(|s| s.to_string());
        let description = v
            .get("description")
            .or_else(|| v.get("desc"))
            .and_then(Value::as_str)
            .map(|s| s.to_string());
        let details = v.clone();
        let info = ItemInfo {
            name,
            unique_name: unique_name.to_string(),
            product_category,
            description,
            details,
        };
        index.entry(unique_name.to_string()).or_default().push(info);
    };

    // Warframes
    if let Ok(raw) = fs::read_to_string(data_dir.join("Warframes.json")) {
        if let Ok(arr) = serde_json::from_str::<itemdata::warframe::Root>(&raw) {
            for item in arr {
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, Some("Suits".to_string()));
            }
        }
    }
    // Primary
    if let Ok(raw) = fs::read_to_string(data_dir.join("Primary.json")) {
        if let Ok(arr) = serde_json::from_str::<itemdata::primary::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.clone());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Secondary
    if let Ok(raw) = fs::read_to_string(data_dir.join("Secondary.json")) {
        if let Ok(arr) = serde_json::from_str::<itemdata::secondary::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.clone());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Melee
    if let Ok(raw) = fs::read_to_string(data_dir.join("Melee.json")) {
        if let Ok(arr) = serde_json::from_str::<itemdata::melee::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.clone());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Archwing suits
    if let Ok(raw) = fs::read_to_string(data_dir.join("Archwing.json")) {
        if let Ok(arr) = serde_json::from_str::<itemdata::archwing::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.clone());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Arch-guns
    if let Ok(raw) = fs::read_to_string(data_dir.join("Arch-Gun.json")) {
        if let Ok(arr) = serde_json::from_str::<itemdata::arch_gun::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.clone());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Arch-melee
    if let Ok(raw) = fs::read_to_string(data_dir.join("Arch-Melee.json")) {
        if let Ok(arr) = serde_json::from_str::<itemdata::arch_melee::Root>(&raw) {
            for item in arr {
                let pc = Some(item.product_category.clone());
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                push_info(&v, pc);
            }
        }
    }
    // Mods (covers Upgrades/RawUpgrades)
    if let Ok(raw) = fs::read_to_string(data_dir.join("Mods.json")) {
        if let Ok(arr) = serde_json::from_str::<itemdata::mods::Root>(&raw) {
            for item in arr {
                let v = serde_json::to_value(&item).unwrap_or(Value::Null);
                // Mods can map to Upgrades or RawUpgrades
                for pc in item.get_product_categories() {
                    push_info(&v, Some(pc));
                }
            }
        }
    }

    index
}

fn find_itemdata_dir() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("WF_ITEMDATA_DIR") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        let path = cwd.join("warframe-items-data").join("json");
        if path.exists() {
            return Some(path);
        }
    }

    let fallback = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("warframe-items-data")
        .join("json");
    if fallback.exists() {
        return Some(fallback);
    }

    None
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

#[derive(Debug, Serialize)]
struct ScreenshotEvent {
    id: String,
    timestamp: DateTime<Utc>,
    action: Option<String>,
    metadata: Option<Value>,
}

fn record_screenshot_event(
    action: Option<String>,
    metadata: Option<Value>,
) -> Result<ScreenshotEvent> {
    let event = ScreenshotEvent {
        id: format!(
            "{}-{}",
            Utc::now().timestamp_millis(),
            rand::random::<u32>()
        ),
        timestamp: Utc::now(),
        action,
        metadata,
    };

    let cache_dir = storage::app_cache_dir()?;
    let last_path = cache_dir.join("last_screenshot.json");
    let raw =
        serde_json::to_string_pretty(&event).context("Failed to serialize screenshot event")?;
    fs::write(&last_path, raw).context("Failed to write last screenshot event")?;

    let log_path = cache_dir.join("screenshot_events.jsonl");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .context("Failed to open screenshot events log")?;
    let line = serde_json::to_string(&event).context("Failed to serialize screenshot event")?;
    writeln!(file, "{}", line).context("Failed to append screenshot event")?;

    Ok(event)
}
