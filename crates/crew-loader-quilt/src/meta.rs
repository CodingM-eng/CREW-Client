//! Quilt Meta API client and version parser.

use crew_core::{CrewError, version_order::compare_versions};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const QUILT_META_BASE_URL: &str = "https://meta.quiltmc.org/v3";
pub const QUILT_DEFAULT_MAVEN_URL: &str = "https://maven.quiltmc.org/repository/release/";

/// A parsed Quilt Loader version record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuiltVersionRecord {
    pub version: String,
    pub stable: bool,
    pub build: Option<u64>,
}

/// Helper struct for deserializing entries from Quilt Meta API.
#[derive(Debug, Deserialize)]
struct QuiltMetaEntry {
    loader: QuiltLoaderMeta,
}

#[derive(Debug, Deserialize)]
struct QuiltLoaderMeta {
    version: String,
    build: Option<u64>,
    stable: Option<bool>,
}

/// Parses the Quilt loader version list from the meta endpoint JSON response.
///
/// Quilt meta entries often do not include an explicit `stable` boolean;
/// version numbers with a hyphen (such as `0.20.0-beta.9`) are marked as unstable.
pub fn parse_quilt_loader_versions(json_str: &str) -> Result<Vec<QuiltVersionRecord>, CrewError> {
    let entries: Vec<QuiltMetaEntry> = serde_json::from_str(json_str)
        .map_err(|e| CrewError::Json(format!("Quilt sürüm listesi çözümlenemedi: {e}")))?;

    let mut records: Vec<QuiltVersionRecord> = entries
        .into_iter()
        .map(|entry| {
            let ver = entry.loader.version;
            let stable = entry.loader.stable.unwrap_or_else(|| !ver.contains('-'));
            QuiltVersionRecord {
                version: ver,
                stable,
                build: entry.loader.build,
            }
        })
        .collect();

    sort_quilt_loader_versions(&mut records);
    Ok(records)
}

/// Sorts Quilt loader versions descending (newest first).
pub fn sort_quilt_loader_versions(records: &mut [QuiltVersionRecord]) {
    records.sort_by(|a, b| compare_versions(&b.version, &a.version));
}

/// Picks the requested Quilt loader version, or the latest stable version if none/latest specified.
pub fn pick_quilt_loader_version(
    records: &[QuiltVersionRecord],
    wanted: Option<&str>,
) -> Result<QuiltVersionRecord, CrewError> {
    if records.is_empty() {
        return Err(CrewError::VersionNotFound("Hiç Quilt loader sürümü bulunamadı.".to_string()));
    }

    match wanted.map(str::trim) {
        None | Some("") | Some("latest") => {
            // Find first stable version, or fallback to first available
            let selected = records
                .iter()
                .find(|r| r.stable)
                .or_else(|| records.first())
                .cloned()
                .unwrap();
            Ok(selected)
        }
        Some(target) => records
            .iter()
            .find(|r| r.version == target)
            .cloned()
            .ok_or_else(|| {
                CrewError::VersionNotFound(format!("İstenen Quilt loader sürümü bulunamadı: {target}"))
            }),
    }
}

/// Constructs the Quilt Meta URL to fetch version listings for a Minecraft version.
pub fn quilt_versions_url(mc_version: &str) -> String {
    format!("{QUILT_META_BASE_URL}/versions/loader/{mc_version}")
}

/// Constructs the Quilt profile JSON endpoint URL.
pub fn quilt_profile_url(mc_version: &str, loader_version: &str) -> String {
    format!("{QUILT_META_BASE_URL}/versions/loader/{mc_version}/{loader_version}/profile/json")
}

/// Constructs standard profile ID for Quilt.
pub fn quilt_profile_id(loader_version: &str, mc_version: &str) -> String {
    format!("quilt-loader-{loader_version}-{mc_version}")
}

/// Extracts Maven library metadata from Quilt profile JSON.
pub fn extract_quilt_libraries(profile_json: &Value) -> Result<Vec<(String, String)>, CrewError> {
    let libs = profile_json["libraries"]
        .as_array()
        .ok_or_else(|| CrewError::InvalidData("Quilt profilinde 'libraries' dizisi bulunamadı.".to_string()))?;

    let mut result = Vec::new();
    for lib in libs {
        let name = lib["name"]
            .as_str()
            .ok_or_else(|| CrewError::InvalidData("Kütüphane 'name' alanı eksik.".to_string()))?;
        let repo = lib["url"]
            .as_str()
            .unwrap_or(QUILT_DEFAULT_MAVEN_URL);
        result.push((name.to_string(), repo.to_string()));
    }
    Ok(result)
}
