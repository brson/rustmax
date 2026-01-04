//! Export functionality for collections.
//!
//! Supports exporting to EPUB and other formats.

mod epub;

pub use epub::{EpubBuilder, EpubConfig, generate_epub};
