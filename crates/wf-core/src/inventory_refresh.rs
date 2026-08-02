use std::time::Duration;

use crate::api;
use crate::process;
use wf_inventory::Inventory;

#[derive(Debug)]
pub struct InventoryFetch {
    pub inventory: Inventory,
}

#[derive(Debug, thiserror::Error)]
pub enum RefreshError {
    #[error(transparent)]
    Scan(#[from] process::ScanError),
    #[error(transparent)]
    Api(#[from] api::ApiError),
}

pub async fn fetch_inventory_from_process(
    client: &reqwest::Client,
    pid: u32,
    scan_retries: u32,
    scan_delay: Duration,
) -> Result<Option<InventoryFetch>, RefreshError> {
    let Some(auth) =
        process::scan_memory_for_auth_with_retry(pid, scan_retries, scan_delay).await?
    else {
        return Ok(None);
    };

    fetch_inventory_with_auth_from_process(client, pid, auth, scan_retries, scan_delay).await
}

pub async fn fetch_inventory_with_auth_from_process(
    client: &reqwest::Client,
    pid: u32,
    auth: process::AuthQuery,
    scan_retries: u32,
    scan_delay: Duration,
) -> Result<Option<InventoryFetch>, RefreshError> {
    match api::fetch_inventory(client, &auth).await {
        Ok(inventory) => Ok(Some(InventoryFetch { inventory })),
        Err(api::ApiError::AuthorizationRejected(_)) => {
            log::warn!("Inventory authorization was rejected; rescanning process memory once");
            let Some(new_auth) =
                process::scan_memory_for_auth_with_retry(pid, scan_retries, scan_delay).await?
            else {
                return Ok(None);
            };
            let inventory = api::fetch_inventory(client, &new_auth).await?;
            Ok(Some(InventoryFetch { inventory }))
        }
        Err(error) => Err(error.into()),
    }
}
