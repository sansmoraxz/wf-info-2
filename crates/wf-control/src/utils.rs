use serde::de::DeserializeOwned;
use serde_json::Value;

pub(super) const WFM_API_BASE: &str = "https://api.warframe.market/v2";
pub(super) const WFM_WS_URL: &str = "wss://ws.warframe.market/socket";
pub(super) const WFM_SUB_PROTOCOL: &str = "wfm";

pub(super) const WFM_AUTH_BASE: &str = "https://api.warframe.market/v1";

#[derive(Debug, thiserror::Error)]
pub(super) enum ParamsError {
    #[error("Invalid params")]
    Invalid(#[source] serde_json::Error),
    #[error("Missing required params")]
    Missing,
}

/// Strict JSON value parser for clap args.
#[cfg(feature = "cli")]
pub(crate) fn parse_json_value(raw: &str) -> Result<Value, String> {
    serde_json::from_str(raw).map_err(|e| format!("Invalid JSON: {e}"))
}

/// Lenient parser for clap args: JSON if it parses, then number, bool,
/// falling back to a plain string.
#[cfg(feature = "cli")]
#[allow(
    clippy::unnecessary_wraps,
    reason = "clap value_parser requires a Result-returning signature"
)]
pub(crate) fn parse_jsonish(raw: &str) -> Result<Value, String> {
    Ok(if let Ok(v) = serde_json::from_str(raw) {
        v
    } else if let Ok(num) = raw.parse::<i64>() {
        Value::Number(num.into())
    } else if let Ok(b) = raw.parse::<bool>() {
        Value::Bool(b)
    } else {
        Value::String(raw.to_owned())
    })
}

/// GET a v2 WFM API path (relative to [`WFM_API_BASE`]) and parse the JSON body.
pub(super) async fn wfm_get<T>(client: &reqwest::Client, path: &str) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
{
    let url = format!("{WFM_API_BASE}/{path}");
    client.get(&url).send().await?.json().await
}

pub(super) fn parse_params<T>(params: Option<Value>) -> Result<T, ParamsError>
where
    T: DeserializeOwned + Default,
{
    params.map_or_else(
        || Ok(T::default()),
        |value| serde_json::from_value(value).map_err(ParamsError::Invalid),
    )
}

/// Like [`parse_params`], but params must be present (no default).
pub(super) fn parse_required_params<T>(params: Option<Value>) -> Result<T, ParamsError>
where
    T: DeserializeOwned,
{
    let value = params.ok_or(ParamsError::Missing)?;
    serde_json::from_value(value).map_err(ParamsError::Invalid)
}
