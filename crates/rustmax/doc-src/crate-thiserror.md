Tools for defining custom error types.

- Crate [`::thiserror`].
- [docs.rs](https://docs.rs/thiserror)
- [crates.io](https://crates.io/crates/thiserror)
- [GitHub](https://github.com/dtolnay/thiserror)

---

`thiserror` provides derive macros for creating custom error types
that implement [`std::error::Error`] with minimal boilerplate.

While [`anyhow`] is ideal for applications that need flexible error handling,
`thiserror` is designed for libraries that want to provide
specific, well-typed errors that callers can match on and handle precisely.

The [`Error`] derive macro automatically implements [`std::error::Error`],
[`Display`], and optionally [`From`] conversions.
It supports error source chaining, custom display messages,
and transparent delegation to underlying errors.

Key features include `#[error("...")]` for custom display messages,
`#[source]` for error source chaining,
`#[from]` for automatic [`From`] implementations.


## Examples

Define a custom error type with automatic [`std::error::Error`] implementation:

```rust
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("failed to parse JSON")]
    Parse(#[from] serde_json::Error),
    #[error("data store disconnected")]
    Disconnect(#[source] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
}

fn example() -> Result<(), MyError> {
    Err(MyError::Redaction("secret_key".to_string()))
}
```

[`Error`]: crate::thiserror::Error
[`std::error::Error`]: crate::std::error::Error
[`Display`]: crate::std::fmt::Display
[`From`]: crate::std::convert::From
[`anyhow`]: crate::anyhow