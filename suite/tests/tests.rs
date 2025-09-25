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

#[test]
fn test_binary_file_command() {
    let output = Command::new(get_binary_path())
        .args(&["file", "test content"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("File operations:"));
    assert!(stdout.contains("matches: true"));
}

#[test]
fn test_binary_parse_command() {
    let output = Command::new(get_binary_path())
        .args(&["parse", "--name", "Test", "--count", "5", "--verbose"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("CLI parsing with clap:"));
    assert!(stdout.contains("name=Test"));
    assert!(stdout.contains("count=5"));
    assert!(stdout.contains("verbose=true"));
}

#[test]
fn test_binary_serialize_json() {
    let output = Command::new(get_binary_path())
        .args(&["serialize", "json"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Serialization test with json:"));
    assert!(stdout.contains("JSON roundtrip successful"));
}

#[test]
fn test_binary_serialize_toml() {
    let output = Command::new(get_binary_path())
        .args(&["serialize", "toml"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Serialization test with toml:"));
    assert!(stdout.contains("TOML roundtrip successful"));
}

#[test]
fn test_binary_crypto_blake3() {
    let output = Command::new(get_binary_path())
        .args(&["crypto", "blake3", "hello"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Cryptographic test with blake3:"));
    assert!(stdout.contains("BLAKE3 hash of 'hello'"));
}

#[test]
fn test_binary_crypto_sha256() {
    let output = Command::new(get_binary_path())
        .args(&["crypto", "sha256", "test"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Cryptographic test with sha256:"));
    assert!(stdout.contains("SHA256 hash of 'test'"));
}

#[test]
fn test_binary_time_chrono() {
    let output = Command::new(get_binary_path())
        .args(&["time", "chrono", "now"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Date/time test with chrono:"));
    assert!(stdout.contains("Current UTC time:"));
}

#[test]
fn test_binary_time_jiff() {
    let output = Command::new(get_binary_path())
        .args(&["time", "jiff", "now"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Date/time test with jiff:"));
    assert!(stdout.contains("Current timestamp:"));
}

#[test]
fn test_binary_regex_command() {
    let output = Command::new(get_binary_path())
        .args(&["regex", r"\w+", "hello world"])
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Regex test:"));
    assert!(stdout.contains("2 matches"));
    assert!(stdout.contains("First: 'hello'"));
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

        let binary_path = format!("{}/{}/rustmax-suite", target_dir, profile);

        // If custom profile binary doesn't exist, try debug fallback
        if !std::path::Path::new(&binary_path).exists() {
            let debug_path = format!("{}/debug/rustmax-suite", target_dir);
            if std::path::Path::new(&debug_path).exists() {
                return debug_path;
            }
            // Also try coverage profile explicitly
            let coverage_path = format!("{}/coverage/rustmax-suite", target_dir);
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

        format!("{}/{}/rustmax-suite", target_dir, profile)
    }
}