//! Quilt Loader installer and profile synthesizer.

use crate::compatibility::validate_quilt_compatibility;
use crate::meta::quilt_profile_url;
use crew_core::{
    CrewError,
    checksum::{verify_sha1, sha1_hex},
    maven::MavenCoordinate,
};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

/// Sanitizes relative path components preventing path traversal attacks.
pub fn sanitize_relative_path(rel: &str) -> Result<PathBuf, CrewError> {
    let mut safe = PathBuf::new();
    for comp in Path::new(rel).components() {
        match comp {
            std::path::Component::Normal(c) => safe.push(c),
            std::path::Component::CurDir => {}
            _ => {
                return Err(CrewError::InvalidData(format!(
                    "Güvensiz göreceli dosya yolu: {rel}"
                )));
            }
        }
    }
    if safe.as_os_str().is_empty() {
        return Err(CrewError::InvalidData("Dosya yolu boş olamaz.".to_string()));
    }
    Ok(safe)
}

/// Installs Quilt Loader profile and libraries for a given Minecraft version.
pub fn install_quilt_profile<F, L>(
    mc_version: &str,
    loader_version: &str,
    game_dir: &Path,
    fetch: F,
    log: L,
) -> Result<String, CrewError>
where
    F: Fn(&str) -> Result<Vec<u8>, CrewError>,
    L: Fn(&str),
{
    // 1. Verify Minecraft version compatibility
    validate_quilt_compatibility(mc_version)?;

    // 2. Fetch Quilt profile JSON
    let profile_url = quilt_profile_url(mc_version, loader_version);
    let profile_bytes = fetch(&profile_url)?;
    let mut profile: Value = serde_json::from_slice(&profile_bytes)
        .map_err(|e| CrewError::Json(format!("Quilt profili çözümlenemedi: {e}")))?;

    // 3. Verify inheritance
    let inherits_from = profile["inheritsFrom"]
        .as_str()
        .ok_or_else(|| CrewError::InvalidData("Profilde inheritsFrom alanı yok.".to_string()))?;
    if inherits_from != mc_version {
        return Err(CrewError::InvalidData(format!(
            "Quilt profili Minecraft {mc_version} yerine {inherits_from} için geçerli."
        )));
    }

    let id = profile["id"]
        .as_str()
        .ok_or_else(|| CrewError::InvalidData("Profilde id alanı yok.".to_string()))?
        .to_string();

    let libraries = profile["libraries"]
        .as_array_mut()
        .ok_or_else(|| CrewError::InvalidData("Profilde libraries dizisi yok.".to_string()))?;

    log(&format!(
        "Quilt Loader {loader_version} indiriliyor ({} kütüphane)...",
        libraries.len()
    ));

    // 4. Download and verify libraries
    for library in libraries.iter_mut() {
        let coord_str = library["name"]
            .as_str()
            .ok_or_else(|| CrewError::InvalidData("Kütüphane koordinat ismi eksik.".to_string()))?;

        let coord = MavenCoordinate::parse(coord_str)?;
        let rel_maven_path = coord.to_path();
        let safe_rel = sanitize_relative_path(&rel_maven_path)?;

        let repo_url = library["url"]
            .as_str()
            .unwrap_or(crate::meta::QUILT_DEFAULT_MAVEN_URL);
        let artifact_url = format!("{}/{}", repo_url.trim_end_matches('/'), rel_maven_path);

        // Check if SHA-1 is specified or fetch from repository
        let sha1 = match library["sha1"].as_str() {
            Some(s) => Some(s.to_string()),
            None => {
                let sha1_url = format!("{artifact_url}.sha1");
                fetch(&sha1_url).ok().and_then(|bytes| {
                    let s = String::from_utf8_lossy(&bytes);
                    s.split_whitespace().next().map(|w| w.to_lowercase())
                }).filter(|h| h.len() == 40 && h.chars().all(|c| c.is_ascii_hexdigit()))
            }
        };

        let local_file = game_dir.join("libraries").join(&safe_rel);
        let matches_existing = if local_file.is_file() {
            if let Some(expected_sha1) = &sha1 {
                if let Ok(existing_bytes) = std::fs::read(&local_file) {
                    verify_sha1(&existing_bytes, expected_sha1)
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        };

        if !matches_existing {
            let data = fetch(&artifact_url)?;
            if let Some(expected_sha1) = &sha1 {
                if !verify_sha1(&data, expected_sha1) {
                    return Err(CrewError::ChecksumMismatch {
                        file: artifact_url.clone(),
                        expected: expected_sha1.clone(),
                        actual: sha1_hex(&data),
                    });
                }
            }

            if let Some(parent) = local_file.parent() {
                std::fs::create_dir_all(parent).map_err(|e| CrewError::Io(e.to_string()))?;
            }
            std::fs::write(&local_file, &data).map_err(|e| CrewError::Io(e.to_string()))?;
        }

        let computed_sha1 = match sha1 {
            Some(h) => h,
            None => {
                let bytes = std::fs::read(&local_file).map_err(|e| CrewError::Io(e.to_string()))?;
                sha1_hex(&bytes)
            }
        };

        library["downloads"] = json!({
            "artifact": {
                "path": rel_maven_path,
                "url": artifact_url,
                "sha1": computed_sha1
            }
        });
    }

    // 5. Write profile JSON
    let safe_id_path = sanitize_relative_path(&id)?;
    let profile_dir = game_dir.join("versions").join(&safe_id_path);
    std::fs::create_dir_all(&profile_dir).map_err(|e| CrewError::Io(e.to_string()))?;
    let profile_file = profile_dir.join(format!("{id}.json"));
    let pretty_json = serde_json::to_vec_pretty(&profile)
        .map_err(|e| CrewError::Json(format!("Profil JSON çıktısı oluşturulamadı: {e}")))?;
    std::fs::write(&profile_file, pretty_json).map_err(|e| CrewError::Io(e.to_string()))?;

    Ok(id)
}
