use std::process::Command;
use std::env;

#[test]
fn test_binary_help() {
    let output = Command::new(get_binary_path())
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Rustmax Suite"));
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("Commands:"));
}

#[test]
fn test_binary_greet_command() {
    let output = Command::new(get_binary_path())
        .args(&["greet", "Alice"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Hello, Alice!"));
}

#[test]
fn test_binary_greet_no_name() {
    let output = Command::new(get_binary_path())
        .args(&["greet"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Hello, World!"));
}

#[test]
fn test_binary_greet_long_name() {
    let output = Command::new(get_binary_path())
        .args(&["greet", "VeryLongNameThatExceedsTwentyCharacters"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Hello, VeryLongNameThatE...!"));
}

#[test]
fn test_binary_count_command() {
    let output = Command::new(get_binary_path())
        .args(&["count", "5"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Counting to 5: 1, 2, 3, 4, 5"));
}

#[test]
fn test_binary_count_zero() {
    let output = Command::new(get_binary_path())
        .args(&["count", "0"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Nothing to count"));
}

#[test]
fn test_binary_count_large_number() {
    let output = Command::new(get_binary_path())
        .args(&["count", "150"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Too big to count"));
}

#[test]
fn test_binary_math_command() {
    let output = Command::new(get_binary_path())
        .args(&["math", "10", "5"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Math operations on 10 and 5:"));
    assert!(stdout.contains("Add: 15"));
    assert!(stdout.contains("Multiply: 50"));
    assert!(stdout.contains("Divide: 2"));
}

#[test]
fn test_binary_math_divide_by_zero() {
    let output = Command::new(get_binary_path())
        .args(&["math", "10", "0"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Cannot divide by zero"));
}

#[test]
fn test_binary_internal_tests() {
    let output = Command::new(get_binary_path())
        .args(&["test"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Running internal tests..."));
    assert!(stdout.contains("All internal tests passed!"));
}

#[test]
fn test_binary_unknown_command() {
    let output = Command::new(get_binary_path())
        .args(&["unknown"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Unknown command"));
    assert!(stdout.contains("Available commands:"));
}

fn get_binary_path() -> String {
    // Check if we're running under cargo-llvm-cov
    if env::var("CARGO_LLVM_COV").is_ok() {
        // Use the llvm-cov instrumented binary
        let target_dir = env::var("CARGO_LLVM_COV_TARGET_DIR")
            .unwrap_or_else(|_| "target/llvm-cov-target".to_string());

        let profile = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };

        format!("{}/{}/rustmax-suite", target_dir, profile)
    } else {
        // Use the normal binary
        let target_dir = env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| "target".to_string());

        let profile = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };

        format!("{}/{}/rustmax-suite", target_dir, profile)
    }
}