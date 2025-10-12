A simple logger that can be configured via environment variables.

- Crate [`::env_logger`].
- [docs.rs](https://docs.rs/env_logger)
- [crates.io](https://crates.io/crates/env_logger)
- [GitHub](https://github.com/rust-cli/env_logger)

---

`env_logger` is a logging implementation for the [`log`] facade
that filters log messages based on environment variables.
It's the most commonly used logging backend in the Rust ecosystem,
particularly for command-line applications.

By default, `env_logger` reads the `RUST_LOG` environment variable
to determine which log messages to display.
The format is `target=level`, where target can be a module path
and level is one of `error`, `warn`, `info`, `debug`, or `trace`.

## Examples

Initialize the logger at the start of your program:

```
use log::info;

fn main() {
    env_logger::init();

    info!("Starting application");
}
```

[`log`]: crate::log
