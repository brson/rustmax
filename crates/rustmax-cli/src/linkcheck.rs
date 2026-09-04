//! Link substitution validation.
//!
//! `src/linksubs.json5` maps `crate::` rustdoc link targets to paths in the
//! built site. The build substitutes them blindly, so an entry naming a page
//! that was never generated produces a dead link with no warning. This checks
//! each substitution against the built site.

use rmx::prelude::*;
use rmx::json5;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Why a substitution target was rejected.
#[derive(Debug)]
pub enum Failure {
    /// No such file under the site root.
    MissingPage { path: String },
    /// The page exists but has no element with the fragment's id.
    MissingAnchor { path: String, anchor: String },
}

/// A substitution whose target does not resolve.
#[derive(Debug)]
pub struct BadLink {
    /// The rustdoc link target being substituted, e.g. `crate::tar::Archive`.
    pub source: String,
    /// The substituted value.
    pub target: String,
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
            }
            println!("  substituting {} -> {}", bad.source, bad.target);
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
                failure: Failure::MissingAnchor {
                    path: path.to_string(),
                    anchor: anchor.to_string(),
                },
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
