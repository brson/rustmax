//! JSON parsing for rustdoc output.

use rmx::prelude::*;
use std::collections::HashMap;
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
/// Files that fail to parse are skipped with a warning.
pub fn load_json_dir(dir: &Path) -> AnyResult<HashMap<String, rustdoc_types::Crate>> {
    let mut crates = HashMap::new();
    let mut skipped = 0;

    for entry in fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|e| e == "json").unwrap_or(false) {
            match load_json(&path) {
                Ok(krate) => {
                    let name = krate.index.get(&krate.root)
                        .and_then(|item| item.name.clone())
                        .unwrap_or_else(|| {
                            path.file_stem()
                                .map(|s| s.to_string_lossy().into_owned())
                                .unwrap_or_default()
                        });
                    crates.insert(name, krate);
                }
                Err(e) => {
                    eprintln!("Warning: skipping {} ({})", path.display(), e);
                    skipped += 1;
                }
            }
        }
    }

    if skipped > 0 {
        eprintln!("Skipped {} files due to parse errors.", skipped);
    }

    Ok(crates)
}

fn parse_json(content: &str) -> AnyResult<rustdoc_types::Crate> {
    rmx::serde_json::from_str(content)
        .context("Failed to parse rustdoc JSON")
}
