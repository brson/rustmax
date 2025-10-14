//! Curated collection of notable Rust blog posts and essays.
//!
//! This crate provides tools to fetch, extract, and present a curated
//! collection of Rust writing from across the web.

#![allow(unused)]

use rmx::prelude::*;

pub mod metadata;
pub mod fetch;
pub mod extract;
pub mod markdown;
pub mod extractors;

pub use metadata::*;
