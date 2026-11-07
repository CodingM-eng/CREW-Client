use crate::error::CrewError;
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::Sha256;
use std::fs;
use std::path::Path;

/// Compute hex SHA-1 of byte buffer.
pub fn sha1_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Compute hex SHA-256 of byte buffer.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Compute hex SHA-1 of a file on disk.
pub fn sha1_file(path: &Path) -> Result<String, CrewError> {
    let bytes = fs::read(path).map_err(|e| CrewError::Io(format!("{}: {e}", path.display())))?;
    Ok(sha1_hex(&bytes))
}

/// Compute hex SHA-256 of a file on disk.
pub fn sha256_file(path: &Path) -> Result<String, CrewError> {
    let bytes = fs::read(path).map_err(|e| CrewError::Io(format!("{}: {e}", path.display())))?;
    Ok(sha256_hex(&bytes))
}

/// Verify that `bytes` matches the expected SHA-1 hex string (case-insensitive).
pub fn verify_sha1(bytes: &[u8], expected_hex: &str) -> bool {
    let actual = sha1_hex(bytes);
    actual.eq_ignore_ascii_case(expected_hex.trim())
}

/// Verify that `bytes` matches the expected SHA-256 hex string (case-insensitive).
pub fn verify_sha256(bytes: &[u8], expected_hex: &str) -> bool {
    let actual = sha256_hex(bytes);
    actual.eq_ignore_ascii_case(expected_hex.trim())
}

/// Verify that a file on disk matches expected SHA-1.
pub fn verify_file_sha1(path: &Path, expected_hex: &str) -> Result<bool, CrewError> {
    let actual = sha1_file(path)?;
    Ok(actual.eq_ignore_ascii_case(expected_hex.trim()))
}

/// Verify that a file on disk matches expected SHA-256.
pub fn verify_file_sha256(path: &Path, expected_hex: &str) -> Result<bool, CrewError> {
    let actual = sha256_file(path)?;
    Ok(actual.eq_ignore_ascii_case(expected_hex.trim()))
}
