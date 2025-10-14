//! Content extraction from HTML.

use rmx::prelude::*;

pub mod default;
pub mod custom;

/// Trait for HTML content extractors.
pub trait Extractor {
    /// Extract the main content from HTML.
    fn extract(&self, html: &str) -> AnyResult<String>;
}

/// Get an extractor by name.
pub fn get_extractor(name: &str) -> AnyResult<Box<dyn Extractor>> {
    match name {
        "default" => Ok(Box::new(default::DefaultExtractor)),
        _ => bail!("Unknown extractor: {}", name),
    }
}
