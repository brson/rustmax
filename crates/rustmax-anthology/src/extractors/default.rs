//! Default HTML content extractor.

use rmx::prelude::*;
use scraper::{Html, Selector};
use super::Extractor;

/// Default extractor that looks for common article structures.
pub struct DefaultExtractor;

impl Extractor for DefaultExtractor {
    fn extract(&self, html: &str) -> AnyResult<String> {
        let document = Html::parse_document(html);

        // Try to find content in order of preference.
        let selectors = [
            "article",
            "main",
            "[role='main']",
            ".post-content",
            ".entry-content",
            ".content",
            "body",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    // Get the inner HTML.
                    let content = element.html();
                    return Ok(content);
                }
            }
        }

        bail!("Could not find main content in HTML");
    }
}
