use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub(crate) fn parse_params<T>(params: Option<Value>) -> Result<T>
where
    T: DeserializeOwned + Default,
{
    match params {
        Some(value) => Ok(serde_json::from_value(value).context("Invalid params")?),
        None => Ok(T::default()),
    }
}
