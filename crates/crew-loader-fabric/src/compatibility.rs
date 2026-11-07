use crew_core::version_order::compare_versions;
use crew_core::CrewError;
use std::cmp::Ordering;

/// Minimum officially supported Minecraft version for standard Fabric loader.
pub const FABRIC_MINIMUM_VERSION: &str = "1.14";

/// Checks if a Minecraft version is compatible with Fabric Loader.
pub fn check_compatibility(mc_version: &str) -> Result<(), CrewError> {
    let clean = mc_version.trim();
    if clean.is_empty() {
        return Err(CrewError::UnsupportedVersion {
            loader: "Fabric".into(),
            mc_version: mc_version.into(),
            reason: "Minecraft sürümü boş olamaz.".into(),
        });
    }

    // Fabric officially supports 1.14 and newer
    if compare_versions(clean, FABRIC_MINIMUM_VERSION) == Ordering::Less {
        return Err(CrewError::UnsupportedVersion {
            loader: "Fabric".into(),
            mc_version: clean.into(),
            reason: format!(
                "Fabric Loader resmi olarak Minecraft 1.14 ve üzerini destekler. ({clean} sürümü desteklenmiyor)"
            ),
        });
    }

    Ok(())
}
