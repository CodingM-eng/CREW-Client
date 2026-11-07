//! CREW Client Core Crate
//! Foundational types, Maven coordinate resolver, SHA-1/SHA-256 checksums,
//! semantic version ordering, and HTTP transport.

pub mod checksum;
pub mod error;
pub mod http;
pub mod maven;
pub mod version_order;

pub use checksum::{sha1_file, sha1_hex, sha256_file, sha256_hex, verify_sha1, verify_sha256};
pub use error::CrewError;
pub use http::HttpClient;
pub use maven::{maven_path, MavenCoordinate};
pub use version_order::{compare_versions, sort_versions, version_key};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksums() {
        let data = b"Hello, CREW Client!";
        let sha1 = sha1_hex(data);
        let sha256 = sha256_hex(data);

        assert_eq!(sha1.len(), 40);
        assert_eq!(sha256.len(), 64);

        assert!(verify_sha1(data, &sha1));
        assert!(verify_sha1(data, &sha1.to_uppercase()));
        assert!(!verify_sha1(data, "0000000000000000000000000000000000000000"));

        assert!(verify_sha256(data, &sha256));
        assert!(verify_sha256(data, &sha256.to_uppercase()));
        assert!(!verify_sha256(data, "0000000000000000000000000000000000000000000000000000000000000000"));
    }

    #[test]
    fn test_maven_parsing_and_paths() {
        let coord = MavenCoordinate::parse("net.fabricmc:fabric-loader:0.16.9").unwrap();
        assert_eq!(coord.group, "net.fabricmc");
        assert_eq!(coord.artifact, "fabric-loader");
        assert_eq!(coord.version, "0.16.9");
        assert_eq!(coord.classifier, None);
        assert_eq!(coord.extension, "jar");
        assert_eq!(
            coord.to_path(),
            "net/fabricmc/fabric-loader/0.16.9/fabric-loader-0.16.9.jar"
        );
        assert_eq!(
            coord.to_url("https://maven.fabricmc.net/"),
            "https://maven.fabricmc.net/net/fabricmc/fabric-loader/0.16.9/fabric-loader-0.16.9.jar"
        );

        // With classifier and custom extension
        let native =
            MavenCoordinate::parse("org.lwjgl:lwjgl:3.3.3:natives-windows@zip").unwrap();
        assert_eq!(native.classifier.as_deref(), Some("natives-windows"));
        assert_eq!(native.extension, "zip");
        assert_eq!(
            native.to_path(),
            "org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows.zip"
        );

        // Invalid coordinates
        assert!(MavenCoordinate::parse("not-a-valid-coord").is_err());
        assert!(MavenCoordinate::parse("group:only").is_err());
    }

    #[test]
    fn test_version_order() {
        use std::cmp::Ordering;

        // Release > Prerelease
        assert_eq!(compare_versions("0.16.9", "0.17.0-beta.1"), Ordering::Less);
        assert_eq!(compare_versions("0.20.0", "0.20.0-beta.9"), Ordering::Greater);
        assert_eq!(compare_versions("0.20.0-beta.10", "0.20.0-beta.9"), Ordering::Greater);
        assert_eq!(compare_versions("1.21.10", "1.21.2"), Ordering::Greater);
        assert_eq!(compare_versions("1.8.9", "1.8.9"), Ordering::Equal);

        let mut versions = vec![
            "0.9.0".to_string(),
            "0.24.0".to_string(),
            "0.20.0-beta.9".to_string(),
            "0.20.0".to_string(),
            "0.20.0-beta.10".to_string(),
            "0.30.1-beta.1".to_string(),
        ];
        sort_versions(&mut versions, true);
        assert_eq!(
            versions,
            vec![
                "0.30.1-beta.1",
                "0.24.0",
                "0.20.0",
                "0.20.0-beta.10",
                "0.20.0-beta.9",
                "0.9.0"
            ]
        );
    }

    #[test]
    fn test_error_display() {
        let err = CrewError::ChecksumMismatch {
            file: "test.jar".into(),
            expected: "abc".into(),
            actual: "def".into(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("test.jar"));
        assert!(msg.contains("Beklenen abc"));
    }
}
