use anyhow::Result;
use std::time::Duration;

use crate::api;
use crate::process;
use wf_inventory::Inventory;

#[derive(Debug)]
pub struct InventoryFetch {
    pub inventory: Inventory,
    pub auth: process::AuthQuery,
}

pub async fn fetch_inventory_from_process(
    client: &reqwest::Client,
    pid: u32,
    scan_retries: u32,
    scan_delay: Duration,
) -> Result<Option<InventoryFetch>> {
    let auth = match process::scan_memory_for_auth_with_retry(pid, scan_retries, scan_delay).await?
    {
        Some(auth) => auth,
        None => return Ok(None),
    };

    fetch_inventory_with_auth_from_process(client, pid, auth, scan_retries, scan_delay).await
}

pub async fn fetch_inventory_with_auth_from_process(
    client: &reqwest::Client,
    pid: u32,
    auth: process::AuthQuery,
    scan_retries: u32,
    scan_delay: Duration,
) -> Result<Option<InventoryFetch>> {
    match api::fetch_inventory(client, &auth).await {
        Ok(inventory) => Ok(Some(InventoryFetch { inventory, auth })),
        Err(error) if api::is_inventory_authorization_rejected(&error) => {
            log::warn!("Inventory authorization was rejected; rescanning process memory once");
            let Some(new_auth) =
                process::scan_memory_for_auth_with_retry(pid, scan_retries, scan_delay).await?
            else {
                return Ok(None);
            };
            let inventory = api::fetch_inventory(client, &new_auth).await?;
            Ok(Some(InventoryFetch {
                inventory,
                auth: new_auth,
            }))
        }
        Err(error) => Err(error),
    }
}
