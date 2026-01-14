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

use crate::profile::Root;

// Get key from build-time environment variable
// Can be any string now
const RAW_KEY_ENV: &str = env!("WF_PROFILE_KEY");

pub fn save_encrypted_profile(profile: &Root) -> anyhow::Result<()> {
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
