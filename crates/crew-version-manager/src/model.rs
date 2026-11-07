//! Data models and Java runtime requirements for Minecraft versions.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported mod loader kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoaderKind {
    Vanilla,
    Fabric,
    Quilt,
}

impl LoaderKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Vanilla => "Vanilla",
            Self::Fabric => "Fabric",
            Self::Quilt => "Quilt",
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Vanilla => "vanilla",
            Self::Fabric => "fabric",
            Self::Quilt => "quilt",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "vanilla" => Some(Self::Vanilla),
            "fabric" => Some(Self::Fabric),
            "quilt" => Some(Self::Quilt),
            _ => None,
        }
    }
}

impl fmt::Display for LoaderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Java runtime version requirement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum JavaRequirement {
    Java8 = 8,
    Java16 = 16,
    Java17 = 17,
    Java21 = 21,
}

impl JavaRequirement {
    pub fn major_version(self) -> u32 {
        self as u32
    }

    /// Determines the recommended Java version for a given Minecraft release.
    pub fn for_minecraft(mc_version: &str) -> Self {
        use crew_core::version_order::compare_versions;
        use std::cmp::Ordering;

        let clean = mc_version.trim();
        let base = clean.split('-').next().unwrap_or(clean);

        // Minecraft 1.20.5+ requires Java 21
        if compare_versions(base, "1.20.5") != Ordering::Less {
            JavaRequirement::Java21
        }
        // Minecraft 1.18 to 1.20.4 requires Java 17
        else if compare_versions(base, "1.18") != Ordering::Less {
            JavaRequirement::Java17
        }
        // Minecraft 1.17 requires Java 16
        else if compare_versions(base, "1.17") != Ordering::Less {
            JavaRequirement::Java16
        }
        // Minecraft <= 1.16.5 requires Java 8
        else {
            JavaRequirement::Java8
        }
    }
}

/// Version release category.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionCategory {
    Release,
    Snapshot,
    OldBeta,
    OldAlpha,
    Custom,
}

/// Categorizes a version string based on naming conventions.
pub fn categorize_version(version: &str) -> VersionCategory {
    let lower = version.to_ascii_lowercase();
    if lower.starts_with('b') && lower.chars().nth(1).map_or(false, |c| c.is_ascii_digit()) {
        VersionCategory::OldBeta
    } else if lower.starts_with('a') && lower.chars().nth(1).map_or(false, |c| c.is_ascii_digit()) {
        VersionCategory::OldAlpha
    } else if lower.contains("w") && lower.len() >= 6 && lower.chars().take(2).all(|c| c.is_ascii_digit()) {
        VersionCategory::Snapshot
    } else if lower.contains("-pre") || lower.contains("-rc") {
        VersionCategory::Snapshot
    } else {
        VersionCategory::Release
    }
}
