use serde::de::DeserializeOwned;
#[cfg(feature = "cli")]
use serde_json::Value;
use serde_json::value::RawValue;

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

/// Strict JSON parser for clap args: validates the text and keeps it
/// unparsed for single-pass deserialization at the destination type.
#[cfg(feature = "cli")]
pub(crate) fn parse_json_value(raw: &str) -> Result<Box<RawValue>, String> {
    RawValue::from_string(raw.to_owned()).map_err(|e| format!("Invalid JSON: {e}"))
}

/// Lenient parser for clap args: JSON if it parses, then number, bool,
/// falling back to a plain string.
#[cfg(feature = "cli")]
#[allow(
    clippy::unnecessary_wraps,
    reason = "clap value_parser requires a Result-returning signature"
)]
pub(crate) fn parse_jsonish(raw: &str) -> Result<Value, String> {
    Ok(serde_json::from_str(raw)
        .ok()
        .or_else(|| raw.parse::<i64>().ok().map(|num| Value::Number(num.into())))
        .or_else(|| raw.parse::<bool>().ok().map(Value::Bool))
        .unwrap_or_else(|| Value::String(raw.to_owned())))
}

/// GET a v2 WFM API path (relative to [`WFM_API_BASE`]) and parse the JSON body.
pub(super) async fn wfm_get<T>(client: &reqwest::Client, path: &str) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
{
    let url = format!("{WFM_API_BASE}/{path}");
    client.get(&url).send().await?.json().await
}

pub(super) fn parse_params<T>(params: Option<Box<RawValue>>) -> Result<T, ParamsError>
where
    T: DeserializeOwned + Default,
{
    params.map_or_else(
        || Ok(T::default()),
        |raw| serde_json::from_str(raw.get()).map_err(ParamsError::Invalid),
    )
}

/// Like [`parse_params`], but params must be present (no default).
pub(super) fn parse_required_params<T>(params: Option<Box<RawValue>>) -> Result<T, ParamsError>
where
    T: DeserializeOwned,
{
    let raw = params.ok_or(ParamsError::Missing)?;
    serde_json::from_str(raw.get()).map_err(ParamsError::Invalid)
}
