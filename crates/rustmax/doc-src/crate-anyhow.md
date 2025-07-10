Flexible error handling.

- Crate [`::anyhow`].
- [docs.rs](https://docs.rs/anyhow)
- [crates.io](https://crates.io/crates/anyhow)
- [GitHub](https://github.com/dtolnay/anyhow)

---

`anyhow` provides [`Error`],
a trait object based error type for use cases
where you want error handling to be easy.

It is oftain said that Rust
libraries should define specific error types that implement [`std::error::Error`],
so their callers can respond to errors precisely;
but applications can often just propagate errors up the call stack
without caring about specific error types.

For these more casual error handling use cases,
it is sometimes ideomatic in low-dependency Rust code
to return [`Box<dyn std::error::Error>`] as a generic
error type. `anyhow` provides a similar generic
error but with more features:
`anyhow::Error` requires no allocation in the success case,
supports downcasting to the original error type,
and can carry a backtrace, among others.

The [`Result<T>`] type alias saves typing,
and the [`Context`] trait provides the [`.context()`] method
for adding contextual information to errors as they propagate,
the [`anyhow!`] and [`bail!`] macros

For defining precise errors,
use [`thiserror`] instead of `anyhow`.

## Examples

Use [`Result<T>`] as your main error type,
and [`.context()`] to add helpful error messages:

```rust
use anyhow::{Context, Result};

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")
        .context("Failed to read cluster config")?;
    let map: ClusterMap = serde_json::from_str(&config)
        .context("Failed to parse cluster config")?;
    Ok(map)
}

#[derive(serde::Deserialize)]
struct ClusterMap {
    // ..
}
```

[`anyhow!`]: crate::anyhow::anyhow
[`bail!`]: crate::anyhow::bail
[`Error`]: crate::anyhow::Error
[`Result<T>`]: crate::anyhow::Result
[`Context`]: crate::anyhow::Context
[`.context()`]: crate::anyhow::Context::context
[`std::error::Error`]: crate::std::error::Error
[`Box<dyn std::error::Error>`]: crate::std::error::Error
[`thiserror`]: crate::thiserror
