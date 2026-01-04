//! Feature flags using bitflags.
//!
//! Controls optional build and rendering features.

use rustmax::prelude::*;
use rustmax::bitflags::bitflags;

bitflags! {
    /// Build feature flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BuildFeatures: u32 {
        /// Include draft documents in output.
        const DRAFTS = 1 << 0;
        /// Generate gzip-compressed versions of files.
        const COMPRESS = 1 << 1;
        /// Generate search index.
        const SEARCH_INDEX = 1 << 2;
        /// Generate RSS feed.
        const RSS = 1 << 3;
        /// Generate sitemap.
        const SITEMAP = 1 << 4;
        /// Process shortcodes in content.
        const SHORTCODES = 1 << 5;
        /// Minify HTML output.
        const MINIFY = 1 << 6;
        /// Inline small images as data URLs.
        const INLINE_IMAGES = 1 << 7;
        /// Generate table of contents.
        const TOC = 1 << 8;
        /// Enable syntax highlighting.
        const SYNTAX_HIGHLIGHT = 1 << 9;

        /// Default features for production builds.
        const PRODUCTION = Self::SEARCH_INDEX.bits()
            | Self::RSS.bits()
            | Self::SITEMAP.bits()
            | Self::SHORTCODES.bits()
            | Self::COMPRESS.bits();

        /// Default features for development.
        const DEVELOPMENT = Self::DRAFTS.bits()
            | Self::SHORTCODES.bits()
            | Self::SYNTAX_HIGHLIGHT.bits();

        /// All features enabled.
        const ALL = Self::DRAFTS.bits()
            | Self::COMPRESS.bits()
            | Self::SEARCH_INDEX.bits()
            | Self::RSS.bits()
            | Self::SITEMAP.bits()
            | Self::SHORTCODES.bits()
            | Self::MINIFY.bits()
            | Self::INLINE_IMAGES.bits()
            | Self::TOC.bits()
            | Self::SYNTAX_HIGHLIGHT.bits();
    }
}

impl BuildFeatures {
    /// Parse features from a comma-separated string.
    pub fn from_str_list(s: &str) -> Self {
        let mut features = Self::empty();

        for part in s.split(',') {
            let part = part.trim().to_lowercase();
            match part.as_str() {
                "drafts" => features |= Self::DRAFTS,
                "compress" => features |= Self::COMPRESS,
                "search" | "search_index" => features |= Self::SEARCH_INDEX,
                "rss" => features |= Self::RSS,
                "sitemap" => features |= Self::SITEMAP,
                "shortcodes" => features |= Self::SHORTCODES,
                "minify" => features |= Self::MINIFY,
                "inline_images" => features |= Self::INLINE_IMAGES,
                "toc" => features |= Self::TOC,
                "syntax" | "syntax_highlight" => features |= Self::SYNTAX_HIGHLIGHT,
                "production" => features |= Self::PRODUCTION,
                "development" | "dev" => features |= Self::DEVELOPMENT,
                "all" => features |= Self::ALL,
                _ => {}
            }
        }

        features
    }

    /// Convert to a list of feature names.
    pub fn to_names(&self) -> Vec<&'static str> {
        let mut names = Vec::new();
        if self.contains(Self::DRAFTS) { names.push("drafts"); }
        if self.contains(Self::COMPRESS) { names.push("compress"); }
        if self.contains(Self::SEARCH_INDEX) { names.push("search_index"); }
        if self.contains(Self::RSS) { names.push("rss"); }
        if self.contains(Self::SITEMAP) { names.push("sitemap"); }
        if self.contains(Self::SHORTCODES) { names.push("shortcodes"); }
        if self.contains(Self::MINIFY) { names.push("minify"); }
        if self.contains(Self::INLINE_IMAGES) { names.push("inline_images"); }
        if self.contains(Self::TOC) { names.push("toc"); }
        if self.contains(Self::SYNTAX_HIGHLIGHT) { names.push("syntax_highlight"); }
        names
    }
}

impl Default for BuildFeatures {
    fn default() -> Self {
        Self::SHORTCODES | Self::SYNTAX_HIGHLIGHT
    }
}

bitflags! {
    /// Content processing flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ContentFlags: u16 {
        /// Document is a draft.
        const DRAFT = 1 << 0;
        /// Document is featured/pinned.
        const FEATURED = 1 << 1;
        /// Document is hidden from listings.
        const HIDDEN = 1 << 2;
        /// Document should not be indexed.
        const NO_INDEX = 1 << 3;
        /// Document has math content.
        const HAS_MATH = 1 << 4;
        /// Document has code blocks.
        const HAS_CODE = 1 << 5;
        /// Document has images.
        const HAS_IMAGES = 1 << 6;
        /// Document has shortcodes.
        const HAS_SHORTCODES = 1 << 7;
    }
}

impl ContentFlags {
    /// Analyze content and return appropriate flags.
    pub fn analyze(content: &str) -> Self {
        let mut flags = Self::empty();

        if content.contains("```") || content.contains("~~~") {
            flags |= Self::HAS_CODE;
        }
        if content.contains("![") || content.contains("<img") {
            flags |= Self::HAS_IMAGES;
        }
        if content.contains("{{<") || content.contains("{{%") {
            flags |= Self::HAS_SHORTCODES;
        }
        if content.contains("$") || content.contains("\\(") || content.contains("\\[") {
            flags |= Self::HAS_MATH;
        }

        flags
    }
}

bitflags! {
    /// Server configuration flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ServerFlags: u8 {
        /// Enable live reload.
        const LIVE_RELOAD = 1 << 0;
        /// Enable CORS headers.
        const CORS = 1 << 1;
        /// Enable directory listings.
        const DIRECTORY_LISTING = 1 << 2;
        /// Enable gzip compression.
        const COMPRESSION = 1 << 3;
        /// Enable caching headers.
        const CACHE = 1 << 4;

        /// Default development server flags.
        const DEV_DEFAULTS = Self::LIVE_RELOAD.bits()
            | Self::CORS.bits()
            | Self::DIRECTORY_LISTING.bits();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_features_default() {
        let features = BuildFeatures::default();
        assert!(features.contains(BuildFeatures::SHORTCODES));
        assert!(features.contains(BuildFeatures::SYNTAX_HIGHLIGHT));
        assert!(!features.contains(BuildFeatures::DRAFTS));
    }

    #[test]
    fn test_build_features_production() {
        let features = BuildFeatures::PRODUCTION;
        assert!(features.contains(BuildFeatures::COMPRESS));
        assert!(features.contains(BuildFeatures::SEARCH_INDEX));
        assert!(features.contains(BuildFeatures::RSS));
        assert!(features.contains(BuildFeatures::SITEMAP));
        assert!(!features.contains(BuildFeatures::DRAFTS));
    }

    #[test]
    fn test_build_features_from_str() {
        let features = BuildFeatures::from_str_list("drafts, compress, rss");
        assert!(features.contains(BuildFeatures::DRAFTS));
        assert!(features.contains(BuildFeatures::COMPRESS));
        assert!(features.contains(BuildFeatures::RSS));
        assert!(!features.contains(BuildFeatures::SITEMAP));
    }

    #[test]
    fn test_build_features_to_names() {
        let features = BuildFeatures::DRAFTS | BuildFeatures::RSS;
        let names = features.to_names();
        assert!(names.contains(&"drafts"));
        assert!(names.contains(&"rss"));
    }

    #[test]
    fn test_content_flags_analyze() {
        let content = "Some text with ```code``` and ![image](url)";
        let flags = ContentFlags::analyze(content);
        assert!(flags.contains(ContentFlags::HAS_CODE));
        assert!(flags.contains(ContentFlags::HAS_IMAGES));
        assert!(!flags.contains(ContentFlags::HAS_MATH));
    }

    #[test]
    fn test_content_flags_shortcodes() {
        let content = "Text with {{< youtube >}} shortcode";
        let flags = ContentFlags::analyze(content);
        assert!(flags.contains(ContentFlags::HAS_SHORTCODES));
    }

    #[test]
    fn test_server_flags() {
        let flags = ServerFlags::DEV_DEFAULTS;
        assert!(flags.contains(ServerFlags::LIVE_RELOAD));
        assert!(flags.contains(ServerFlags::CORS));
        assert!(!flags.contains(ServerFlags::CACHE));
    }

    #[test]
    fn test_bitflags_operations() {
        let mut features = BuildFeatures::empty();
        features |= BuildFeatures::DRAFTS;
        features |= BuildFeatures::COMPRESS;

        assert!(features.contains(BuildFeatures::DRAFTS));
        assert!(features.contains(BuildFeatures::COMPRESS));

        features.remove(BuildFeatures::DRAFTS);
        assert!(!features.contains(BuildFeatures::DRAFTS));
        assert!(features.contains(BuildFeatures::COMPRESS));
    }
}
