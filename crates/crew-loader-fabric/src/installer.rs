use crate::meta::{profile_id, profile_url, FABRIC_MAVEN_URL};
use crew_core::checksum::{sha1_file, sha1_hex, verify_sha1};
use crew_core::maven::maven_path;
use crew_core::CrewError;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

/// Type alias for download fetch function: given a URL, returns byte buffer or CrewError.
pub type FetchFn = dyn Fn(&str) -> Result<Vec<u8>, CrewError>;

/// Sanitizes relative path to prevent directory traversal (`..` or root absolute paths).
pub fn relative_path(path_str: &str) -> Result<std::path::PathBuf, CrewError> {
    let p = Path::new(path_str);
    for component in p.components() {
        if matches!(
            component,
            std::path::Component::ParentDir | std::path::Component::RootDir | std::path::Component::Prefix(_)
        ) {
            return Err(CrewError::InvalidData(format!(
                "Güvensiz dosya yolu tespit edildi: {path_str}"
            )));
        }
    }
    Ok(p.to_path_buf())
}

/// Installs a Fabric loader profile and its Maven libraries into `game_dir`.
///
/// Returns the profile ID string (e.g. `fabric-loader-0.16.9-1.21.4`).
pub fn install_fabric(
    mc_version: &str,
    loader_version: &str,
    game_dir: &Path,
    fetch: &FetchFn,
    log: &dyn Fn(String),
) -> Result<String, CrewError> {
    let id = profile_id(loader_version, mc_version);
    let url = profile_url(mc_version, loader_version);

    log(format!("Fabric {loader_version} profili indiriliyor: {url}"));
    let profile_bytes = fetch(&url)?;
    let mut profile: Value = serde_json::from_slice(&profile_bytes)
        .map_err(|e| CrewError::Json(format!("Fabric profil JSON hatası: {e}")))?;

    if profile["inheritsFrom"] != mc_version {
        return Err(CrewError::InvalidData(format!(
            "Fabric profili Minecraft {} için, istenen {mc_version} ile uyuşmuyor.",
            profile["inheritsFrom"]
        )));
    }

    let libraries = profile["libraries"]
        .as_array_mut()
        .ok_or_else(|| CrewError::InvalidData("Fabric profilinde 'libraries' alanı yok.".into()))?;

    log(format!(
        "Fabric {loader_version} için {} kütüphane çözümleniyor...",
        libraries.len()
    ));

    for library in libraries.iter_mut() {
        let name = library["name"]
            .as_str()
            .ok_or_else(|| CrewError::InvalidData("Kütüphane adı (name) eksik.".into()))?;

        let rel_maven = maven_path(name)?;
        let safe_rel = relative_path(&rel_maven)?;

        let repo = library["url"].as_str().unwrap_or(FABRIC_MAVEN_URL);
        let lib_url = format!("{}/{}", repo.trim_end_matches('/'), rel_maven);

        // Determine expected SHA-1
        let sha1_opt: Option<String> = match library["sha1"].as_str() {
            Some(s) if s.len() == 40 => Some(s.to_string()),
            _ => fetch(&format!("{lib_url}.sha1"))
                .ok()
                .and_then(|bytes| {
                    let s = String::from_utf8_lossy(&bytes);
                    s.split_whitespace().next().map(|w| w.to_string())
                })
                .filter(|h| h.len() == 40 && h.chars().all(|c| c.is_ascii_hexdigit())),
        };

        let file_path = game_dir.join("libraries").join(&safe_rel);

        // Check if file already exists and matches checksum
        let is_valid = if file_path.is_file() {
            if let Some(expected) = &sha1_opt {
                sha1_file(&file_path).map(|actual| actual.eq_ignore_ascii_case(expected)).unwrap_or(false)
            } else {
                true
            }
        } else {
            false
        };

        let final_sha1 = if !is_valid {
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let bytes = fetch(&lib_url)?;
            if let Some(expected) = &sha1_opt {
                if !verify_sha1(&bytes, expected) {
                    return Err(CrewError::ChecksumMismatch {
                        file: lib_url,
                        expected: expected.clone(),
                        actual: sha1_hex(&bytes),
                    });
                }
            }
            fs::write(&file_path, &bytes)?;
            sha1_opt.unwrap_or_else(|| sha1_hex(&bytes))
        } else {
            sha1_opt.unwrap_or_else(|| sha1_file(&file_path).unwrap_or_default())
        };

        // Standardize download metadata as Mojang format
        library["downloads"] = json!({
            "artifact": {
                "path": rel_maven,
                "url": lib_url,
                "sha1": final_sha1
            }
        });
    }

    // Write profile JSON to disk
    let safe_id_path = relative_path(&id)?;
    let profile_out_dir = game_dir.join("versions").join(&safe_id_path);
    fs::create_dir_all(&profile_out_dir)?;

    let out_file = profile_out_dir.join(format!("{id}.json"));
    let pretty_json = serde_json::to_vec_pretty(&profile)?;
    fs::write(&out_file, &pretty_json)?;

    log(format!("Fabric profili kaydedildi: {}", out_file.display()));
    Ok(id)
}
