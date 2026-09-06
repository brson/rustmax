//! Generated link validation.
//!
//! `src/linksubs.json5` maps `crate::` rustdoc link targets to paths in the
//! built site. The build substitutes them blindly, so an entry naming a page
//! that was never generated produces a dead link with no warning. This checks
//! each substitution against the built site.
//!
//! The search index has the same problem: `work/search-index.json` carries a
//! path per topic, most of them derived from the topic id, and a wrong
//! derivation is a search result that 404s. Those are checked here too.

use rmx::prelude::*;
use rmx::json5;
use rmx::serde::Deserialize;
use rmx::serde_json;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Why a link target was rejected.
#[derive(Debug)]
pub enum Failure {
    /// No such file under the site root.
    MissingPage { path: String },
    /// The page exists but has no element with the fragment's id.
    MissingAnchor { path: String, anchor: String },
    /// The entry names no page at all.
    NoPath,
}

/// What produced a link.
#[derive(Debug)]
pub enum Origin {
    /// An entry in `linksubs.json5`.
    LinkSub,
    /// An entry in the generated search index.
    SearchTopic,
}

/// A link whose target does not resolve.
#[derive(Debug)]
pub struct BadLink {
    /// What the link was written as: a rustdoc link target like
    /// `crate::tar::Archive`, or a topic id like `std-option`.
    pub source: String,
    /// The path the link resolves to.
    pub target: String,
    pub origin: Origin,
    pub failure: Failure,
}

/// The outcome of checking every substitution.
#[derive(Debug, Default)]
pub struct Report {
    pub checked: usize,
    pub external: usize,
    pub bad: Vec<BadLink>,
}

impl Report {
    pub fn is_ok(&self) -> bool {
        self.bad.is_empty()
    }

    pub fn print(&self) {
        for bad in &self.bad {
            match &bad.failure {
                Failure::MissingPage { path } => {
                    println!("no such page: {path}");
                }
                Failure::MissingAnchor { path, anchor } => {
                    println!("no anchor `{anchor}` in {path}");
                }
                Failure::NoPath => {
                    println!("no page for search topic `{}`", bad.source);
                }
            }
            match bad.origin {
                Origin::LinkSub => {
                    println!("  substituting {} -> {}", bad.source, bad.target);
                }
                Origin::SearchTopic => {
                    println!("  search topic {} -> {}", bad.source, bad.target);
                }
            }
        }

        println!(
            "{} links checked, {} external, {} broken",
            self.checked,
            self.external,
            self.bad.len(),
        );
    }
}

/// Check every substitution in `linksubs` against the site built at `site_dir`.
pub fn check(linksubs_file: &Path, site_dir: &Path) -> AnyResult<Report> {
    let text = fs::read_to_string(linksubs_file)
        .context(linksubs_file.display().to_string())?;
    let linksubs: BTreeMap<String, String> =
        json5::from_str(&text).context(linksubs_file.display().to_string())?;

    let mut report = Report::default();

    for (source, target) in &linksubs {
        // Links off the site are somebody else's to keep alive.
        if target.starts_with("http://") || target.starts_with("https://") {
            report.external += 1;
            continue;
        }

        report.checked += 1;

        let (path, anchor) = match target.split_once('#') {
            Some((path, anchor)) => (path, Some(anchor)),
            None => (target.as_str(), None),
        };

        let full_path = site_dir.join(path);
        if !full_path.is_file() {
            report.bad.push(BadLink {
                source: source.clone(),
                target: target.clone(),
                origin: Origin::LinkSub,
                failure: Failure::MissingPage { path: path.to_string() },
            });
            continue;
        }

        if let Some(anchor) = anchor
            && !has_anchor(&full_path, anchor)?
        {
            report.bad.push(BadLink {
                source: source.clone(),
                target: target.clone(),
                origin: Origin::LinkSub,
                failure: Failure::MissingAnchor {
                    path: path.to_string(),
                    anchor: anchor.to_string(),
                },
            });
        }
    }

    Ok(report)
}

/// The fields of a search index entry this check needs.
#[derive(Debug, Deserialize)]
struct SearchEntry {
    id: String,
    path: Option<String>,
}

/// Check every search index entry's path against the site built at `site_dir`.
///
/// Every topic is expected to link somewhere; a topic with no path is a
/// result the user can click and land nowhere, so it is reported too.
pub fn check_search_index(index_file: &Path, site_dir: &Path) -> AnyResult<Report> {
    let text = fs::read_to_string(index_file).context(index_file.display().to_string())?;
    let entries: Vec<SearchEntry> =
        serde_json::from_str(&text).context(index_file.display().to_string())?;

    let mut report = Report::default();

    for entry in &entries {
        report.checked += 1;

        let Some(path) = &entry.path else {
            report.bad.push(BadLink {
                source: entry.id.clone(),
                target: String::new(),
                origin: Origin::SearchTopic,
                failure: Failure::NoPath,
            });
            continue;
        };

        // A directory path is served by its index page.
        let full_path = if path.ends_with('/') {
            site_dir.join(path).join("index.html")
        } else {
            site_dir.join(path)
        };

        if !full_path.is_file() {
            report.bad.push(BadLink {
                source: entry.id.clone(),
                target: path.clone(),
                origin: Origin::SearchTopic,
                failure: Failure::MissingPage { path: path.clone() },
            });
        }
    }

    Ok(report)
}

/// Whether `page` contains an element with the given id.
fn has_anchor(page: &Path, anchor: &str) -> AnyResult<bool> {
    let html = fs::read_to_string(page).context(page.display().to_string())?;
    Ok(html.contains(&format!("id=\"{anchor}\"")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmx::tempfile;

    /// Build a site and a linksubs file to check against it.
    fn fixture(pages: &[(&str, &str)], linksubs: &str) -> (tempfile::TempDir, PathBuf, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let site = dir.path().join("out");

        for (path, html) in pages {
            let full = site.join(path);
            fs::create_dir_all(full.parent().unwrap()).unwrap();
            fs::write(&full, html).unwrap();
        }
        fs::create_dir_all(&site).unwrap();

        let linksubs_file = dir.path().join("linksubs.json5");
        fs::write(&linksubs_file, linksubs).unwrap();

        (dir, linksubs_file, site)
    }

    #[test]
    fn resolving_links_pass() {
        let (_dir, linksubs, site) = fixture(
            &[("api/tar/struct.Archive.html", "<h1>Archive</h1>")],
            r#"{ "crate::tar::Archive": "api/tar/struct.Archive.html" }"#,
        );

        let report = check(&linksubs, &site).unwrap();

        assert!(report.is_ok());
        assert_eq!(report.checked, 1);
    }

    #[test]
    fn a_page_that_was_never_generated_is_reported() {
        let (_dir, linksubs, site) = fixture(
            &[("api/tar/index.html", "<h1>tar</h1>")],
            r#"{ "crate::tar::Archive": "api/tar/struct.Archive.html" }"#,
        );

        let report = check(&linksubs, &site).unwrap();

        assert!(!report.is_ok());
        assert_eq!(report.bad.len(), 1);
        assert_eq!(report.bad[0].source, "crate::tar::Archive");
        assert!(matches!(report.bad[0].failure, Failure::MissingPage { .. }));
    }

    /// The case that motivated this check: `EntryType` is an enum, so the
    /// page is `enum.EntryType.html`, and a `struct.` guess silently 404s.
    #[test]
    fn a_wrong_item_kind_is_reported() {
        let (_dir, linksubs, site) = fixture(
            &[("api/tar/enum.EntryType.html", "<h1>EntryType</h1>")],
            r#"{ "crate::tar::EntryType": "api/tar/struct.EntryType.html" }"#,
        );

        let report = check(&linksubs, &site).unwrap();

        assert_eq!(report.bad.len(), 1);
    }

    #[test]
    fn a_present_anchor_passes_and_a_missing_one_is_reported() {
        let page = r#"<section id="method.unpack">unpack</section>"#;
        let linksubs = r#"{
            "crate::tar::Archive::unpack": "api/tar/struct.Archive.html#method.unpack",
            "crate::tar::Archive::unpack_in": "api/tar/struct.Archive.html#method.unpack_in",
        }"#;
        let (_dir, linksubs, site) =
            fixture(&[("api/tar/struct.Archive.html", page)], linksubs);

        let report = check(&linksubs, &site).unwrap();

        assert_eq!(report.checked, 2);
        assert_eq!(report.bad.len(), 1);
        assert_eq!(report.bad[0].source, "crate::tar::Archive::unpack_in");
        assert!(matches!(report.bad[0].failure, Failure::MissingAnchor { .. }));
    }

    /// Links off the site are somebody else's to keep alive.
    #[test]
    fn external_links_are_not_checked() {
        let (_dir, linksubs, site) = fixture(
            &[],
            r#"{ "crate::fern": "https://docs.rs/fern" }"#,
        );

        let report = check(&linksubs, &site).unwrap();

        assert!(report.is_ok());
        assert_eq!(report.checked, 0);
        assert_eq!(report.external, 1);
    }

    /// Build a site and a search index to check against it.
    fn search_fixture(pages: &[&str], index: &str) -> (tempfile::TempDir, PathBuf, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let site = dir.path().join("out");

        for path in pages {
            let full = site.join(path);
            fs::create_dir_all(full.parent().unwrap()).unwrap();
            fs::write(&full, "<h1>page</h1>").unwrap();
        }
        fs::create_dir_all(&site).unwrap();

        let index_file = dir.path().join("search-index.json");
        fs::write(&index_file, index).unwrap();

        (dir, index_file, site)
    }

    #[test]
    fn resolving_search_topics_pass() {
        let (_dir, index, site) = search_fixture(
            &["api/rustmax/tokio/index.html", "library/trpl/index.html"],
            r#"[
                { "id": "tokio", "path": "api/rustmax/tokio/index.html" },
                { "id": "trpl", "path": "library/trpl/" }
            ]"#,
        );

        let report = check_search_index(&index, &site).unwrap();

        assert!(report.is_ok());
        assert_eq!(report.checked, 2);
    }

    /// The case that motivated this check: std re-exports `option` from
    /// `core`, so there is no page under `api/std/` and the derived path
    /// silently 404s.
    #[test]
    fn a_search_topic_pointing_at_no_page_is_reported() {
        let (_dir, index, site) = search_fixture(
            &["api/core/option/index.html"],
            r#"[{ "id": "std-option", "path": "api/std/option/index.html" }]"#,
        );

        let report = check_search_index(&index, &site).unwrap();

        assert_eq!(report.bad.len(), 1);
        assert_eq!(report.bad[0].source, "std-option");
        assert!(matches!(report.bad[0].failure, Failure::MissingPage { .. }));
    }

    #[test]
    fn a_search_topic_with_no_path_is_reported() {
        let (_dir, index, site) = search_fixture(&[], r#"[{ "id": "orphan" }]"#);

        let report = check_search_index(&index, &site).unwrap();

        assert_eq!(report.bad.len(), 1);
        assert!(matches!(report.bad[0].failure, Failure::NoPath));
    }

    /// A directory path is served by its index page, not by the directory.
    #[test]
    fn a_search_topic_directory_without_an_index_page_is_reported() {
        let (_dir, index, site) = search_fixture(
            &["library/trpl/ch01.html"],
            r#"[{ "id": "trpl", "path": "library/trpl/" }]"#,
        );

        let report = check_search_index(&index, &site).unwrap();

        assert_eq!(report.bad.len(), 1);
    }

    #[test]
    fn a_directory_is_not_a_page() {
        let (_dir, linksubs, site) = fixture(
            &[("api/tar/index.html", "<h1>tar</h1>")],
            r#"{ "crate::tar": "api/tar" }"#,
        );

        let report = check(&linksubs, &site).unwrap();

        assert_eq!(report.bad.len(), 1);
    }
}
