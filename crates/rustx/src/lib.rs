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
    //! TODO
    //!
    //! See crate [`::anyhow`].
    pub use ::anyhow::*;
}

#[cfg(feature = "big_s")]
pub mod big_s {
    //! TODO
    //!
    //! See crate [`::big_s`].
    pub use ::big_s::*;
}

#[cfg(feature = "blake3")]
pub mod blake3 {
    //! TODO
    //!
    //! See crate [`::blake3`].
    pub use ::blake3::*;
}

#[cfg(feature = "byteorder")]
pub mod byteorder {
    //! TODO
    //!
    //! See crate [`::byteorder`].
    pub use ::byteorder::*;
}

#[cfg(feature = "bytes")]
pub mod bytes {
    //! Abstractions for working with byte buffers: [`Bytes`], [`Buf`], and [`BufMut`].
    //!
    //! See crate [`bytes`].
    pub use ::bytes::*;
}
