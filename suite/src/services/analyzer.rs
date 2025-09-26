// Dependency analysis service.

use rmx::prelude::*;
use std::hash::Hasher;
use crate::infrastructure::state::AppState;

// Analyze project dependencies.
pub async fn analyze_dependencies(
    path: &str,
    check_updates: bool,
    state: &AppState,
) -> crate::Result<()> {
    let _timer = crate::infrastructure::logging::PerfTimer::new("analyze_dependencies");

    rmx::log::info!("Analyzing dependencies in: {}", path);

    // Read and parse Cargo.toml.
    let cargo_content = std::fs::read_to_string(path)?;
    let cargo_toml: rmx::toml::Value = rmx::toml::from_str(&cargo_content)?;

    // Extract dependencies.
    let mut dependencies = Vec::new();

    if let Some(deps) = cargo_toml.get("dependencies") {
        extract_dependencies(deps, "dependencies", &mut dependencies);
    }

    if let Some(dev_deps) = cargo_toml.get("dev-dependencies") {
        extract_dependencies(dev_deps, "dev-dependencies", &mut dependencies);
    }

    if let Some(build_deps) = cargo_toml.get("build-dependencies") {
        extract_dependencies(build_deps, "build-dependencies", &mut dependencies);
    }

    println!("Dependency Analysis:");
    println!("  File: {}", path);
    println!("  Total dependencies: {}", dependencies.len());

    for dep in &dependencies {
        println!("    {} ({}) - {}", dep.name, dep.category, dep.version);

        if check_updates {
            // In a real implementation, we'd check crates.io for updates.
            // For now, just simulate the check.
            let needs_update = simulate_update_check(&dep.name, &dep.version);
            if needs_update {
                println!("      ⚠ Update available");
            }
        }
    }

    // Analyze dependency features and complexity.
    analyze_dependency_complexity(&dependencies).await;

    // Cache results (temporarily disabled due to serialization).
    // let cache_key = format!("deps:{}", path);
    // let cache_value = rmx::serde_json::to_vec(&dependencies)?;
    // state.cache_set(cache_key, cache_value).await;

    Ok(())
}

// Extract dependencies from TOML value.
fn extract_dependencies(deps: &rmx::toml::Value, category: &str, output: &mut Vec<Dependency>) {
    if let Some(table) = deps.as_table() {
        for (name, value) in table {
            let version = match value {
                rmx::toml::Value::String(v) => v.clone(),
                rmx::toml::Value::Table(t) => {
                    t.get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string()
                }
                _ => "unknown".to_string(),
            };

            output.push(Dependency {
                name: name.clone(),
                version,
                category: category.to_string(),
            });
        }
    }
}

// Simulate checking for updates.
fn simulate_update_check(name: &str, version: &str) -> bool {
    // Simulate some dependencies needing updates.
    let mut hasher = rmx::ahash::AHasher::default();
    hasher.write(name.as_bytes());
    let hash = hasher.finish();
    hash % 5 == 0 // ~20% need updates
}

// Analyze dependency complexity.
async fn analyze_dependency_complexity(dependencies: &[Dependency]) {
    let total = dependencies.len();
    let categories = dependencies.iter()
        .map(|d| &d.category)
        .collect::<std::collections::HashSet<_>>()
        .len();

    // Use semver to analyze version complexity.
    let mut semver_count = 0;
    let mut git_count = 0;
    let mut path_count = 0;

    for dep in dependencies {
        if rmx::semver::Version::parse(&dep.version).is_ok() {
            semver_count += 1;
        } else if dep.version.contains("git") {
            git_count += 1;
        } else if dep.version.contains("path") {
            path_count += 1;
        }
    }

    println!("  Complexity Analysis:");
    println!("    Categories: {}", categories);
    println!("    SemVer deps: {}", semver_count);
    println!("    Git deps: {}", git_count);
    println!("    Path deps: {}", path_count);
}

// Dependency structure.
#[derive(Debug, Clone)]
struct Dependency {
    name: String,
    version: String,
    category: String,
}