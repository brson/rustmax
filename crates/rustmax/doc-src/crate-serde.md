Serialization and deserialization framework.

- Crate [`::serde`].
- [docs.rs](https://docs.rs/serde)
- [crates.io](https://crates.io/crates/serde)
- [GitHub](https://github.com/serde-rs/serde)

---

`serde` is a framework for serializing and deserializing Rust data structures
efficiently and generically.

The core of `serde` consists of the [`Serialize`] and [`Deserialize`] traits,
which can be automatically derived for most data structures.
These traits define how your types convert to and from
various data formats like JSON, YAML, MessagePack, and many others
through separate format crates.

Serde operates on a data model that is independent of the
underlying data format. This means you can serialize your data
to JSON with [`serde_json`], YAML with `serde_yaml`,
or any other supported format without changing your data structures.

The design is intentionally modular:
`serde` provides the serialization framework,
while separate crates like [`serde_json`] provide format-specific implementations.

## Examples

Deriving serialization for a struct:

```rust,ignore
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

let person = Person {
    name: "Alice".to_string(),
    age: 30,
    email: Some("alice@example.com".to_string()),
};

// Serialize to JSON (requires serde_json)
let json = serde_json::to_string(&person).unwrap();
println!("{}", json);
// Output: {"name":"Alice","age":30,"email":"alice@example.com"}

// Deserialize from JSON
let parsed: Person = serde_json::from_str(&json).unwrap();
println!("{:?}", parsed);
```

Customizing field names and handling missing fields:

```rust,ignore
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename = "server-port")]
    port: u16,

    #[serde(default = "default_timeout")]
    timeout_ms: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    debug_mode: Option<bool>,
}

fn default_timeout() -> u64 {
    5000
}

let json = r#"{"server-port": 8080}"#;
let config: Config = serde_json::from_str(json).unwrap();
assert_eq!(config.port, 8080);
assert_eq!(config.timeout_ms, 5000);
```

[`Serialize`]: crate::serde::Serialize
[`Deserialize`]: crate::serde::Deserialize
[`serde_json`]: crate::serde_json