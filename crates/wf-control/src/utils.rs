use serde::de::DeserializeOwned;
use serde_json::Value;

pub const WFM_API_BASE: &str = "https://api.warframe.market/v2";
pub const WFM_WS_URL: &str = "wss://ws.warframe.market/socket";
pub const WFM_SUB_PROTOCOL: &str = "wfm";

pub const WFM_AUTH_BASE: &str = "https://api.warframe.market/v1";

#[derive(Debug, thiserror::Error)]
pub enum ParamsError {
    #[error("Invalid params")]
    Invalid(#[source] serde_json::Error),
    #[error("Missing required params")]
    Missing,
}

/// GET a v2 WFM API path (relative to [`WFM_API_BASE`]) and parse the JSON body.
pub async fn wfm_get<T: DeserializeOwned>(
    client: &reqwest::Client,
    path: &str,
) -> Result<T, reqwest::Error> {
    let url = format!("{WFM_API_BASE}/{path}");
    client.get(&url).send().await?.json().await
}

pub fn parse_params<T>(params: Option<Value>) -> Result<T, ParamsError>
where
    T: DeserializeOwned + Default,
{
    params.map_or_else(
        || Ok(T::default()),
        |value| serde_json::from_value(value).map_err(ParamsError::Invalid),
    )
}

/// Like [`parse_params`], but params must be present (no default).
pub fn parse_required_params<T: DeserializeOwned>(params: Option<Value>) -> Result<T, ParamsError> {
    let value = params.ok_or(ParamsError::Missing)?;
    serde_json::from_value(value).map_err(ParamsError::Invalid)
}
