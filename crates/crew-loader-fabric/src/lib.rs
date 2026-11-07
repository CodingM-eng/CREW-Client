//! CREW Loader Fabric Engine
//! Dedicated Fabric Loader version discovery, profile generation, dependency
//! resolution, and installation.

pub mod compatibility;
pub mod installer;
pub mod meta;

pub use compatibility::{check_compatibility, FABRIC_MINIMUM_VERSION};
pub use installer::{install_fabric, FetchFn};
pub use meta::{
    parse_loader_versions, pick_loader_version, profile_id, profile_url, FabricGameVersion,
    FabricLoaderVersion, FABRIC_MAVEN_URL, FABRIC_META_URL,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crew_core::checksum::sha1_hex;
    use serde_json::json;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_fabric_compatibility() {
        assert!(check_compatibility("1.21.4").is_ok());
        assert!(check_compatibility("1.20.1").is_ok());
        assert!(check_compatibility("1.14").is_ok());
        assert!(check_compatibility("1.12.2").is_err());
        assert!(check_compatibility("1.8.9").is_err());
    }

    #[test]
    fn test_parse_and_pick_loader_versions() {
        let json_data = r#"[
            {"loader": {"version": "0.17.0-beta.1", "stable": false}},
            {"loader": {"version": "0.16.9", "stable": true}},
            {"loader": {"version": "0.15.11", "stable": true}}
        ]"#;

        let versions = parse_loader_versions(json_data).unwrap();
        assert_eq!(versions.len(), 3);
        assert_eq!(versions[0].version, "0.17.0-beta.1");
        assert_eq!(versions[1].version, "0.16.9");

        // Pick latest stable
        assert_eq!(pick_loader_version(&versions, None).unwrap(), "0.16.9");
        assert_eq!(pick_loader_version(&versions, Some("latest")).unwrap(), "0.16.9");

        // Pick explicit version
        assert_eq!(
            pick_loader_version(&versions, Some("0.17.0-beta.1")).unwrap(),
            "0.17.0-beta.1"
        );

        // Pick non-existent version
        assert!(pick_loader_version(&versions, Some("0.99.99")).is_err());
    }

    #[test]
    fn test_fabric_profile_id() {
        assert_eq!(
            profile_id("0.16.9", "1.21.4"),
            "fabric-loader-0.16.9-1.21.4"
        );
    }

    #[test]
    fn test_mock_fabric_installation() {
        let temp_dir = std::env::temp_dir().join(format!("crew-fabric-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);

        let loader_bytes = b"mock-fabric-loader-jar".to_vec();
        let asm_bytes = b"mock-asm-jar".to_vec();

        let loader_sha1 = sha1_hex(&loader_bytes);
        let asm_sha1 = sha1_hex(&asm_bytes);

        let profile_json = json!({
            "id": "fabric-loader-0.16.9-1.21.4",
            "inheritsFrom": "1.21.4",
            "mainClass": "net.fabricmc.loader.impl.launch.knot.KnotClient",
            "libraries": [
                {
                    "name": "net.fabricmc:fabric-loader:0.16.9",
                    "url": "https://maven.fabricmc.net/",
                    "sha1": loader_sha1
                },
                {
                    "name": "org.ow2.asm:asm:9.7",
                    "url": "https://maven.fabricmc.net/"
                }
            ]
        });

        let mut mock_server: HashMap<String, Vec<u8>> = HashMap::new();
        mock_server.insert(
            profile_url("1.21.4", "0.16.9"),
            profile_json.to_string().into_bytes(),
        );
        mock_server.insert(
            "https://maven.fabricmc.net/net/fabricmc/fabric-loader/0.16.9/fabric-loader-0.16.9.jar".into(),
            loader_bytes.clone(),
        );
        mock_server.insert(
            "https://maven.fabricmc.net/org/ow2/asm/asm/9.7/asm-9.7.jar".into(),
            asm_bytes.clone(),
        );
        mock_server.insert(
            "https://maven.fabricmc.net/org/ow2/asm/asm/9.7/asm-9.7.jar.sha1".into(),
            format!("{asm_sha1}  asm-9.7.jar").into_bytes(),
        );

        let fetch = move |url: &str| -> Result<Vec<u8>, crew_core::CrewError> {
            mock_server
                .get(url)
                .cloned()
                .ok_or_else(|| crew_core::CrewError::Network(format!("404 Not Found: {url}")))
        };

        let result = install_fabric("1.21.4", "0.16.9", &temp_dir, &fetch, &|_| {});
        assert_eq!(result.unwrap(), "fabric-loader-0.16.9-1.21.4");

        // Verify generated files
        let installed_profile = temp_dir
            .join("versions")
            .join("fabric-loader-0.16.9-1.21.4")
            .join("fabric-loader-0.16.9-1.21.4.json");
        assert!(installed_profile.is_file());

        let installed_loader_jar = temp_dir
            .join("libraries")
            .join("net/fabricmc/fabric-loader/0.16.9/fabric-loader-0.16.9.jar");
        assert!(installed_loader_jar.is_file());

        let installed_asm_jar = temp_dir
            .join("libraries")
            .join("org/ow2/asm/asm/9.7/asm-9.7.jar");
        assert!(installed_asm_jar.is_file());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
