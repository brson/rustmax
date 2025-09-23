A lightweight logging facade for Rust.

- Crate [`::log`].
- [docs.rs](https://docs.rs/log)
- [crates.io](https://crates.io/crates/log)
- [GitHub](https://github.com/rust-lang/log)

---

`log` provides a single logging API that abstracts over the actual logging implementation.
Libraries can use the logging API provided by this crate,
and the executable can choose the logging implementation
that is most suitable for its use case.

The `log` crate provides macros for logging at various levels:
[`error!`], [`warn!`], [`info!`], [`debug!`], and [`trace!`],
where `error!` represents the highest priority and `trace!` the lowest.

The logging facade itself doesn't perform any logging;
it needs to be paired with a logging implementation like [`env_logger`],
[`simplelog`], [`fern`], or [`tracing-subscriber`].
The implementation is responsible for filtering log messages,
formatting them, and outputting them to the appropriate destination.

## Examples

```
use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::init();

    process_data(42);
}

fn process_data(value: i32) {
    trace!("Starting data processing");
    debug!("Processing value: {}", value);

    if value < 0 {
        error!("Invalid value: {}", value);
        return;
    }

    if value == 0 {
        warn!("Processing zero value");
    }

    info!("Successfully processed value: {}", value);
}
```

[`error!`]: crate::log::error
[`warn!`]: crate::log::warn
[`info!`]: crate::log::info
[`debug!`]: crate::log::debug
[`trace!`]: crate::log::trace
[`env_logger`]: crate::env_logger
[`simplelog`]: crate::simplelog
[`fern`]: crate::fern
[`tracing-subscriber`]: crate::tracing-subscriber