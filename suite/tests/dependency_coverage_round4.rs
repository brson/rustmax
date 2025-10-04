// Round 4: Deep dive on high-value, partially-covered dependencies.
// Goal: Increase coverage on deps that show potential but have shallow tests.

use rustmax_suite::*;

// ============================================================================
// REGEX - Comprehensive testing (current: 5.95%, target: 20%+)
// ============================================================================

#[test]
fn test_regex_basic_operations() {
    use rmx::regex::Regex;

    let re = Regex::new(r"(\w+)@(\w+)\.(\w+)").unwrap();
    let text = "test@example.com";

    assert!(re.is_match(text));

    let caps = re.captures(text).unwrap();
    assert_eq!(&caps[1], "test");
    assert_eq!(&caps[2], "example");
    assert_eq!(&caps[3], "com");

    // Test find operations.
    let m = re.find(text).unwrap();
    assert_eq!(m.as_str(), "test@example.com");
    assert_eq!(m.start(), 0);
    assert_eq!(m.end(), 16);
}

#[test]
fn test_regex_find_iter() {
    use rmx::regex::Regex;

    let re = Regex::new(r"\d+").unwrap();
    let text = "abc 123 def 456 ghi 789";

    let numbers: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    assert_eq!(numbers, vec!["123", "456", "789"]);

    let positions: Vec<(usize, usize)> = re
        .find_iter(text)
        .map(|m| (m.start(), m.end()))
        .collect();
    assert_eq!(positions, vec![(4, 7), (12, 15), (20, 23)]);
}

#[test]
fn test_regex_captures_iter() {
    use rmx::regex::Regex;

    let re = Regex::new(r"(\w+)=(\d+)").unwrap();
    let text = "x=10, y=20, z=30";

    let pairs: Vec<(String, String)> = re
        .captures_iter(text)
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
        .collect();

    assert_eq!(
        pairs,
        vec![
            ("x".to_string(), "10".to_string()),
            ("y".to_string(), "20".to_string()),
            ("z".to_string(), "30".to_string())
        ]
    );
}

#[test]
fn test_regex_split() {
    use rmx::regex::Regex;

    let re = Regex::new(r"\s+").unwrap();
    let text = "hello   world  \t  foo";

    let parts: Vec<&str> = re.split(text).collect();
    assert_eq!(parts, vec!["hello", "world", "foo"]);

    let re2 = Regex::new(r",\s*").unwrap();
    let csv = "a, b,c,  d";
    let fields: Vec<&str> = re2.split(csv).collect();
    assert_eq!(fields, vec!["a", "b", "c", "d"]);
}

#[test]
fn test_regex_replace() {
    use rmx::regex::Regex;

    let re = Regex::new(r"\d+").unwrap();
    let result = re.replace("age: 42", "XX");
    assert_eq!(result, "age: XX");

    let result_all = re.replace_all("I have 3 cats and 2 dogs", "many");
    assert_eq!(result_all, "I have many cats and many dogs");
}

#[test]
fn test_regex_builder() {
    use rmx::regex::RegexBuilder;

    let re = RegexBuilder::new(r"hello")
        .case_insensitive(true)
        .build()
        .unwrap();

    assert!(re.is_match("HELLO"));
    assert!(re.is_match("Hello"));
    assert!(re.is_match("hello"));

    let re_multiline = RegexBuilder::new(r"^test")
        .multi_line(true)
        .build()
        .unwrap();

    assert!(re_multiline.is_match("first\ntest\nlast"));
}

#[test]
fn test_regex_set() {
    use rmx::regex::RegexSet;

    let set = RegexSet::new(&[r"\w+@\w+\.com", r"\d{3}-\d{4}", r"[A-Z]{2}\d{5}"])
        .unwrap();

    assert!(set.is_match("test@example.com"));
    assert!(set.is_match("555-1234"));
    assert!(set.is_match("CA12345"));

    let matches: Vec<_> = set.matches("Contact: test@example.com or 555-1234").iter().collect();
    assert_eq!(matches, vec![0, 1]);
}

#[test]
fn test_regex_bytes() {
    use rmx::regex::bytes::Regex;

    let re = Regex::new(r"[0-9]+").unwrap();
    let text = b"The answer is 42";

    assert!(re.is_match(text));

    let m = re.find(text).unwrap();
    assert_eq!(m.as_bytes(), b"42");

    let all: Vec<&[u8]> = re.find_iter(b"1 2 3").map(|m| m.as_bytes()).collect();
    assert_eq!(all, vec![b"1", b"2", b"3"]);
}

// ============================================================================
// BYTES - Deep Buf/BufMut testing (current: 5.75%, target: 15%+)
// ============================================================================

#[test]
fn test_bytes_basic_operations() {
    use rmx::bytes::{Bytes, BytesMut, BufMut};

    let mut buf = BytesMut::with_capacity(64);
    buf.put_u8(1);
    buf.put_u16(256);
    buf.put_slice(b"test");

    let bytes: Bytes = buf.freeze();
    assert_eq!(bytes.len(), 7);
}

#[test]
fn test_bytes_buf_trait() {
    use rmx::bytes::{Buf, Bytes};

    let data = Bytes::from_static(b"\x01\x02\x03\x04test");
    let mut buf = data;

    assert_eq!(buf.remaining(), 8);

    let byte = buf.get_u8();
    assert_eq!(byte, 1);

    let word = buf.get_u16();
    assert_eq!(word, 0x0203);

    let mut dest = [0u8; 5];
    buf.copy_to_slice(&mut dest);
    assert_eq!(&dest, b"\x04test");

    assert_eq!(buf.remaining(), 0);
}

#[test]
fn test_bytes_buf_mut_additional() {
    use rmx::bytes::{BufMut, BytesMut};

    let mut buf = BytesMut::with_capacity(32);

    buf.put_i8(-1);
    buf.put_i16(-256);
    buf.put_i32(-1000);
    buf.put_i64(-1_000_000);

    buf.put_u32(0xDEADBEEF);
    buf.put_u64(0x0123456789ABCDEF);

    assert_eq!(buf.len(), 1 + 2 + 4 + 8 + 4 + 8);

    buf.put(&b"hello"[..]);
    assert!(buf.len() > 27);
}

#[test]
fn test_bytes_chain() {
    use rmx::bytes::{Buf, Bytes};

    let a = Bytes::from_static(b"hello");
    let b = Bytes::from_static(b"world");

    let mut chain = a.chain(b);

    assert_eq!(chain.remaining(), 10);

    let mut dest = [0u8; 10];
    chain.copy_to_slice(&mut dest);
    assert_eq!(&dest, b"helloworld");
}

#[test]
fn test_bytes_take() {
    use rmx::bytes::{Buf, Bytes};

    let data = Bytes::from_static(b"hello world");
    let mut limited = data.take(5);

    assert_eq!(limited.remaining(), 5);

    let mut dest = vec![0u8; 5];
    limited.copy_to_slice(&mut dest);
    assert_eq!(&dest, b"hello");
}

#[test]
fn test_bytes_clone_split() {
    use rmx::bytes::{Bytes, BytesMut};

    let data = Bytes::from_static(b"hello world");
    let cloned = data.clone();
    assert_eq!(data, cloned);

    let mut mut_data = BytesMut::from(&b"abcdefgh"[..]);
    let split_off = mut_data.split_off(4);
    assert_eq!(mut_data.as_ref(), b"abcd");
    assert_eq!(split_off.as_ref(), b"efgh");

    let second_split = mut_data.split();
    assert_eq!(second_split.as_ref(), b"abcd");
    assert_eq!(mut_data.len(), 0);
}

// ============================================================================
// AHASH - HashMap/HashSet direct usage (current: 17%, target: 30%+)
// ============================================================================

#[test]
fn test_ahash_hashmap() {
    use rmx::ahash::AHashMap;

    let mut map = AHashMap::new();
    map.insert("key1", 100);
    map.insert("key2", 200);
    map.insert("key3", 300);

    assert_eq!(map.get("key1"), Some(&100));
    assert_eq!(map.get("key2"), Some(&200));
    assert_eq!(map.len(), 3);

    map.remove("key2");
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key2"), None);

    for (k, v) in &map {
        assert!(k == &"key1" || k == &"key3");
        assert!(v == &100 || v == &300);
    }
}

#[test]
fn test_ahash_hashset() {
    use rmx::ahash::AHashSet;

    let mut set = AHashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(!set.contains(&4));

    assert_eq!(set.len(), 3);

    set.remove(&2);
    assert_eq!(set.len(), 2);
    assert!(!set.contains(&2));

    let mut other = AHashSet::new();
    other.insert(3);
    other.insert(4);

    let intersection: Vec<_> = set.intersection(&other).collect();
    assert_eq!(intersection, vec![&3]);
}

#[test]
fn test_ahash_custom_hasher() {
    use rmx::ahash::RandomState;
    use std::collections::HashMap;

    let hasher = RandomState::new();
    let mut map: HashMap<&str, i32, RandomState> = HashMap::with_hasher(hasher);

    map.insert("a", 1);
    map.insert("b", 2);

    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&2));
}

// ============================================================================
// SERDE - Additional serialization tests (current: 0%, target: 5%+)
// ============================================================================
// Note: Manual Serialize/Deserialize impls conflict with test infrastructure's
// StdError trait bounds. Focusing on usage patterns instead.

#[test]
fn test_serde_value_operations() {
    use rmx::serde_json::{json, Value};

    let mut obj = json!({
        "name": "test",
        "count": 42,
        "active": true,
        "tags": ["a", "b", "c"]
    });

    // Value manipulation.
    if let Some(map) = obj.as_object_mut() {
        map.insert("new_field".to_string(), json!("new_value"));
        map.remove("count");
    }

    assert!(obj["name"].is_string());
    assert!(obj["tags"].is_array());
    assert!(obj["new_field"].is_string());
    assert!(obj.get("count").is_none());
}

#[test]
fn test_serde_json_number() {
    use rmx::serde_json::{json, Number};

    let val = json!(42);
    if let Some(num) = val.as_i64() {
        assert_eq!(num, 42);
    }

    let float_val = json!(3.14);
    if let Some(f) = float_val.as_f64() {
        assert!((f - 3.14).abs() < 0.001);
    }

    let num = Number::from(100);
    assert_eq!(num.as_u64(), Some(100));
}

#[test]
fn test_serde_to_from_value() {
    use rmx::serde_json::{json, to_value, from_value};

    let data = vec![1, 2, 3, 4, 5];
    let val = to_value(&data).unwrap();

    assert!(val.is_array());

    let back: Vec<i32> = from_value(val).unwrap();
    assert_eq!(back, data);

    let complex = json!({
        "items": [1, 2, 3],
        "meta": {
            "count": 3
        }
    });

    assert_eq!(complex["items"][0], 1);
    assert_eq!(complex["meta"]["count"], 3);
}

// ============================================================================
// SEMVER - Comprehensive version operations (current: 7%, target: 15%+)
// ============================================================================

#[test]
fn test_semver_parsing() {
    use rmx::semver::Version;
    use std::str::FromStr;

    let v = Version::parse("1.2.3").unwrap();
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 3);

    let v2 = Version::from_str("2.0.0-alpha.1+build.123").unwrap();
    assert_eq!(v2.major, 2);
    assert_eq!(v2.pre.as_str(), "alpha.1");
    assert_eq!(v2.build.as_str(), "build.123");
}

#[test]
fn test_semver_comparison() {
    use rmx::semver::Version;

    let v1 = Version::parse("1.2.3").unwrap();
    let v2 = Version::parse("1.2.4").unwrap();
    let v3 = Version::parse("1.3.0").unwrap();
    let v4 = Version::parse("2.0.0").unwrap();

    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v3 < v4);

    assert_eq!(v1, Version::parse("1.2.3").unwrap());
}

#[test]
fn test_semver_version_req() {
    use rmx::semver::{Version, VersionReq};

    let req = VersionReq::parse(">=1.2.3, <2.0.0").unwrap();

    assert!(req.matches(&Version::parse("1.2.3").unwrap()));
    assert!(req.matches(&Version::parse("1.5.0").unwrap()));
    assert!(!req.matches(&Version::parse("2.0.0").unwrap()));
    assert!(!req.matches(&Version::parse("1.2.2").unwrap()));

    let caret = VersionReq::parse("^1.2.3").unwrap();
    assert!(caret.matches(&Version::parse("1.2.3").unwrap()));
    assert!(caret.matches(&Version::parse("1.9.9").unwrap()));
    assert!(!caret.matches(&Version::parse("2.0.0").unwrap()));

    let tilde = VersionReq::parse("~1.2.3").unwrap();
    assert!(tilde.matches(&Version::parse("1.2.3").unwrap()));
    assert!(tilde.matches(&Version::parse("1.2.9").unwrap()));
    assert!(!tilde.matches(&Version::parse("1.3.0").unwrap()));
}

// ============================================================================
// TOML - Complex structures (current: 9%, target: 15%+)
// ============================================================================

#[test]
fn test_toml_nested_structures() {
    use rmx::toml;

    let toml_str = r#"
        [package]
        name = "test"
        version = "1.0.0"

        [package.metadata]
        author = "Test Author"

        [[dependencies]]
        name = "serde"
        version = "1.0"

        [[dependencies]]
        name = "regex"
        version = "1.5"
    "#;

    let value: toml::Value = toml::from_str(toml_str).unwrap();

    assert_eq!(value["package"]["name"].as_str(), Some("test"));
    assert_eq!(value["package"]["version"].as_str(), Some("1.0.0"));
    assert_eq!(
        value["package"]["metadata"]["author"].as_str(),
        Some("Test Author")
    );

    let deps = value["dependencies"].as_array().unwrap();
    assert_eq!(deps.len(), 2);
    assert_eq!(deps[0]["name"].as_str(), Some("serde"));
}

#[test]
fn test_toml_serialize_complex() {
    use rmx::serde_json::json;
    use rmx::toml;

    let data = json!({
        "server": {
            "host": "localhost",
            "port": 8080,
            "ssl": true
        },
        "database": {
            "url": "postgres://localhost/db",
            "pool_size": 10
        }
    });

    let toml_string = toml::to_string(&data).unwrap();
    assert!(toml_string.contains("host"));
    assert!(toml_string.contains("8080"));
    assert!(toml_string.contains("postgres"));

    let parsed: toml::Value = toml::from_str(&toml_string).unwrap();
    assert_eq!(parsed["server"]["port"].as_integer(), Some(8080));
}

#[test]
fn test_toml_inline_tables() {
    use rmx::toml;

    let toml_str = r#"
        point = { x = 10, y = 20 }
        color = { r = 255, g = 128, b = 0 }
    "#;

    let value: toml::Value = toml::from_str(toml_str).unwrap();
    assert_eq!(value["point"]["x"].as_integer(), Some(10));
    assert_eq!(value["point"]["y"].as_integer(), Some(20));
    assert_eq!(value["color"]["r"].as_integer(), Some(255));
}

// ============================================================================
// URL - Comprehensive parsing (current: 18%, target: 25%+)
// ============================================================================

#[test]
fn test_url_query_params() {
    use rmx::url::Url;

    let mut url = Url::parse("https://example.com/search").unwrap();
    url.query_pairs_mut()
        .append_pair("q", "rust programming")
        .append_pair("page", "1")
        .append_pair("lang", "en");

    assert!(url.as_str().contains("q=rust+programming"));
    assert!(url.as_str().contains("page=1"));
    assert!(url.as_str().contains("lang=en"));

    let pairs: Vec<_> = url.query_pairs().collect();
    assert_eq!(pairs.len(), 3);
}

#[test]
fn test_url_path_segments() {
    use rmx::url::Url;

    let mut url = Url::parse("https://example.com/").unwrap();
    url.path_segments_mut()
        .unwrap()
        .push("api")
        .push("v1")
        .push("users");

    assert_eq!(url.path(), "/api/v1/users");

    let segments: Vec<_> = url.path_segments().unwrap().collect();
    assert_eq!(segments, vec!["api", "v1", "users"]);
}

#[test]
fn test_url_host_port() {
    use rmx::url::Url;

    let url = Url::parse("https://example.com:8080/path").unwrap();
    assert_eq!(url.host_str(), Some("example.com"));
    assert_eq!(url.port(), Some(8080));
    assert_eq!(url.scheme(), "https");

    let url2 = Url::parse("http://192.168.1.1/").unwrap();
    assert_eq!(url2.host_str(), Some("192.168.1.1"));
    assert_eq!(url2.port(), None);

    let url3 = Url::parse("https://user:pass@example.com/").unwrap();
    assert_eq!(url3.username(), "user");
    assert_eq!(url3.password(), Some("pass"));
}

#[test]
fn test_url_join() {
    use rmx::url::Url;

    let base = Url::parse("https://example.com/api/v1/").unwrap();
    let joined = base.join("users").unwrap();
    assert_eq!(joined.as_str(), "https://example.com/api/v1/users");

    let joined2 = base.join("../v2/posts").unwrap();
    assert_eq!(joined2.as_str(), "https://example.com/api/v2/posts");

    let absolute = base.join("https://other.com/page").unwrap();
    assert_eq!(absolute.as_str(), "https://other.com/page");
}
