//! Minecraft version compatibility checks for Quilt Loader.
//!
//! Quilt officially supports Minecraft 1.18.2 and newer versions.
//! Older Minecraft releases (such as 1.16.5, 1.12.2, 1.8.9) are not supported by Quilt.

use std::cmp::Ordering;
use crew_core::{CrewError, version_order::compare_versions};

/// Minimum Minecraft version supported by Quilt.
pub const MINIMUM_QUILT_MC_VERSION: &str = "1.18.2";

/// Checks if a given Minecraft version is compatible with Quilt Loader.
pub fn is_quilt_supported(mc_version: &str) -> bool {
    let clean = mc_version.trim();
    if clean.is_empty() {
        return false;
    }
    // Snapshots or release candidate checks
    let base = clean.split('-').next().unwrap_or(clean);
    compare_versions(base, MINIMUM_QUILT_MC_VERSION) != Ordering::Less
}

/// Validates Minecraft version for Quilt, returning an error if unsupported.
pub fn validate_quilt_compatibility(mc_version: &str) -> Result<(), CrewError> {
    if is_quilt_supported(mc_version) {
        Ok(())
    } else {
        Err(CrewError::UnsupportedVersion {
            loader: "Quilt".to_string(),
            mc_version: mc_version.trim().to_string(),
            reason: format!(
                "Quilt Loader resmi olarak Minecraft 1.18.2 ve üzerini destekler. ({mc_version} sürümü desteklenmiyor)"
            ),
        })
    }
}
