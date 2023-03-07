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

#[cfg(feature = "big_s")]
pub mod big_s {
    //! Succinct `String` "literals".
    //!
    //! See crate [`::big_s`].

    pub use ::big_s::*;
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

#[cfg(feature = "log")]
pub mod log {
    //! A simple logging framework.
    //!
    //! See crate [`::log`].

    pub use ::log::*;
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
