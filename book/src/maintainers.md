# Trusted Maintainers

The Rust ecosystem is built by thousands of contributors,
but certain individuals and organizations have established
exceptional reputations for creating and maintaining
high-quality, widely-trusted libraries.

For new Rustaceans, understanding who maintains the crates
you depend on can help you make informed decisions
about which libraries to use in your projects.

---

## <a id="dtolnay"></a> David Tolnay

**GitHub:** [dtolnay](https://github.com/dtolnay)

Prolific Rust library author and maintainer.
Core contributor to the Rust compiler and tooling.

**Notable crates:**
- [`serde`](https://docs.rs/serde) — The standard serialization framework
- [`syn`](https://docs.rs/syn) — Parser for Rust source code, essential for proc macros
- [`quote`](https://docs.rs/quote) — Code generation for proc macros
- [`thiserror`](https://docs.rs/thiserror) — Derive macro for error types
- [`anyhow`](https://docs.rs/anyhow) — Flexible error handling for applications
- [`proc-macro2`](https://docs.rs/proc-macro2) — Wrapper around proc_macro for better ergonomics

David's crates are characterized by exceptional documentation,
thoughtful API design, and comprehensive testing.

---

## <a id="burntsushi"></a> Andrew Gallant (BurntSushi)

**GitHub:** [BurntSushi](https://github.com/BurntSushi)

Creator of ripgrep and author of numerous widely-used libraries.

**Notable crates:**
- [`regex`](https://docs.rs/regex) — Fast, Unicode-aware regular expressions
- [`walkdir`](https://docs.rs/walkdir) — Recursive directory traversal
- [`byteorder`](https://docs.rs/byteorder) — Reading/writing numbers in big-endian and little-endian
- [`jiff`](https://docs.rs/jiff) — Modern date and time library
- [`csv`](https://docs.rs/csv) — Fast CSV parser

Andrew's libraries are known for their performance,
correctness, and extensive documentation.
His blog posts often provide deep insights into Rust performance optimization.

---

## <a id="alexcrichton"></a> Alex Crichton

**GitHub:** [alexcrichton](https://github.com/alexcrichton)

Former Rust core team member and prolific contributor to foundational crates.

**Notable crates:**
- [`toml`](https://docs.rs/toml) — TOML parser
- [`wasm-bindgen`](https://docs.rs/wasm-bindgen) — Facilitating WebAssembly and JavaScript interop
- Many foundational async and FFI crates in the ecosystem

Alex's contributions span compiler internals, tooling,
and critical ecosystem infrastructure.

---

## <a id="seanmonstar"></a> Sean McArthur

**GitHub:** [seanmonstar](https://github.com/seanmonstar)

Creator and maintainer of core HTTP ecosystem libraries.
Member of the Tokio team.

**Notable crates:**
- [`hyper`](https://docs.rs/hyper) — Low-level HTTP library
- [`reqwest`](https://docs.rs/reqwest) — High-level HTTP client
- [`tower`](https://docs.rs/tower) — Service abstractions for building robust clients and servers

Sean's HTTP libraries form the foundation of most HTTP
and web applications in Rust.

---

## <a id="bluss"></a> Ulrik Sverdrup (bluss)

**GitHub:** [bluss](https://github.com/bluss)

Author and maintainer of itertools and other iterator-focused utility crates.

**Notable crates:**
- [`itertools`](https://docs.rs/itertools) — Extra iterator adaptors and methods

---

## <a id="servo"></a> Servo Project

**GitHub:** [servo](https://github.com/servo)

The Servo browser engine project, maintaining web-platform crates.

**Notable crates:**
- [`url`](https://docs.rs/url) — URL parser
- Many other web platform implementation crates

---

## <a id="matklad"></a> Aleksey Kladov

**GitHub:** [matklad](https://github.com/matklad)

Creator of rust-analyzer. Former Rust core team member.

**Notable crates:**
- [`xshell`](https://docs.rs/xshell) — Ergonomic shell scripting in Rust

---

## <a id="rust-lang"></a> Rust Language Team

**GitHub:** [rust-lang](https://github.com/rust-lang)

The official Rust project organization.

Maintains the Rust compiler, standard library, and core tooling:
- `rustc` — The Rust compiler
- [`cargo`](https://docs.rs/cargo) — Rust's package manager and build system
- `rustup` — Rust toolchain installer
- `rustfmt` — Code formatter
- `clippy` — Linting tool
- [`futures`](https://docs.rs/futures) — Fundamental async primitives

All official Rust tooling is developed under this organization.

---

## <a id="tokio-rs"></a> Tokio Project

**GitHub:** [tokio-rs](https://github.com/tokio-rs)

Organization maintaining the async runtime ecosystem.

**Notable crates:**
- [`tokio`](https://docs.rs/tokio) — Async runtime for writing reliable network applications
- [`axum`](https://docs.rs/axum) — Web application framework
- [`tower`](https://docs.rs/tower) — Library of modular and reusable components for networking
- [`bytes`](https://docs.rs/bytes) — Utilities for working with bytes

The Tokio project provides the foundation for async programming
in Rust, including runtime, I/O, and HTTP abstractions.

---

## <a id="rustcrypto"></a> RustCrypto

**GitHub:** [RustCrypto](https://github.com/RustCrypto)

Organization maintaining pure Rust implementations of cryptographic
algorithms and protocols.

**Notable crates:**
- [`sha2`](https://docs.rs/sha2) — SHA-2 hash functions
- Many other cryptographic primitives following uniform APIs

RustCrypto crates are characterized by rigorous security practices,
constant-time implementations where appropriate,
and comprehensive cryptographic algorithm coverage.

---

## <a id="rustsec"></a> RustSec

**GitHub:** [rustsec](https://github.com/rustsec)

Organization maintaining the Rust security advisory database
and related security tooling.

**Notable projects:**
- [`cargo-audit`](https://docs.rs/cargo-audit) — Audits Cargo.lock for crates with security vulnerabilities
- Rust Security Advisory Database

RustSec provides essential security infrastructure for the Rust ecosystem.

---

## Recognition Criteria

Trusted maintainers are recognized based on:

- **Quality** — Crates are well-designed, performant, and reliable
- **Documentation** — Comprehensive docs and examples
- **Maintenance** — Active development and responsive to issues
- **Community standing** — Positive reputation in the Rust community
- **Impact** — Widespread use and foundational importance

This is not an exhaustive list.
Many other excellent maintainers contribute to the Rust ecosystem.
