//! JSON parsing for rustdoc output.

use rmx::prelude::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Load and parse rustdoc JSON from a file path.
pub fn load_json(path: &Path) -> AnyResult<rustdoc_types::Crate> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read rustdoc JSON from {}", path.display()))?;
    parse_json(&content)
}

/// Load and parse rustdoc JSON from bytes.
pub fn load_bytes(json: &[u8]) -> AnyResult<rustdoc_types::Crate> {
    let content = std::str::from_utf8(json)
        .context("Invalid UTF-8 in rustdoc JSON")?;
    parse_json(content)
}

/// Load all rustdoc JSON files from a directory.
///
/// Returns a map from crate name to parsed crate data.
/// Any file that fails to parse is an error,
/// since it usually means the JSON format has drifted from `rustdoc-types`.
pub fn load_json_dir(dir: &Path) -> AnyResult<BTreeMap<String, rustdoc_types::Crate>> {
    let mut crates = BTreeMap::new();

    for entry in fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let krate = load_json(&path)
                .with_context(|| format!("Failed to load {}", path.display()))?;
            let name = krate.index.get(&krate.root)
                .and_then(|item| item.name.clone())
                .unwrap_or_else(|| {
                    path.file_stem()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_default()
                });
            crates.insert(name, krate);
        }
    }

    Ok(crates)
}

/// Parse rustdoc JSON with no recursion limit.
///
/// Deeply nested types, like typenum's, exceed serde_json's default limit,
/// so the stack is grown on demand instead.
fn parse_json(content: &str) -> AnyResult<rustdoc_types::Crate> {
    let mut de = serde_json::Deserializer::from_str(content);
    de.disable_recursion_limit();
    let de = serde_stacker::Deserializer::new(&mut de);
    let krate = rustdoc_types::Crate::deserialize(de)
        .context("Failed to parse rustdoc JSON")?;
    Ok(krate)
}
