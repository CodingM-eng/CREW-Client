//! CREW Client Quilt Mod Loader Engine
//!
//! Provides Quilt Meta API version discovery, semver/beta ordering,
//! Minecraft compatibility validation (>= 1.18.2), and profile/library installation.

pub mod compatibility;
pub mod installer;
pub mod meta;

pub use compatibility::{
    is_quilt_supported, validate_quilt_compatibility, MINIMUM_QUILT_MC_VERSION,
};
pub use installer::{install_quilt_profile, sanitize_relative_path};
pub use meta::{
    extract_quilt_libraries, parse_quilt_loader_versions, pick_quilt_loader_version,
    quilt_profile_id, quilt_profile_url, quilt_versions_url, sort_quilt_loader_versions,
    QuiltVersionRecord, QUILT_DEFAULT_MAVEN_URL, QUILT_META_BASE_URL,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crew_core::checksum::sha1_hex;
    use std::collections::HashMap;

    #[test]
    fn test_quilt_compatibility() {
        // Quilt requires MC >= 1.18.2
        assert!(!is_quilt_supported("1.8.9"));
        assert!(!is_quilt_supported("1.12.2"));
        assert!(!is_quilt_supported("1.16.5"));
        assert!(!is_quilt_supported("1.17.1"));
        assert!(!is_quilt_supported("1.18.1"));

        assert!(validate_quilt_compatibility("1.16.5").is_err());
        assert!(validate_quilt_compatibility("1.18.1").is_err());

        // Supported versions
        assert!(is_quilt_supported("1.18.2"));
        assert!(is_quilt_supported("1.19.4"));
        assert!(is_quilt_supported("1.20.1"));
        assert!(is_quilt_supported("1.21.4"));
        assert!(validate_quilt_compatibility("1.20.1").is_ok());
    }

    #[test]
    fn test_parse_and_sort_quilt_versions() {
        let json_data = r#"[
            {
                "loader": {
                    "version": "0.20.0-beta.9",
                    "build": 9
                }
            },
            {
                "loader": {
                    "version": "0.27.1",
                    "build": 27
                }
            },
            {
                "loader": {
                    "version": "0.26.0",
                    "build": 26
                }
            }
        ]"#;

        let records = parse_quilt_loader_versions(json_data).expect("Failed to parse versions");
        assert_eq!(records.len(), 3);

        // Sorting: 0.27.1 > 0.26.0 > 0.20.0-beta.9
        assert_eq!(records[0].version, "0.27.1");
        assert!(records[0].stable);

        assert_eq!(records[1].version, "0.26.0");
        assert!(records[1].stable);

        assert_eq!(records[2].version, "0.20.0-beta.9");
        assert!(!records[2].stable); // beta contains '-'

        // Pick latest
        let latest = pick_quilt_loader_version(&records, None).expect("Failed to pick latest");
        assert_eq!(latest.version, "0.27.1");

        // Pick specific
        let specific = pick_quilt_loader_version(&records, Some("0.20.0-beta.9"))
            .expect("Failed to pick specific");
        assert_eq!(specific.version, "0.20.0-beta.9");

        // Pick unknown
        assert!(pick_quilt_loader_version(&records, Some("9.9.9")).is_err());
    }

    #[test]
    fn test_quilt_profile_id_and_urls() {
        let id = quilt_profile_id("0.27.1", "1.21.4");
        assert_eq!(id, "quilt-loader-0.27.1-1.21.4");

        let p_url = quilt_profile_url("1.21.4", "0.27.1");
        assert_eq!(
            p_url,
            "https://meta.quiltmc.org/v3/versions/loader/1.21.4/0.27.1/profile/json"
        );

        let v_url = quilt_versions_url("1.21.4");
        assert_eq!(v_url, "https://meta.quiltmc.org/v3/versions/loader/1.21.4");
    }

    #[test]
    fn test_mock_quilt_installation() {
        let tmp = std::env::temp_dir().join(format!("crew_quilt_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).expect("Failed to create temp dir");

        let mc_ver = "1.20.1";
        let loader_ver = "0.27.1";
        let expected_profile_id = format!("quilt-loader-{loader_ver}-{mc_ver}");

        let lib_data = b"dummy quilt jar content";
        let lib_sha1 = sha1_hex(lib_data);

        let profile_json_str = format!(
            r#"{{
                "id": "{expected_profile_id}",
                "inheritsFrom": "{mc_ver}",
                "type": "release",
                "mainClass": "org.quiltmc.loader.impl.launch.knot.KnotClient",
                "libraries": [
                    {{
                        "name": "org.quiltmc:quilt-loader:{loader_ver}",
                        "url": "https://maven.quiltmc.org/repository/release/",
                        "sha1": "{lib_sha1}"
                    }}
                ]
            }}"#
        );

        let mut mock_server: HashMap<String, Vec<u8>> = HashMap::new();
        let prof_url = quilt_profile_url(mc_ver, loader_ver);
        mock_server.insert(prof_url, profile_json_str.into_bytes());

        let artifact_url = format!(
            "https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-loader/{loader_ver}/quilt-loader-{loader_ver}.jar"
        );
        mock_server.insert(artifact_url, lib_data.to_vec());

        let fetch = move |url: &str| -> Result<Vec<u8>, crew_core::CrewError> {
            mock_server
                .get(url)
                .cloned()
                .ok_or_else(|| crew_core::CrewError::VersionNotFound(format!("Not found: {url}")))
        };

        let result = install_quilt_profile(mc_ver, loader_ver, &tmp, fetch, |_| {});
        assert!(result.is_ok(), "Installation failed: {:?}", result.err());
        assert_eq!(result.unwrap(), expected_profile_id);

        // Check profile file
        let profile_file = tmp
            .join("versions")
            .join(&expected_profile_id)
            .join(format!("{expected_profile_id}.json"));
        assert!(profile_file.is_file(), "Profile file not generated");

        // Check library file
        let lib_file = tmp
            .join("libraries")
            .join("org")
            .join("quiltmc")
            .join("quilt-loader")
            .join(loader_ver)
            .join(format!("quilt-loader-{loader_ver}.jar"));
        assert!(lib_file.is_file(), "Library file not downloaded");
        assert_eq!(std::fs::read(&lib_file).unwrap(), lib_data);

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
