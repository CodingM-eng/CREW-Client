use std::cmp::Ordering;

/// Structured version key for ordering: (numerical parts, is_stable, prerelease parts).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionKey {
    pub numbers: Vec<u64>,
    pub is_release: bool,
    pub prerelease: Vec<(u64, String)>,
}

/// Compute sortable key for a version string.
/// Release versions sort above pre-releases (`0.20.0` > `0.20.0-beta.9`).
/// Pre-releases sort by segment numbers (`beta.10` > `beta.9`).
pub fn version_key(version: &str) -> VersionKey {
    let (core, pre) = match version.split_once('-') {
        Some((core, pre)) => (core, Some(pre)),
        None => (version.split('+').next().unwrap_or(version), None),
    };

    let numbers: Vec<u64> = core
        .split('.')
        .map(|n| n.parse::<u64>().unwrap_or(0))
        .collect();

    let is_release = pre.is_none();

    let prerelease: Vec<(u64, String)> = pre
        .into_iter()
        .flat_map(|p| p.split(['.', '-', '+']))
        .map(|part| (part.parse::<u64>().unwrap_or(0), part.to_string()))
        .collect();

    VersionKey {
        numbers,
        is_release,
        prerelease,
    }
}

/// Compare two versions.
pub fn compare_versions(a: &str, b: &str) -> Ordering {
    let key_a = version_key(a);
    let key_b = version_key(b);

    // First compare core numbers
    match key_a.numbers.cmp(&key_b.numbers) {
        Ordering::Equal => {}
        non_eq => return non_eq,
    }

    // Release ranks higher than prerelease
    match key_a.is_release.cmp(&key_b.is_release) {
        Ordering::Equal => {}
        non_eq => return non_eq,
    }

    // Compare prerelease tokens
    key_a.prerelease.cmp(&key_b.prerelease)
}

/// Sort a slice of version strings.
/// When `newest_first` is true, larger versions come first.
pub fn sort_versions(versions: &mut [String], newest_first: bool) {
    if newest_first {
        versions.sort_by(|a, b| compare_versions(b, a));
    } else {
        versions.sort_by(|a, b| compare_versions(a, b));
    }
}
