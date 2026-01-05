use rmx::prelude::*;
use std::path::Path;
use std::process::{Command, Stdio};

/// Build the test crate.
pub fn build_test_crate(work_dir: &Path) -> AnyResult<()> {
    let manifest_path = work_dir.join("Cargo.toml");

    let status = Command::new("cargo")
        .arg("build")
        .arg("--manifest-path")
        .arg(&manifest_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        bail!("Failed to build test crate");
    }

    Ok(())
}

/// Run tests in the test crate.
pub fn run_tests(work_dir: &Path, test_args: &[String]) -> AnyResult<()> {
    let manifest_path = work_dir.join("Cargo.toml");

    let mut cmd = Command::new("cargo");
    cmd.arg("test")
        .arg("--manifest-path")
        .arg(&manifest_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    for arg in test_args {
        cmd.arg(arg);
    }

    let status = cmd.status()?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
