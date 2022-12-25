// https://github.com/RustCrypto/hashes

use sha2::{Sha256, Digest};
use std::mem;

use crate::models;

pub fn encrypt(data: models::KeyDataModel) -> String {
    let mut hasher = Sha256::new();


    // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    hasher.update(data.keyType.to_string());
    hasher.update(data.envName);

    // Note that calling `finalize()` consumes hasher
    let hash = hasher.finalize();
    let secret_hash: String = format!("{:x}", hash);

    secret_hash
}
