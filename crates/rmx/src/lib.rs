#![doc = include_str!("../doc-src/root-docs.md")]

/* ---------- */


#![no_std]


/* ---------- */


pub mod prelude {
    //! The `rmx` prelude.

    #[cfg(feature = "anyhow")]
    pub use ::anyhow::{
        Result as AnyResult,
        Error as AnyError,
        Context as _,
        anyhow, bail,
    };

    #[cfg(feature = "cfg-if")]
    pub use ::cfg_if::cfg_if;

    #[cfg(feature = "extension-trait")]
    pub use ::extension_trait::extension_trait;

    #[cfg(feature = "log")]
    pub use ::log::{error, warn, info, debug, trace};

    pub use crate::extras::{
        default,
    };

    #[cfg(feature = "rmx-rustlib-alloc")]
    pub use crate::extras::S;

    pub use crate::extras::OptionExpect as _;
    pub use crate::extras::ResultExpect as _;
}

pub mod extras {
    //! Additional tidbits defined by `rmx`.

    pub fn default<T: Default>() -> T {
        Default::default()
    }

    pub fn init() {
        #[cfg(feature = "env_logger")]
        fn maybe_init_env_logger() {
            crate::env_logger::Builder::new()
                .filter_level(log::LevelFilter::Info)
                .parse_default_env()
                .init();
        }
        #[cfg(not(feature = "env_logger"))]
        fn maybe_init_env_logger() { }

        maybe_init_env_logger();
    }

    #[cfg(feature = "rmx-rustlib-alloc")]
    #[allow(non_snake_case)]
    pub fn S(s: &'static str) -> crate::alloc::string::String {
        core::convert::From::from(s)
    }

    #[extension_trait::extension_trait]
    pub impl<T> OptionExpect<T> for Option<T> {
        #[track_caller]
        #[allow(non_snake_case)]
        fn X(self) -> T {
            match self {
                Some(v) => v,
                None => panic!("impossible `None` option"),
            }
        }
    }

    #[cfg(feature = "rmx-rustlib-std")]
    #[extension_trait::extension_trait]
    pub impl<T, E> ResultExpect<T, E> for Result<T, E>
    where E: std::error::Error
    {
        #[track_caller]
        #[allow(non_snake_case)]
        fn X(self) -> T {
            match self {
                Ok(v) => v,
                Err(e) => panic!("impossible `Err` result: {e}"),
            }
        }
    }
}


/* ---------- */


#[cfg(feature = "rmx-rustlib-core")]
pub extern crate core;

#[cfg(feature = "rmx-rustlib-alloc")]
pub extern crate alloc;

#[cfg(feature = "rmx-rustlib-std")]
pub extern crate std;

#[cfg(feature = "rmx-rustlib-proc_macro")]
pub extern crate proc_macro;


/* ---------- */


#[cfg(feature = "ahash")]
pub mod ahash {
    #![doc = include_str!("../doc-src/crate-ahash.md")]

    pub use ::ahash::*;
}


#[cfg(feature = "anyhow")]
pub mod anyhow {
    //! Easy error handling.
    //!
    //! See crate [`::anyhow`].

    pub use ::anyhow::*;
}

#[cfg(feature = "axum")]
pub mod axum {
    //! Web application framework based on [`tokio`](super::tokio).
    //!
    //! See crate [`::axum`].

    pub use ::axum::*;
}

#[cfg(feature = "backtrace")]
pub mod backtrace {
    //! Callstack backtraces on demand.
    //!
    //! See crate [`::backtrace`].

    pub use ::backtrace::*;
}

#[cfg(feature = "base64")]
pub mod base64 {
    //! Base-64 encoding and decoding.
    //!
    //! See crate [`::base64`].

    pub use ::base64::*;
}

#[cfg(feature = "bindgen")]
pub mod bindgen {
    //! Generate Rust bindings to C and C++ libraries.
    //!
    //! See crate [`::bindgen`].

    pub use ::bindgen::*;
}

#[cfg(feature = "bitflags")]
pub mod bitflags {
    //! Types in which the bits are individually addressable.
    //!
    //! See crate [`::bitflags`].

    pub use ::bitflags::*;
}

#[cfg(feature = "blake3")]
pub mod blake3 {
    //! The BLAKE3 cryptographic hash function.
    //!
    //! See crate [`::blake3`].

    pub use ::blake3::*;
}

#[cfg(feature = "byteorder")]
pub mod byteorder {
    //! Big-endian and little-endian encoding.
    //!
    //! See crate [`::byteorder`].

    pub use ::byteorder::*;
}

#[cfg(feature = "bytes")]
pub mod bytes {
    //! Abstractions for working with byte buffers: [`Bytes`], [`Buf`], and [`BufMut`].
    //!
    //! See crate [`::bytes`].

    pub use ::bytes::*;
}

#[cfg(feature = "cc")]
pub mod cc {
    //! A basic cross-platform C compiler driver.
    //!
    //! See crate [`::cc`].

    pub use ::cc::*;
}

#[cfg(feature = "cfg-if")]
pub mod cfg_if {
    //! A macro for writing conditional compilation as `if` / `else` blocks.
    //!
    //! See crate [`::cfg_if`].

    pub use ::cfg_if::*;
}

#[cfg(feature = "chrono")]
pub mod chrono {
    //! Dates and time (legacy).
    //!
    //! See crate [`::chrono`].

    pub use ::chrono::*;
}

#[cfg(feature = "clap")]
pub mod clap {
    //! Command line parsing.
    //!
    //! See crate [`::clap`].

    pub use ::clap::*;
}

#[cfg(feature = "ctrlc")]
pub mod ctrlc {
    //! Simple handling of CTRL-C for CLI programs.
    //!
    //! See crate [`::ctrlc`].

    pub use ::ctrlc::*;
}

#[cfg(feature = "crossbeam")]
pub mod crossbeam {
    //! Concurrency tools to supplement [`std::sync`], including fast channels.
    //!
    //! See crate [`::crossbeam`].

    pub use ::crossbeam::*;
}

#[cfg(feature = "cxx")]
pub mod cxx {
    //! C++ bridge runtime support; paired with [`cxx_build`](super::cxx_build).
    //!
    //! See crate [`::cxx`].

    pub use ::cxx::*;
}

#[cfg(feature = "cxx-build")]
pub mod cxx_build {
    //! C++ bridge generator; paired with [`cxx`](super::cxx).
    //!
    //! See crate [`::cxx`].

    pub use ::cxx::*;
}

#[cfg(feature = "derive_more")]
pub mod derive_more {
    //! `derive` for more standard traits.
    //!
    //! See crate [`::derive_more`].

    pub use ::derive_more::*;
}

#[cfg(feature = "env_logger")]
pub mod env_logger {
    //! A basic logger to use with the [`log`](super::log) crate.
    //!
    //! See crate [`::env_logger`].

    pub use ::env_logger::*;
}

#[cfg(feature = "extension-trait")]
pub mod extension_trait {
    //! A macro for defining extension methods to external types.
    //!
    //! See crate [`::extension_trait`].

    pub use ::extension_trait::*;
}

#[cfg(feature = "futures")]
pub mod futures {
    //! Abstractions for asynchronous programming.
    //!
    //! See crate [`::futures`].

    pub use ::futures::*;
}

#[cfg(feature = "hex")]
pub mod hex {
    //! Encoding and decoding hexidecimal strings.
    //!
    //! See crate [`::hex`].

    pub use ::hex::*;
}

#[cfg(feature = "http")]
pub mod http {
    //! Shared definitions related to the HTTP protocol.
    //!
    //! See crate [`::http`].

    pub use ::http::*;
}

#[cfg(feature = "hyper")]
pub mod hyper {
    //! HTTP, versions 1 and 2.
    //!
    //! See crate [`::hyper`].

    pub use ::hyper::*;
}

#[cfg(feature = "itertools")]
pub mod itertools {
    //! Additional methods for iterators.
    //!
    //! See crate [`::itertools`].

    pub use ::itertools::*;
}

#[cfg(feature = "jiff")]
pub mod jiff {
    //! Dates and time.
    //!
    //! See crate [`::jiff`].

    pub use ::jiff::*;
}

#[cfg(feature = "json5")]
pub mod json5 {
    //! JSON5, a superset of JSON with expanded syntax.
    //!
    //! See crate [`::json5`].

    pub use ::json5::*;
}

#[cfg(feature = "libc")]
pub mod libc {
    //! Bindings to the C standard library.
    //!
    //! See crate [`::libc`].

    pub use ::libc::*;
}

#[cfg(feature = "log")]
pub mod log {
    //! A simple logging framework.
    //!
    //! See crate [`::log`].

    pub use ::log::*;
}

#[cfg(feature = "mime")]
pub mod mime {
    //! MIME media types.
    //!
    //! See crate [`::mime`].

    pub use ::mime::*;
}

#[cfg(feature = "nom")]
pub mod nom {
    //! An efficient parser combinator.
    //!
    //! See crate [`::nom`].

    pub use ::nom::*;
}

#[cfg(feature = "num-bigint")]
pub mod num_bigint {
    //! Arbitrary-sized integers.
    //!
    //! See crate [`::num_bigint`].

    pub use ::num_bigint::*;
}

#[cfg(feature = "num_cpus")]
pub mod num_cpus {
    //! Get the number of CPUS on a machine.
    //!
    //! See crate [`::num_cpus`].

    pub use ::num_cpus::*;
}

#[cfg(feature = "num_enum")]
pub mod num_enum {
    //! Conversions between numbers and enums.
    //!
    //! See crate [`::num_enum`].

    pub use ::num_enum::*;
}

#[cfg(feature = "proc-macro2")]
pub mod proc_macro2 {
    //! A preferred wrapper around the standard [`proc_macro`] crate.
    //!
    //! See crate [`::proc_macro2`].

    pub use ::proc_macro2::*;
}

#[cfg(feature = "proptest")]
pub mod proptest {
    //! Testing over generated inputs, ala QuickCheck.
    //!
    //! See crate [`::proptest`].

    pub use ::proptest::*;
}

#[cfg(feature = "quote")]
pub mod quote {
    //! The `quote!` macro for turning code blocks into source tokens.
    //!
    //! See crate [`::quote`].

    pub use ::quote::*;
}

#[cfg(feature = "rand")]
pub mod rand {
    //! Random number generators.
    //!
    //! See crate [`::rand`].

    pub use ::rand::*;
}

#[cfg(feature = "rand_chacha")]
pub mod rand_chacha {
    //! The ChaCha cryptographically-secure random number generators.
    //!
    //! See crate [`::rand_chacha`].

    pub use ::rand_chacha::*;
}

#[cfg(feature = "rand_pcg")]
pub mod rand_pcg {
    //! The PCG non-cryptographically-secure random number generators.
    //!
    //! See crate [`::rand_pcg`].

    pub use ::rand_pcg::*;
}

#[cfg(feature = "rayon")]
pub mod rayon {
    //! Parallel iterators and other parallel processing tools.
    //!
    //! See crate [`::rayon`].

    pub use ::rayon::*;
}

#[cfg(feature = "regex")]
pub mod regex {
    //! Regular expressions.
    //!
    //! See crate [`::regex`].

    pub use ::regex::*;
}

#[cfg(feature = "reqwest")]
pub mod reqwest {
    //! Simple HTTP requests, synchronous and asynchronous.
    //!
    //! See crate [`::reqwest`].

    pub use ::reqwest::*;
}

#[cfg(feature = "rustyline")]
pub mod rustyline {
    //! Command-line input reading with history.
    //!
    //! See crate [`::rustyline`].
    
    pub use ::rustyline::*;
}

#[cfg(feature = "serde")]
pub mod serde {
    //! The standard Rust serialization framework.
    //!
    //! See crate [`::serde`].

    pub use ::serde::*;
}

#[cfg(feature = "serde_json")]
pub mod serde_json {
    //! JSON serialization / deserialization with [`serde`](super::serde).
    //!
    //! See crate [`::serde_json`].

    pub use ::serde_json::*;
}

#[cfg(feature = "sha2")]
pub mod sha2 {
    //! The SHA2 cryptographic hash functions.
    //!
    //! See crate [`::sha2`].

    pub use ::sha2::*;
}

#[cfg(feature = "socket2")]
pub mod socket2 {
    //! Low-level network socket programming beyond [`std::net`].
    //!
    //! See crate [`::socket2`].

    pub use ::socket2::*;
}

#[cfg(feature = "static_assertions")]
pub mod static_assertions {
    //! Compile-time assertions about constants, types, etc.
    //!
    //! See crate [`::static_assertions`].

    pub use ::static_assertions::*;
}

#[cfg(feature = "syn")]
pub mod syn {
    //! A Rust parser used by procedural macros.
    //!
    //! See crate [`::syn`].

    pub use ::syn::*;
}

#[cfg(feature = "tempfile")]
pub mod tempfile {
    //! Temporary files and directories.
    //!
    //! See crate [`::tempfile`].

    pub use ::tempfile::*;
}

#[cfg(feature = "tera")]
pub mod tera {
    //! A text template engine based on Jinja2.
    //!
    //! See crate [`::tera`].

    pub use ::tera::*;
}

#[cfg(feature = "termcolor")]
pub mod termcolor {
    //! Cross-platform library for writing colored output to the terminal.
    //!
    //! See crate [`::termcolor`].

    pub use ::termcolor::*;
}

#[cfg(feature = "thiserror")]
pub mod thiserror {
    //! Tools for defining custom error types.
    //!
    //! See crate [`::thiserror`].

    pub use ::thiserror::*;
}

#[cfg(feature = "tokio")]
pub mod tokio {
    //! An async task runtime and I/O library.
    //!
    //! See crate [`::tokio`].

    pub use ::tokio::*;
}

#[cfg(feature = "tower")]
pub mod tower {
    //! Service request/response abstraction (HTTP middleware)
    //! for [`tokio`](super::tokio) and [`axum`](super::axum).
    //!
    //! See crate [`::tower`].

    pub use ::tower::*;
}

#[cfg(feature = "toml")]
pub mod toml {
    //! TOML serialization / deserialization with `serde`.
    //!
    //! See crate [`::toml`].

    pub use ::toml::*;
}

#[cfg(feature = "unicode-segmentation")]
pub mod unicode_segmentation {
    //! Splitting strings on grapheme cluster, word, and sentence boundaries.
    //!
    //! See crate [`::unicode_segmentation`].

    pub use ::unicode_segmentation::*;
}

#[cfg(feature = "url")]
pub mod url {
    //! URL parsing and data structures.
    //!
    //! See crate [`::url`].

    pub use ::url::*;
}

#[cfg(feature = "walkdir")]
pub mod walkdir {
    //! Efficient directory traversal.
    //!
    //! See crate [`::walkdir`].

    pub use ::walkdir::*;
}

#[cfg(feature = "xshell")]
pub mod xshell {
    //! A Swiss-army knife for writing shell-style scripts in Rust.
    //!
    //! See crate [`::xshell`].

    pub use ::xshell::*;
}
