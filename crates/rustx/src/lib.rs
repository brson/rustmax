//! TODO
//!
//! # Features
//!
//! todo
//!
//! - `default`
//!   - `all-crates`
//!   - `std`
//! - `std`
//!   - `anyhow/std`
//! - `backtrace`
//!   - `std`
//!   - `anyhow/backtrace`
//! - `all-crates`
//!
//!
//! # TODO
//!
//! - update big_s and og_fmt to be no_std
//! - update big_s to `use format as fmt`


/* ---------- */


#![no_std]


/* ---------- */


pub mod prelude {
    //! The `rustx` prelude.

    #[cfg(feature = "big_s")]
    pub use big_s::S;

    #[cfg(feature = "og_fmt")]
    pub use og_fmt::fmt;

    #[cfg(feature = "static_assertions")]
    pub use static_assertions::*;
}


/* ---------- */


#[cfg(feature = "rx-rustlib-core")]
pub extern crate core;

#[cfg(feature = "rx-rustlib-alloc")]
pub extern crate alloc;

#[cfg(feature = "rx-rustlib-std")]
pub extern crate std;

#[cfg(feature = "rx-rustlib-proc_macro")]
pub extern crate proc_macro;


/* ---------- */


#[cfg(feature = "anyhow")]
pub mod anyhow {
    //! Easy error handling.
    //!
    //! See crate [`::anyhow`].

    pub use ::anyhow::*;
}

#[cfg(feature = "backtrace")]
pub mod backtrace {
    //! Callstack backtraces.
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

#[cfg(feature = "big_s")]
pub mod big_s {
    //! Succinct `String` "literals".
    //!
    //! See crate [`::big_s`].

    pub use ::big_s::*;
}

#[cfg(feature = "bitflags")]
pub mod bitflags {
    //! A macro that generates structs that behave as bitflags.
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
    //! Dates and time.
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

#[cfg(feature = "crossbeam")]
pub mod crossbeam {
    //! Concurrency tools.
    //!
    //! See crate [`::crossbeam`].

    pub use ::crossbeam::*;
}

#[cfg(feature = "cxx")]
pub mod cxx {
    //! C++ bindings generator.
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
    //! A basic logger to use with the `log` crate.
    //!
    //! See crate [`::env_logger`].

    pub use ::env_logger::*;
}

#[cfg(feature = "extension-trait")]
pub mod extension_trait {
    //! A macro for defining extension methods to foreign types.
    //!
    //! See crate [`::extension_trait`].

    pub use ::extension_trait::*;
}

#[cfg(feature = "fnv")]
pub mod fnv {
    //! A fast non-cryptographic hash function for use with `HashMap`.
    //!
    //! See crate [`::fnv`].

    pub use ::fnv::*;
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

#[cfg(feature = "json5")]
pub mod json5 {
    //! JSON5, a superset of JSON with expanded syntax.
    //!
    //! See crate [`::json5`].

    pub use ::json5::*;
}

#[cfg(feature = "lazy_static")]
pub mod lazy_static {
    //! Lazy initialization of static variables.
    //!
    //! See crate [`::lazy_static`].

    pub use ::lazy_static::*;
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

#[cfg(feature = "num_cpus")]
pub mod num_cpus {
    //! Get the number of CPUS on a machine.
    //!
    //! See crate [`::num_cpus`].

    pub use ::num_cpus::*;
}

#[cfg(feature = "og_fmt")]
pub mod og_fmt {
    //! The `fmt!` synonym for `format!`.
    //!
    //! See crate [`::og_fmt`].

    pub use ::og_fmt::*;
}

#[cfg(feature = "once_cell")]
pub mod once_cell {
    //! Values that are lazily initialized.
    //!
    //! See crate [`::once_cell`].

    pub use ::once_cell::*;
}

#[cfg(feature = "proc-macro2")]
pub mod proc_macro2 {
    //! A preferred wrapper around the standard `proc_macro` crate.
    //!
    //! See crate [`::proc_macro2`].

    pub use ::proc_macro2::*;
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
    //! Simple HTTP requests.
    //!
    //! See crate [`::reqwest`].

    pub use ::reqwest::*;
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
    //! JSON serialization / deserialization.
    //!
    //! See crate [`::serde_json`].

    pub use ::serde_json::*;
}

#[cfg(feature = "socket2")]
pub mod socket2 {
    //! Low-level network socket programming.
    //!
    //! See crate [`::socket2`].

    pub use ::socket2::*;
}

#[cfg(feature = "static_assertions")]
pub mod static_assertions {
    //! Assertions about constants, types, and more.
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
