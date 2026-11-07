//! Profile inheritance and library deduplication engine.

use crew_core::CrewError;
use serde_json::{Value, json};
use std::collections::HashSet;

/// Extracts a normalized key for a library entry: `group:artifact[:classifier]`.
/// Version is excluded so that newer or loader-provided libraries replace vanilla ones.
pub fn library_key(library: &Value) -> String {
    let name = library["name"].as_str().unwrap_or_default();
    let (coordinates, _) = name.split_once('@').unwrap_or((name, "jar"));
    let mut parts = coordinates.split(':');
    let group = parts.next().unwrap_or_default();
    let artifact = parts.next().unwrap_or_default();
    let _version = parts.next();
    let classifier = parts.next();

    match classifier {
        Some(c) => format!("{group}:{artifact}:{c}"),
        None => format!("{group}:{artifact}"),
    }
}

/// Merges a child profile (e.g. Fabric or Quilt) with its parent version JSON (e.g. Mojang Vanilla).
///
/// Follows official launcher inheritance specification:
/// 1. Child libraries take precedence and appear first; parent libraries with matching keys are discarded.
/// 2. Arguments are concatenated: parent arguments followed by child arguments.
/// 3. MainClass from child overrides parent.
/// 4. `jar` points to parent's jar if not explicitly overridden.
pub fn merge_version_profiles(parent: &Value, child: &Value) -> Result<Value, CrewError> {
    let mut merged = parent.clone();

    // ID is child's profile ID
    if let Some(child_id) = child["id"].as_str() {
        merged["id"] = json!(child_id);
    }

    // Type (e.g. release)
    if let Some(child_type) = child["type"].as_str() {
        merged["type"] = json!(child_type);
    }

    // Main class
    if let Some(main_class) = child["mainClass"].as_str() {
        merged["mainClass"] = json!(main_class);
    }

    // Preserve parent jar reference if child didn't specify one
    if child["jar"].is_null() && merged["jar"].is_null() {
        if let Some(parent_id) = parent["id"].as_str() {
            merged["jar"] = json!(parent_id);
        }
    }

    // Java Version override if specified in child
    if !child["javaVersion"].is_null() {
        merged["javaVersion"] = child["javaVersion"].clone();
    }

    // Merge libraries: Child libraries first, then parent libraries not superseded by child
    let child_libs: Vec<Value> = child["libraries"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    let mut seen_keys = HashSet::new();
    let mut merged_libs = Vec::new();

    for lib in child_libs {
        let key = library_key(&lib);
        seen_keys.insert(key);
        merged_libs.push(lib);
    }

    if let Some(parent_libs) = parent["libraries"].as_array() {
        for lib in parent_libs {
            let key = library_key(lib);
            if !seen_keys.contains(&key) {
                seen_keys.insert(key);
                merged_libs.push(lib.clone());
            }
        }
    }

    merged["libraries"] = json!(merged_libs);

    // Merge arguments: Parent args + Child args
    if child.get("arguments").is_some() || parent.get("arguments").is_some() {
        let mut game_args = Vec::new();
        let mut jvm_args = Vec::new();

        // 1. Parent arguments
        if let Some(args) = parent.get("arguments") {
            if let Some(g) = args["game"].as_array() {
                game_args.extend(g.clone());
            }
            if let Some(j) = args["jvm"].as_array() {
                jvm_args.extend(j.clone());
            }
        }

        // 2. Child arguments
        if let Some(args) = child.get("arguments") {
            if let Some(g) = args["game"].as_array() {
                game_args.extend(g.clone());
            }
            if let Some(j) = args["jvm"].as_array() {
                jvm_args.extend(j.clone());
            }
        }

        merged["arguments"] = json!({
            "game": game_args,
            "jvm": jvm_args
        });
    }

    // Legacy minecraftArguments support
    if let Some(child_args) = child["minecraftArguments"].as_str() {
        merged["minecraftArguments"] = json!(child_args);
    }

    Ok(merged)
}
