//! Version policy enforcement and loader compatibility validation.

use crate::model::LoaderKind;
use crew_core::{CrewError, version_order::compare_versions};
use std::cmp::Ordering;

/// Minimum officially supported Minecraft version across the CREW ecosystem.
pub const MINIMUM_SUPPORTED_MC_VERSION: &str = "1.8.9";

/// Checks if a Minecraft version meets the minimum version policy (>= 1.8.9).
pub fn is_supported_mc_version(mc_version: &str) -> bool {
    let clean = mc_version.trim();
    if clean.is_empty() {
        return false;
    }
    let base = clean.split('-').next().unwrap_or(clean);
    compare_versions(base, MINIMUM_SUPPORTED_MC_VERSION) != Ordering::Less
}

/// Validates that the requested Minecraft version meets CREW Client minimum policy.
pub fn validate_version_support(mc_version: &str) -> Result<(), CrewError> {
    if is_supported_mc_version(mc_version) {
        Ok(())
    } else {
        Err(CrewError::UnsupportedVersion {
            loader: "CREW Client".to_string(),
            mc_version: mc_version.trim().to_string(),
            reason: format!(
                "CREW Client en az Minecraft {MINIMUM_SUPPORTED_MC_VERSION} sürümünü destekler."
            ),
        })
    }
}

/// Validates that a mod loader is compatible with the given Minecraft version.
pub fn validate_loader_support(loader: LoaderKind, mc_version: &str) -> Result<(), CrewError> {
    // 1. Verify global Minecraft version policy
    validate_version_support(mc_version)?;

    // 2. Loader-specific compatibility rules
    match loader {
        LoaderKind::Vanilla => Ok(()),
        LoaderKind::Fabric => crew_loader_fabric::check_compatibility(mc_version),
        LoaderKind::Quilt => crew_loader_quilt::validate_quilt_compatibility(mc_version),
    }
}
