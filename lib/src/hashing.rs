use sha2::{Digest, Sha256};

pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    // Convert result (a byte array) to a hexadecimal string
    hex::encode(result)
}
/// Validates if the SHA-256 hash of `input` matches the given `expected_hash`.
pub fn validate_sha256(input: &str, expected_hash: &str) -> bool {
    let computed_hash = hash_sha256(input);
    computed_hash.eq_ignore_ascii_case(expected_hash)
}
