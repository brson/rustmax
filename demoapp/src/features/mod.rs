//! Optional build outputs, as bitflags.
//!
//! A collection chooses them with `[build] features` in its config,
//! written as a list of lowercase names:
//!
//! ```toml
//! [build]
//! features = ["search", "feeds", "sitemap", "shortcodes"]
//! ```
//!
//! `folio build --drafts` and `--compress` add to whatever the config enables.

use rmx::prelude::*;
use rmx::bitflags::bitflags;
use rmx::serde::de::{self, Deserializer, SeqAccess, Visitor};
use rmx::serde::ser::{SerializeSeq, Serializer};
use rmx::serde::{Deserialize, Serialize};
use std::fmt;

bitflags! {
    /// What a build produces beyond the document pages themselves.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BuildFeatures: u8 {
        /// Build draft documents too.
        const DRAFTS = 1 << 0;
        /// Write a `.gz` alongside every compressible file.
        const COMPRESS = 1 << 1;
        /// Write the search index and client-side search script.
        const SEARCH = 1 << 2;
        /// Write RSS, Atom and JSON feeds.
        const FEEDS = 1 << 3;
        /// Write `sitemap.xml`.
        const SITEMAP = 1 << 4;
        /// Expand shortcodes in documents.
        const SHORTCODES = 1 << 5;
    }
}

impl Default for BuildFeatures {
    fn default() -> Self {
        Self::SEARCH | Self::FEEDS | Self::SITEMAP | Self::SHORTCODES
    }
}

impl BuildFeatures {
    /// The config name of every feature, in declaration order.
    pub fn names() -> impl Iterator<Item = String> {
        Self::all().iter_names().map(|(name, _)| name.to_lowercase())
    }

    /// Look up a feature by its config name.
    pub fn from_config_name(name: &str) -> Option<Self> {
        // Flag names are upper case, and config names are their lower case.
        if name.bytes().any(|b| b.is_ascii_uppercase()) {
            return None;
        }
        Self::from_name(&name.to_ascii_uppercase())
    }
}

impl Serialize for BuildFeatures {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(None)?;
        for (name, _) in self.iter_names() {
            seq.serialize_element(&name.to_lowercase())?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for BuildFeatures {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FeatureList;

        impl<'de> Visitor<'de> for FeatureList {
            type Value = BuildFeatures;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a list of build features")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<BuildFeatures, A::Error> {
                let mut features = BuildFeatures::empty();
                while let Some(name) = seq.next_element::<String>()? {
                    let feature = BuildFeatures::from_config_name(&name).ok_or_else(|| {
                        let known = BuildFeatures::names().collect::<Vec<_>>().join(", ");
                        de::Error::custom(format!("unknown build feature `{name}`, expected one of: {known}"))
                    })?;
                    features |= feature;
                }
                Ok(features)
            }
        }

        deserializer.deserialize_seq(FeatureList)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rmx::derive(Debug, Serialize, Deserialize)]
    struct Wrapper {
        features: BuildFeatures,
    }

    fn parse(toml: &str) -> Result<BuildFeatures, rmx::toml::de::Error> {
        rmx::toml::from_str::<Wrapper>(toml).map(|w| w.features)
    }

    #[test]
    fn parses_a_list_of_names() {
        let features = parse(r#"features = ["drafts", "sitemap"]"#).unwrap();
        assert_eq!(features, BuildFeatures::DRAFTS | BuildFeatures::SITEMAP);
    }

    #[test]
    fn an_empty_list_disables_everything() {
        assert_eq!(parse("features = []").unwrap(), BuildFeatures::empty());
    }

    #[test]
    fn unknown_names_are_rejected_with_the_alternatives() {
        let err = parse(r#"features = ["minify"]"#).unwrap_err().to_string();
        assert!(err.contains("unknown build feature `minify`"), "{err}");
        assert!(err.contains("shortcodes"), "{err}");
    }

    #[test]
    fn names_are_case_sensitive() {
        assert!(parse(r#"features = ["SEARCH"]"#).is_err());
    }

    #[test]
    fn round_trips_through_toml() {
        let original = Wrapper { features: BuildFeatures::all() };
        let text = rmx::toml::to_string(&original).unwrap();
        assert_eq!(parse(&text).unwrap(), BuildFeatures::all());
    }

    #[test]
    fn every_flag_has_a_config_name() {
        assert_eq!(BuildFeatures::names().count(), BuildFeatures::all().iter().count());
        for name in BuildFeatures::names() {
            assert!(BuildFeatures::from_config_name(&name).is_some(), "{name}");
        }
    }
}
