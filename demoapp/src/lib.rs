//! Folio: A document publishing platform.
//!
//! Folio processes collections of markdown documents into publishable outputs
//! such as static HTML sites and JSON exports. It serves as a demonstration of
//! the rustmax crate ecosystem.

pub mod cli;
pub mod collection;
pub mod build;
pub mod search;
pub mod serve;
pub mod lint;
pub mod remote;
pub mod util;
pub mod shortcode;
pub mod site;
pub mod features;
pub mod feeds;
pub mod watch;
pub mod export;
pub mod error;

pub use error::{Error, Result};
