# The Rust Max Radar

This is where we collect crates and tools of interest,
but that are not yet part of Rust Max.


## Crates

- [`backtrace-on-stack-overflow`](https://docs.rs/backtrace-on-stack-overflow).
  Nice missing feature, but code looks underdeveloped.
- [`console`](https://docs.rs/console),
  [`dialoguer`](https://docs.rs/dialoguer), and
  [`indicatif`](https://docs.rs/indicatif).
- [`criterion`](https://docs.rs/criterion).
  Advanced benchmarking.
- [`dashmap`](https://docs.rs/dashmap).
  Concurrent hash map.
- [`datatest-stable`](https://docs.rs/datatest-stable)
- [`derive-new`](https://docs.rs/derive-new)
- [`memchr`](https://docs.rs/memchr)
- [`nix`](https://docs.rs/nix)
- [`notify`](https://docs.rs/notify).
  File system notification.
- [`num`](https://docs.rs/num)
- [`num-traits`](https://docs.rs/num-traits)
- [`rust-embed`](https://docs.rs/rust-embed).
  Embedding of file system resources into binaries,
  with hot-reloading during development.
- [`smallvec`](https://docs.rs/smallvec).
  The "small vector" optimization.
  There may be better / newer options.
- [`stacker`](https://docs.rs/stacker).
  Manually-growable call stacks.
- [`time`](https://docs.rs/time).
  Another time crate.
- [`tracing`](https://docs.rs/tracing).
- [`tracing-subscriber`](https://docs.rs/tracing-subscriber).
- [`tracing-tracy`](https://docs.rs/tracing-tracy).


## Tools

- [`cargo-duplicates`](https://crates.io/crates/cargo-duplicates)
- [`cargo-outdated`](https://crates.io/crates/cargo-outdated)
- [`rust-analyzer`](https://rust-analyzer.github.io/)


## Wanted

- SHA3
- gRPC
- wasm crates and tools


## Graveyard

These projects were once useful or notable,
but are now deprecated by others.

- [`lazy_static`](https://docs.rs/lazy_static).
  Use [`std::sync::LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html).
- [`once_cell`](https://docs.rs/once_cell).
  Use [`std::sync::OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html).
