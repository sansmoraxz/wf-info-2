use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub(crate) const WFM_API_BASE: &str = "https://api.warframe.market/v2";
pub(crate) const WFM_WS_URL: &str = "wss://ws.warframe.market/socket";
pub(crate) const WFM_SUB_PROTOCOL: &str = "wfm";

pub(crate) const WFM_AUTH_BASE: &str = "https://api.warframe.market/v1";

pub(crate) fn parse_params<T>(params: Option<Value>) -> Result<T>
where
    T: DeserializeOwned + Default,
{
    match params {
        Some(value) => Ok(serde_json::from_value(value).context("Invalid params")?),
        None => Ok(T::default()),
    }
}
