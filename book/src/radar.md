# The Rust Max Radar

This is where we collect crates and tools of interest,
but that are not yet part of Rust Max.


## Crates

- [`backtrace-on-stack-overflow`](https://docs.rs/backtrace-on-stack-overflow).
  Nice missing feature, but code looks underdeveloped.
- [`bitvec`](https://docs.rs/bitvec).
  Operate directly on bits.
- [`boringtun`](https://docs.rs/boringtun).
  WireGuard.
- [`comrak`](https://docs.rs/comrak).
  Markdown parser.
- [`console`](https://docs.rs/console),
  [`dialoguer`](https://docs.rs/dialoguer), and
  [`indicatif`](https://docs.rs/indicatif).
  Pretty terminals.
- [`criterion`](https://docs.rs/criterion).
  Advanced benchmarking.
- [`dashmap`](https://docs.rs/dashmap).
  Concurrent hash map.
- [`datatest-stable`](https://docs.rs/datatest-stable)
  Data-driven tests.
- [`dotenv`](https://docs.rs/dotenv).
- [`derive-new`](https://docs.rs/derive-new)
  Derive the `new` function.
- [`ena`](https://docs.rs/ena).
  The union find algorithm.
- [`encoding`](https://docs.rs/encoding),
  [`charset`](https://docs.rs/charset),
  [`codepage`](https://docs.rs/codepage),
  [`oem_cp`](https://docs.rs/oem_cp),
  [`icu_normalizer`](https://docs.rs/icu_normalizer),
  [`detone`](https://docs.rs/detone).
  Text encoding.
- [`enum-iterator`](https://docs.rs/enum-iterator).
  Iterate over variants.
- [`eyre`](https://docs.rs/eyre).
  Sophisticated error handling.
- [`flate2`](https://docs.rs/flate2).
- [`fnv`](https://docs.rs/fnv) or some other non-ahash fast hash
- [`hashbrown`](https://docs.rs/hashbrown).
  Hash maps with no-std.
- [`home`](https://docs.rs/home)
- [`ignore`](https://docs.rs/ignore).
  Like `walkdir` but obeys `.gitignore`.
- [`include_dir`](https://docs.rs/include_dir).
- [`indexmap`](https://docs.rs/indexmap)
- [`libm`](https://docs.rs/libm).
  Useful for no-std.
- [`memchr`](https://docs.rs/memchr)
- [`memmap`](hthtps://docs.rs/memmap)
- [`ndarray`](https://docs.rs/ndarray)
- [`nix`](https://docs.rs/nix)
- [`notify`](https://docs.rs/notify).
  File system notification.
- [`num`](https://docs.rs/num).
- [`num-traits`](https://docs.rs/num-traits)
- [`ordered-float`](https://docs.rs/ordered-float)
- [`parking_lot`](https://docs.rs/parking_lot)
  Non-poisoning mutexes, etc.
- [`petgraph`](https://docs.rs/petgraph)
- [`ratatui`](https://docs.rs/ratatui).
  Seriously cool CLIs.
- [`rangetools`](https://docs.rs/rangetools)
- [`rustls`](https://docs.rs/rustls). TLS.
- [`rustversion`](https://docs.rs/rustversion)
- [`rust-embed`](https://docs.rs/rust-embed).
  Embedding of file system resources into binaries,
  with hot-reloading during development.
- [`scopeguard`](https://docs.rs/scopeguard).
  Like `defer`.
- [`smallvec`](https://docs.rs/smallvec).
  The "small vector" optimization.
  There may be better / newer options.
- [`sqlx`](https://docs.rs/sqlx).
- [`tar`](https://docs.rs/tar).
- [`tungstenite`](https://docs.rs/tungstenite). WebSockets.
- [`stacker`](https://docs.rs/stacker).
  Manually-growable call stacks.
- [`time`](https://docs.rs/time).
  Another time crate.
- [`tracing`](https://docs.rs/tracing).
- [`tracing-subscriber`](https://docs.rs/tracing-subscriber).
- [`tracing-tracy`](https://docs.rs/tracing-tracy).
- [`unicode-xid`](https://docs.rs/unicode-xid).
- [`xdg`](https://docs.rs/xdg)


## Tools

- [`cargo-duplicates`](https://crates.io/crates/cargo-duplicates)
- [`cargo-hack`](https://crates.io/crates/cargo-hack)
- [`cargo-outdated`](https://crates.io/crates/cargo-outdated)
- [`hyperfine`](https://github.com/sharkdp/hyperfine)
- [`rust-analyzer`](https://rust-analyzer.github.io/)


## Wanted

- SHA3
- gRPC
- wasm crates and tools, wasm-bindgen, stdweb
- threadpool
- zip, gzip
- parser generator (pest?)
- alternative to bitflags
- gui stuff
  - winit, wgpu vs glow, morphorm, css, iced vs egui
- i18n
- QUIC - either quinn or quiche
- HTTP3
- markdown
- csv
- small string, smartstring
- rational numbers
- fixed-point, decimal numbers, rust-decimal


## Replacements

- num_cpu -> std::thread::available_parallelism


## Graveyard

These projects were once useful or notable,
but are now deprecated by others.

- [`lazy_static`](https://docs.rs/lazy_static).
  Use [`std::sync::LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html).
- [`once_cell`](https://docs.rs/once_cell).
  Use [`std::sync::OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html).
