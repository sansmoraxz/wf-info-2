use crate::process::AuthQuery;
use crate::profile::ProfileData;
use wf_inventory::Inventory;

const PLAYER_INFO_URL: &str = "https://api.warframe.com/cdn/getProfileViewingData.php";
const INVENTORY_URL: &str = "https://api.warframe.com/api/inventory.php";

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Inventory authorization was rejected with status {0}")]
    AuthorizationRejected(reqwest::StatusCode),
    #[error("Inventory API returned status: {0}")]
    Status(reqwest::StatusCode),
    #[error("Inventory request failed: {0}")]
    Request(#[source] reqwest::Error),
    #[error("Failed to parse inventory JSON: {0}")]
    Parse(#[source] reqwest::Error),
}

/// Fetches the player's profile data from the Warframe API using the provided account ID.
/// Returns a ProfileData struct on success.
pub async fn fetch_player_profile(
    client: &reqwest::Client,
    account_id: &str,
) -> Result<ProfileData, reqwest::Error> {
    client
        .get(PLAYER_INFO_URL)
        .query(&[("playerId", account_id)])
        .send()
        .await?
        .json()
        .await
}

/// Fetches the player's full inventory using the authenticated query.
/// Returns the deserialized Inventory on success.
pub async fn fetch_inventory(
    client: &reqwest::Client,
    auth: &AuthQuery,
) -> Result<Inventory, ApiError> {
    log::info!("Fetching inventory from API...");

    let response = client
        .get(INVENTORY_URL)
        .query(&[("accountId", &auth.account_id), ("nonce", &auth.nonce)])
        .send()
        .await
        .map_err(|error| ApiError::Request(error.without_url()))?;

    if !response.status().is_success() {
        if matches!(
            response.status(),
            reqwest::StatusCode::BAD_REQUEST
                | reqwest::StatusCode::UNAUTHORIZED
                | reqwest::StatusCode::FORBIDDEN
        ) {
            return Err(ApiError::AuthorizationRejected(response.status()));
        }
        return Err(ApiError::Status(response.status()));
    }

    let inventory: Inventory = response
        .json()
        .await
        .map_err(|error| ApiError::Parse(error.without_url()))?;

    log::info!("Successfully fetched inventory data");
    Ok(inventory)
}
