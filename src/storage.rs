use aes_gcm::{
    Aes256Gcm,
    Key, // Key is a type alias, but passing &[u8] via from_slice works
    Nonce,
    aead::{Aead, KeyInit},
};
use anyhow::Context;
use rand::{Rng, rng};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;

use crate::{inventory, profile::ProfileData};

// Get key from build-time environment variable
// Can be any string now
const RAW_KEY_ENV: &str = env!("WF_PROFILE_KEY");

// AES-128-CBC key and IV matching the C++ reference implementation
// Used for inventory data compatibility with other tools
const INVENTORY_KEY: [u8; 16] = [
    76, 69, 79, 45, 65, 76, 69, 67, 9, 69, 79, 45, 65, 76, 69, 67,
];
const INVENTORY_IV: [u8; 16] = [
    49, 50, 70, 71, 66, 51, 54, 45, 76, 69, 51, 45, 113, 61, 57, 0,
];

pub fn save_encrypted_profile(profile: &ProfileData) -> anyhow::Result<()> {
    let json = serde_json::to_vec(profile).context("Failed to serialize profile")?;

    // Hash the raw string key to get a 32-byte key
    let mut hasher = Sha256::new();
    hasher.update(RAW_KEY_ENV.as_bytes());
    let key_bytes = hasher.finalize();

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, json.as_ref())
        .map_err(|e| anyhow::anyhow!("Encryption failure: {}", e))?;

    // Store nonce + ciphertext
    let mut final_data = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
    final_data.extend_from_slice(&nonce_bytes);
    final_data.extend_from_slice(&ciphertext);

    let cache_dir =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?;
    let app_cache_dir = cache_dir.join("wf-info-2");

    if !app_cache_dir.exists() {
        fs::create_dir_all(&app_cache_dir).context("Failed to create cache directory")?;
    }

    let file_path = app_cache_dir.join("userstats.dat");
    let mut file = File::create(&file_path).context("Failed to create output file")?;
    file.write_all(&final_data)
        .context("Failed to write to file")?;

    log::info!("Saved encrypted profile to {}", file_path.display());

    Ok(())
}

pub fn delete_profile() -> anyhow::Result<()> {
    let cache_dir =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?;
    let app_cache_dir = cache_dir.join("wf-info-2");
    let file_path = app_cache_dir.join("userstats.dat");

    if file_path.exists() {
        fs::remove_file(&file_path).context("Failed to delete profile file")?;
        log::info!("Deleted profile data at {}", file_path.display());
    }

    Ok(())
}

/// Saves inventory data in two formats:
/// 1. inventory.json - Pretty-printed JSON for human readability
/// 2. lastData.dat - AES-128-CBC encrypted (compatible with C++ reference)
pub fn save_inventory(inventory: &inventory::Inventory) -> anyhow::Result<()> {
    use aes::cipher::{BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
    type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

    let cache_dir =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?;
    let app_cache_dir = cache_dir.join("wf-info-2");

    if !app_cache_dir.exists() {
        fs::create_dir_all(&app_cache_dir).context("Failed to create cache directory")?;
    }

    // Save pretty-printed JSON
    let json_path = app_cache_dir.join("inventory.json");
    let pretty_json =
        serde_json::to_string_pretty(inventory).context("Failed to serialize inventory")?;
    fs::write(&json_path, &pretty_json).context("Failed to write inventory.json")?;
    log::info!("Saved inventory JSON to {}", json_path.display());

    // Save encrypted lastData.dat (AES-128-CBC with PKCS7 padding)
    let json_bytes = serde_json::to_vec(inventory).context("Failed to serialize inventory")?;

    // Calculate padded size (PKCS7 pads to block size boundary)
    let block_size = 16;
    let padded_len = ((json_bytes.len() / block_size) + 1) * block_size;
    let mut buffer = vec![0u8; padded_len];
    buffer[..json_bytes.len()].copy_from_slice(&json_bytes);

    let cipher = Aes128CbcEnc::new(&INVENTORY_KEY.into(), &INVENTORY_IV.into());
    let ciphertext = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, json_bytes.len())
        .map_err(|e| anyhow::anyhow!("Encryption error: {:?}", e))?;

    let dat_path = app_cache_dir.join("lastData.dat");
    fs::write(&dat_path, ciphertext).context("Failed to write lastData.dat")?;
    log::info!("Saved encrypted inventory to {}", dat_path.display());

    Ok(())
}
