// Project scanning service.

use rmx::prelude::*;
use crate::infrastructure::state::AppState;

// Scan a directory for project analysis.
pub async fn scan_directory(
    path: &str,
    max_depth: Option<usize>,
    state: &AppState,
) -> crate::Result<()> {
    let _timer = crate::infrastructure::logging::PerfTimer::new("scan_directory");

    let config = state.config().await;
    let effective_depth = max_depth.unwrap_or(config.services.scanner.max_depth);

    rmx::log::info!("Scanning directory: {} (max depth: {})", path, effective_depth);

    // Use walkdir to traverse the directory.
    let mut file_count = 0;
    let mut dir_count = 0;
    let mut total_size = 0u64;

    for entry in rmx::walkdir::WalkDir::new(path)
        .max_depth(effective_depth)
        .follow_links(config.services.scanner.follow_symlinks)
    {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                // Skip ignored patterns.
                if should_ignore(&path, &config.services.scanner.ignore_patterns) {
                    continue;
                }

                if entry.file_type().is_file() {
                    file_count += 1;
                    if let Ok(metadata) = entry.metadata() {
                        total_size += metadata.len();
                    }
                } else if entry.file_type().is_dir() {
                    dir_count += 1;
                }

                // Log interesting files.
                if is_interesting_file(&path) {
                    rmx::log::debug!("Found: {}", path.display());
                }
            }
            Err(e) => {
                rmx::log::warn!("Scan error: {}", e);
            }
        }
    }

    println!("Scan Results:");
    println!("  Path: {}", path);
    println!("  Directories: {}", dir_count);
    println!("  Files: {}", file_count);
    println!("  Total size: {} bytes", total_size);

    // Cache the results (temporarily disabled due to serialization).
    // let results = ScanResults {
    //     path: path.to_string(),
    //     dir_count,
    //     file_count,
    //     total_size,
    // };

    // let cache_key = format!("scan:{}", path);
    // let cache_value = rmx::serde_json::to_vec(&results)?;
    // state.cache_set(cache_key, cache_value).await;

    Ok(())
}

// Check if a path should be ignored.
fn should_ignore(path: &std::path::Path, ignore_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();

    for pattern in ignore_patterns {
        if path_str.contains(pattern) {
            return true;
        }
    }

    false
}

// Check if a file is interesting for analysis.
fn is_interesting_file(path: &std::path::Path) -> bool {
    if let Some(extension) = path.extension() {
        matches!(extension.to_str(), Some("rs" | "toml" | "json" | "md" | "yml" | "yaml"))
    } else {
        matches!(path.file_name().and_then(|n| n.to_str()),
                Some("Cargo.toml" | "README.md" | ".gitignore" | "Dockerfile"))
    }
}

// Scan results structure.
#[derive(Debug, Clone)]
struct ScanResults {
    path: String,
    dir_count: usize,
    file_count: usize,
    total_size: u64,
}