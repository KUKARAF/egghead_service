use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};

include!(concat!(env!("OUT_DIR"), "/emoji_pool.rs"));

pub fn generate_api_token() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let plaintext = format!("egghead_{}", URL_SAFE_NO_PAD.encode(bytes));
    let hash = hash_key(&plaintext);
    (plaintext, hash)
}

pub fn generate_device_token() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let plaintext = format!("device_{}", URL_SAFE_NO_PAD.encode(bytes));
    let hash = hash_key(&plaintext);
    (plaintext, hash)
}

pub fn generate_emoji_sequence() -> String {
    let mut rng = rand::thread_rng();
    rand::seq::index::sample(&mut rng, EMOJI_POOL.len(), 3)
        .iter()
        .filter_map(|i| EMOJI_POOL.get(i).copied())
        .collect::<Vec<_>>()
        .join("")
}

pub fn hash_key(plaintext: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(plaintext.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn generate_session_token() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let plaintext = URL_SAFE_NO_PAD.encode(bytes);
    let hash = hash_key(&plaintext);
    (plaintext, hash)
}
