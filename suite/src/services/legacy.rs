// Legacy command implementations for backward compatibility.

use rmx::prelude::*;

// Legacy command handlers.
pub async fn greet_command(name: &str) -> crate::Result<()> {
    let result = greet_user(name);
    println!("Hello, {}!", result);
    Ok(())
}

pub async fn count_command(num: i32) -> crate::Result<()> {
    let result = count_to(num);
    println!("Counting to {}: {}", num, result);
    Ok(())
}

pub async fn math_command(a: i32, b: i32) -> crate::Result<()> {
    println!("Math operations on {} and {}:", a, b);
    println!("  Add: {}", add_numbers(a, b));
    println!("  Multiply: {}", multiply_numbers(a, b));
    if b != 0 {
        println!("  Divide: {}", divide_numbers(a, b));
    } else {
        println!("  Divide: Cannot divide by zero");
    }
    Ok(())
}

pub async fn file_command(content: &str) -> crate::Result<()> {
    println!("File operations:");
    match file_operations(content) {
        Ok(result) => println!("  {}", result),
        Err(e) => println!("  Error: {}", e),
    }
    Ok(())
}

pub async fn serialize_command(format: &str, data: &str) -> crate::Result<()> {
    println!("Serialization test with {}:", format);
    match serialization_demo(format, data) {
        Ok(result) => println!("  {}", result),
        Err(e) => println!("  Error: {}", e),
    }
    Ok(())
}

pub async fn crypto_command(algorithm: &str, data: &str) -> crate::Result<()> {
    println!("Cryptographic test with {}:", algorithm);
    match crypto_demo(algorithm, data) {
        Ok(result) => println!("  {}", result),
        Err(e) => println!("  Error: {}", e),
    }
    Ok(())
}

pub async fn time_command(library: &str, operation: &str) -> crate::Result<()> {
    println!("Date/time test with {}:", library);
    match time_demo(library, operation) {
        Ok(result) => println!("  {}", result),
        Err(e) => println!("  Error: {}", e),
    }
    Ok(())
}

pub async fn regex_command(pattern: &str, text: &str) -> crate::Result<()> {
    println!("Regex test:");
    match regex_demo(pattern, text) {
        Ok(result) => println!("  {}", result),
        Err(e) => println!("  Error: {}", e),
    }
    Ok(())
}

pub async fn async_command(operation: &str, count: usize) -> crate::Result<()> {
    println!("Async test with {} (count: {}):", operation, count);
    match async_demo(operation, count).await {
        Ok(result) => println!("  {}", result),
        Err(e) => println!("  Error: {}", e),
    }
    Ok(())
}

pub async fn parallel_command(items: usize, threads: Option<usize>) -> crate::Result<()> {
    let thread_info = if let Some(t) = threads {
        format!("{} threads", t)
    } else {
        "default threads".to_string()
    };
    println!("Parallel test with {} items ({}): ", items, thread_info);

    let operation = if items > 1000 { "compute" } else { "simple" };
    match parallel_demo(operation, items) {
        Ok(result) => println!("  {}", result),
        Err(e) => println!("  Error: {}", e),
    }
    Ok(())
}

// Core implementation functions (extracted from main.rs).
fn greet_user(name: &str) -> String {
    if name.is_empty() {
        "Anonymous".to_string()
    } else if name.len() > 20 {
        format!("{}...", &name[..17])
    } else {
        name.to_string()
    }
}

fn count_to(num: i32) -> String {
    if num <= 0 {
        "Nothing to count".to_string()
    } else if num > 100 {
        "Too big to count".to_string()
    } else {
        (1..=num).map(|i| i.to_string()).collect::<Vec<_>>().join(", ")
    }
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply_numbers(a: i32, b: i32) -> i32 {
    a * b
}

fn divide_numbers(a: i32, b: i32) -> f64 {
    a as f64 / b as f64
}

fn file_operations(content: &str) -> AnyResult<String> {
    use rmx::tempfile::NamedTempFile;
    use std::io::Write;

    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "{}", content)?;
    let temp_path = temp_file.path();
    let read_content = std::fs::read_to_string(temp_path)?;
    let read_content = read_content.trim();
    let matches = read_content == content;
    let file_size = temp_file.as_file().metadata()?.len();

    Ok(format!("Wrote '{}' ({} bytes), read '{}', matches: {}",
               content, file_size, read_content, matches))
}

fn serialization_demo(format: &str, input_data: &str) -> AnyResult<String> {
    use std::collections::HashMap;

    let mut test_data = HashMap::new();

    if input_data == "default" {
        test_data.insert("name".to_string(), rmx::serde_json::Value::String("rustmax-suite".to_string()));
        test_data.insert("count".to_string(), rmx::serde_json::Value::Number(42.into()));
        test_data.insert("active".to_string(), rmx::serde_json::Value::Bool(true));
        test_data.insert("tags".to_string(), rmx::serde_json::Value::Array(vec![
            rmx::serde_json::Value::String("test".to_string()),
            rmx::serde_json::Value::String("rustmax".to_string()),
        ]));
    } else {
        test_data.insert("name".to_string(), rmx::serde_json::Value::String(input_data.to_string()));
        test_data.insert("count".to_string(), rmx::serde_json::Value::Number((input_data.len() as u64).into()));
        test_data.insert("active".to_string(), rmx::serde_json::Value::Bool(!input_data.is_empty()));
        let tags: Vec<rmx::serde_json::Value> = input_data.split(',')
            .map(|s| rmx::serde_json::Value::String(s.trim().to_string()))
            .collect();
        test_data.insert("tags".to_string(), rmx::serde_json::Value::Array(tags));
    }

    match format {
        "json" => {
            let json_value = rmx::serde_json::to_value(&test_data)?;
            let json_str = rmx::serde_json::to_string_pretty(&json_value)?;
            let parsed: rmx::serde_json::Value = rmx::serde_json::from_str(&json_str)?;
            Ok(format!("JSON roundtrip successful: {} bytes -> parsed matches: {}",
                       json_str.len(), json_value == parsed))
        },
        "toml" => {
            let simple_data = if input_data == "default" {
                "name = \"rustmax-suite\"\ncount = 42\nactive = true"
            } else {
                "name = \"custom\"\ncount = 1\nactive = true"
            };

            let parsed: rmx::toml::Value = rmx::toml::from_str(simple_data)?;
            let back_to_string = rmx::toml::to_string(&parsed)?;
            Ok(format!("TOML roundtrip successful: {} chars -> {} chars",
                       simple_data.len(), back_to_string.len()))
        },
        _ => Ok(format!("Unsupported format '{}', use 'json' or 'toml'", format))
    }
}

fn crypto_demo(algorithm: &str, data: &str) -> AnyResult<String> {
    let input_bytes = data.as_bytes();

    match algorithm {
        "blake3" => {
            let hash = rmx::blake3::hash(input_bytes);
            let hex_hash = rmx::hex::encode(hash.as_bytes());
            Ok(format!("BLAKE3 hash of '{}' ({} bytes): {}", data, input_bytes.len(), hex_hash))
        },
        "sha256" => {
            use rmx::sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(input_bytes);
            let result = hasher.finalize();
            let hex_hash = rmx::hex::encode(result);
            Ok(format!("SHA256 hash of '{}' ({} bytes): {}", data, input_bytes.len(), hex_hash))
        },
        _ => Ok(format!("Unsupported algorithm '{}', use 'blake3' or 'sha256'", algorithm))
    }
}

fn time_demo(library: &str, operation: &str) -> AnyResult<String> {
    match library {
        "chrono" => {
            use rmx::chrono::{DateTime, Utc, Duration};

            match operation {
                "now" => {
                    let now = Utc::now();
                    Ok(format!("Current UTC time: {}", now.format("%Y-%m-%d %H:%M:%S UTC")))
                },
                "add" => {
                    let now = Utc::now();
                    let future = now + Duration::hours(24);
                    Ok(format!("Now: {}, +24h: {}",
                              now.format("%Y-%m-%d %H:%M:%S"),
                              future.format("%Y-%m-%d %H:%M:%S")))
                },
                _ => Ok(format!("Unknown chrono operation: {}", operation))
            }
        },
        "jiff" => {
            use rmx::jiff::{Timestamp, civil::DateTime, Unit};

            match operation {
                "now" => {
                    let now = Timestamp::now();
                    Ok(format!("Current timestamp: {}", now))
                },
                "add" => {
                    let now = Timestamp::now();
                    // Simplified example without using duration API
                    Ok(format!("Now: {} (add operation simplified)", now))
                },
                _ => Ok(format!("Unknown jiff operation: {}", operation))
            }
        },
        _ => Ok(format!("Unknown time library: {}", library))
    }
}

fn regex_demo(pattern: &str, text: &str) -> AnyResult<String> {
    let re = rmx::regex::Regex::new(pattern)?;
    let matches: Vec<_> = re.find_iter(text).collect();

    if matches.is_empty() {
        Ok(format!("Pattern '{}' found no matches in '{}'", pattern, text))
    } else {
        let match_strs: Vec<_> = matches.iter()
            .map(|m| format!("'{}'", m.as_str()))
            .collect();
        Ok(format!("Pattern '{}' found {} matches in '{}': {}",
                  pattern, matches.len(), text, match_strs.join(", ")))
    }
}

async fn async_demo(operation: &str, count: usize) -> AnyResult<String> {
    use rmx::futures::{future, stream::{self, StreamExt}};
    use rmx::tokio::time::{sleep, Duration};

    match operation {
        "futures" => {
            let tasks: Vec<_> = (0..count)
                .map(|i| async move {
                    sleep(Duration::from_millis(100)).await;
                    format!("Task {}", i)
                })
                .collect();

            let results = future::join_all(tasks).await;
            Ok(format!("Completed {} async tasks: {}", count, results.join(", ")))
        },
        "stream" => {
            let results: Vec<String> = stream::iter(0..count)
                .map(|i| async move {
                    sleep(Duration::from_millis(50)).await;
                    format!("Stream {}", i)
                })
                .buffer_unordered(3)
                .collect()
                .await;

            Ok(format!("Processed {} stream items: {}", count, results.join(", ")))
        },
        _ => Ok(format!("Unknown async operation: {}", operation))
    }
}

fn parallel_demo(operation: &str, size: usize) -> AnyResult<String> {
    match operation {
        "simple" => {
            use rmx::rayon::prelude::*;
            let sum: usize = (0..size).into_par_iter().map(|i| i * 2).sum();
            Ok(format!("Parallel sum of doubled numbers 0..{}: {}", size, sum))
        },
        "compute" => {
            use rmx::rayon::prelude::*;
            let results: Vec<_> = (0..size)
                .into_par_iter()
                .map(|i| {
                    // Simulate some computation.
                    let mut result = i;
                    for _ in 0..1000 {
                        result = result.wrapping_mul(2).wrapping_add(1) % 1000000;
                    }
                    result
                })
                .collect();

            let sum: usize = results.iter().sum();
            Ok(format!("Parallel computation on {} items, sum: {}", size, sum))
        },
        _ => Ok(format!("Unknown parallel operation: {}", operation))
    }
}