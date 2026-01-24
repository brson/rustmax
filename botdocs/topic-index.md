# Topic Index Brainstorming

A curated list of topics for search discovery across www, books, and API docs.

The goal: when someone searches for a concept, they find relevant results from:
- The Rustmax book
- The library (official Rust books)
- API documentation
- Crate examples


## Design considerations

**What makes a good topic?**
- Something people actually search for
- Has multiple relevant resources across sources
- Canonical name with aliases (e.g. "async" vs "asynchronous")
- Connects abstract concepts to concrete crates/APIs

**Index structure per topic:**
- Canonical name
- Aliases / alternate spellings
- Category
- Related crates
- Related book chapters
- Related std/API modules
- Brief description (for search snippets)


## Topic categories

### 1. Language features

Core Rust language concepts that appear in the reference and TRPL.

- Ownership and borrowing
- Lifetimes (`'a`, `'static`, lifetime elision)
- Traits (trait bounds, trait objects, `dyn`, impl Trait)
- Generics (type parameters, const generics)
- Pattern matching (`match`, `if let`, `let else`, `@` bindings)
- Enums and algebraic data types
- Structs (named, tuple, unit)
- Error handling (`Result`, `Option`, `?` operator, `panic!`)
- Closures (`Fn`, `FnMut`, `FnOnce`, move closures)
- Iterators (`Iterator`, `IntoIterator`, iterator adapters)
- Smart pointers (`Box`, `Rc`, `Arc`, `RefCell`, `Cell`)
- Interior mutability
- References and dereferencing
- Slices (`[T]`, `&[T]`, `&str`)
- Arrays and tuples
- Type aliases
- Newtype pattern
- Associated types
- GATs (generic associated types)
- Higher-ranked trait bounds (HRTB, `for<'a>`)
- Coercion and `Deref`
- Unsafe Rust (`unsafe`, raw pointers, `transmute`)
- FFI and `extern`
- Modules and visibility (`mod`, `pub`, `use`)
- Crates and packages
- Attributes (`#[derive]`, `#[cfg]`, `#[allow]`, custom)
- Conditional compilation (`cfg`, `cfg_attr`, feature flags)
- Macros (declarative `macro_rules!`, procedural)
- Derive macros
- Async/await (`async`, `.await`, `Future`)
- Pinning (`Pin`, `Unpin`)
- Strings (`String`, `&str`, `OsString`, `CString`)
- Paths (`Path`, `PathBuf`)
- Collections (`Vec`, `HashMap`, `BTreeMap`, `HashSet`)
- Type inference
- Turbofish (`::<>`)
- Never type (`!`)
- Unit type (`()`)
- `Copy` vs `Clone`
- `Send` and `Sync`
- `Drop` and destructors
- `Default`
- `From` and `Into` conversions
- `TryFrom` and `TryInto`
- `AsRef` and `AsMut`
- `Borrow` and `ToOwned`
- Operator overloading (`Add`, `Deref`, `Index`, etc.)
- `Display` and `Debug` formatting
- Comparison traits (`Eq`, `Ord`, `PartialEq`, `PartialOrd`)
- `Hash`
- Zero-sized types (ZST)
- Phantom types (`PhantomData`)
- Marker traits
- Object safety
- Orphan rules
- Coherence
- Blanket implementations
- Method resolution
- Auto traits
- `?Sized`


### 2. Standard library modules

Key std modules that deserve dedicated topic entries.

- `std::collections` (HashMap, BTreeMap, VecDeque, etc.)
- `std::io` (Read, Write, BufReader, stdin/stdout)
- `std::fs` (File, read/write, metadata, permissions)
- `std::net` (TcpStream, TcpListener, UdpSocket)
- `std::sync` (Mutex, RwLock, Arc, atomic types)
- `std::sync::mpsc` (channels)
- `std::thread` (spawn, JoinHandle, thread_local)
- `std::process` (Command, Child, exit)
- `std::env` (args, vars, current_dir)
- `std::path` (Path, PathBuf)
- `std::ffi` (CString, OsString, extern types)
- `std::mem` (size_of, align_of, swap, take, replace)
- `std::ptr` (null, read, write, copy)
- `std::cell` (Cell, RefCell, UnsafeCell)
- `std::rc` and `std::sync::Arc`
- `std::marker` (PhantomData, Send, Sync)
- `std::ops` (operator traits)
- `std::cmp` (comparison traits)
- `std::hash` (Hash, Hasher, BuildHasher)
- `std::fmt` (Display, Debug, formatters)
- `std::error` (Error trait)
- `std::panic` (catch_unwind, set_hook)
- `std::backtrace`
- `std::any` (Any, TypeId)
- `std::time` (Duration, Instant, SystemTime)
- `std::future` (Future, poll)
- `std::task` (Context, Waker, Poll)
- `std::pin` (Pin, Unpin)
- `std::alloc` (allocators, GlobalAlloc)
- `std::simd` (portable SIMD)


### 3. Programming domains / tasks

What people are trying to accomplish.

**Error handling:**
- Error types, error propagation, Result, anyhow, thiserror
- Backtraces, panic handling, error context

**Serialization:**
- JSON, TOML, YAML, MessagePack, bincode
- serde, Serialize, Deserialize
- Custom serialization

**Parsing:**
- Parser combinators (nom, winnow, chumsky)
- Regex
- Lexing and tokenization
- AST design

**Encoding:**
- Base64, hex, percent-encoding
- Compression (gzip, zlib, deflate, zstd, brotli)
- Character encodings (UTF-8, UTF-16, ASCII)

**Time and dates:**
- Timestamps, duration, intervals
- Time zones, UTC, local time
- Parsing and formatting dates
- chrono vs jiff
- ISO 8601, RFC 3339

**Random numbers:**
- CSPRNG, PRNG
- Seeding, reproducibility
- Distributions
- rand, rand_chacha, rand_pcg

**Cryptography:**
- Hashing (SHA-2, BLAKE3, MD5)
- Encryption (AES, ChaCha20)
- Digital signatures
- TLS/SSL
- Password hashing (argon2, bcrypt)
- RustCrypto ecosystem

**Networking:**
- TCP, UDP sockets
- HTTP client/server
- WebSockets
- DNS resolution
- TLS
- HTTP/2, HTTP/3
- REST APIs, GraphQL

**Web development:**
- Web frameworks (axum, actix-web, warp, rocket)
- Middleware, routing, extractors
- Templates (tera, askama, minijinja)
- Static files, CORS
- Sessions, cookies, authentication

**Async I/O:**
- Async runtimes (tokio, async-std, smol)
- Futures, streams, sinks
- Async channels
- Timeouts, cancellation
- Select, join
- Async traits
- Pinning in async

**Concurrency:**
- Threads, thread pools
- Mutexes, RwLocks
- Atomics, memory ordering
- Channels (mpsc, crossbeam)
- rayon, parallel iterators
- Data parallelism

**CLI applications:**
- Argument parsing (clap, argh)
- Terminal colors (termcolor, colored, owo-colors)
- Progress bars (indicatif)
- Interactive prompts (dialoguer)
- Line editing (rustyline)
- Ctrl-C handling

**File system:**
- Reading/writing files
- Directory traversal (walkdir, ignore)
- File watching (notify)
- Temporary files (tempfile)
- Glob patterns
- Symlinks, permissions

**Logging and tracing:**
- log facade
- env_logger, tracing, log4rs
- Structured logging
- Log levels
- Spans and instrumentation

**Testing:**
- Unit tests, integration tests
- Test fixtures
- Mocking
- Property-based testing (proptest, quickcheck)
- Fuzzing (cargo-fuzz, afl)
- Benchmarking (criterion, divan)
- Test coverage

**Build and compilation:**
- Cargo, Cargo.toml
- Build scripts (build.rs)
- Feature flags
- Workspaces
- Cross-compilation
- Linking (static, dynamic)
- LTO, PGO optimization

**FFI:**
- C interop (libc, bindgen)
- C++ interop (cxx)
- Python interop (PyO3)
- WASM (wasm-bindgen, wasm-pack)
- Calling conventions
- Memory layout, repr(C)

**Embedded / no_std:**
- no_std, alloc
- Embedded HAL
- Bare metal
- RTOS
- Cortex-M, RISC-V

**Procedural macros:**
- syn, quote, proc-macro2
- Derive macros
- Attribute macros
- Function-like macros
- Token manipulation


### 4. Crates (rustmax curated)

Each rustmax crate should be a searchable topic.

**Core:**
anyhow, thiserror, log, env_logger

**Collections:**
ahash, bitflags, bytes, itertools

**Serialization:**
serde, serde_json, toml, json5

**Encoding:**
base64, hex, flate2, zip

**Parsing:**
regex, nom, memchr, comrak

**Time:**
chrono, jiff

**Random:**
rand, rand_chacha, rand_pcg

**Crypto:**
blake3, sha2

**Async:**
tokio, futures, crossbeam

**Parallel:**
rayon

**Networking:**
reqwest, hyper, axum, http, url, mime, tower, socket2

**Web:**
tera

**CLI:**
clap, termcolor, indicatif, rustyline, ctrlc

**Filesystem:**
walkdir, ignore, notify, tempfile, glob

**System:**
libc, xshell

**FFI:**
bindgen, cc, cxx, cxx-build

**Macros:**
proc-macro2, syn, quote, derive_more, extension-trait, cfg-if

**Testing:**
proptest

**Numerics:**
num_bigint, num_enum, semver

**Unicode:**
unicode-segmentation

**Image/media:**
image, cpal, rodio


### 5. Tools

Rust toolchain and ecosystem tools.

**Standard:**
- rustc (compiler)
- cargo (package manager)
- rustup (toolchain manager)
- rustfmt (formatter)
- clippy (linter)
- rustdoc (documentation)
- miri (undefined behavior detector)

**Cargo plugins:**
- cargo-edit (add/rm/upgrade)
- cargo-deny (license/security auditing)
- cargo-audit (security vulnerabilities)
- cargo-generate (project templates)
- cargo-expand (macro expansion)
- cargo-asm (assembly output)
- cargo-bloat (binary size)
- cargo-udeps (unused dependencies)
- cargo-machete (unused dependencies)
- cargo-semver-checks (API compatibility)
- cargo-fuzz (fuzzing)

**General tools written in Rust:**
- ripgrep (search)
- fd (find)
- sd (sed replacement)
- just (command runner)
- tokei (code statistics)
- hyperfine (benchmarking)
- bat (cat with syntax highlighting)
- exa/eza (ls replacement)
- delta (diff viewer)
- dust (disk usage)
- bottom/btm (system monitor)
- starship (shell prompt)


### 6. Books and references

**Official:**
- The Rust Programming Language (TRPL)
- Rust By Example
- The Rust Reference
- The Rustonomicon
- Rust Edition Guide
- The Cargo Book
- rustdoc Book
- rustc Book
- Embedded Rust Book
- Rust API Guidelines
- Unsafe Code Guidelines

**Community:**
- The Little Book of Rust Macros
- Rust Cookbook
- Async Book
- Too Many Linked Lists


### 7. Concepts and idioms

Rust-specific patterns and concepts.

- RAII (Resource Acquisition Is Initialization)
- Newtype pattern
- Builder pattern
- Typestate pattern
- Extension traits
- Sealed traits
- Blanket implementations
- Deref polymorphism
- Interior mutability pattern
- Cow (clone-on-write)
- Entry API (HashMap)
- Iterator chains
- Error handling strategies (fail fast, recover, propagate)
- The `?` operator idioms
- `impl Trait` vs `dyn Trait`
- Zero-cost abstractions
- Fearless concurrency
- Memory safety without GC
- Borrow checker patterns
- Fighting the borrow checker
- NLL (non-lexical lifetimes)
- Two-phase borrows
- Polonius
- MIR
- Monomorphization
- Trait specialization
- Orphan rules workarounds
- Tower service pattern
- Actor model
- Message passing vs shared state


### 8. Error messages and diagnostics

Common compiler errors that deserve dedicated help pages.

- E0382: borrow of moved value
- E0502: cannot borrow as mutable because also borrowed as immutable
- E0597: does not live long enough
- E0277: trait bound not satisfied
- E0308: mismatched types
- E0499: cannot borrow as mutable more than once
- E0515: cannot return reference to local variable
- E0716: temporary value dropped while borrowed
- "cannot move out of borrowed content"
- "borrowed value does not live long enough"
- "lifetime may not live long enough"
- "closure may outlive the current function"
- "expected `()`, found `T`"
- "`impl Trait` not allowed outside of function and method return types"


### 9. Cultural / ecosystem topics

Rust community concepts and history.

**People/organizations:**
- Rust Project
- Rust Foundation
- Mozilla
- AWS Rust team
- Google Rust team
- Microsoft Rust team
- dtolnay, BurntSushi, alexcrichton, etc. (trusted maintainers)

**Events:**
- RustConf
- Rust Nation
- EuroRust
- Oxidize
- RustFest

**Governance:**
- Teams (compiler, lang, libs, etc.)
- RFCs
- MCPs (Major Change Proposals)
- FCPs (Final Comment Period)

**Editions:**
- Rust 2015, 2018, 2021, 2024
- Edition migration

**Historical:**
- Pre-1.0 Rust
- Stability guarantee
- Crater
- libcpocalypse
- Turbofish
- Crab mascot (Ferris)


### 10. Platform-specific

- Linux
- macOS
- Windows
- WebAssembly / WASM
- iOS
- Android
- FreeBSD
- Embedded targets
- Cross-compilation


### 11. Unicode and text

- UTF-8, UTF-16, UTF-32
- Code points vs grapheme clusters
- Normalization (NFC, NFD, NFKC, NFKD)
- Case folding
- Collation
- Bidirectional text
- WTF-8


## Implementation notes

**Search relevance:**
- Exact match on topic name: highest weight
- Alias match: high weight
- Related crate/module: medium weight
- Mentioned in description: lower weight

**Topic aliases to consider:**
- "hashmap" -> HashMap, std::collections::HashMap, ahash
- "json" -> serde_json, JSON, serialization
- "regex" -> regex crate, regular expressions, pattern matching
- "http" -> reqwest, hyper, axum, web
- "thread" -> std::thread, concurrency, parallelism
- "mutex" -> std::sync::Mutex, synchronization, locking

**Cross-referencing:**
- Topic pages should link to each other
- "See also" sections
- "Related topics" in search results


## Open questions

1. How deep should the topic hierarchy go? (e.g., "async" -> "async traits" -> "async methods in traits")

2. Should individual struct/trait names be topics? (e.g., is `Iterator` a topic separate from "iterators"?)

important traits should be topics

3. How to handle versioned features? (e.g., "let else" is 1.65+)

anything stable is fine, don't worry about versions

4. Should we have "negative" topics for anti-patterns? (e.g., "don't use Rc<RefCell<T>> for this")

5. How to weight freshness? (Some topics are evergreen, some are version-specific)
