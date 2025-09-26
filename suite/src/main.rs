use rmx::prelude::*;

// Test if thiserror is available
#[allow(unused_imports)]
use rmx::thiserror;
use std::env;

fn main() -> AnyResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "greet" => {
                let default_name = "World".to_string();
                let name = args.get(2).unwrap_or(&default_name);
                println!("Hello, {}!", greet_user(name));
            }
            "count" => {
                let default_num = "10".to_string();
                let num_str = args.get(2).unwrap_or(&default_num);
                let num: i32 = num_str.parse().unwrap_or(10);
                println!("Counting to {}: {}", num, count_to(num));
            }
            "math" => {
                let default_a = "5".to_string();
                let default_b = "3".to_string();
                let a_str = args.get(2).unwrap_or(&default_a);
                let b_str = args.get(3).unwrap_or(&default_b);
                let a: i32 = a_str.parse().unwrap_or(5);
                let b: i32 = b_str.parse().unwrap_or(3);
                println!("Math operations on {} and {}:", a, b);
                println!("  Add: {}", add_numbers(a, b));
                println!("  Multiply: {}", multiply_numbers(a, b));
                if b != 0 {
                    println!("  Divide: {}", divide_numbers(a, b));
                } else {
                    println!("  Divide: Cannot divide by zero");
                }
            }
            "test" => {
                println!("Running internal tests...");
                run_internal_tests();
            }
            "file" => {
                let default_content = "Hello from rustmax suite!".to_string();
                let content = args.get(2).unwrap_or(&default_content);
                println!("File operations:");
                match file_operations(content) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "parse" => {
                println!("CLI parsing with clap:");
                match cli_parsing_demo(&args[2..]) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "serialize" => {
                let format = args.get(2).map(|s| s.as_str()).unwrap_or("json");
                let data = args.get(3).map(|s| s.as_str()).unwrap_or("default");
                println!("Serialization test with {}:", format);
                match serialization_demo(format, data) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "crypto" => {
                let algorithm = args.get(2).map(|s| s.as_str()).unwrap_or("blake3");
                let data = args.get(3).map(|s| s.as_str()).unwrap_or("rustmax-suite");
                println!("Cryptographic test with {}:", algorithm);
                match crypto_demo(algorithm, data) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "time" => {
                let library = args.get(2).map(|s| s.as_str()).unwrap_or("chrono");
                let operation = args.get(3).map(|s| s.as_str()).unwrap_or("now");
                println!("Date/time test with {}:", library);
                match time_demo(library, operation) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "regex" => {
                let pattern = args.get(2).map(|s| s.as_str()).unwrap_or(r"\b\w+\b");
                let text = args.get(3).map(|s| s.as_str()).unwrap_or("Hello world 123!");
                println!("Regex test:");
                match regex_demo(pattern, text) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "async" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("futures");
                let count = args.get(3)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(3);
                println!("Async/futures test with {}:", operation);
                match async_demo(operation, count) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "parallel" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("map");
                let size = args.get(3)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1000);
                println!("Parallel processing test with {}:", operation);
                match parallel_demo(operation, size) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "util" => {
                let utility = args.get(2).map(|s| s.as_str()).unwrap_or("itertools");
                let data = args.get(3).map(|s| s.as_str()).unwrap_or("test data");
                println!("Utility crates test with {}:", utility);
                match util_demo(utility, data) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "walk" => {
                let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
                let max_depth = args.get(3)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(2);
                println!("Directory walk test:");
                match walk_demo(path, max_depth) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "rand" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("generate");
                let count = args.get(3)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(5);
                println!("Random number test with {}:", operation);
                match rand_demo(operation, count) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "url" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("parse");
                let url_str = args.get(3).map(|s| s.as_str()).unwrap_or("https://example.com/path?key=value#fragment");
                println!("URL test with {}:", operation);
                match url_demo(operation, url_str) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "nom" => {
                let parser_type = args.get(2).map(|s| s.as_str()).unwrap_or("numbers");
                let input = args.get(3).map(|s| s.as_str()).unwrap_or("123 456 789");
                println!("Parser combinators test with {}:", parser_type);
                match nom_demo(parser_type, input) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "thiserror" => {
                let error_type = args.get(2).map(|s| s.as_str()).unwrap_or("validation");
                let message = args.get(3).map(|s| s.as_str()).unwrap_or("test error");
                println!("Custom error handling test with {}:", error_type);
                match thiserror_demo(error_type, message) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "xshell" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("info");
                let command = args.get(3).map(|s| s.as_str()).unwrap_or("echo hello");
                println!("Shell execution test with {}:", operation);
                match xshell_demo(operation, command) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "crossbeam" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("channel");
                let count = args.get(3)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(5);
                println!("Advanced concurrency test with {}:", operation);
                match crossbeam_demo(operation, count) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "tempfile" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("create");
                let content = args.get(3).map(|s| s.as_str()).unwrap_or("temp content");
                println!("Temporary file operations test with {}:", operation);
                match tempfile_demo(operation, content) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "json5" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("parse");
                let data = args.get(3).map(|s| s.as_str()).unwrap_or(r#"{ name: 'test', /* comment */ value: 42 }"#);
                println!("JSON5 relaxed parsing test with {}:", operation);
                match json5_demo(operation, data) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "tera" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("render");
                let name = args.get(3).map(|s| s.as_str()).unwrap_or("World");
                println!("Template engine test with {}:", operation);
                match tera_demo(operation, name) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "unicode" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("graphemes");
                let text = args.get(3).map(|s| s.as_str()).unwrap_or("Hello 🌍 World! 👨‍👩‍👧‍👦");
                println!("Unicode text segmentation test with {}:", operation);
                match unicode_demo(operation, text) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "logging" => {
                let level = args.get(2).map(|s| s.as_str()).unwrap_or("info");
                let message = args.get(3).map(|s| s.as_str()).unwrap_or("test message");
                println!("Logging infrastructure test with {}:", level);
                match logging_demo(level, message) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "proptest" => {
                let test_type = args.get(2).map(|s| s.as_str()).unwrap_or("basic");
                let iterations = args.get(3)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(100);
                println!("Property-based testing with {}:", test_type);
                match proptest_demo(test_type, iterations) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "anyhow" => {
                let error_type = args.get(2).map(|s| s.as_str()).unwrap_or("basic");
                let message = args.get(3).map(|s| s.as_str()).unwrap_or("example error");
                println!("Error handling with anyhow using {}:", error_type);
                match anyhow_demo(error_type, message) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            "reqwest" => {
                let operation = args.get(2).map(|s| s.as_str()).unwrap_or("client");
                let url = args.get(3).map(|s| s.as_str()).unwrap_or("https://httpbin.org/json");
                println!("HTTP client operations with {}:", operation);
                match reqwest_demo(operation, url) {
                    Ok(result) => println!("  {}", result),
                    Err(e) => println!("  Error: {}", e),
                }
            }
            _ => {
                println!("Unknown command. Available commands: greet, count, math, test, file, parse, serialize, crypto, time, regex, async, parallel, util, walk, rand, url, nom, thiserror, xshell, crossbeam, tempfile, json5, tera, unicode, logging, proptest, anyhow, reqwest");
            }
        }
    } else {
        println!("Rustmax Suite - Integration test application");
        println!("Usage: {} <command> [args...]", args[0]);
        println!("Commands:");
        println!("  greet [name]              - Greet someone");
        println!("  count [num]               - Count to a number");
        println!("  math [a] [b]              - Perform math operations");
        println!("  test                      - Run internal tests");
        println!("  file [content]            - Test file I/O operations");
        println!("  parse <args...>           - Test CLI parsing with clap");
        println!("  serialize [fmt] [data]    - Test JSON/TOML serialization");
        println!("  crypto [algo] [data]      - Test cryptographic operations");
        println!("  time [lib] [op]           - Test date/time operations");
        println!("  regex [pattern] [text]    - Test regex pattern matching");
        println!("  async [op] [count]        - Test async/futures with tokio");
        println!("  parallel [op] [size]      - Test parallel processing with rayon");
        println!("  util [crate] [data]       - Test utility crates (itertools, bytes, etc.)");
        println!("  walk [path] [depth]       - Test directory walking with walkdir");
        println!("  rand [op] [count]         - Test random number generation with rand");
        println!("  url [op] [url]            - Test URL parsing and manipulation");
        println!("  nom [parser] [input]      - Test parser combinators with nom");
        println!("  thiserror [type] [msg]    - Test custom error types with thiserror");
        println!("  xshell [op] [cmd]         - Test shell execution with xshell");
        println!("  crossbeam [op] [count]    - Test advanced concurrency with crossbeam");
        println!("  tempfile [op] [content]   - Test temporary file operations with tempfile");
        println!("  json5 [op] [data]         - Test relaxed JSON parsing with json5");
        println!("  tera [op] [name]          - Test template engine with tera");
        println!("  unicode [op] [text]       - Test Unicode text segmentation");
        println!("  logging [level] [msg]     - Test logging with env_logger");
        println!("  proptest [type] [iters]   - Test property-based testing with proptest");
        println!("  anyhow [type] [msg]       - Test enhanced error handling with anyhow");
        println!("  reqwest [op] [url]        - Test HTTP client operations with reqwest");
    }

    Ok(())
}

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

fn run_internal_tests() {
    assert_eq!(greet_user("Alice"), "Alice");
    assert_eq!(greet_user(""), "Anonymous");
    assert_eq!(greet_user("VeryLongNameThatExceedsTwentyCharacters"), "VeryLongNameThatE...");

    assert_eq!(count_to(3), "1, 2, 3");
    assert_eq!(count_to(0), "Nothing to count");
    assert_eq!(count_to(101), "Too big to count");

    assert_eq!(add_numbers(2, 3), 5);
    assert_eq!(multiply_numbers(4, 5), 20);
    assert_eq!(divide_numbers(10, 2), 5.0);

    println!("All internal tests passed!");
}

fn file_operations(content: &str) -> AnyResult<String> {
    use rmx::tempfile::NamedTempFile;
    use std::io::Write;

    // Create a temporary file using tempfile crate.
    let mut temp_file = NamedTempFile::new()?;

    // Write content to the temporary file.
    writeln!(temp_file, "{}", content)?;

    // Get the path before reading.
    let temp_path = temp_file.path();

    // Read content back.
    let read_content = std::fs::read_to_string(temp_path)?;
    let read_content = read_content.trim();

    // Check if content matches.
    let matches = read_content == content;

    // Get file info.
    let file_size = temp_file.as_file().metadata()?.len();

    // File will be automatically cleaned up when temp_file is dropped!

    Ok(format!("Wrote '{}' ({} bytes), read '{}', matches: {}",
               content, file_size, read_content, matches))
}

fn cli_parsing_demo(args: &[String]) -> AnyResult<String> {
    use rmx::clap::{Arg, Command};

    let app = Command::new("parse-demo")
        .arg(Arg::new("name")
            .short('n')
            .long("name")
            .value_name("NAME")
            .help("Sets a name value"))
        .arg(Arg::new("count")
            .short('c')
            .long("count")
            .value_name("COUNT")
            .help("Sets a count value")
            .value_parser(rmx::clap::value_parser!(u32)))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(rmx::clap::ArgAction::SetTrue)
            .help("Enable verbose mode"));

    match app.try_get_matches_from(std::iter::once("parse-demo".to_string()).chain(args.iter().cloned())) {
        Ok(matches) => {
            let name = matches.get_one::<String>("name").map(|s| s.as_str()).unwrap_or("default");
            let count = matches.get_one::<u32>("count").copied().unwrap_or(1);
            let verbose = matches.get_flag("verbose");

            Ok(format!("Parsed: name={}, count={}, verbose={}", name, count, verbose))
        },
        Err(e) => Ok(format!("Parse error: {}", e)),
    }
}

fn serialization_demo(format: &str, input_data: &str) -> AnyResult<String> {
    use std::collections::HashMap;

    // Create test data using simple HashMap instead of custom struct
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
            // For TOML we'll create a simpler test since we don't have derive
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
        "sha512" => {
            use rmx::sha2::{Digest, Sha512};
            let mut hasher = Sha512::new();
            hasher.update(input_bytes);
            let result = hasher.finalize();
            let hex_hash = rmx::hex::encode(result);
            Ok(format!("SHA512 hash of '{}' ({} bytes): {}", data, input_bytes.len(), hex_hash))
        },
        _ => Ok(format!("Unsupported algorithm '{}', use 'blake3', 'sha256', or 'sha512'", algorithm))
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
                "parse" => {
                    let date_str = "2023-12-25T10:30:00Z";
                    match DateTime::parse_from_rfc3339(date_str) {
                        Ok(dt) => Ok(format!("Parsed '{}': {} (day of week: {})",
                                          date_str, dt.format("%B %d, %Y at %H:%M"),
                                          dt.format("%A"))),
                        Err(e) => Ok(format!("Parse error: {}", e))
                    }
                },
                "math" => {
                    let now = Utc::now();
                    let future = now + Duration::days(30);
                    let past = now - Duration::hours(24);
                    Ok(format!("Now: {} | +30 days: {} | -24 hours: {}",
                              now.format("%m/%d"), future.format("%m/%d"), past.format("%m/%d")))
                },
                _ => Ok(format!("Unsupported chrono operation '{}', use 'now', 'parse', or 'math'", operation))
            }
        },
        "jiff" => {
            use rmx::jiff::{Timestamp, Span};

            match operation {
                "now" => {
                    let now = Timestamp::now();
                    Ok(format!("Current timestamp: {}", now))
                },
                "math" => {
                    let now = Timestamp::now();
                    let future = now.checked_add(Span::new().days(30))?;
                    let past = now.checked_sub(Span::new().days(1))?;
                    Ok(format!("Now: {} | +30 days: {} | -1 day: {}",
                              now.strftime("%m/%d"), future.strftime("%m/%d"), past.strftime("%m/%d")))
                },
                _ => Ok(format!("Unsupported jiff operation '{}', use 'now' or 'math'", operation))
            }
        },
        _ => Ok(format!("Unsupported library '{}', use 'chrono' or 'jiff'", library))
    }
}

fn regex_demo(pattern: &str, text: &str) -> AnyResult<String> {
    use rmx::regex::Regex;

    match Regex::new(pattern) {
        Ok(re) => {
            let matches: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
            let count = matches.len();

            if count > 0 {
                let captures = re.captures(text);
                let first_match = matches[0];
                let replaced = re.replace_all(text, "[$0]");

                let capture_info = if let Some(caps) = captures {
                    if caps.len() > 1 {
                        format!(" | Groups: {}", (1..caps.len())
                               .filter_map(|i| caps.get(i).map(|m| m.as_str()))
                               .collect::<Vec<_>>()
                               .join(", "))
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                Ok(format!("Pattern '{}' in '{}': {} matches | First: '{}' | Replaced: '{}'{}",
                          pattern, text, count, first_match, replaced, capture_info))
            } else {
                Ok(format!("Pattern '{}' in '{}': No matches found", pattern, text))
            }
        },
        Err(e) => Ok(format!("Invalid regex pattern '{}': {}", pattern, e))
    }
}

fn async_demo(operation: &str, count: usize) -> AnyResult<String> {
    use rmx::tokio;
    use std::time::Duration;

    let runtime = tokio::runtime::Runtime::new()?;

    match operation {
        "futures" => {
            let result = runtime.block_on(async {
                let futures: Vec<_> = (0..count)
                    .map(|i| async move {
                        tokio::time::sleep(Duration::from_millis(10)).await;
                        i * i
                    })
                    .collect();

                let results = rmx::futures::future::join_all(futures).await;
                results
            });

            Ok(format!("Ran {} async futures, results: {:?}", count, result))
        },
        "spawn" => {
            let result = runtime.block_on(async {
                let handles: Vec<_> = (0..count)
                    .map(|i| {
                        tokio::spawn(async move {
                            tokio::time::sleep(Duration::from_millis(10)).await;
                            format!("task-{}", i)
                        })
                    })
                    .collect();

                let mut results = Vec::new();
                for handle in handles {
                    results.push(handle.await.unwrap());
                }
                results
            });

            Ok(format!("Spawned {} tasks, completed: {}", count, result.len()))
        },
        "select" => {
            let result = runtime.block_on(async {
                let fut1 = async {
                    tokio::time::sleep(Duration::from_millis(20)).await;
                    "fast"
                };

                let fut2 = async {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    "slow"
                };

                use rmx::futures::future::Either;
                let winner = rmx::futures::future::select(Box::pin(fut1), Box::pin(fut2)).await;

                match winner {
                    Either::Left((val, _)) => format!("Left won: {}", val),
                    Either::Right((val, _)) => format!("Right won: {}", val),
                }
            });

            Ok(format!("Select result: {}", result))
        },
        _ => Ok(format!("Unsupported operation '{}', use 'futures', 'spawn', or 'select'", operation))
    }
}

fn parallel_demo(operation: &str, size: usize) -> AnyResult<String> {
    use rmx::rayon::prelude::*;
    use std::time::Instant;

    match operation {
        "map" => {
            let data: Vec<usize> = (0..size).collect();

            let start = Instant::now();
            let result: Vec<usize> = data.par_iter().map(|&x| x * x).collect();
            let duration = start.elapsed();

            let sum: usize = result.iter().sum();
            Ok(format!("Parallel map on {} items: sum={}, time={:?}", size, sum, duration))
        },
        "filter" => {
            let data: Vec<usize> = (0..size).collect();

            let start = Instant::now();
            let result: Vec<usize> = data.par_iter()
                .filter(|&&x| x % 2 == 0)
                .copied()
                .collect();
            let duration = start.elapsed();

            Ok(format!("Parallel filter on {} items: {} even numbers, time={:?}",
                      size, result.len(), duration))
        },
        "reduce" => {
            let data: Vec<usize> = (1..=size).collect();

            let start = Instant::now();
            let result = data.par_iter().map(|&x| x as u64).reduce(|| 0, |a, b| a + b);
            let duration = start.elapsed();

            Ok(format!("Parallel reduce on {} items: sum={}, time={:?}", size, result, duration))
        },
        "sort" => {
            let mut data: Vec<usize> = (0..size).rev().collect();

            let start = Instant::now();
            data.par_sort();
            let duration = start.elapsed();

            let is_sorted = data.windows(2).all(|w| w[0] <= w[1]);
            Ok(format!("Parallel sort on {} items: sorted={}, time={:?}", size, is_sorted, duration))
        },
        _ => Ok(format!("Unsupported operation '{}', use 'map', 'filter', 'reduce', or 'sort'", operation))
    }
}

fn util_demo(utility: &str, data: &str) -> AnyResult<String> {
    match utility {
        "itertools" => {
            let numbers = vec![1, 2, 3, 4, 5];
            let result = numbers.iter()
                .cartesian_product(numbers.iter())
                .take(5)
                .map(|(a, b)| format!("({},{})", a, b))
                .join(", ");

            let chunks: Vec<Vec<i32>> = (1..=10).chunks(3).into_iter().map(|c| c.collect()).collect();

            Ok(format!("Itertools - cartesian: {} | chunks: {:?}", result, chunks))
        },
        "bytes" => {
            use rmx::bytes::{BytesMut, BufMut};

            let mut buf = BytesMut::with_capacity(64);
            buf.put_slice(data.as_bytes());
            buf.put_u8(b'!');
            buf.put_u32(12345);

            let frozen = buf.freeze();

            Ok(format!("Bytes - len: {}, capacity was 64, data: {:?}...",
                      frozen.len(), &frozen[..data.len().min(10)]))
        },
        "bigint" => {
            use rmx::num_bigint::BigInt;

            let a: BigInt = data.parse().unwrap_or_else(|_| BigInt::from(12345));
            let b = &a * &a;
            let c = &b + &a;

            Ok(format!("BigInt - a: {}, a²: {}, a²+a: {}", a, b, c))
        },
        "semver" => {
            use rmx::semver::Version;

            let v1 = Version::parse(data).unwrap_or(Version::new(1, 2, 3));
            let v2 = Version::new(v1.major, v1.minor, v1.patch + 1);

            Ok(format!("Semver - v1: {}, v2: {}, v1<v2: {}", v1, v2, v1 < v2))
        },
        "base64" => {
            use rmx::base64::{Engine as _, engine::general_purpose};

            let encoded = general_purpose::STANDARD.encode(data.as_bytes());
            let decoded = general_purpose::STANDARD.decode(&encoded)?;
            let decoded_str = String::from_utf8(decoded)?;

            Ok(format!("Base64 - encoded: '{}', roundtrip: {}", encoded, decoded_str == data))
        },
        _ => Ok(format!("Unsupported utility '{}', use 'itertools', 'bytes', 'bigint', 'semver', or 'base64'", utility))
    }
}

fn walk_demo(path: &str, max_depth: usize) -> AnyResult<String> {
    use rmx::walkdir::WalkDir;

    let mut file_count = 0;
    let mut dir_count = 0;
    let mut total_size = 0u64;

    for entry in WalkDir::new(path).max_depth(max_depth) {
        let entry = entry?;
        if entry.file_type().is_file() {
            file_count += 1;
            if let Ok(metadata) = entry.metadata() {
                total_size += metadata.len();
            }
        } else if entry.file_type().is_dir() {
            dir_count += 1;
        }
    }

    Ok(format!("Walked '{}' (depth {}): {} dirs, {} files, total size: {} bytes",
              path, max_depth, dir_count, file_count, total_size))
}

fn rand_demo(operation: &str, count: usize) -> AnyResult<String> {
    use rmx::rand::{rng, Rng};

    match operation {
        "generate" => {
            let mut rng = rng();
            let numbers: Vec<u32> = (0..count).map(|_| rng.random_range(1..=100)).collect();
            let sum: u32 = numbers.iter().sum();
            let avg = sum as f64 / count as f64;

            Ok(format!("Generated {} random numbers (1-100): {:?}, avg: {:.1}",
                      count, numbers, avg))
        },
        "shuffle" => {
            let mut data: Vec<usize> = (1..=count).collect();
            let original = data.clone();

            // Simple Fisher-Yates shuffle
            let mut rng = rng();
            for i in (1..data.len()).rev() {
                let j = rng.random_range(0..=i);
                data.swap(i, j);
            }

            Ok(format!("Shuffled 1-{}: {:?} -> {:?}",
                      count, original, data))
        },
        "types" => {
            let mut rng = rng();

            let bool_val: bool = rng.random();
            let f64_val: f64 = rng.random_range(0.0..1.0);
            let char_val = rng.random_range('a'..='z');
            let bytes: Vec<u8> = (0..count.min(8)).map(|_| rng.random()).collect();

            Ok(format!("Random types: bool={}, f64={:.3}, char='{}', bytes={:?}",
                      bool_val, f64_val, char_val, bytes))
        },
        "choice" => {
            let choices = vec!["apple", "banana", "cherry", "date", "elderberry"];
            let mut rng = rng();

            let selected: Vec<&str> = (0..count)
                .map(|_| choices[rng.random_range(0..choices.len())])
                .collect();

            Ok(format!("Random choices from {:?}: {:?}", choices, selected))
        },
        _ => Ok(format!("Unsupported operation '{}', use 'generate', 'shuffle', 'types', or 'choice'", operation))
    }
}

fn url_demo(operation: &str, url_str: &str) -> AnyResult<String> {
    use rmx::url::Url;

    match operation {
        "parse" => {
            let url = Url::parse(url_str)?;

            Ok(format!("Parsed URL: scheme='{}', host={:?}, port={:?}, path='{}', query={:?}, fragment={:?}",
                      url.scheme(),
                      url.host_str(),
                      url.port(),
                      url.path(),
                      url.query(),
                      url.fragment()))
        },
        "manipulate" => {
            let mut url = Url::parse(url_str)?;

            // Add/modify query parameters.
            url.query_pairs_mut()
                .append_pair("added", "by_rustmax")
                .append_pair("timestamp", "12345");

            // Try to set a new path.
            url.set_path("/new/path");

            Ok(format!("Modified URL: {} -> {}",
                      url_str, url.as_str()))
        },
        "join" => {
            let base = Url::parse(url_str)?;
            let relative_paths = vec!["../other", "subdir/file.html", "/absolute"];

            let joined: Vec<String> = relative_paths.iter()
                .filter_map(|&path| base.join(path).ok())
                .map(|url| url.to_string())
                .collect();

            Ok(format!("Joined with base '{}': {:?}",
                      url_str, joined))
        },
        "validate" => {
            match Url::parse(url_str) {
                Ok(url) => {
                    let has_host = url.host().is_some();
                    let is_secure = url.scheme() == "https";
                    let has_query = url.query().is_some();

                    Ok(format!("URL validation: valid=true, has_host={}, is_secure={}, has_query={}",
                              has_host, is_secure, has_query))
                },
                Err(e) => {
                    Ok(format!("URL validation: valid=false, error='{}'", e))
                }
            }
        },
        _ => Ok(format!("Unsupported operation '{}', use 'parse', 'manipulate', 'join', or 'validate'", operation))
    }
}

fn nom_demo(parser_type: &str, input: &str) -> AnyResult<String> {
    use rmx::nom::{
        IResult,
        character::complete::{digit1, alpha1, space0, space1, char},
        combinator::map_res,
        multi::separated_list0,
        sequence::{delimited, preceded},
        branch::alt,
        Parser,
    };

    match parser_type {
        "numbers" => {
            fn parse_number(input: &str) -> IResult<&str, i32> {
                map_res(digit1, |s: &str| s.parse::<i32>()).parse(input)
            }

            fn parse_numbers(input: &str) -> IResult<&str, Vec<i32>> {
                separated_list0(space1, parse_number).parse(input)
            }

            match parse_numbers(input) {
                Ok((remaining, numbers)) => {
                    let sum: i32 = numbers.iter().sum();
                    Ok(format!("Parsed numbers: {:?}, sum: {}, remaining: '{}'", numbers, sum, remaining))
                },
                Err(e) => Ok(format!("Parse failed: {}", e))
            }
        },
        "email" => {
            fn parse_email(input: &str) -> IResult<&str, (String, String)> {
                let (input, username) = alpha1(input)?;
                let (input, _) = char('@')(input)?;
                let (input, domain) = alpha1(input)?;
                let (input, _) = char('.')(input)?;
                let (input, tld) = alpha1(input)?;
                Ok((input, (username.to_string(), format!("{}.{}", domain, tld))))
            }

            match parse_email(input) {
                Ok((remaining, (username, domain))) => {
                    Ok(format!("Parsed email: user='{}', domain='{}', remaining: '{}'", username, domain, remaining))
                },
                Err(e) => Ok(format!("Email parse failed: {}", e))
            }
        },
        "json_simple" => {
            fn parse_string_value(input: &str) -> IResult<&str, String> {
                delimited(char('"'), alpha1, char('"')).parse(input)
                    .map(|(i, s)| (i, s.to_string()))
            }

            fn parse_number_value(input: &str) -> IResult<&str, i32> {
                map_res(digit1, |s: &str| s.parse::<i32>()).parse(input)
            }

            fn parse_key_value(input: &str) -> IResult<&str, (String, String)> {
                let (input, _) = space0.parse(input)?;
                let (input, key) = delimited(char('"'), alpha1, char('"')).parse(input)?;
                let (input, _) = space0.parse(input)?;
                let (input, _) = char(':').parse(input)?;
                let (input, _) = space0.parse(input)?;
                let (input, value) = alt((
                    |i| parse_string_value(i).map(|(i, s)| (i, format!("\"{}\"", s))),
                    |i| parse_number_value(i).map(|(i, n)| (i, n.to_string())),
                )).parse(input)?;
                Ok((input, (key.to_string(), value)))
            }

            fn parse_simple_json(input: &str) -> IResult<&str, Vec<(String, String)>> {
                delimited(
                    char('{'),
                    separated_list0(char(','), parse_key_value),
                    preceded(space0, char('}')),
                ).parse(input)
            }

            match parse_simple_json(input) {
                Ok((remaining, pairs)) => {
                    Ok(format!("Parsed JSON pairs: {:?}, remaining: '{}'", pairs, remaining))
                },
                Err(e) => Ok(format!("JSON parse failed: {}", e))
            }
        },
        _ => Ok(format!("Unsupported parser type '{}', use 'numbers', 'email', or 'json_simple'", parser_type))
    }
}

fn thiserror_demo(error_type: &str, message: &str) -> AnyResult<String> {
    use std::fmt;

    // Demonstrate thiserror concepts manually for testing
    #[derive(Debug)]
    enum CustomError {
        Validation { message: String },
        Network(String),
        Parse { pos: usize, source: String },
        Io(std::io::Error),
        Multiple(Vec<CustomError>),
        Unknown(String),
    }

    impl fmt::Display for CustomError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CustomError::Validation { message } => {
                    write!(f, "Validation failed: {}", message)
                }
                CustomError::Network(msg) => write!(f, "Network error: {}", msg),
                CustomError::Parse { pos, source } => {
                    write!(f, "Parse error at position {}: {}", pos, source)
                }
                CustomError::Io(err) => write!(f, "IO error: {}", err),
                CustomError::Multiple(_) => write!(f, "Multiple errors occurred"),
                CustomError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
            }
        }
    }

    impl std::error::Error for CustomError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                CustomError::Io(err) => Some(err),
                _ => None,
            }
        }
    }

    impl From<std::io::Error> for CustomError {
        fn from(err: std::io::Error) -> Self {
            CustomError::Io(err)
        }
    }

    match error_type {
        "validation" => {
            let err = CustomError::Validation { message: message.to_string() };
            Ok(format!("Created validation error: {}", err))
        },
        "network" => {
            let err = CustomError::Network(message.to_string());
            Ok(format!("Created network error: {}", err))
        },
        "parse" => {
            let err = CustomError::Parse {
                pos: message.len(),
                source: message.to_string()
            };
            Ok(format!("Created parse error: {}", err))
        },
        "io" => {
            let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, message);
            let err = CustomError::Io(io_err);
            Ok(format!("Created IO error: {}", err))
        },
        "multiple" => {
            let errors = vec![
                CustomError::Validation { message: message.to_string() },
                CustomError::Network(format!("Connection failed: {}", message)),
            ];
            let err = CustomError::Multiple(errors);
            Ok(format!("Created multiple errors: {}", err))
        },
        "chain" => {
            fn inner_function() -> Result<(), CustomError> {
                Err(CustomError::Network("Connection timeout".to_string()))
            }

            fn outer_function() -> Result<(), CustomError> {
                inner_function().map_err(|e| CustomError::Unknown(format!("Wrapped: {}", e)))
            }

            match outer_function() {
                Err(e) => Ok(format!("Error chaining example: {}", e)),
                Ok(_) => Ok("No error occurred".to_string()),
            }
        },
        "result" => {
            fn fallible_operation(should_fail: bool, msg: &str) -> Result<String, CustomError> {
                if should_fail {
                    Err(CustomError::Unknown(msg.to_string()))
                } else {
                    Ok("Success!".to_string())
                }
            }

            let should_fail = message.contains("fail");
            match fallible_operation(should_fail, message) {
                Ok(result) => Ok(format!("Operation succeeded: {}", result)),
                Err(e) => Ok(format!("Operation failed: {}", e)),
            }
        },
        _ => {
            let err = CustomError::Unknown(format!("Unsupported error type '{}'", error_type));
            Ok(format!("Available types: validation, network, parse, io, multiple, chain, result. Error: {}", err))
        }
    }
}

fn xshell_demo(operation: &str, command: &str) -> AnyResult<String> {
    use rmx::xshell::{Shell, cmd};

    match operation {
        "info" => {
            let sh = Shell::new()?;
            let cwd = sh.current_dir();
            Ok(format!("Shell info: current directory = {}", cwd.display()))
        },
        "echo" => {
            let sh = Shell::new()?;
            let output = cmd!(sh, "echo {command}").read()?;
            Ok(format!("Echo output: '{}'", output.trim()))
        },
        "pwd" => {
            let sh = Shell::new()?;
            let output = cmd!(sh, "pwd").read()?;
            Ok(format!("Current directory: '{}'", output.trim()))
        },
        "ls" => {
            let sh = Shell::new()?;
            let output = cmd!(sh, "ls -la").read().unwrap_or_else(|_| "ls command failed".to_string());
            let lines = output.lines().take(5).collect::<Vec<_>>();
            Ok(format!("Directory listing (first 5 lines):\n{}", lines.join("\n")))
        },
        "env" => {
            let sh = Shell::new()?;
            let key = command;
            if let Ok(value) = sh.var(key) {
                Ok(format!("Environment variable {}: '{}'", key, value))
            } else {
                Ok(format!("Environment variable {} not found", key))
            }
        },
        "pushd" => {
            let sh = Shell::new()?;
            let start_dir = sh.current_dir();
            let path = if command.is_empty() { "." } else { command };

            let _guard = sh.push_dir(path);
            let new_dir = sh.current_dir();
            Ok(format!("Push dir: {} -> {} (with guard)", start_dir.display(), new_dir.display()))
        },
        "cmd" => {
            let sh = Shell::new()?;
            let parts: Vec<&str> = command.split_whitespace().collect();
            if parts.is_empty() {
                return Ok("No command provided".to_string());
            }

            match parts[0] {
                "echo" => {
                    let msg = parts.get(1).unwrap_or(&"xshell test");
                    let output = cmd!(sh, "echo {msg}").read()?;
                    Ok(format!("Command '{}' output: '{}'", command, output.trim()))
                },
                "date" => {
                    let output = cmd!(sh, "date").read().unwrap_or_else(|_| "date command not available".to_string());
                    Ok(format!("Command '{}' output: '{}'", command, output.trim()))
                },
                _ => {
                    Ok(format!("Command '{}' not supported in demo. Available: echo, date", command))
                }
            }
        },
        "pipe" => {
            let sh = Shell::new()?;
            // Demonstrate command pipelining concept (xshell doesn't have direct pipe support)
            let echo_output = cmd!(sh, "echo {command}").read()?;
            let word_count = echo_output.split_whitespace().count();
            Ok(format!("Pipe simulation: 'echo {}' has {} words", command, word_count))
        },
        _ => {
            Ok(format!("Unsupported operation '{}'. Available: info, echo, pwd, ls, env, pushd, cmd, pipe", operation))
        }
    }
}

fn crossbeam_demo(operation: &str, count: usize) -> AnyResult<String> {
    use rmx::crossbeam::channel::{bounded, unbounded, select};
    use std::time::Duration;
    use std::thread;

    match operation {
        "channel" => {
            let (tx, rx) = unbounded();

            // Send messages from multiple threads
            let handles: Vec<_> = (0..count)
                .map(|i| {
                    let tx = tx.clone();
                    thread::spawn(move || {
                        tx.send(format!("Message {}", i)).ok();
                    })
                })
                .collect();

            // Wait for threads and collect messages
            for handle in handles {
                handle.join().ok();
            }
            drop(tx); // Close sender to end receiver loop

            let mut messages = Vec::new();
            while let Ok(msg) = rx.recv() {
                messages.push(msg);
            }

            Ok(format!("Channel demo: Sent {} messages, received {} messages", count, messages.len()))
        },
        "bounded" => {
            let (tx, rx) = bounded(2); // Small buffer

            let sender = thread::spawn(move || {
                for i in 0..count {
                    let msg = format!("Bounded {}", i);
                    if tx.send(msg).is_err() {
                        break;
                    }
                    thread::sleep(Duration::from_millis(10));
                }
                count
            });

            let receiver = thread::spawn(move || {
                let mut received = 0;
                while let Ok(_) = rx.recv() {
                    received += 1;
                }
                received
            });

            let sent = sender.join().unwrap_or(0);
            let received = receiver.join().unwrap_or(0);

            Ok(format!("Bounded channel demo: sent={}, received={}", sent, received))
        },
        "select" => {
            let (tx1, rx1) = unbounded();
            let (tx2, rx2) = unbounded();

            // Send on different channels
            thread::spawn(move || {
                for i in 0..count {
                    tx1.send(format!("Channel1-{}", i)).ok();
                    thread::sleep(Duration::from_millis(5));
                }
            });

            thread::spawn(move || {
                for i in 0..count {
                    tx2.send(format!("Channel2-{}", i)).ok();
                    thread::sleep(Duration::from_millis(7));
                }
            });

            let mut results = Vec::new();
            let mut total_received = 0;

            // Use select to receive from multiple channels
            loop {
                if total_received >= count * 2 {
                    break;
                }

                select! {
                    recv(rx1) -> msg => {
                        if let Ok(msg) = msg {
                            results.push(msg);
                            total_received += 1;
                        }
                    }
                    recv(rx2) -> msg => {
                        if let Ok(msg) = msg {
                            results.push(msg);
                            total_received += 1;
                        }
                    }
                    default(Duration::from_millis(100)) => {
                        break;
                    }
                }
            }

            Ok(format!("Select demo: received {} messages from 2 channels", results.len()))
        },
        "scope" => {
            use rmx::crossbeam::scope;

            let data: Vec<i32> = (0..count as i32).collect();
            let mut results = Vec::new();

            scope(|s| {
                // Spawn scoped threads that can borrow from the stack
                let handles: Vec<_> = data
                    .chunks(count / 2 + 1)
                    .enumerate()
                    .map(|(chunk_id, chunk)| {
                        s.spawn(move |_| {
                            let sum: i32 = chunk.iter().sum();
                            (chunk_id, sum, chunk.len())
                        })
                    })
                    .collect();

                for handle in handles {
                    if let Ok(result) = handle.join() {
                        results.push(result);
                    }
                }
            }).unwrap();

            Ok(format!("Scoped threads demo: processed {} chunks, total results: {:?}", results.len(), results))
        },
        "deque" => {
            use rmx::crossbeam::deque::Worker;

            let worker = Worker::new_fifo();
            let stealer = worker.stealer();

            // Push items
            for i in 0..count {
                worker.push(i);
            }

            // Steal items from another thread
            let stealer_thread = thread::spawn(move || {
                let mut stolen = Vec::new();
                loop {
                    match stealer.steal() {
                        rmx::crossbeam::deque::Steal::Success(item) => stolen.push(item),
                        rmx::crossbeam::deque::Steal::Empty => break,
                        rmx::crossbeam::deque::Steal::Retry => continue,
                    }
                }
                stolen
            });

            // Pop items from worker
            let mut popped = Vec::new();
            while let Some(item) = worker.pop() {
                popped.push(item);
            }

            let stolen = stealer_thread.join().unwrap();

            Ok(format!("Work-stealing deque: pushed={}, popped={}, stolen={}",
                     count, popped.len(), stolen.len()))
        },
        _ => {
            Ok(format!("Unsupported operation '{}'. Available: channel, bounded, select, scope, deque", operation))
        }
    }
}

fn tempfile_demo(operation: &str, content: &str) -> AnyResult<String> {
    use rmx::tempfile::{tempfile, tempdir, NamedTempFile, TempDir};
    use std::io::{Write, Read, Seek, SeekFrom};

    match operation {
        "create" => {
            let mut temp_file = tempfile()?;
            write!(temp_file, "{}", content)?;
            temp_file.seek(SeekFrom::Start(0))?;

            let mut read_content = String::new();
            temp_file.read_to_string(&mut read_content)?;

            Ok(format!("Created anonymous tempfile, wrote {} bytes, read back: '{}'",
                     content.len(), read_content))
        },
        "named" => {
            let mut named_file = NamedTempFile::new()?;
            let path = named_file.path().display().to_string();

            write!(named_file, "{}", content)?;
            named_file.seek(SeekFrom::Start(0))?;

            let mut read_content = String::new();
            named_file.read_to_string(&mut read_content)?;

            Ok(format!("Created named tempfile at '{}', content: '{}'",
                     path, read_content))
        },
        "persist" => {
            let temp_file = NamedTempFile::new()?;
            let temp_path = temp_file.path().display().to_string();

            let mut file = temp_file.reopen()?;
            write!(file, "{}", content)?;

            // Note: In a real scenario we'd persist, but for testing we'll just show the path
            Ok(format!("Would persist tempfile '{}' with content: '{}'",
                     temp_path, content))
        },
        "dir" => {
            let temp_dir = tempdir()?;
            let dir_path = temp_dir.path().display().to_string();

            // Create a file inside the temp directory
            let file_path = temp_dir.path().join("test.txt");
            std::fs::write(&file_path, content)?;

            let read_content = std::fs::read_to_string(&file_path)?;

            Ok(format!("Created tempdir '{}', wrote file with content: '{}'",
                     dir_path, read_content))
        },
        "scoped" => {
            let temp_dir = TempDir::new()?;
            let dir_path = temp_dir.path().display().to_string();

            // Create multiple files
            for i in 0..3 {
                let file_path = temp_dir.path().join(format!("file_{}.txt", i));
                std::fs::write(&file_path, format!("{}-{}", content, i))?;
            }

            let entries = std::fs::read_dir(temp_dir.path())?
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
                .collect::<Vec<_>>();

            Ok(format!("Created scoped tempdir '{}' with {} files: {:?}",
                     dir_path, entries.len(), entries))
        },
        "builder" => {
            use rmx::tempfile::Builder;

            let temp_file = Builder::new()
                .prefix("rustmax-")
                .suffix(".tmp")
                .tempfile()?;

            let path = temp_file.path().display().to_string();
            let mut file = temp_file.reopen()?;
            write!(file, "{}", content)?;

            Ok(format!("Built custom tempfile '{}' with prefix/suffix, content: '{}'",
                     path, content))
        },
        "multiple" => {
            let mut files = Vec::new();
            let mut paths = Vec::new();

            // Create multiple temporary files
            for i in 0..3 {
                let mut temp_file = NamedTempFile::new()?;
                let path = temp_file.path().display().to_string();
                write!(temp_file, "{}-{}", content, i)?;

                paths.push(path);
                files.push(temp_file);
            }

            Ok(format!("Created {} tempfiles: first at '{}', last at '{}'",
                     files.len(), paths.first().unwrap_or(&"none".to_string()),
                     paths.last().unwrap_or(&"none".to_string())))
        },
        _ => {
            Ok(format!("Unsupported operation '{}'. Available: create, named, persist, dir, scoped, builder, multiple", operation))
        }
    }
}

fn json5_demo(operation: &str, data: &str) -> AnyResult<String> {
    use rmx::json5;
    use rmx::serde_json::Value;

    match operation {
        "parse" => {
            match json5::from_str::<Value>(data) {
                Ok(value) => Ok(format!("JSON5 parsed successfully: {}", value)),
                Err(e) => Ok(format!("JSON5 parse error: {}", e)),
            }
        },
        "comments" => {
            let json5_with_comments = r#"{
                // This is a comment
                "name": "example", /* inline comment */
                "numbers": [1, 2, 3], // trailing comment
                /* multi-line
                   comment */
                "value": 42
            }"#;

            match json5::from_str::<Value>(json5_with_comments) {
                Ok(value) => Ok(format!("JSON5 with comments parsed: {}", value)),
                Err(e) => Ok(format!("JSON5 comments parse error: {}", e)),
            }
        },
        "trailing" => {
            let json5_with_trailing = r#"{
                "items": [1, 2, 3,],  // trailing comma in array
                "object": {
                    "key1": "value1",
                    "key2": "value2", // trailing comma in object
                },
            }"#;

            match json5::from_str::<Value>(json5_with_trailing) {
                Ok(value) => Ok(format!("JSON5 with trailing commas: {}", value)),
                Err(e) => Ok(format!("JSON5 trailing commas error: {}", e)),
            }
        },
        "keys" => {
            let json5_unquoted = r#"{
                unquoted: "value1",
                'single-quoted': "value2",
                "double-quoted": "value3",
                validIdentifier: 123,
                number: 42,
            }"#;

            match json5::from_str::<Value>(json5_unquoted) {
                Ok(value) => Ok(format!("JSON5 with unquoted keys: {}", value)),
                Err(e) => Ok(format!("JSON5 unquoted keys error: {}", e)),
            }
        },
        "strings" => {
            let json5_strings = r#"{
                "multiline": "This is a \
long string that spans \
multiple lines",
                "escapes": "\x41\u0042\u{43}",
                "single": 'Single quoted string',
                "template": `Template ${literal} string`,
            }"#;

            match json5::from_str::<Value>(json5_strings) {
                Ok(value) => Ok(format!("JSON5 string features: {}", value)),
                Err(e) => Ok(format!("JSON5 strings error: {}", e)),
            }
        },
        "numbers" => {
            let json5_numbers = r#"{
                "hex": 0xFF,
                "positive": +42,
                "infinity": Infinity,
                "negative_infinity": -Infinity,
                "not_a_number": NaN,
            }"#;

            match json5::from_str::<Value>(json5_numbers) {
                Ok(value) => Ok(format!("JSON5 number formats: {}", value)),
                Err(e) => Ok(format!("JSON5 numbers error: {}", e)),
            }
        },
        "compare" => {
            let standard_json = r#"{"name": "test", "value": 42}"#;
            let json5_equivalent = r#"{
                // JSON5 version with comments
                name: 'test', /* unquoted key, single quotes */
                value: +42,   // explicit positive sign
            }"#;

            let json_result = json5::from_str::<Value>(standard_json)?;
            let json5_result = json5::from_str::<Value>(json5_equivalent)?;

            Ok(format!("Standard JSON: {} | JSON5: {} | Equal: {}",
                     json_result, json5_result, json_result == json5_result))
        },
        _ => {
            Ok(format!("Unsupported operation '{}'. Available: parse, comments, trailing, keys, strings, numbers, compare", operation))
        }
    }
}

fn tera_demo(operation: &str, name: &str) -> AnyResult<String> {
    use rmx::tera::{Tera, Context};
    use rmx::serde_json::json;

    match operation {
        "render" => {
            let mut tera = Tera::new("templates/**/*").unwrap_or_else(|_| Tera::default());
            let template = "Hello {{ name }}! Welcome to {{ app_name }}.";

            let mut context = Context::new();
            context.insert("name", name);
            context.insert("app_name", "Rustmax Suite");

            match tera.render_str(template, &context) {
                Ok(result) => Ok(format!("Template rendered: '{}'", result)),
                Err(e) => Ok(format!("Template error: {}", e)),
            }
        },
        "loops" => {
            let mut tera = Tera::default();
            let template = r#"Items: {% for item in items %}{{ item }}{% if not loop.last %}, {% endif %}{% endfor %}"#;

            let mut context = Context::new();
            context.insert("items", &vec!["apple", "banana", "cherry"]);

            match tera.render_str(template, &context) {
                Ok(result) => Ok(format!("Loop template: '{}'", result)),
                Err(e) => Ok(format!("Loop template error: {}", e)),
            }
        },
        "conditions" => {
            let mut tera = Tera::default();
            let template = r#"{% if user.admin %}Admin: {{ user.name }}{% else %}User: {{ user.name }}{% endif %}"#;

            let mut context = Context::new();
            context.insert("user", &json!({
                "name": name,
                "admin": name == "admin"
            }));

            match tera.render_str(template, &context) {
                Ok(result) => Ok(format!("Conditional template: '{}'", result)),
                Err(e) => Ok(format!("Conditional template error: {}", e)),
            }
        },
        "filters" => {
            let mut tera = Tera::default();
            let template = r#"Original: {{ text }} | Upper: {{ text | upper }} | Length: {{ text | length }}"#;

            let mut context = Context::new();
            context.insert("text", name);

            match tera.render_str(template, &context) {
                Ok(result) => Ok(format!("Filter template: '{}'", result)),
                Err(e) => Ok(format!("Filter template error: {}", e)),
            }
        },
        "inheritance" => {
            let mut tera = Tera::default();

            // Add base template
            let base_template = r#"<!DOCTYPE html>
<html>
<head><title>{{ title }}</title></head>
<body>
    <header>{{ app_name }}</header>
    <main>{% block content %}Default content{% endblock %}</main>
</body>
</html>"#;

            let child_template = r#"{% extends "base.html" %}
{% block content %}
<h1>Hello {{ name }}!</h1>
<p>This is child template content.</p>
{% endblock %}"#;

            tera.add_raw_template("base.html", base_template).unwrap();
            tera.add_raw_template("child.html", child_template).unwrap();

            let mut context = Context::new();
            context.insert("title", "Tera Demo");
            context.insert("app_name", "Rustmax Suite");
            context.insert("name", name);

            match tera.render("child.html", &context) {
                Ok(result) => Ok(format!("Inheritance template rendered ({} chars)", result.len())),
                Err(e) => Ok(format!("Inheritance template error: {}", e)),
            }
        },
        "macros" => {
            let mut tera = Tera::default();
            let template = r#"
{% macro input(label, name, type="text") %}
<label>{{ label }}: <input type="{{ type }}" name="{{ name }}" /></label>
{% endmacro %}

Form: {{ self::input(label="Name", name="user_name") }}
"#;

            let context = Context::new();
            match tera.render_str(template, &context) {
                Ok(result) => Ok(format!("Macro template: '{}'", result.trim())),
                Err(e) => Ok(format!("Macro template error: {}", e)),
            }
        },
        "globals" => {
            let mut tera = Tera::default();

            // Add global variables
            tera.add_raw_template("test", "App: {{ APP_NAME }} | Version: {{ VERSION }} | User: {{ user }}").unwrap();

            let mut context = Context::new();
            context.insert("APP_NAME", "Rustmax Suite");
            context.insert("VERSION", "1.0.0");
            context.insert("user", name);

            match tera.render("test", &context) {
                Ok(result) => Ok(format!("Global variables: '{}'", result)),
                Err(e) => Ok(format!("Globals template error: {}", e)),
            }
        },
        _ => {
            Ok(format!("Unsupported operation '{}'. Available: render, loops, conditions, filters, inheritance, macros, globals", operation))
        }
    }
}

fn unicode_demo(operation: &str, text: &str) -> AnyResult<String> {
    use rmx::unicode_segmentation::UnicodeSegmentation;

    match operation {
        "graphemes" => {
            let graphemes: Vec<&str> = text.graphemes(true).collect();
            let count = graphemes.len();
            let first_few = graphemes.iter().take(5).cloned().collect::<Vec<_>>();

            Ok(format!("Graphemes: {} total, first few: {:?}", count, first_few))
        },
        "words" => {
            let words: Vec<&str> = text.split_word_bounds().collect();
            let word_count = words.iter().filter(|w| !w.chars().all(char::is_whitespace)).count();
            let actual_words: Vec<&str> = words.iter()
                .filter(|w| !w.chars().all(char::is_whitespace))
                .take(5)
                .cloned()
                .collect();

            Ok(format!("Words: {} total, first few: {:?}", word_count, actual_words))
        },
        "sentences" => {
            let sentences: Vec<&str> = text.split_sentence_bounds().collect();
            let sentence_count = sentences.len();
            let first_sentence = sentences.first().unwrap_or(&"");

            Ok(format!("Sentences: {} total, first: '{}'", sentence_count, first_sentence.trim()))
        },
        "indices" => {
            let grapheme_indices: Vec<(usize, &str)> = text.grapheme_indices(true).collect();
            let indices_info = grapheme_indices.iter()
                .take(3)
                .map(|(i, g)| format!("{}:'{}'", i, g))
                .collect::<Vec<_>>();

            Ok(format!("Grapheme indices (first 3): [{}]", indices_info.join(", ")))
        },
        "width" => {
            // Calculate display width of text (accounting for wide chars, etc.)
            let chars: Vec<char> = text.chars().collect();
            let char_count = chars.len();
            let grapheme_count = text.graphemes(true).count();
            let byte_count = text.len();

            // Simple width calculation (this is approximation)
            let estimated_width = text.chars()
                .map(|c| if c.is_ascii() { 1 } else { 2 })
                .sum::<usize>();

            Ok(format!("Text analysis: {} bytes, {} chars, {} graphemes, ~{} display width",
                     byte_count, char_count, grapheme_count, estimated_width))
        },
        "boundaries" => {
            let word_boundaries: Vec<usize> = text.split_word_bound_indices()
                .map(|(i, _)| i)
                .take(5)
                .collect();
            let sentence_boundaries: Vec<usize> = text.split_sentence_bound_indices()
                .map(|(i, _)| i)
                .take(3)
                .collect();

            Ok(format!("Boundaries - words: {:?}, sentences: {:?}",
                     word_boundaries, sentence_boundaries))
        },
        "reverse" => {
            // Reverse by graphemes (preserving complex characters)
            let reversed: String = text.graphemes(true).rev().collect();

            Ok(format!("Original: '{}' | Reversed by graphemes: '{}'", text, reversed))
        },
        "compare" => {
            let simple_text = "Hello World";
            let complex_text = "Hello 🌍👨‍👩‍👧‍👦";

            let simple_chars = simple_text.chars().count();
            let simple_graphemes = simple_text.graphemes(true).count();
            let complex_chars = complex_text.chars().count();
            let complex_graphemes = complex_text.graphemes(true).count();

            Ok(format!("Simple '{}': {} chars, {} graphemes | Complex '{}': {} chars, {} graphemes",
                     simple_text, simple_chars, simple_graphemes,
                     complex_text, complex_chars, complex_graphemes))
        },
        _ => {
            Ok(format!("Unsupported operation '{}'. Available: graphemes, words, sentences, indices, width, boundaries, reverse, compare", operation))
        }
    }
}

fn logging_demo(level: &str, message: &str) -> AnyResult<String> {
    use rmx::log::{info, warn, error, debug, trace};

    match level {
        "init" => {
            // Initialize env_logger (this would normally be done once at startup)
            unsafe { std::env::set_var("RUST_LOG", "debug"); }
            let _ = rmx::env_logger::try_init();

            Ok("Logger initialized with RUST_LOG=debug".to_string())
        },
        "levels" => {
            // Demonstrate different log levels
            trace!("Trace message: {}", message);
            debug!("Debug message: {}", message);
            info!("Info message: {}", message);
            warn!("Warning message: {}", message);
            error!("Error message: {}", message);

            Ok(format!("Logged '{}' at all levels (trace, debug, info, warn, error)", message))
        },
        "structured" => {
            // Demonstrate structured logging
            info!(target: "app::module", "Structured log with message: {}", message);
            warn!(target: "app::security", "Security alert: {}", message);
            error!(target: "app::database", "Database issue: {}", message);

            Ok(format!("Structured logging demo with targets for message: '{}'", message))
        },
        "macros" => {
            // Demonstrate different log macro features
            let user_id = 12345;
            let operation = "test_op";

            info!("User {} performed operation: {} with data: {}", user_id, operation, message);
            debug!("Debug info for operation {}: data length = {}", operation, message.len());

            Ok(format!("Macro logging demo: user={}, op={}, msg='{}'", user_id, operation, message))
        },
        "conditional" => {
            // Demonstrate conditional logging
            let is_debug = true;
            let error_count = 5;

            if rmx::log::log_enabled!(rmx::log::Level::Debug) {
                debug!("Debug logging is enabled: {}", message);
            }

            if error_count > 3 {
                warn!("Error count {} is high, last error: {}", error_count, message);
            }

            Ok(format!("Conditional logging: debug_enabled={}, error_count={}", is_debug, error_count))
        },
        "capture" => {
            // Capture log output (simplified example)
            // In a real scenario, you'd use a custom logger implementation
            // For demo, we'll just show the concept
            info!("Message to be captured: {}", message);
            debug!("Debug info: processing '{}'", message);
            warn!("Warning about '{}'", message);

            let captured_count = 3; // Simulated captured messages
            Ok(format!("Log capture simulation: would capture {} messages containing '{}'", captured_count, message))
        },
        "performance" => {
            use std::time::Instant;

            let start = Instant::now();

            // Simulate some work with logging
            for i in 0..10 {
                debug!("Processing item {}: {}", i, message);
            }

            let duration = start.elapsed();
            info!("Performance test completed in {:?}", duration);

            Ok(format!("Performance logging: 10 debug messages in {:?}", duration))
        },
        _ => {
            warn!("Unknown logging operation: {}", level);
            Ok(format!("Unknown operation '{}'. Available: init, levels, structured, macros, conditional, capture, performance", level))
        }
    }
}

fn proptest_demo(test_type: &str, iterations: u32) -> AnyResult<String> {
    match test_type {
        "basic" => {
            // Basic property test - addition is commutative
            let mut passed_tests = 0;
            let mut total_tests = 0;

            // Simulate property testing (in real scenario, proptest! macro would handle this)
            for _ in 0..std::cmp::min(iterations, 20) {
                let a = rmx::rand::random::<i32>() % 1000;
                let b = rmx::rand::random::<i32>() % 1000;

                total_tests += 1;
                if a + b == b + a {
                    passed_tests += 1;
                }
            }

            Ok(format!("Commutative addition property: {}/{} tests passed", passed_tests, total_tests))
        },
        "string" => {
            // String property testing - reverse of reverse equals original
            let mut passed_tests = 0;
            let test_count = std::cmp::min(iterations, 50) as usize;

            for _ in 0..test_count {
                let original = format!("test_{}", rmx::rand::random::<u32>());
                let double_reversed: String = original.chars().rev().collect::<String>()
                    .chars().rev().collect();

                if original == double_reversed {
                    passed_tests += 1;
                }
            }

            Ok(format!("String reverse property: {}/{} tests passed", passed_tests, test_count))
        },
        "collection" => {
            // Collection properties - length preservation
            let mut passed_length = 0;
            let mut passed_contains = 0;
            let test_count = std::cmp::min(iterations, 30) as usize;

            for _ in 0..test_count {
                let original: Vec<i32> = (0..10).map(|_| rmx::rand::random::<i32>() % 100).collect();
                let mut sorted = original.clone();
                sorted.sort();

                // Length should be preserved
                if original.len() == sorted.len() {
                    passed_length += 1;
                }

                // All original elements should be present
                let all_present = original.iter().all(|item| sorted.contains(item));
                if all_present {
                    passed_contains += 1;
                }
            }

            Ok(format!("Collection sort properties: length={}/{}, contains={}/{}",
                      passed_length, test_count, passed_contains, test_count))
        },
        "numeric" => {
            // Numeric properties - multiplication and division
            let mut passed_tests = 0;
            let test_count = std::cmp::min(iterations, 25) as usize;

            for _ in 0..test_count {
                let a = (rmx::rand::random::<u32>() % 1000) + 1; // Avoid zero
                let b = (rmx::rand::random::<u32>() % 100) + 1;

                let product = a * b;
                let quotient = product / b;

                if quotient == a {
                    passed_tests += 1;
                }
            }

            Ok(format!("Multiplication-division property: {}/{} tests passed", passed_tests, test_count))
        },
        "roundtrip" => {
            // Roundtrip property - serialize then deserialize using serde_json directly
            let mut passed_tests = 0;
            let test_count = std::cmp::min(iterations, 15) as usize;

            for i in 0..test_count {
                // Use a simple data structure that doesn't need derive macros
                let data = rmx::serde_json::json!({
                    "id": i as u32,
                    "name": format!("test_{}", rmx::rand::random::<u32>()),
                    "active": rmx::rand::random::<bool>()
                });

                match rmx::serde_json::to_string(&data) {
                    Ok(json) => {
                        match rmx::serde_json::from_str::<rmx::serde_json::Value>(&json) {
                            Ok(deserialized) => {
                                if data == deserialized {
                                    passed_tests += 1;
                                }
                            },
                            Err(_) => {},
                        }
                    },
                    Err(_) => {},
                }
            }

            Ok(format!("JSON roundtrip property: {}/{} tests passed", passed_tests, test_count))
        },
        "invariant" => {
            // Invariant testing - data structure constraints
            let mut passed_tests = 0;
            let test_count = std::cmp::min(iterations, 20) as usize;

            for _ in 0..test_count {
                let mut data: Vec<i32> = (0..10).map(|_| rmx::rand::random::<i32>() % 100).collect();

                // Apply operations that should preserve invariants
                data.push(42);
                data.sort();

                // Invariant: sorted vector should be in non-decreasing order
                let is_sorted = data.windows(2).all(|w| w[0] <= w[1]);
                // Invariant: should contain our added element
                let contains_42 = data.contains(&42);

                if is_sorted && contains_42 {
                    passed_tests += 1;
                }
            }

            Ok(format!("Invariant testing: {}/{} tests passed", passed_tests, test_count))
        },
        "edge" => {
            // Edge case handling
            let mut boundary_tests = 0;
            let mut null_tests = 0;
            let test_count = std::cmp::min(iterations, 10) as usize;

            for _ in 0..test_count {
                // Test empty collections
                let empty_vec: Vec<i32> = vec![];
                let empty_string = String::new();

                if empty_vec.is_empty() && empty_string.is_empty() {
                    null_tests += 1;
                }

                // Test boundary values
                let max_val = i32::MAX;
                let min_val = i32::MIN;

                // Simple boundary arithmetic that shouldn't overflow in this test
                if max_val > min_val {
                    boundary_tests += 1;
                }
            }

            Ok(format!("Edge case testing: null={}/{}, boundary={}/{}",
                      null_tests, test_count, boundary_tests, test_count))
        },
        _ => {
            Ok(format!("Unknown test type '{}'. Available: basic, string, collection, numeric, roundtrip, invariant, edge", test_type))
        }
    }
}

fn anyhow_demo(error_type: &str, message: &str) -> AnyResult<String> {
    match error_type {
        "basic" => {
            // Basic anyhow error creation and handling
            use rmx::anyhow::anyhow;

            let should_fail = message.contains("fail");

            if should_fail {
                return Err(anyhow!("Basic error: {}", message));
            }

            Ok(format!("Basic anyhow success: processed '{}'", message))
        },
        "context" => {
            // Context chaining with anyhow
            use rmx::anyhow::{anyhow, Context};

            let operation = || -> rmx::anyhow::Result<i32> {
                if message.len() < 5 {
                    return Err(anyhow!("Message too short"));
                }
                Ok(message.len() as i32)
            };

            let result = operation()
                .with_context(|| format!("Failed to process message: '{}'", message))
                .with_context(|| "Error in anyhow context demo");

            match result {
                Ok(len) => Ok(format!("Context success: message length is {}", len)),
                Err(e) => Ok(format!("Context chain: {:#}", e)),
            }
        },
        "conversion" => {
            // Error conversion and std::error::Error compatibility
            use rmx::anyhow::{anyhow, Context};

            let parse_error = message.parse::<i32>();

            let result: rmx::anyhow::Result<String> = match parse_error {
                Ok(num) => Ok(format!("Parsed number: {}", num)),
                Err(std_err) => {
                    // Convert std::num::ParseIntError to anyhow::Error
                    Err(anyhow!("Parse conversion failed").context(std_err))
                }
            };

            match result {
                Ok(success_msg) => Ok(format!("Conversion success: {}", success_msg)),
                Err(e) => Ok(format!("Conversion error demo: {}", e)),
            }
        },
        "backtrace" => {
            // Backtrace demonstration (simplified)
            use rmx::anyhow::bail;

            fn inner_function(msg: &str) -> rmx::anyhow::Result<()> {
                if msg.is_empty() {
                    bail!("Empty message in inner function");
                }
                Ok(())
            }

            fn middle_function(msg: &str) -> rmx::anyhow::Result<String> {
                inner_function(msg)?;
                Ok(format!("Processed: {}", msg))
            }

            match middle_function(message) {
                Ok(result) => Ok(format!("Backtrace demo success: {}", result)),
                Err(e) => Ok(format!("Backtrace captured error: {}", e)),
            }
        },
        "chain" => {
            // Error chain demonstration
            use rmx::anyhow::Context;
            use std::fs;

            // Simulate a chain of operations that could fail
            let file_operation = || -> rmx::anyhow::Result<String> {
                // This will fail as the file doesn't exist, demonstrating error chaining
                fs::read_to_string("/nonexistent/path/file.txt")
                    .with_context(|| "Failed to read configuration file")
                    .with_context(|| format!("During processing of '{}'", message))
            };

            match file_operation() {
                Ok(content) => Ok(format!("Chain success: read {} bytes", content.len())),
                Err(e) => {
                    // Show the full error chain
                    let mut error_chain = vec![];
                    let mut current: &dyn std::error::Error = e.as_ref();
                    error_chain.push(current.to_string());

                    while let Some(source) = current.source() {
                        error_chain.push(source.to_string());
                        current = source;
                    }

                    Ok(format!("Error chain ({}): {}", error_chain.len(), error_chain.join(" → ")))
                }
            }
        },
        "custom" => {
            // Custom error types with anyhow
            #[derive(Debug)]
            struct CustomAppError {
                code: u32,
                message: String,
            }

            impl std::fmt::Display for CustomAppError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "AppError({}): {}", self.code, self.message)
                }
            }

            impl std::error::Error for CustomAppError {}

            let create_custom_error = || -> rmx::anyhow::Result<String> {
                let custom = CustomAppError {
                    code: 42,
                    message: format!("Custom error for: {}", message),
                };

                Err(rmx::anyhow::Error::new(custom))
            };

            match create_custom_error() {
                Ok(result) => Ok(format!("Custom success: {}", result)),
                Err(e) => Ok(format!("Custom error integration: {}", e)),
            }
        },
        "macro" => {
            // anyhow! macro variations
            use rmx::anyhow::{bail, ensure};

            let demo_macros = || -> rmx::anyhow::Result<String> {
                // ensure! macro - like assert but returns error
                ensure!(!message.is_empty(), "Message cannot be empty");
                ensure!(message.len() <= 100, "Message too long: {} chars", message.len());

                // bail! macro - early return with error
                if message.contains("panic") {
                    bail!("Refusing to process panic message: {}", message);
                }

                Ok(format!("Macro validation passed for: {}", message))
            };

            match demo_macros() {
                Ok(result) => Ok(format!("Macro demo success: {}", result)),
                Err(e) => Ok(format!("Macro validation failed: {}", e)),
            }
        },
        "format" => {
            // Error formatting demonstration
            use rmx::anyhow::{anyhow, Context};

            let complex_error = || -> rmx::anyhow::Result<()> {
                Err(anyhow!("Inner error with data: {}", message))
                    .context("Level 2 context")
                    .context("Level 1 context")
            };

            match complex_error() {
                Ok(_) => Ok("Format success".to_string()),
                Err(e) => {
                    let debug_format = format!("Debug format: {:?}", e);
                    let display_format = format!("Display format: {}", e);
                    let alt_format = format!("Alt format: {:#}", e);

                    Ok(format!("Format demo - Debug: {} chars, Display: {} chars, Alt: {} chars",
                              debug_format.len(), display_format.len(), alt_format.len()))
                }
            }
        },
        _ => {
            Ok(format!("Unknown error type '{}'. Available: basic, context, conversion, backtrace, chain, custom, macro, format", error_type))
        }
    }
}

fn reqwest_demo(operation: &str, url: &str) -> AnyResult<String> {
    match operation {
        "client" => {
            // Basic client creation and configuration
            let _client = rmx::reqwest::blocking::Client::new();

            // Just confirm client creation since timeout() method isn't available on the client
            Ok(format!("Client created successfully. Ready for HTTP operations."))
        },
        "get" => {
            // Simple GET request
            let client = rmx::reqwest::blocking::Client::new();

            match client.get(url).send() {
                Ok(response) => {
                    let status = response.status();
                    let headers_count = response.headers().len();

                    Ok(format!("GET request successful: status={}, headers_count={}, url='{}'",
                              status, headers_count, url))
                },
                Err(e) => Ok(format!("GET request failed: {}", e)),
            }
        },
        "headers" => {
            // Request with custom headers
            let client = rmx::reqwest::blocking::Client::new();

            match client.get(url)
                .header("User-Agent", "rustmax-suite/1.0")
                .header("Accept", "application/json")
                .header("X-Custom-Header", "test-value")
                .send()
            {
                Ok(response) => {
                    let status = response.status();
                    let content_type = response.headers()
                        .get("content-type")
                        .map(|v| v.to_str().unwrap_or("unknown"))
                        .unwrap_or("none");

                    Ok(format!("Headers request: status={}, content-type={}", status, content_type))
                },
                Err(e) => Ok(format!("Headers request failed: {}", e)),
            }
        },
        "json" => {
            // JSON request and response handling
            let client = rmx::reqwest::blocking::Client::new();

            match client.get(url).send() {
                Ok(response) => {
                    let status = response.status();

                    match response.json::<rmx::serde_json::Value>() {
                        Ok(json_value) => {
                            let json_type = match &json_value {
                                rmx::serde_json::Value::Object(_) => "object",
                                rmx::serde_json::Value::Array(_) => "array",
                                rmx::serde_json::Value::String(_) => "string",
                                rmx::serde_json::Value::Number(_) => "number",
                                rmx::serde_json::Value::Bool(_) => "boolean",
                                rmx::serde_json::Value::Null => "null",
                            };

                            Ok(format!("JSON response: status={}, type={}, size={} chars",
                                      status, json_type, json_value.to_string().len()))
                        },
                        Err(e) => Ok(format!("JSON parsing failed: status={}, error={}", status, e)),
                    }
                },
                Err(e) => Ok(format!("JSON request failed: {}", e)),
            }
        },
        "timeout" => {
            // Timeout configuration
            let client = rmx::reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_millis(100))
                .build()
                .map_err(|e| rmx::anyhow::anyhow!("Client build failed: {}", e))?;

            match client.get(url).send() {
                Ok(response) => Ok(format!("Timeout test: surprisingly succeeded with status {}", response.status())),
                Err(e) => {
                    if e.is_timeout() {
                        Ok("Timeout test: correctly timed out".to_string())
                    } else {
                        Ok(format!("Timeout test: failed with non-timeout error: {}", e))
                    }
                }
            }
        },
        "builder" => {
            // Client builder with various options
            let client = rmx::reqwest::blocking::Client::builder()
                .user_agent("rustmax-suite-builder/1.0")
                .timeout(std::time::Duration::from_secs(10))
                .redirect(rmx::reqwest::redirect::Policy::limited(5))
                .build()
                .map_err(|e| rmx::anyhow::anyhow!("Client build failed: {}", e))?;

            match client.get(url).send() {
                Ok(response) => {
                    Ok(format!("Builder client: status={}, final_url='{}'",
                              response.status(), response.url()))
                },
                Err(e) => Ok(format!("Builder client request failed: {}", e)),
            }
        },
        "post" => {
            // POST request with JSON body
            let client = rmx::reqwest::blocking::Client::new();
            let test_data = rmx::serde_json::json!({
                "message": "test from rustmax-suite",
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                "type": "integration_test"
            });

            // Use httpbin.org for POST testing
            let post_url = if url.contains("httpbin") {
                "https://httpbin.org/post"
            } else {
                url
            };

            match client.post(post_url)
                .json(&test_data)
                .send()
            {
                Ok(response) => {
                    let status = response.status();
                    let content_length = response.content_length().unwrap_or(0);

                    Ok(format!("POST request: status={}, content_length={}", status, content_length))
                },
                Err(e) => Ok(format!("POST request failed: {}", e)),
            }
        },
        "async_demo" => {
            // Demonstrate that we have access to async reqwest (though we're in sync context)
            let rt = rmx::tokio::runtime::Runtime::new()
                .map_err(|e| rmx::anyhow::anyhow!("Failed to create runtime: {}", e))?;

            let result = rt.block_on(async {
                let client = rmx::reqwest::Client::new();
                match client.get(url).send().await {
                    Ok(response) => Ok(format!("Async GET: status={}, url='{}'", response.status(), response.url())),
                    Err(e) => Ok(format!("Async GET failed: {}", e)),
                }
            });

            result
        },
        _ => {
            Ok(format!("Unknown operation '{}'. Available: client, get, headers, json, timeout, builder, post, async_demo", operation))
        }
    }
}

fn dead_code() {
    eprintln!("dead code");
}
