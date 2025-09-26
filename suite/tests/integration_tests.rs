// Integration tests for the modular rustmax suite architecture.

use std::process::Command;
use std::env;

#[test]
fn test_simple_binary_runs() {
    let output = Command::new(get_simple_binary_path())
        .output()
        .expect("Failed to execute simple binary");

    assert!(output.status.success(), "Simple binary should run successfully");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Rustmax Suite v"), "Should show version");
    assert!(stdout.contains("Modular architecture successfully created!"), "Should show success message");
    assert!(stdout.contains("Available commands:"), "Should show available commands");
}

#[test]
fn test_library_version_accessible() {
    // Test that we can access the library VERSION constant
    use rustmax_suite::VERSION;
    assert!(!VERSION.is_empty(), "VERSION should not be empty");
    assert!(VERSION.contains('.'), "VERSION should be semver format");
}

#[test]
fn test_library_config_creation() {
    // Test that we can create a default config
    use rustmax_suite::Config;
    let config = Config::default();
    assert_eq!(config.name, "rustmax-suite");
    assert_eq!(config.web.port, 8080);
}

#[test]
fn test_library_app_state_creation() {
    // Test that we can create app state
    use rustmax_suite::{Config, AppState};
    let config = Config::default();
    let state = AppState::new(config);

    // This is async but we just check it compiles and creates
    assert!(std::mem::size_of_val(&state) > 0);
}

fn get_simple_binary_path() -> String {
    let target_dir = env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string());

    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    format!("{}/{}/rmx-suite-simple", target_dir, profile)
}