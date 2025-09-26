// Simplified integration tests that work with the rmx-suite-simple binary.
// These tests focus on basic functionality and library coverage.

use std::process::Command;
use std::env;

#[test]
fn test_simple_binary_runs() {
    let output = Command::new(get_binary_path())
        .output()
        .expect("Failed to execute simple binary");

    assert!(output.status.success(), "Simple binary should run successfully");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Rustmax Suite v"), "Should show version");
    assert!(stdout.contains("Modular architecture successfully created!"), "Should show success message");
    assert!(stdout.contains("Available commands:"), "Should show available commands");
}

#[test]
fn test_library_functionality() {
    // Test that we can use the library functions directly
    use rustmax_suite::VERSION;
    assert!(!VERSION.is_empty());
    assert!(VERSION.contains('.'), "Should be semver format");
}

#[test]
fn test_config_system() {
    use rustmax_suite::Config;
    let config = Config::default();

    // Test basic config structure
    assert_eq!(config.name, "rustmax-suite");
    assert_eq!(config.web.port, 8080);
    assert_eq!(config.web.host, "127.0.0.1");
}

#[test]
fn test_app_state_creation() {
    use rustmax_suite::{Config, AppState};
    let config = Config::default();
    let state = AppState::new(config);

    // Just verify we can create state without panicking
    assert!(std::mem::size_of_val(&state) > 0);
}

fn get_binary_path() -> String {
    // Check if we're running under cargo-llvm-cov
    if env::var("CARGO_LLVM_COV").is_ok() {
        // Use the llvm-cov instrumented binary
        let target_dir = env::var("CARGO_LLVM_COV_TARGET_DIR")
            .unwrap_or_else(|_| "target/llvm-cov-target".to_string());

        // Check for custom profile first, then fallback to debug/release
        let profile = env::var("CARGO_PROFILE")
            .unwrap_or_else(|_| {
                if cfg!(debug_assertions) {
                    "debug".to_string()
                } else {
                    "release".to_string()
                }
            });

        let binary_path = format!("{}/{}/rmx-suite-simple", target_dir, profile);

        // If custom profile binary doesn't exist, try debug fallback
        if !std::path::Path::new(&binary_path).exists() {
            let debug_path = format!("{}/debug/rmx-suite-simple", target_dir);
            if std::path::Path::new(&debug_path).exists() {
                return debug_path;
            }
            // Also try coverage profile explicitly
            let coverage_path = format!("{}/coverage/rmx-suite-simple", target_dir);
            if std::path::Path::new(&coverage_path).exists() {
                return coverage_path;
            }
        }

        binary_path
    } else {
        // Use the normal binary
        let target_dir = env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| "target".to_string());

        let profile = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };

        format!("{}/{}/rmx-suite-simple", target_dir, profile)
    }
}