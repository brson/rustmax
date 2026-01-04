//! Anthology: A document publishing platform.
//!
//! Anthology processes collections of markdown documents into publishable outputs
//! such as static HTML sites and JSON exports. It serves as a demonstration of
//! the rustmax crate ecosystem.

pub mod cli;
pub mod collection;
pub mod build;
pub mod serve;
pub mod search;
pub mod remote;
pub mod util;
pub mod crypto;
pub mod text;
pub mod time;
pub mod shortcode;
pub mod features;
pub mod concurrency;
pub mod error;

pub use error::{Error, Result};
