JSON5 parsing and serialization.

- Crate [`::json5`].
- [docs.rs](https://docs.rs/json5)
- [crates.io](https://crates.io/crates/json5)
- [GitHub](https://github.com/callum-oakley/json5-rs)

---

`json5` provides parsing and serialization for the [JSON5](https://json5.org/) format
using the [`serde`] framework.
JSON5 is a superset of JSON that allows comments,
trailing commas, unquoted keys, single-quoted strings,
and other conveniences that make it friendlier for hand-edited config files.

The main functions are [`from_str`] for deserialization
and [`to_string`] for serialization.

## Examples

Deserializing a JSON5 config with comments and trailing commas:

```
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct Config {
    name: String,
    port: u16,
    debug: bool,
}

let json5_str = "{
    // server name
    name: 'my-app',
    port: 8080,
    debug: true,
}";

let config: Config = json5::from_str(json5_str).unwrap();
assert_eq!(config.name, "my-app");
assert_eq!(config.port, 8080);
assert!(config.debug);
```

Serializing a struct to JSON5:

```
use serde::Serialize;

#[derive(Serialize)]
struct Settings {
    title: String,
    max_retries: u32,
}

let settings = Settings {
    title: "Example".to_string(),
    max_retries: 3,
};

let output = json5::to_string(&settings).unwrap();
assert!(output.contains("Example"));
assert!(output.contains("3"));
```

[`serde`]: crate::serde
[`from_str`]: crate::json5::from_str
[`to_string`]: crate::json5::to_string
