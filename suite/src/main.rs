use rmx::prelude::*;
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
            _ => {
                println!("Unknown command. Available commands: greet, count, math, test, file, parse, serialize, crypto, time, regex, async, parallel, util, walk, rand, url");
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

fn dead_code() {
    eprintln!("dead code");
}
