//! URL and content rewriting using regex.

use rustmax::prelude::*;
use rustmax::regex;
use rustmax::regex::{Regex, Captures};
use std::borrow::Cow;

/// URL rewriter for transforming links in content.
pub struct UrlRewriter {
    rules: Vec<RewriteRule>,
}

/// A single rewrite rule.
pub struct RewriteRule {
    pattern: Regex,
    replacement: String,
}

impl UrlRewriter {
    /// Create a new URL rewriter.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rewrite rule.
    ///
    /// The pattern is a regular expression, and the replacement can use
    /// capture groups like `$1`, `$2`, etc.
    pub fn add_rule(&mut self, pattern: &str, replacement: &str) -> Result<(), regex::Error> {
        let regex = Regex::new(pattern)?;
        self.rules.push(RewriteRule {
            pattern: regex,
            replacement: replacement.to_string(),
        });
        Ok(())
    }

    /// Apply all rules to rewrite URLs in content.
    pub fn rewrite(&self, content: &str) -> String {
        let mut result = content.to_string();
        for rule in &self.rules {
            result = rule.pattern.replace_all(&result, &rule.replacement).to_string();
        }
        result
    }
}

impl Default for UrlRewriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Rewrite relative URLs to absolute.
pub fn make_urls_absolute(content: &str, base_url: &str) -> String {
    // Pattern matches href="..." and src="..." attributes with relative URLs.
    let href_pattern = Regex::new(r#"(href|src)="(/[^"]*)""#).unwrap();

    href_pattern.replace_all(content, |caps: &Captures| {
        format!(r#"{}="{}{}""#, &caps[1], base_url.trim_end_matches('/'), &caps[2])
    }).to_string()
}

/// Rewrite internal markdown links to HTML links.
pub fn rewrite_md_links(content: &str) -> String {
    // Pattern matches [text](path.md) links.
    let md_link_pattern = Regex::new(r"\[([^\]]+)\]\(([^)]+)\.md\)").unwrap();

    md_link_pattern.replace_all(content, |caps: &Captures| {
        format!("[{}]({}/)", &caps[1], &caps[2])
    }).to_string()
}

/// Extract all URLs from content.
pub fn extract_urls(content: &str) -> Vec<String> {
    let url_pattern = Regex::new(r#"https?://[^\s<>"')\]]+[^\s<>"')\].,!?;:]"#).unwrap();

    url_pattern.find_iter(content)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Replace all occurrences of a pattern in content.
pub fn replace_pattern<'a>(content: &'a str, pattern: &str, replacement: &str) -> Cow<'a, str> {
    match Regex::new(pattern) {
        Ok(re) => re.replace_all(content, replacement),
        Err(_) => Cow::Borrowed(content),
    }
}

/// Validate that a string looks like a valid URL slug.
pub fn is_valid_slug(slug: &str) -> bool {
    let slug_pattern = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    slug_pattern.is_match(slug)
}

/// Transform a title into a URL-safe slug.
pub fn slugify(title: &str) -> String {
    // Remove accents and special characters, convert to lowercase.
    let cleaned: String = title
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect();

    // Collapse multiple dashes and trim.
    let dash_pattern = Regex::new(r"-+").unwrap();
    let result = dash_pattern.replace_all(&cleaned, "-");
    result.trim_matches('-').to_string()
}

/// Find all internal links (links starting with /).
pub fn find_internal_links(content: &str) -> Vec<String> {
    let link_pattern = Regex::new(r#"href="(/[^"]+)""#).unwrap();

    link_pattern.captures_iter(content)
        .map(|caps| caps[1].to_string())
        .collect()
}

/// Verify all internal links point to existing paths.
pub fn verify_links(content: &str, valid_paths: &[&str]) -> Vec<String> {
    let links = find_internal_links(content);

    links.into_iter()
        .filter(|link| {
            let normalized = link.trim_end_matches('/');
            !valid_paths.iter().any(|p| p.trim_end_matches('/') == normalized)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_rewriter() {
        let mut rewriter = UrlRewriter::new();
        rewriter.add_rule(r"/old/", "/new/").unwrap();
        rewriter.add_rule(r"example\.com", "example.org").unwrap();

        let content = r#"<a href="/old/page">Link</a>"#;
        let result = rewriter.rewrite(content);
        assert_eq!(result, r#"<a href="/new/page">Link</a>"#);

        let content = "Visit https://example.com/";
        let result = rewriter.rewrite(content);
        assert_eq!(result, "Visit https://example.org/");
    }

    #[test]
    fn test_make_urls_absolute() {
        let content = r#"<a href="/page">Link</a> <img src="/img/photo.jpg">"#;
        let result = make_urls_absolute(content, "https://example.com");
        assert_eq!(result, r#"<a href="https://example.com/page">Link</a> <img src="https://example.com/img/photo.jpg">"#);
    }

    #[test]
    fn test_rewrite_md_links() {
        let content = "See [other page](other.md) for details.";
        let result = rewrite_md_links(content);
        assert_eq!(result, "See [other page](other/) for details.");
    }

    #[test]
    fn test_extract_urls() {
        let content = "Visit https://example.com and http://test.org/page for more.";
        let urls = extract_urls(content);
        assert_eq!(urls.len(), 2);
        assert!(urls[0].contains("example.com"));
        assert!(urls[1].contains("test.org"));
    }

    #[test]
    fn test_is_valid_slug() {
        assert!(is_valid_slug("hello-world"));
        assert!(is_valid_slug("test"));
        assert!(is_valid_slug("my-post-123"));
        assert!(!is_valid_slug("Hello World"));
        assert!(!is_valid_slug("test_underscore"));
        assert!(!is_valid_slug("-leading-dash"));
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("My  Blog  Post"), "my-blog-post");
        assert_eq!(slugify("Test 123!"), "test-123");
        assert_eq!(slugify("--dashes--"), "dashes");
    }

    #[test]
    fn test_find_internal_links() {
        let content = r#"<a href="/page1">P1</a> <a href="https://ext.com">Ext</a> <a href="/page2">P2</a>"#;
        let links = find_internal_links(content);
        assert_eq!(links, vec!["/page1", "/page2"]);
    }

    #[test]
    fn test_verify_links() {
        let content = r#"<a href="/exists">OK</a> <a href="/missing">Bad</a>"#;
        let valid = vec!["/exists", "/other"];
        let broken = verify_links(content, &valid);
        assert_eq!(broken, vec!["/missing"]);
    }
}
