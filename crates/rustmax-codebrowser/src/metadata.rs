//! Workspace structure from `cargo metadata`.

use rmx::prelude::*;
use rmx::serde_json;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

/// The subset of `cargo metadata` output the browser uses.
#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub packages: Vec<Package>,
    pub workspace_members: Vec<String>,
    pub workspace_root: PathBuf,
    pub target_directory: PathBuf,
    pub resolve: Option<Resolve>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub manifest_path: PathBuf,
    pub dependencies: Vec<Dependency>,
    pub targets: Vec<Target>,
    pub features: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub req: String,
    pub kind: Option<String>,
    pub optional: bool,
    pub target: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    pub name: String,
    pub kind: Vec<String>,
    pub src_path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Resolve {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub id: String,
    pub deps: Vec<NodeDep>,
}

#[derive(Debug, Deserialize)]
pub struct NodeDep {
    pub pkg: String,
}

impl Metadata {
    /// Run `cargo metadata` for the workspace containing `dir`.
    pub fn load(dir: &Path) -> AnyResult<Metadata> {
        let output = Command::new("cargo")
            .args(["metadata", "--format-version", "1"])
            .current_dir(dir)
            .output()
            .context("failed to run `cargo metadata`")?;
        if !output.status.success() {
            bail!(
                "`cargo metadata` failed:\n{}",
                String::from_utf8_lossy(&output.stderr),
            );
        }
        serde_json::from_slice(&output.stdout).context("failed to parse `cargo metadata` output")
    }

    /// Workspace member packages, sorted by name.
    pub fn members(&self) -> Vec<&Package> {
        let mut members: Vec<&Package> = self
            .packages
            .iter()
            .filter(|p| self.workspace_members.contains(&p.id))
            .collect();
        members.sort_by(|a, b| a.name.cmp(&b.name));
        members
    }

    /// The packages each package's dependencies resolved to, by package name.
    ///
    /// A package can depend on two versions of one crate;
    /// the last one in the resolve wins, which only affects the version shown.
    pub fn resolved_deps(&self) -> HashMap<&str, HashMap<&str, &Package>> {
        let by_id: HashMap<&str, &Package> =
            self.packages.iter().map(|p| (p.id.as_str(), p)).collect();
        let mut out = HashMap::new();
        for node in self.resolve.iter().flat_map(|r| &r.nodes) {
            let deps = node
                .deps
                .iter()
                .filter_map(|d| by_id.get(d.pkg.as_str()))
                .map(|p| (p.name.as_str(), *p))
                .collect();
            out.insert(node.id.as_str(), deps);
        }
        out
    }
}

impl Package {
    /// The package directory, relative to the workspace root, with `/` separators.
    ///
    /// The root package of a workspace has the empty path.
    pub fn rel_dir(&self, workspace_root: &Path) -> AnyResult<String> {
        let dir = self
            .manifest_path
            .parent()
            .ok_or_else(|| anyhow!("manifest path has no parent: {}", self.manifest_path.display()))?;
        let rel = dir.strip_prefix(workspace_root).with_context(|| {
            format!("package {} is outside the workspace root", self.name)
        })?;
        Ok(path_to_slash(rel))
    }
}

/// Render a relative path with `/` separators on every platform.
pub fn path_to_slash(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
