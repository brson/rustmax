//! Production analytics.
//!
//! The site is assembled from several generators: the static `www` pages,
//! pipeliner's news posts, rmxbook, the library books, the code browser
//! and rmxdoc. Rather than teach each of them about deployment, the
//! production build adds the tracker to every page of the finished site
//! in one pass.

use rmx::prelude::*;
use std::fs;
use std::path::Path;

const TRACKER: &str = r#"<script data-goatcounter="https://rustmax.goatcounter.com/count"
        async src="//gc.zgo.at/count.js"></script>
"#;

/// Add the tracker to every HTML page under `site`, returning the number of pages changed.
///
/// HTML files without an `<html` tag are fragments that pages load into
/// themselves, and are left alone.
pub fn add_to_site(site: &Path) -> AnyResult<usize> {
    let mut count = 0;
    add_to_dir(site, &mut count)?;
    Ok(count)
}

fn add_to_dir(dir: &Path, count: &mut usize) -> AnyResult<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            add_to_dir(&path, count)?;
        } else if path.extension().is_some_and(|ext| ext == "html") {
            let html = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
            if let Some(html) = add_to_page(&html).with_context(|| format!("in {}", path.display()))? {
                fs::write(&path, html).with_context(|| format!("writing {}", path.display()))?;
                *count += 1;
            }
        }
    }
    Ok(())
}

/// Insert the tracker before `</head>`, or return `None` for a fragment.
fn add_to_page(html: &str) -> AnyResult<Option<String>> {
    if !html.contains("<html") {
        return Ok(None);
    }
    if html.contains(TRACKER) {
        bail!("page already has the tracker");
    }
    let Some(head_end) = html.find("</head>") else {
        bail!("page has no </head>");
    };
    let mut out = String::with_capacity(html.len() + TRACKER.len());
    out.push_str(&html[..head_end]);
    out.push_str(TRACKER);
    out.push_str(&html[head_end..]);
    Ok(Some(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_goes_at_the_end_of_head() {
        let page = "<!DOCTYPE html>\n<html>\n<head>\n<title>x</title>\n</head>\n<body></body>\n</html>\n";
        let html = add_to_page(page).unwrap().unwrap();
        assert!(html.contains(&format!("<title>x</title>\n{TRACKER}</head>")), "{html}");
    }

    #[test]
    fn test_fragments_are_left_alone() {
        assert!(add_to_page("<div id=\"search\"></div>\n").unwrap().is_none());
    }

    #[test]
    fn test_page_without_head_is_an_error() {
        assert!(add_to_page("<html><body></body></html>").is_err());
    }

    #[test]
    fn test_adding_twice_is_an_error() {
        let html = add_to_page("<html><head></head></html>").unwrap().unwrap();
        assert!(add_to_page(&html).is_err());
    }
}
