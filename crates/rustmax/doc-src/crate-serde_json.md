JSON serialization and deserialization.

- Crate [`::serde_json`].
- [docs.rs](https://docs.rs/serde_json)
- [crates.io](https://crates.io/crates/serde_json)
- [GitHub](https://github.com/serde-rs/json)

---

`serde_json` provides JSON serialization and deserialization
for Rust data structures using the [`serde`] framework.
It supports converting between Rust types and JSON text,
with both strongly-typed and loosely-typed approaches.

The main functions are [`to_string`] and [`from_str`]
for basic JSON serialization and deserialization.
For more control, use [`to_writer`] and [`from_reader`]
to work with I/O streams,
or [`to_value`] and [`from_value`] to work with
the generic [`Value`] type that can represent any JSON data.

The [`Value`] enum can hold any JSON value
and is useful for dynamic JSON manipulation
when you don't know the structure at compile time.

## Examples

Serializing and deserializing structured data:

```
use serde::{Deserialize, Serialize};
use serde_json::{to_string, from_str};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let person = Person {
    name: "Alice".to_string(),
    age: 30,
};

// Serialize to JSON string
let json = to_string(&person).unwrap();
println!("JSON: {}", json); // {"name":"Alice","age":30}

// Deserialize back from JSON
let parsed: Person = from_str(&json).unwrap();
assert_eq!(person, parsed);
```

Working with dynamic JSON using [`Value`]:

```
use serde_json::{Value, json};

// Create JSON using the json! macro
let data = json!({
    "name": "Bob",
    "hobbies": ["reading", "coding"],
    "active": true
});

// Access values dynamically
if let Some(name) = data["name"].as_str() {
    println!("Name: {}", name);
}

if let Some(hobbies) = data["hobbies"].as_array() {
    println!("Has {} hobbies", hobbies.len());
}
```

[`serde`]: crate::serde
[`to_string`]: crate::serde_json::to_string
[`from_str`]: crate::serde_json::from_str
[`to_writer`]: crate::serde_json::to_writer
[`from_reader`]: crate::serde_json::from_reader
[`to_value`]: crate::serde_json::to_value
[`from_value`]: crate::serde_json::from_value
[`Value`]: crate::serde_json::Value