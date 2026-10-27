//! Generated stylesheets and scripts, and their Subresource Integrity hashes.
//!
//! Pages link these files, so a page can tell the browser what each one
//! should hash to. A file that has been altered on its way to the reader,
//! by a CDN or anything else in between, then fails to load
//! instead of running.

use rmx::prelude::*;
use rmx::base64::{Engine, engine::general_purpose::STANDARD};
use rmx::sha2::{Digest, Sha384};
use std::collections::BTreeMap;

use crate::collection::Config;
use crate::features::BuildFeatures;

use super::{generate_highlight_css, generate_search_js, generate_toc_css};

/// A file the build generates rather than copies.
#[derive(Debug, Clone)]
pub struct Asset {
    /// Path relative to the output directory.
    pub path: &'static str,
    pub contents: String,
}

impl Asset {
    /// The template variable holding this asset's integrity hash:
    /// the path with every non-alphanumeric character replaced by `_`.
    pub fn key(&self) -> String {
        self.path
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect()
    }
}

/// The generated files a build of `config` writes.
pub fn site_assets(config: &Config) -> Vec<Asset> {
    let mut assets = vec![Asset {
        path: "toc.css",
        contents: generate_toc_css().to_string(),
    }];

    if config.highlight.enabled {
        assets.push(Asset {
            path: "highlight.css",
            contents: generate_highlight_css(&config.highlight.to_options()),
        });
    }

    if config.build.features.contains(BuildFeatures::SEARCH) {
        assets.push(Asset {
            path: "search.js",
            contents: generate_search_js(),
        });
    }

    assets
}

/// The Subresource Integrity value for `data`, as used in an `integrity`
/// attribute.
///
/// SHA-384 is what the SRI specification recommends.
pub fn integrity(data: &[u8]) -> String {
    format!("sha384-{}", STANDARD.encode(Sha384::digest(data)))
}

/// Integrity hashes for `assets`, keyed by [`Asset::key`].
pub fn integrity_map(assets: &[Asset]) -> BTreeMap<String, String> {
    assets
        .iter()
        .map(|asset| (asset.key(), integrity(asset.contents.as_bytes())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrity_matches_a_known_digest() {
        // From the SRI specification's example: `alert('Hello, world.');`.
        assert_eq!(
            integrity(b"alert('Hello, world.');"),
            "sha384-H8BRh8j48O9oYatfu5AZzq6A9RINhZO5H16dQZngK7T62em8MUt1FLm52t+eX6xO",
        );
    }

    #[test]
    fn keys_are_template_identifiers() {
        let asset = Asset { path: "highlight.css", contents: String::new() };
        assert_eq!(asset.key(), "highlight_css");
    }

    #[test]
    fn assets_follow_the_config() {
        let mut config = Config::default();
        let paths = |config: &Config| site_assets(config).iter().map(|a| a.path).collect::<Vec<_>>();
        assert_eq!(paths(&config), ["toc.css", "highlight.css", "search.js"]);

        config.highlight.enabled = false;
        config.build.features.remove(BuildFeatures::SEARCH);
        assert_eq!(paths(&config), ["toc.css"]);
    }
}
