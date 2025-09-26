// Basic integration tests for the new modular architecture.

use std::process::Command;
use std::env;

#[test]
fn test_simple_binary_runs() {
    let output = Command::new(get_simple_binary_path())
        .output()
        .expect("Failed to execute simple binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Rustmax Suite v"));
    assert!(stdout.contains("Modular architecture successfully created!"));
}

#[test]
fn test_library_functionality() {
    // Test that we can use the library functions directly
    use rustmax_suite::VERSION;
    assert!(!VERSION.is_empty());
}

fn get_simple_binary_path() -> String {
    let target_dir = env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string());

    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    // Use the simple binary that we know works
    format!("{}/{}/rmx-suite-simple", target_dir, profile)
}