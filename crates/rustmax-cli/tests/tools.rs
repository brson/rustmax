//! End-to-end tests of the tool management commands.
//!
//! These run real `cargo install`s from crates.io into a temporary install root,
//! so they need the network and are ignored by default.
//! Run them with `just test-tools`.

use rmx::prelude::*;
use rmx::tempfile::TempDir;
use std::path::Path;
use std::process::Command;

/// A small crate with no `--version` flag and several releases.
const PACKAGE: &str = "jsonxf";
const OLD_VERSION: &str = "1.1.0";

fn rustmax(root: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rustmax"))
        .args(args)
        .env("CARGO_INSTALL_ROOT", root)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "rustmax {args:?} failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    stdout
}

fn status(root: &Path) -> String {
    let out = rustmax(root, &["tool-status", PACKAGE]);
    out.trim().to_string()
}

#[test]
#[ignore]
fn install_update_uninstall() -> AnyResult<()> {
    let root = TempDir::with_prefix("rustmax-tools-test")?;
    let root = root.path();

    assert!(status(root).ends_with("not installed"), "{}", status(root));

    let ok = Command::new("cargo")
        .args(["install", &format!("{PACKAGE}@{OLD_VERSION}")])
        .env("CARGO_INSTALL_ROOT", root)
        .status()?
        .success();
    assert!(ok);
    let s = status(root);
    assert!(
        s.contains(OLD_VERSION) && s.contains("update available"),
        "{s}"
    );

    rustmax(root, &["update-tool", PACKAGE]);
    let s = status(root);
    assert!(s.ends_with("up to date") && !s.contains(OLD_VERSION), "{s}");

    let out = rustmax(root, &["update-tool", PACKAGE]);
    assert!(!out.contains("Running: cargo"), "{out}");

    std::fs::remove_file(root.join("bin").join(PACKAGE))?;
    let s = status(root);
    assert!(s.ends_with(&format!("missing binaries: {PACKAGE}")), "{s}");
    rustmax(root, &["update-tool", PACKAGE]);
    assert!(root.join("bin").join(PACKAGE).exists());
    assert!(status(root).ends_with("up to date"));

    rustmax(root, &["uninstall-tool", PACKAGE]);
    assert!(status(root).ends_with("not installed"));

    rustmax(root, &["install-tool", PACKAGE]);
    assert!(status(root).ends_with("up to date"));

    Ok(())
}
