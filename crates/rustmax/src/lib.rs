#![doc = include_str!("../doc-src/root-docs.md")]
#![allow(clippy::needless_doctest_main)]
/* ---------- */
#![no_std]

/* ---------- */

pub mod prelude {
    //! The `rmx` prelude.

    /* standard library preludes */

    #[cfg(all(feature = "rmx-rustlib-core", not(feature = "rmx-rustlib-std")))]
    pub use ::core::prelude::rust_2021::*;

    #[cfg(feature = "rmx-rustlib-std")]
    pub use ::std::prelude::rust_2021::*;

    /* standard library macros */

    #[cfg(feature = "rmx-rustlib-alloc")]
    pub use ::alloc::{format, vec};

    #[cfg(feature = "rmx-rustlib-alloc")]
    pub use crate::extras::fmt;

    /* standard library exports that aren't in its prelude */

    // Ordering is recommended by
    // `clippy::comparison_chain` and if it's
    // important enough that the compiler suggests
    // using it instead of comparison operator syntax,
    // let's put it in the prelude.
    #[cfg(feature = "rmx-rustlib-core")]
    pub use ::core::cmp::Ordering;

    /* other preludes */

    #[cfg(feature = "futures")]
    pub use ::futures::prelude::*;

    /* common non-std imports */

    #[cfg(feature = "anyhow")]
    pub use ::anyhow::{Context as _, anyhow, bail};

    #[cfg(feature = "anyhow")]
    pub use crate::extras::{A, AnyError, AnyResult};

    #[cfg(feature = "cfg-if")]
    pub use ::cfg_if::cfg_if;

    #[cfg(feature = "extension-trait")]
    pub use ::extension_trait::extension_trait;

    #[cfg(feature = "log")]
    pub use ::log::{debug, error, info, trace, warn};

    #[cfg(all(feature = "futures", feature = "rmx-feature-default"))]
    pub use ::futures::{executor::block_on, future::Either};

    #[cfg(feature = "itertools")]
    pub use ::itertools::Itertools as _;

    #[cfg(feature = "rand")]
    pub use ::rand::RngCore as _;
    #[cfg(feature = "rand")]
    pub use ::rand::Rng as _;
    #[cfg(feature = "rand")]
    pub use ::rand::SeedableRng as _;
    #[cfg(feature = "rand")]
    pub use ::rand::Fill as _;

    /* extras */

    pub use crate::extras::default;

    #[cfg(feature = "rmx-rustlib-core")]
    pub use crate::bug;

    #[cfg(feature = "rmx-rustlib-alloc")]
    pub use crate::extras::S;

    #[cfg(feature = "rmx-rustlib-alloc")]
    pub use crate::extras::O;

    #[cfg(all(feature = "extension-trait", feature = "anyhow"))]
    pub use crate::extras::AnyResultExpect as _;
    #[cfg(feature = "extension-trait")]
    pub use crate::extras::OptionExpect as _;
    #[cfg(feature = "extension-trait")]
    pub use crate::extras::QuickClone as _;
    #[cfg(feature = "extension-trait")]
    pub use crate::extras::QuickToOwned as _;
    #[cfg(feature = "extension-trait")]
    pub use crate::extras::QuickToString as _;
    #[cfg(feature = "extension-trait")]
    pub use crate::extras::RangeExt as _;
    #[cfg(feature = "extension-trait")]
    pub use crate::extras::ResultExpect as _;
    #[cfg(feature = "extension-trait")]
    pub use crate::extras::ResultIgnore as _;
}

pub mod extras {
    //! Additional tidbits defined by `rmx`.

    /// Like 'unimplemented' but shorter to type.
    #[cfg(feature = "rmx-rustlib-core")]
    #[macro_export]
    macro_rules! bug {
        () => {
            core::panic!("unexpected case (bug!)")
        };
        ($($arg:tt)+) => {
            core::panic!("unexpected case (bug!): {}", $crate::format_args!($($arg)+))
        };
    }

    #[cfg(feature = "anyhow")]
    pub use ::anyhow::{Error as AnyError, Result as AnyResult, anyhow as A};

    #[cfg(feature = "rmx-rustlib-alloc")]
    pub use ::alloc::format as fmt;

    pub fn default<T: Default>() -> T {
        Default::default()
    }

    pub fn init() {
        #[cfg(feature = "env_logger")]
        fn maybe_init_env_logger() {
            crate::env_logger::Builder::new()
                .filter_level(log::LevelFilter::Info)
                .format_timestamp(None)
                .parse_default_env()
                .init();
        }
        #[cfg(not(feature = "env_logger"))]
        fn maybe_init_env_logger() {}

        maybe_init_env_logger();
    }

    pub fn init_crate_name(crate_name: &str) {
        #[cfg(feature = "env_logger")]
        fn maybe_init_env_logger(crate_name: &str) {
            crate::env_logger::Builder::new()
                .filter_module(crate_name, log::LevelFilter::Info)
                .format_timestamp(None)
                .parse_default_env()
                .init();
        }
        #[cfg(not(feature = "env_logger"))]
        fn maybe_init_env_logger(_crate_name: &str) {}

        maybe_init_env_logger(crate_name);
    }

    pub fn recurse<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        // todo could grow stack here
        f()
    }

    #[cfg(feature = "rmx-rustlib-alloc")]
    #[allow(non_snake_case)]
    pub fn S<T>(s: &T) -> crate::alloc::string::String
    where
        T: crate::alloc::string::ToString + ?Sized,
    {
        crate::alloc::string::ToString::to_string(s)
    }

    #[cfg(feature = "rmx-rustlib-alloc")]
    #[allow(non_snake_case)]
    pub fn O<T>(o: &T) -> T::Owned
    where
        T: crate::alloc::borrow::ToOwned + ?Sized,
    {
        crate::alloc::borrow::ToOwned::to_owned(o)
    }

    #[cfg(feature = "extension-trait")]
    #[extension_trait::extension_trait]
    pub impl<T> QuickToString for T
    where
        T: crate::alloc::string::ToString + ?Sized,
    {
        #[allow(non_snake_case)]
        fn S(&self) -> crate::alloc::string::String {
            crate::alloc::string::ToString::to_string(self)
        }
    }

    #[cfg(feature = "extension-trait")]
    #[extension_trait::extension_trait]
    pub impl<T> QuickToOwned for T
    where
        T: crate::alloc::borrow::ToOwned,
    {
        type Owned = T::Owned;

        #[allow(non_snake_case)]
        fn O(&self) -> Self::Owned {
            crate::alloc::borrow::ToOwned::to_owned(self)
        }
    }

    #[cfg(feature = "extension-trait")]
    #[extension_trait::extension_trait]
    pub impl<T> QuickClone<T> for T
    where
        T: Clone,
    {
        #[allow(non_snake_case)]
        fn C(&self) -> T {
            self.clone()
        }
    }

    #[cfg(feature = "extension-trait")]
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

    #[cfg(feature = "extension-trait")]
    #[extension_trait::extension_trait]
    pub impl<T, E> ResultExpect<T, E> for Result<T, E>
    where
        E: core::error::Error,
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

    #[cfg(all(feature = "extension-trait", feature = "anyhow"))]
    #[extension_trait::extension_trait]
    pub impl<T> AnyResultExpect<T> for AnyResult<T> {
        #[track_caller]
        #[allow(non_snake_case)]
        fn X(self) -> T {
            match self {
                Ok(v) => v,
                Err(e) => panic!("impossible `Err` result: {e}"),
            }
        }
    }

    /// Ignore a `Result`.
    ///
    /// This is nice because the common idiom of `let _ = ...` is untyped
    /// and can accidentally be applied to non-`Result` types like `Future`s.
    #[cfg(feature = "extension-trait")]
    #[extension_trait::extension_trait]
    pub impl<T, E> ResultIgnore<T, E> for Result<T, E> {
        #[track_caller]
        #[allow(non_snake_case)]
        fn I(self) {
            let _ = self;
        }
    }

    // todo: define this for generic Range<N>
    // todo: put this in a crate and elaborate
    #[cfg(feature = "extension-trait")]
    #[extension_trait::extension_trait]
    pub impl RangeExt for core::ops::Range<usize> {
        fn from_start_len(start: usize, len: usize) -> Option<core::ops::Range<usize>> {
            Some(start..start.checked_add(len)?)
        }

        fn subrange(&self, sub: core::ops::Range<usize>) -> Option<core::ops::Range<usize>> {
            if sub.start >= self.len() || sub.end > self.len() {
                return None;
            }
            let new_start = self.start.checked_add(sub.start);
            let new_end = self.start.checked_add(sub.end);
            match (new_start, new_end) {
                (Some(new_start), Some(new_end)) => Some(new_start..new_end),
                _ => None,
            }
        }

        fn checked_sub(&self, other: usize) -> Option<core::ops::Range<usize>> {
            let new_start = self.start.checked_sub(other);
            let new_end = self.end.checked_sub(other);
            match (new_start, new_end) {
                (Some(new_start), Some(new_end)) => Some(new_start..new_end),
                _ => None,
            }
        }
    }

    #[cfg(feature = "rmx-rustlib-std")]
    pub fn copy_dir_recursive(
        src: &crate::std::path::Path,
        dst: &crate::std::path::Path,
    ) -> crate::std::io::Result<()> {
        crate::std::fs::create_dir_all(dst)?;

        for entry in crate::std::fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                crate::std::fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }

    /// Use in constant contexts to assert a type is `Sync + Sync`.p
    ///
    /// ```
    /// # use rustmax as rmx;
    /// use rmx::extras::assert_send_sync;
    ///
    /// struct DbPathGen(());
    ///
    /// const _ASSERT_SEND_SYNC: () = assert_send_sync::<DbPathGen>();
    /// ```
    pub const fn assert_send_sync<T: Send + Sync>() { }    
}

/* ---------- */

#[cfg(feature = "rmx-rustlib-core")]
#[doc(inline)]
pub extern crate core;

#[cfg(feature = "rmx-rustlib-alloc")]
#[doc(inline)]
pub extern crate alloc;

#[cfg(feature = "rmx-rustlib-std")]
#[doc(inline)]
pub extern crate std;

#[cfg(feature = "rmx-rustlib-proc_macro")]
#[doc(inline)]
pub extern crate proc_macro;

/* ---------- */

#[cfg(feature = "ahash")]
pub mod ahash {
    #![doc = include_str!("../doc-src/crate-ahash.md")]

    pub use ::ahash::*;
}

#[cfg(feature = "anyhow")]
pub mod anyhow {
    #![doc = include_str!("../doc-src/crate-anyhow.md")]

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
    #![doc = include_str!("../doc-src/crate-base64.md")]

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
    #![doc = include_str!("../doc-src/crate-bitflags.md")]

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
    #![doc = include_str!("../doc-src/crate-bytes.md")]

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
    #![doc = include_str!("../doc-src/crate-chrono.md")]

    pub use ::chrono::*;
}

#[cfg(feature = "clap")]
pub mod clap {
    #![doc = include_str!("../doc-src/crate-clap.md")]

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
    #![doc = include_str!("../doc-src/crate-futures.md")]

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
    #![doc = include_str!("../doc-src/crate-itertools.md")]

    pub use ::itertools::*;
}

#[cfg(feature = "jiff")]
pub mod jiff {
    #![doc = include_str!("../doc-src/crate-jiff.md")]

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
    #![doc = include_str!("../doc-src/crate-log.md")]

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
    #![doc = include_str!("../doc-src/crate-num-bigint.md")]

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
    #![doc = include_str!("../doc-src/crate-rayon.md")]

    pub use ::rayon::*;
}

#[cfg(feature = "regex")]
pub mod regex {
    #![doc = include_str!("../doc-src/crate-regex.md")]

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

#[cfg(feature = "semver")]
pub mod semver {
    //! The software versioning standard used by Rust
    //!
    //! See crate [`::semver`].

    pub use ::semver::*;
}

#[cfg(feature = "serde")]
pub mod serde {
    #![doc = include_str!("../doc-src/crate-serde.md")]

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
    #![doc = include_str!("../doc-src/crate-thiserror.md")]

    pub use ::thiserror::*;
}

#[cfg(feature = "tokio")]
pub mod tokio {
    #![doc = include_str!("../doc-src/crate-tokio.md")]

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
