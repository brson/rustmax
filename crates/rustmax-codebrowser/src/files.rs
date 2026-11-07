//! Discovery of the workspace files to mirror.

use crate::metadata::path_to_slash;
use rmx::ignore::WalkBuilder;
use rmx::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Files larger than this are listed but not rendered.
const MAX_RENDERED_SIZE: u64 = 1024 * 1024;

pub struct SourceFile {
    /// Workspace-relative path with `/` separators.
    pub path: String,
    pub size: u64,
    /// The contents, if the file is UTF-8 text small enough to render.
    pub text: Option<String>,
}

impl SourceFile {
    pub fn name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(&self.path)
    }

    pub fn dir(&self) -> &str {
        self.path.rsplit_once('/').map(|(d, _)| d).unwrap_or("")
    }
}

/// Walk the workspace, honoring `.gitignore` and skipping hidden files and `excludes`.
///
/// Returns files sorted by path.
pub fn walk(root: &Path, excludes: &[PathBuf]) -> AnyResult<Vec<SourceFile>> {
    let excludes: Vec<PathBuf> = excludes.iter().filter_map(|p| p.canonicalize().ok()).collect();
    let walker = WalkBuilder::new(root)
        .filter_entry(move |entry| {
            let Ok(path) = entry.path().canonicalize() else { return true };
            !excludes.iter().any(|e| path.starts_with(e))
        })
        .build();

    let mut files = Vec::new();
    for entry in walker {
        let entry = entry?;
        if !entry.file_type().is_some_and(|t| t.is_file()) {
            continue;
        }
        let rel = entry.path().strip_prefix(root)?;
        let size = entry.metadata()?.len();
        let text = if size <= MAX_RENDERED_SIZE {
            let bytes = fs::read(entry.path())
                .with_context(|| format!("failed to read {}", entry.path().display()))?;
            String::from_utf8(bytes).ok().filter(|t| !t.contains('\0'))
        } else {
            None
        };
        files.push(SourceFile { path: path_to_slash(rel), size, text });
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(files)
}
