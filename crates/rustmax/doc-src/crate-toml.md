TOML parsing and serialization.

- Crate [`::toml`].
- [docs.rs](https://docs.rs/toml)
- [crates.io](https://crates.io/crates/toml)
- [GitHub](https://github.com/toml-lang/toml-rs)

---

`toml` provides parsing and serialization support for the TOML format.
TOML is a configuration file format that's designed to be easy to read and write,
making it popular for configuration files in the Rust ecosystem, including `Cargo.toml`.

## Examples

Parsing TOML from a string:

```
let toml_str = "name = \"example\"\nversion = 1";
let parsed: toml::Value = toml::from_str(toml_str).unwrap();
assert_eq!(parsed["name"].as_str(), Some("example"));
assert_eq!(parsed["version"].as_integer(), Some(1));
```

Deserializing to a struct with serde:

```
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    title: String,
    database: Database,
}

#[derive(Deserialize)]
struct Database {
    host: String,
    port: u16,
}

let toml_str = r#"title = "Example"

[database]
host = "localhost"
port = 5432
"#;

let config: Config = toml::from_str(toml_str).unwrap();
assert_eq!(config.title, "Example");
assert_eq!(config.database.host, "localhost");
```

Serializing a struct to TOML:

```
use serde::Serialize;

#[derive(Serialize)]
struct Config {
    title: String,
    debug: bool,
}

let config = Config {
    title: "My App".to_string(),
    debug: true,
};

let toml_string = toml::to_string(&config).unwrap();
assert!(toml_string.contains("title = \"My App\""));
assert!(toml_string.contains("debug = true"));
```

[`Value`]: crate::toml::Value
[`from_str`]: crate::toml::from_str
[`to_string`]: crate::toml::to_string