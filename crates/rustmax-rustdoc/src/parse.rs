//! JSON parsing for rustdoc output.

use rmx::prelude::*;
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

fn parse_json(content: &str) -> AnyResult<rustdoc_types::Crate> {
    rmx::serde_json::from_str(content)
        .context("Failed to parse rustdoc JSON")
}
