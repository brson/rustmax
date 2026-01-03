//! Configuration file parsing for anthology.toml.

use rustmax::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::{Error, Result};

/// Main configuration for a collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub collection: CollectionConfig,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub content: ContentConfig,
    #[serde(default)]
    pub server: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            collection: CollectionConfig::default(),
            build: BuildConfig::default(),
            content: ContentConfig::default(),
            server: ServerConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from a directory containing anthology.toml.
    pub fn load(root: &Path) -> Result<Self> {
        let config_path = root.join("anthology.toml");

        if !config_path.exists() {
            return Err(Error::CollectionNotFound {
                path: root.to_path_buf(),
            });
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: Config =
            rustmax::toml::from_str(&content).map_err(|e| Error::ConfigParse {
                path: config_path,
                source: e,
            })?;

        Ok(config)
    }
}

/// Collection metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    /// Title of the collection/site.
    #[serde(default = "default_title")]
    pub title: String,
    /// Base URL for the site.
    #[serde(default)]
    pub base_url: String,
    /// Description of the collection.
    #[serde(default)]
    pub description: String,
    /// Author name.
    #[serde(default)]
    pub author: String,
    /// Language code (e.g., "en").
    #[serde(default = "default_language")]
    pub language: String,
}

impl Default for CollectionConfig {
    fn default() -> Self {
        Self {
            title: default_title(),
            base_url: String::new(),
            description: String::new(),
            author: String::new(),
            language: default_language(),
        }
    }
}

fn default_title() -> String {
    "My Collection".to_string()
}

fn default_language() -> String {
    "en".to_string()
}

/// Build configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Output directory for built site.
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    /// Theme directory name.
    #[serde(default = "default_theme")]
    pub theme: String,
    /// Whether to include drafts in builds.
    #[serde(default)]
    pub drafts: bool,
    /// Minify HTML output.
    #[serde(default)]
    pub minify: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output_dir: default_output_dir(),
            theme: default_theme(),
            drafts: false,
            minify: false,
        }
    }
}

fn default_output_dir() -> String {
    "output".to_string()
}

fn default_theme() -> String {
    "default".to_string()
}

/// Content configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentConfig {
    /// Date format for parsing and display.
    #[serde(default = "default_date_format")]
    pub date_format: String,
    /// Default template for documents.
    #[serde(default = "default_template")]
    pub default_template: String,
    /// Excerpt separator in content.
    #[serde(default = "default_excerpt_separator")]
    pub excerpt_separator: String,
}

impl Default for ContentConfig {
    fn default() -> Self {
        Self {
            date_format: default_date_format(),
            default_template: default_template(),
            excerpt_separator: default_excerpt_separator(),
        }
    }
}

fn default_date_format() -> String {
    "%Y-%m-%d".to_string()
}

fn default_template() -> String {
    "default.html".to_string()
}

fn default_excerpt_separator() -> String {
    "<!--more-->".to_string()
}

/// Development server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Port to listen on.
    #[serde(default = "default_port")]
    pub port: u16,
    /// Whether to open browser on start.
    #[serde(default)]
    pub open: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            open: false,
        }
    }
}

fn default_port() -> u16 {
    3000
}
