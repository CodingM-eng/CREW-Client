use crew_core::version_order::compare_versions;
use crew_core::CrewError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const FABRIC_META_URL: &str = "https://meta.fabricmc.net/v2";
pub const FABRIC_MAVEN_URL: &str = "https://maven.fabricmc.net/";

/// One of a Fabric loader's versions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabricLoaderVersion {
    pub version: String,
    pub stable: bool,
}

/// Official Fabric game version entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FabricGameVersion {
    pub version: String,
    pub stable: bool,
}

/// Parse Fabric loader versions JSON list returned by meta.fabricmc.net.
/// Sorts newest versions first, prioritizing stable releases.
pub fn parse_loader_versions(text: &str) -> Result<Vec<FabricLoaderVersion>, CrewError> {
    let list: Value = serde_json::from_str(text)
        .map_err(|e| CrewError::Json(format!("Fabric sürüm listesi ayrıştırılamadı: {e}")))?;

    let array = list
        .as_array()
        .ok_or_else(|| CrewError::InvalidData("Fabric sürüm verisi dizi değil.".into()))?;

    let mut versions: Vec<FabricLoaderVersion> = array
        .iter()
        .filter_map(|entry| {
            let loader = &entry["loader"];
            let version = loader["version"].as_str()?.to_string();
            let stable = loader["stable"].as_bool().unwrap_or(!version.contains('-'));
            Some(FabricLoaderVersion { version, stable })
        })
        .collect();

    // Sort: Newest version first. If same version, stable first.
    versions.sort_by(|a, b| {
        let cmp = compare_versions(&b.version, &a.version);
        if cmp == std::cmp::Ordering::Equal {
            b.stable.cmp(&a.stable)
        } else {
            cmp
        }
    });

    Ok(versions)
}

/// Pick a suitable loader version among available candidates.
/// If `wanted` is None or "latest", returns newest stable (or first available).
pub fn pick_loader_version(
    versions: &[FabricLoaderVersion],
    wanted: Option<&str>,
) -> Result<String, CrewError> {
    if versions.is_empty() {
        return Err(CrewError::VersionNotFound(
            "Hiçbir Fabric Loader sürümü bulunamadı.".into(),
        ));
    }

    let found = match wanted.map(str::trim) {
        None | Some("latest") => versions.iter().find(|l| l.stable).or(versions.first()),
        Some(v) => versions.iter().find(|l| l.version == v),
    };

    found.map(|l| l.version.clone()).ok_or_else(|| {
        CrewError::VersionNotFound(format!(
            "Fabric Loader {} sürümü bu Minecraft sürümü için mevcut değil.",
            wanted.unwrap_or("latest")
        ))
    })
}

/// Generates standard Fabric profile ID string: `fabric-loader-{version}-{minecraft}`.
pub fn profile_id(loader_version: &str, mc_version: &str) -> String {
    format!("fabric-loader-{loader_version}-{mc_version}")
}

/// URL to download the version profile JSON from Fabric meta.
pub fn profile_url(mc_version: &str, loader_version: &str) -> String {
    format!(
        "{FABRIC_META_URL}/versions/loader/{mc_version}/{loader_version}/profile/json"
    )
}
