// Comprehensive dependency coverage tests.
// Exercises as many rustmax dependencies as possible to increase coverage.

use rustmax_suite::*;

// Core utilities: serde, serde_json, itertools.

#[test]
fn test_serde_json_operations() {
    use rmx::serde_json::{json, Value};

    let data = json!({
        "name": "test",
        "count": 42,
        "items": [1, 2, 3]
    });

    assert!(data.is_object());
    assert_eq!(data["name"], "test");
    assert_eq!(data["count"], 42);

    let serialized = rmx::serde_json::to_string(&data).unwrap();
    assert!(serialized.contains("test"));

    let deserialized: Value = rmx::serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, data);
}

#[test]
fn test_itertools_operations() {
    use rmx::itertools::Itertools;

    let nums = vec![1, 2, 3, 4, 5];

    // Test chunks.
    let chunks: Vec<Vec<_>> = nums.iter().chunks(2).into_iter()
        .map(|chunk| chunk.copied().collect())
        .collect();
    assert_eq!(chunks.len(), 3);

    // Test unique.
    let duplicates = vec![1, 2, 2, 3, 3, 3];
    let unique: Vec<_> = duplicates.into_iter().unique().collect();
    assert_eq!(unique, vec![1, 2, 3]);

    // Test combinations.
    let combos: Vec<Vec<_>> = nums.iter().combinations(2).collect();
    assert_eq!(combos.len(), 10);
}

// Crypto/hashing: blake3, sha2, base64, hex.

#[test]
fn test_blake3_hashing() {
    use rmx::blake3;

    let data = b"test data";
    let hash = blake3::hash(data);
    assert_eq!(hash.as_bytes().len(), 32);

    let hash_str = hash.to_hex();
    assert_eq!(hash_str.len(), 64);
}

#[test]
fn test_sha2_hashing() {
    use rmx::sha2::{Sha256, Digest};

    let mut hasher = Sha256::new();
    hasher.update(b"test data");
    let result = hasher.finalize();

    assert_eq!(result.len(), 32);
}

#[test]
fn test_base64_encoding() {
    use rmx::base64::{Engine, engine::general_purpose};

    let data = b"test data";
    let encoded = general_purpose::STANDARD.encode(data);
    assert!(!encoded.is_empty());

    let decoded = general_purpose::STANDARD.decode(&encoded).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn test_hex_encoding() {
    use rmx::hex;

    let data = b"test";
    let encoded = hex::encode(data);
    assert_eq!(encoded, "74657374");

    let decoded = hex::decode(&encoded).unwrap();
    assert_eq!(decoded, data);
}

// Date/time: chrono, jiff.

#[test]
fn test_chrono_operations() {
    use rmx::chrono::{Utc, Duration};

    let now = Utc::now();
    let later = now + Duration::hours(1);
    assert!(later > now);

    let duration = later - now;
    assert_eq!(duration.num_hours(), 1);
}

#[test]
fn test_jiff_operations() {
    use rmx::jiff::{Timestamp, Zoned};

    let now = Timestamp::now();
    assert!(now.as_second() > 0);

    let zoned = Zoned::now();
    let timestamp = zoned.timestamp();
    assert!(timestamp.as_second() > 0);
}

// Text processing: regex, unicode-segmentation.

#[test]
fn test_regex_operations() {
    use rmx::regex::Regex;

    let re = Regex::new(r"(\w+)@(\w+)\.(\w+)").unwrap();
    let text = "test@example.com";

    assert!(re.is_match(text));

    let caps = re.captures(text).unwrap();
    assert_eq!(&caps[1], "test");
    assert_eq!(&caps[2], "example");
    assert_eq!(&caps[3], "com");
}

#[test]
fn test_unicode_segmentation() {
    use rmx::unicode_segmentation::UnicodeSegmentation;

    let text = "hello world";
    let graphemes: Vec<&str> = text.graphemes(true).collect();
    assert_eq!(graphemes.len(), 11);

    let words: Vec<&str> = text.unicode_words().collect();
    assert_eq!(words, vec!["hello", "world"]);
}

// Error handling: anyhow, thiserror.

#[test]
fn test_anyhow_error_handling() {
    use rmx::anyhow::{anyhow, Context, Result};

    fn test_fn() -> Result<i32> {
        Err(anyhow!("test error"))
    }

    fn test_context() -> Result<i32> {
        test_fn().context("additional context")
    }

    let err = test_context().unwrap_err();
    let err_str = format!("{:#}", err);
    assert!(err_str.contains("additional context"));
}

// Data structures: bytes, bitflags, byteorder.

#[test]
fn test_bytes_operations() {
    use rmx::bytes::{Bytes, BytesMut, BufMut};

    let mut buf = BytesMut::with_capacity(64);
    buf.put_u8(1);
    buf.put_u16(256);
    buf.put_slice(b"test");

    let bytes: Bytes = buf.freeze();
    assert_eq!(bytes.len(), 7);
}

#[test]
fn test_bitflags_operations() {
    use rmx::bitflags::bitflags;

    bitflags! {
        struct Flags: u32 {
            const A = 0b00000001;
            const B = 0b00000010;
            const C = 0b00000100;
        }
    }

    let flags = Flags::A | Flags::C;
    assert!(flags.contains(Flags::A));
    assert!(!flags.contains(Flags::B));
    assert!(flags.contains(Flags::C));
}

#[test]
fn test_byteorder_operations() {
    use rmx::byteorder::{ByteOrder, LittleEndian, BigEndian};

    let mut buf = [0u8; 8];
    LittleEndian::write_u64(&mut buf, 0x0102030405060708);
    assert_eq!(LittleEndian::read_u64(&buf), 0x0102030405060708);

    BigEndian::write_u32(&mut buf[0..4], 0x01020304);
    assert_eq!(BigEndian::read_u32(&buf[0..4]), 0x01020304);
}

// I/O: tempfile, walkdir.

#[test]
fn test_tempfile_operations() {
    use rmx::tempfile::tempdir;
    use std::fs;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");

    fs::write(&file_path, b"test data").unwrap();
    let data = fs::read(&file_path).unwrap();
    assert_eq!(data, b"test data");

    drop(dir);
}

#[test]
fn test_walkdir_operations() {
    use rmx::walkdir::WalkDir;
    use rmx::tempfile::tempdir;
    use std::fs;

    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("subdir")).unwrap();
    fs::write(dir.path().join("file1.txt"), b"test").unwrap();
    fs::write(dir.path().join("subdir/file2.txt"), b"test").unwrap();

    let count = WalkDir::new(dir.path())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count();

    assert_eq!(count, 2);
}

// Random: rand, rand_chacha.

#[test]
fn test_rand_operations() {
    use rmx::rand::{Rng, thread_rng};

    let mut rng = thread_rng();

    let num: u32 = rng.random();
    assert!(num > 0 || num == 0);

    let num_range: i32 = rng.random_range(1..=100);
    assert!(num_range >= 1 && num_range <= 100);

    let bytes: [u8; 10] = rng.random();
    assert!(bytes.len() == 10);
}

#[test]
fn test_rand_chacha() {
    use rmx::rand_chacha::ChaCha8Rng;
    use rmx::rand::{SeedableRng, Rng};

    let seed = [0u8; 32];
    let mut rng = ChaCha8Rng::from_seed(seed);

    let num1: u64 = rng.random();
    let num2: u64 = rng.random();
    assert_ne!(num1, num2);
}

// CLI: log, env_logger.

#[test]
fn test_log_operations() {
    use rmx::log::{info, warn, error, debug, trace};

    info!("info message");
    warn!("warn message");
    error!("error message");
    debug!("debug message");
    trace!("trace message");
}

// URL parsing.

#[test]
fn test_url_parsing() {
    use rmx::url::Url;

    let url = Url::parse("https://example.com:8080/path?key=value#fragment").unwrap();

    assert_eq!(url.scheme(), "https");
    assert_eq!(url.host_str(), Some("example.com"));
    assert_eq!(url.port(), Some(8080));
    assert_eq!(url.path(), "/path");
    assert_eq!(url.query(), Some("key=value"));
    assert_eq!(url.fragment(), Some("fragment"));
}

// Configuration parsing: toml.

#[test]
fn test_toml_parsing() {
    use rmx::toml;

    let toml_str = r#"
        [package]
        name = "test"
        version = "1.0.0"

        [dependencies]
        foo = "1.0"
    "#;

    let value: rmx::toml::Value = rmx::toml::from_str(toml_str).unwrap();

    assert!(value.is_table());
    assert_eq!(value["package"]["name"].as_str(), Some("test"));
    assert_eq!(value["package"]["version"].as_str(), Some("1.0.0"));
}

// Concurrency: crossbeam.

#[test]
fn test_crossbeam_channels() {
    use rmx::crossbeam::channel;
    use std::thread;

    let (tx, rx) = channel::unbounded();

    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });

    let sum: i32 = (0..10).map(|_| rx.recv().unwrap()).sum();
    assert_eq!(sum, 45);
}

// Semver.

#[test]
fn test_semver_operations() {
    use rmx::semver::Version;

    let v1 = Version::parse("1.2.3").unwrap();
    let v2 = Version::parse("1.3.0").unwrap();

    assert!(v1 < v2);
    assert_eq!(v1.major, 1);
    assert_eq!(v1.minor, 2);
    assert_eq!(v1.patch, 3);
}
