//! CREW Client Minecraft Version and Profile Management Engine
//!
//! Provides unified Minecraft version modeling, policy enforcement (>= 1.8.9),
//! recommended Java runtime mapping, modloader compatibility validation,
//! and profile inheritance resolution with library deduplication.

pub mod inheritance;
pub mod model;
pub mod policy;

pub use inheritance::{library_key, merge_version_profiles};
pub use model::{categorize_version, JavaRequirement, LoaderKind, VersionCategory};
pub use policy::{
    is_supported_mc_version, validate_loader_support, validate_version_support,
    MINIMUM_SUPPORTED_MC_VERSION,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_version_policy() {
        assert!(!is_supported_mc_version("1.7.10"));
        assert!(!is_supported_mc_version("1.8.8"));
        assert!(validate_version_support("1.7.10").is_err());

        assert!(is_supported_mc_version("1.8.9"));
        assert!(is_supported_mc_version("1.12.2"));
        assert!(is_supported_mc_version("1.16.5"));
        assert!(is_supported_mc_version("1.21.4"));
        assert!(validate_version_support("1.8.9").is_ok());
    }

    #[test]
    fn test_java_runtime_mapping() {
        assert_eq!(JavaRequirement::for_minecraft("1.8.9"), JavaRequirement::Java8);
        assert_eq!(JavaRequirement::for_minecraft("1.12.2"), JavaRequirement::Java8);
        assert_eq!(JavaRequirement::for_minecraft("1.16.5"), JavaRequirement::Java8);

        assert_eq!(JavaRequirement::for_minecraft("1.17"), JavaRequirement::Java16);
        assert_eq!(JavaRequirement::for_minecraft("1.17.1"), JavaRequirement::Java16);

        assert_eq!(JavaRequirement::for_minecraft("1.18"), JavaRequirement::Java17);
        assert_eq!(JavaRequirement::for_minecraft("1.19.4"), JavaRequirement::Java17);
        assert_eq!(JavaRequirement::for_minecraft("1.20.1"), JavaRequirement::Java17);
        assert_eq!(JavaRequirement::for_minecraft("1.20.4"), JavaRequirement::Java17);

        assert_eq!(JavaRequirement::for_minecraft("1.20.5"), JavaRequirement::Java21);
        assert_eq!(JavaRequirement::for_minecraft("1.20.6"), JavaRequirement::Java21);
        assert_eq!(JavaRequirement::for_minecraft("1.21.4"), JavaRequirement::Java21);
    }

    #[test]
    fn test_loader_compatibility_matrix() {
        // Vanilla
        assert!(validate_loader_support(LoaderKind::Vanilla, "1.8.9").is_ok());
        assert!(validate_loader_support(LoaderKind::Vanilla, "1.7.10").is_err());

        // Fabric (>= 1.14)
        assert!(validate_loader_support(LoaderKind::Fabric, "1.14").is_ok());
        assert!(validate_loader_support(LoaderKind::Fabric, "1.20.1").is_ok());
        assert!(validate_loader_support(LoaderKind::Fabric, "1.12.2").is_err());
        assert!(validate_loader_support(LoaderKind::Fabric, "1.8.9").is_err());

        // Quilt (>= 1.18.2)
        assert!(validate_loader_support(LoaderKind::Quilt, "1.18.2").is_ok());
        assert!(validate_loader_support(LoaderKind::Quilt, "1.21.4").is_ok());
        assert!(validate_loader_support(LoaderKind::Quilt, "1.18.1").is_err());
        assert!(validate_loader_support(LoaderKind::Quilt, "1.16.5").is_err());
    }

    #[test]
    fn test_profile_inheritance_merging() {
        let parent = json!({
            "id": "1.20.1",
            "type": "release",
            "mainClass": "net.minecraft.client.main.Main",
            "arguments": {
                "game": ["--version", "1.20.1"],
                "jvm": ["-Xss1M"]
            },
            "libraries": [
                {
                    "name": "org.ow2.asm:asm:9.2"
                },
                {
                    "name": "org.lwjgl:lwjgl:3.3.1"
                }
            ]
        });

        let child = json!({
            "id": "fabric-loader-0.15.11-1.20.1",
            "inheritsFrom": "1.20.1",
            "type": "release",
            "mainClass": "net.fabricmc.loader.impl.launch.knot.KnotClient",
            "arguments": {
                "jvm": ["-Dfabric.dli.env=client"]
            },
            "libraries": [
                {
                    "name": "org.ow2.asm:asm:9.6"
                },
                {
                    "name": "net.fabricmc:sponge-mixin:0.12.5"
                }
            ]
        });

        let merged = merge_version_profiles(&parent, &child).expect("Merge failed");

        assert_eq!(merged["id"], "fabric-loader-0.15.11-1.20.1");
        assert_eq!(merged["mainClass"], "net.fabricmc.loader.impl.launch.knot.KnotClient");
        assert_eq!(merged["jar"], "1.20.1");

        // Libraries: child's asm:9.6 should override parent's asm:9.2, total libs = 3
        let libs = merged["libraries"].as_array().expect("Libraries not array");
        assert_eq!(libs.len(), 3);
        assert_eq!(libs[0]["name"], "org.ow2.asm:asm:9.6");
        assert_eq!(libs[1]["name"], "net.fabricmc:sponge-mixin:0.12.5");
        assert_eq!(libs[2]["name"], "org.lwjgl:lwjgl:3.3.1");

        // Arguments: jvm arguments should be merged
        let jvm_args = merged["arguments"]["jvm"].as_array().expect("JVM args not array");
        assert_eq!(jvm_args.len(), 2);
        assert_eq!(jvm_args[0], "-Xss1M");
        assert_eq!(jvm_args[1], "-Dfabric.dli.env=client");
    }
}
