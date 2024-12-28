# The Rust Max Radar

This is where we collect crates and tools of interest,
but that are not yet part of Rust Max.


## Crates

- [`backtrace-on-stack-overflow`](https://docs.rs/backtrace-on-stack-overflow).
  Nice missing feature, but code looks underdeveloped.
- [`bitvec`](https://docs.rs/bitvec).
  Operate directly on bits.
- [`console`](https://docs.rs/console),
  [`dialoguer`](https://docs.rs/dialoguer), and
  [`indicatif`](https://docs.rs/indicatif).
- [`criterion`](https://docs.rs/criterion).
  Advanced benchmarking.
- [`dashmap`](https://docs.rs/dashmap).
  Concurrent hash map.
- [`datatest-stable`](https://docs.rs/datatest-stable)
- [`derive-new`](https://docs.rs/derive-new)
- [`ena`](https://docs.rs/ena).
  The union find algorithm.
- [`encoding`](https://docs.rs/encoding),
  [`charset`](https://docs.rs/charset),
  [`codepage`](https://docs.rs/codepage),
  [`oem_cp`](https://docs.rs/oem_cp),
  [`icu_normalizer`](https://docs.rs/icu_normalizer),
  [`detone`](https://docs.rs/detone).
- [`flate2`](https://docs.rs/flate2).
- [`fnv`](https://docs.rs/fnv) or some other non-ahash fast hash
- [`home`](https://docs.rs/home)
- [`memchr`](https://docs.rs/memchr)
- [`memmap`](hthtps://docs.rs/memmap)
- [`ndarray`](https://docs.rs/ndarray)
- [`nix`](https://docs.rs/nix)
- [`notify`](https://docs.rs/notify).
  File system notification.
- [`num`](https://docs.rs/num).
- [`num-traits`](https://docs.rs/num-traits)
- [`ordered-float`](https://docs.rs/ordered-float)
- [`petgraph`](https://docs.rs/petgraph)
- [`rustversion`](https://docs.rs/rustversion)
- [`rust-embed`](https://docs.rs/rust-embed).
  Embedding of file system resources into binaries,
  with hot-reloading during development.
- [`semver`](https://docs.rs/semver).
- [`smallvec`](https://docs.rs/smallvec).
  The "small vector" optimization.
  There may be better / newer options.
- [`tar`](https://docs.rs/tar).
- [`stacker`](https://docs.rs/stacker).
  Manually-growable call stacks.
- [`time`](https://docs.rs/time).
  Another time crate.
- [`tracing`](https://docs.rs/tracing).
- [`tracing-subscriber`](https://docs.rs/tracing-subscriber).
- [`tracing-tracy`](https://docs.rs/tracing-tracy).
- [`xdg`](https://docs.rs/xdg)


## Tools

- [`cargo-duplicates`](https://crates.io/crates/cargo-duplicates)
- [`cargo-outdated`](https://crates.io/crates/cargo-outdated)
- [`rust-analyzer`](https://rust-analyzer.github.io/)


## Wanted

- SHA3
- gRPC
- wasm crates and tools
- threadpool
- zip
- parser generator (pest?)


## Graveyard

These projects were once useful or notable,
but are now deprecated by others.

- [`lazy_static`](https://docs.rs/lazy_static).
  Use [`std::sync::LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html).
- [`once_cell`](https://docs.rs/once_cell).
  Use [`std::sync::OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html).
