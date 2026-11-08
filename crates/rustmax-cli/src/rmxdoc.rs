//! Generating rustdoc JSON for `rustmax rmxdoc`.

use rmx::prelude::*;
use rmx::serde_json;
use rustmax_rustdoc::JSON_TOOLCHAIN;
use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// What to document, as passed through to `cargo doc`.
pub struct Selection<'a> {
    /// Packages to document. Empty means the whole workspace.
    pub packages: &'a [String],
    pub features: &'a [String],
    pub all_features: bool,
    pub no_default_features: bool,
    /// Document only the selected packages, not their dependencies.
    pub no_deps: bool,
}

/// The parts of `cargo metadata` that `rmxdoc` needs.
pub struct Workspace {
    pub target_directory: PathBuf,
    /// Crate names of the workspace members' library and binary targets.
    pub member_crates: Vec<String>,
}

/// Read the current workspace from `cargo metadata`.
pub fn workspace() -> AnyResult<Workspace> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .context("failed to run `cargo metadata`")?;
    if !output.status.success() {
        bail!("`cargo metadata` failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let Some(target_directory) = metadata["target_directory"].as_str() else {
        bail!("`cargo metadata` output has no target_directory");
    };

    let mut member_crates = BTreeSet::new();
    for package in metadata["packages"].as_array().into_iter().flatten() {
        for target in package["targets"].as_array().into_iter().flatten() {
            let documented = target["kind"].as_array().into_iter().flatten().any(|kind| {
                matches!(kind.as_str(), Some("lib" | "rlib" | "proc-macro" | "bin"))
            });
            if let (true, Some(name)) = (documented, target["name"].as_str()) {
                member_crates.insert(name.replace('-', "_"));
            }
        }
    }

    Ok(Workspace {
        target_directory: PathBuf::from(target_directory),
        member_crates: member_crates.into_iter().collect(),
    })
}

/// Run `cargo doc` with JSON output into `target_dir`.
///
/// Returns the JSON files cargo reports for this build,
/// so files left over from earlier builds with other dependencies are not included.
pub fn generate_json(selection: &Selection, target_dir: &std::path::Path) -> AnyResult<Vec<PathBuf>> {
    let mut cmd = Command::new("cargo");
    cmd.arg(format!("+{JSON_TOOLCHAIN}"))
        .args(["doc", "-Zunstable-options", "--output-format", "json"])
        .args(["--message-format", "json-render-diagnostics"])
        .arg("--target-dir")
        .arg(target_dir);
    if selection.packages.is_empty() {
        cmd.arg("--workspace");
    }
    for package in selection.packages {
        cmd.args(["--package", package]);
    }
    for features in selection.features {
        cmd.args(["--features", features]);
    }
    if selection.all_features {
        cmd.arg("--all-features");
    }
    if selection.no_default_features {
        cmd.arg("--no-default-features");
    }
    if selection.no_deps {
        cmd.arg("--no-deps");
    }

    let mut child = cmd
        .stdout(Stdio::piped())
        .spawn()
        .context("failed to run `cargo doc`")?;
    let stdout = child.stdout.take().expect("stdout is piped");

    let mut paths = BTreeSet::new();
    for line in BufReader::new(stdout).lines() {
        let message: serde_json::Value = serde_json::from_str(&line?)?;
        if message["reason"] != "compiler-artifact" {
            continue;
        }
        let Some(filenames) = message["filenames"].as_array() else {
            continue;
        };
        for filename in filenames.iter().filter_map(|f| f.as_str()) {
            if filename.ends_with(".json") {
                paths.insert(PathBuf::from(filename));
            }
        }
    }

    let status = child.wait()?;
    if !status.success() {
        bail!("`cargo doc` failed");
    }
    Ok(paths.into_iter().collect())
}
