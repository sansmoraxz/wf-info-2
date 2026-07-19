use crate::process::AuthQuery;
use crate::profile::ProfileData;
use std::fmt;
use wf_inventory::Inventory;

const PLAYER_INFO_URL: &str = "https://api.warframe.com/cdn/getProfileViewingData.php";
const INVENTORY_URL: &str = "https://api.warframe.com/api/inventory.php";

#[derive(Debug)]
pub struct InventoryAuthorizationRejected(reqwest::StatusCode);

impl fmt::Display for InventoryAuthorizationRejected {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Inventory authorization was rejected with status {}",
            self.0
        )
    }
}

impl std::error::Error for InventoryAuthorizationRejected {}

pub fn is_inventory_authorization_rejected(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<InventoryAuthorizationRejected>()
        .is_some()
}

/// Fetches the player's profile data from the Warframe API using the provided account ID.
/// Returns a ProfileData struct on success.
pub async fn fetch_player_profile(account_id: &str) -> Result<ProfileData, reqwest::Error> {
    reqwest::Client::new()
        .get(PLAYER_INFO_URL)
        .query(&[("playerId", account_id)])
        .send()
        .await?
        .json()
        .await
}

/// Fetches the player's full inventory using the authenticated query.
/// Returns the raw JSON response as a serde_json::Value.
pub async fn fetch_inventory(auth: &AuthQuery) -> anyhow::Result<Inventory> {
    log::info!("Fetching inventory from API...");

    let response = reqwest::Client::new()
        .get(INVENTORY_URL)
        .query(&[("accountId", &auth.account_id), ("nonce", &auth.nonce)])
        .send()
        .await
        .map_err(|error| anyhow::anyhow!("Inventory request failed: {}", error.without_url()))?;

    if !response.status().is_success() {
        if matches!(
            response.status(),
            reqwest::StatusCode::BAD_REQUEST
                | reqwest::StatusCode::UNAUTHORIZED
                | reqwest::StatusCode::FORBIDDEN
        ) {
            return Err(InventoryAuthorizationRejected(response.status()).into());
        }
        return Err(anyhow::anyhow!(
            "Inventory API returned status: {}",
            response.status()
        ));
    }

    let inventory: Inventory = response.json().await.map_err(|error| {
        anyhow::anyhow!("Failed to parse inventory JSON: {}", error.without_url())
    })?;

    log::info!("Successfully fetched inventory data");
    Ok(inventory)
}
