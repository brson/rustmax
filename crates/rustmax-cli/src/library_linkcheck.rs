//! Link checking for the rendered library.
//!
//! Each book in `work/library` is checked for links within the site, from
//! chapter content, that name a missing file or a missing anchor. Links off
//! the site are not followed.
//!
//! Upstream books carry some broken links of their own, so this reports
//! rather than fails. Compare the counts before and after a change to the
//! renderer or a bump of the pins.

use rmx::prelude::*;
use rmx::regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::LazyLock;

static ARTICLE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<article>(.*)</article>").unwrap());
static LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<(?:a|img|source|iframe)\b[^>]*?\b(?:href|src)="([^"]*)""#).unwrap());
static ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\b(?:id|name)="([^"]*)""#).unwrap());
static SCHEME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9+.-]*:").unwrap());

/// A link that doesn't resolve.
pub struct BadLink {
    pub page: String,
    pub link: String,
    pub missing_anchor: bool,
}

/// The links of one book.
#[derive(Default)]
pub struct BookReport {
    pub checked: usize,
    pub bad: Vec<BadLink>,
}

/// Check every book under `library_dir`.
pub fn check(library_dir: &Path) -> AnyResult<BTreeMap<String, BookReport>> {
    let mut pages = Vec::new();
    collect_pages(library_dir, &mut pages)?;

    let mut ids: HashMap<PathBuf, HashSet<String>> = HashMap::new();
    let mut contents = Vec::new();
    for page in pages {
        let html = fs::read_to_string(&page)?;
        let page_ids = ID_RE.captures_iter(&html).map(|c| c[1].to_string()).collect();
        ids.insert(page.clone(), page_ids);
        contents.push((page, html));
    }

    let mut reports: BTreeMap<String, BookReport> = BTreeMap::new();
    for (page, html) in &contents {
        let rel = page.strip_prefix(library_dir).expect("pages are under the library");
        let book = rel.components().next().expect("pages are in a book");
        let book = book.as_os_str().to_string_lossy().to_string();
        let report = reports.entry(book).or_default();

        let Some(article) = ARTICLE_RE.captures(html) else {
            // Redirect pages have no content.
            continue;
        };

        for caps in LINK_RE.captures_iter(&article[1]) {
            let link = caps[1].replace("&amp;", "&");
            if link.is_empty() || link.starts_with("//") || SCHEME_RE.is_match(&link) {
                continue;
            }
            report.checked += 1;

            let (path, fragment) = match link.split_once('#') {
                Some((path, fragment)) => (path, Some(fragment)),
                None => (link.as_str(), None),
            };
            let path = path.split('?').next().expect("split always yields one part");

            let target = if path.is_empty() {
                page.clone()
            } else {
                let joined = normalize(&page.parent().expect("a page has a parent").join(path));
                if joined.is_dir() { joined.join("index.html") } else { joined }
            };

            if !target.exists() {
                report.bad.push(BadLink { page: rel.display().to_string(), link, missing_anchor: false });
                continue;
            }

            // A bare `#` is the top of the page.
            let fragment = fragment.filter(|f| !f.is_empty());
            if let (Some(fragment), Some(target_ids)) = (fragment, ids.get(&target)) {
                let decoded = percent_decode(fragment);
                if !target_ids.contains(fragment) && !target_ids.contains(&decoded) {
                    report.bad.push(BadLink { page: rel.display().to_string(), link, missing_anchor: true });
                }
            }
        }
    }

    Ok(reports)
}

fn collect_pages(dir: &Path, pages: &mut Vec<PathBuf>) -> AnyResult<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_pages(&path, pages)?;
        } else if path.extension().is_some_and(|ext| ext == "html") {
            pages.push(path);
        }
    }
    Ok(())
}

/// Resolve `.` and `..` without touching the filesystem.
fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    PathBuf::from(percent_decode(&out.to_string_lossy()))
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && let Some(hex) = s.get(i + 1..i + 3)
            && let Ok(byte) = u8::from_str_radix(hex, 16)
        {
            out.push(byte);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(body: &str) -> String {
        format!("<html><nav><a href=\"nowhere.html\">x</a></nav><article>{body}</article></html>")
    }

    #[test]
    fn test_library_links() {
        let dir = rmx::tempfile::tempdir().unwrap();
        let book = dir.path().join("book");
        fs::create_dir_all(book.join("sub")).unwrap();
        fs::create_dir_all(dir.path().join("other")).unwrap();
        fs::write(dir.path().join("other/index.html"), page("<h2 id=\"there\">x</h2>")).unwrap();
        fs::write(book.join("img.png"), "").unwrap();
        fs::write(
            book.join("sub/a.html"),
            page(
                "<h2 id=\"here\">h</h2>\
                 <a href=\"#here\">ok</a> <a href=\"#\">top</a> <a href=\"#gone\">bad anchor</a>\
                 <a href=\"../img.png\">ok</a> <img src=\"../missing.png\">\
                 <a href=\"../../other/#there\">ok</a> <a href=\"../../other/index.html#nope\">bad</a>\
                 <a href=\"https://example.com/x\">external</a>",
            ),
        )
        .unwrap();

        let reports = check(dir.path()).unwrap();
        let report = &reports["book"];
        assert_eq!(report.checked, 7);
        let bad: Vec<(&str, bool)> =
            report.bad.iter().map(|b| (b.link.as_str(), b.missing_anchor)).collect();
        assert_eq!(
            bad,
            [("#gone", true), ("../missing.png", false), ("../../other/index.html#nope", true)]
        );
    }
}
