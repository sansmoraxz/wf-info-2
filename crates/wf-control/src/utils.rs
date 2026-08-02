use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub(crate) const WFM_API_BASE: &str = "https://api.warframe.market/v2";
pub(crate) const WFM_WS_URL: &str = "wss://ws.warframe.market/socket";
pub(crate) const WFM_SUB_PROTOCOL: &str = "wfm";

pub(crate) const WFM_AUTH_BASE: &str = "https://api.warframe.market/v1";

/// GET a v2 WFM API path (relative to [`WFM_API_BASE`]) and parse the JSON body.
pub(crate) async fn wfm_get<T: DeserializeOwned>(client: &reqwest::Client, path: &str) -> Result<T> {
    let url = format!("{}/{}", WFM_API_BASE, path);
    Ok(client.get(&url).send().await?.json().await?)
}

pub(crate) fn parse_params<T>(params: Option<Value>) -> Result<T>
where
    T: DeserializeOwned + Default,
{
    match params {
        Some(value) => Ok(serde_json::from_value(value).context("Invalid params")?),
        None => Ok(T::default()),
    }
}

/// Like [`parse_params`], but params must be present (no default).
pub(crate) fn parse_required_params<T: DeserializeOwned>(params: Option<Value>) -> Result<T> {
    let value = params.context("Missing required params")?;
    serde_json::from_value(value).context("Invalid params")
}
