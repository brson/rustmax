//! What the pages of a built collection can link to.
//!
//! [`SiteIndex`] knows every page and file a build produces,
//! and resolves a link the way a browser would,
//! against the URL of the page it appears on (with `url`).
//! The build uses it to turn `[text](other.md)` links into links to the
//! other document's page and to give local images their dimensions;
//! `folio check` uses it to report links and images that go nowhere.

use rmx::prelude::*;
use rmx::regex::{Captures, Regex};
use rmx::url::Url;
use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};
use std::sync::LazyLock;

use crate::collection::{Collection, Config, Document};
use crate::features::BuildFeatures;
use crate::Result;

/// Where a link leads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// Another site, or another scheme such as `mailto:`. Not checked.
    External,
    /// A fragment of the page the link is on.
    SamePage,
    /// A document's page, at `url`.
    Document { url: String },
    /// A draft document's page, which this build does not write.
    Draft { url: String },
    /// Some other page or file the build writes.
    Built,
    /// A file copied from `static/`.
    Static(PathBuf),
    /// Nothing the build writes.
    Missing,
}

#[derive(Debug, Clone)]
struct DocumentPage {
    url: String,
    built: bool,
}

/// Every page and file a build of a collection produces.
#[derive(Debug, Clone)]
pub struct SiteIndex {
    content_dir: PathBuf,
    static_dir: PathBuf,
    /// Document pages, by normalized source path.
    documents: HashMap<PathBuf, DocumentPage>,
    /// Built page paths, without a trailing slash; the index page is `""`.
    pages: HashSet<String>,
    /// Pages of drafts this build leaves out, keyed like `pages`.
    draft_pages: HashSet<String>,
    /// Paths relative to the output directory of everything else written.
    files: HashSet<String>,
}

impl SiteIndex {
    /// Index what building `collection` with `config` would produce.
    pub fn new(collection: &Collection, config: &Config) -> Result<Self> {
        let drafts_built = config.build.features.contains(BuildFeatures::DRAFTS);
        let content_dir = normalize(&collection.root.join("content"));
        let static_dir = collection.root.join("static");

        let mut documents = HashMap::new();
        let mut pages = HashSet::from([String::new()]);
        if config.build.features.contains(BuildFeatures::SEARCH) {
            pages.insert("/search".to_string());
        }
        let mut draft_pages = HashSet::new();

        for doc in &collection.documents {
            let url = doc.url_path();
            let built = drafts_built || !doc.frontmatter.draft;
            let key = page_key(&url).to_string();
            if built {
                pages.insert(key);
                for tag in &doc.frontmatter.tags {
                    pages.insert(format!("/tags/{tag}"));
                }
            } else {
                draft_pages.insert(key);
            }
            documents.insert(normalize(&doc.source_path), DocumentPage { url, built });
        }

        let mut files: HashSet<String> = crate::build::generated_files(config)
            .into_iter()
            .map(str::to_string)
            .collect();

        if static_dir.is_dir() {
            for entry in rmx::walkdir::WalkDir::new(&static_dir) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    let relative = entry.path().strip_prefix(&static_dir).expect("walkdir yields paths under its root");
                    files.insert(url_path_of(relative));
                }
            }
        }

        Ok(Self { content_dir, static_dir, documents, pages, draft_pages, files })
    }

    /// Where `href`, written in `doc`, leads.
    pub fn resolve(&self, doc: &Document, href: &str) -> Target {
        let href = href.trim();
        if href.is_empty() || href.starts_with('#') {
            return Target::SamePage;
        }
        if href.starts_with("//") || Url::parse(href).is_ok() {
            return Target::External;
        }

        let path = href.split(['#', '?']).next().unwrap_or("");
        if path.ends_with(".md") || path.ends_with(".markdown") {
            return self.resolve_source(doc, path);
        }

        let page = SITE.join(&doc.url_path()).expect("document URLs are paths");
        let Ok(url) = page.join(href) else {
            return Target::Missing;
        };
        let path = percent_decode(url.path());
        let key = page_key(&path);

        if self.pages.contains(key) {
            return Target::Built;
        }
        if self.draft_pages.contains(key) {
            return Target::Draft { url: format!("{key}/") };
        }
        if let Some(dir) = key.strip_suffix("/index.html")
            && self.pages.contains(dir)
        {
            return Target::Built;
        }

        let file = path.trim_start_matches('/');
        if self.files.contains(file) {
            let in_static = self.static_dir.join(file);
            return if in_static.is_file() { Target::Static(in_static) } else { Target::Built };
        }

        Target::Missing
    }

    /// Resolve a link to a markdown source file, relative to `doc`'s source,
    /// or to `content/` if it starts with `/`.
    fn resolve_source(&self, doc: &Document, path: &str) -> Target {
        let path = percent_decode(path);
        let base = match path.strip_prefix('/') {
            Some(_) => self.content_dir.clone(),
            None => normalize(doc.source_path.parent().unwrap_or(Path::new(""))),
        };
        let target = normalize(&base.join(path.trim_start_matches('/')));

        match self.documents.get(&target) {
            Some(page) if page.built => Target::Document { url: page.url.clone() },
            Some(page) => Target::Draft { url: page.url.clone() },
            None => Target::Missing,
        }
    }

    /// Point every `.md` link in a rendered page at the document's page.
    ///
    /// A link that resolves to nothing is left as written; `folio check`
    /// reports it.
    pub fn rewrite_source_links(&self, doc: &Document, html: &str) -> String {
        static HREF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r##"href="([^"#?]+\.(?:md|markdown))((?:[#?][^"]*)?)""##).expect("valid regex"));

        HREF.replace_all(html, |caps: &Captures| {
            let href = caps[1].replace("&amp;", "&");
            match self.resolve(doc, &href) {
                Target::Document { url } => format!(r#"href="{url}{}""#, &caps[2]),
                _ => caps[0].to_string(),
            }
        })
        .into_owned()
    }

    /// Give every local image in a rendered page its `width` and `height`,
    /// so the browser can lay the page out before the image arrives.
    ///
    /// Images that already have a width, are not in `static/`,
    /// or cannot be read as a raster image (SVGs among them) are left alone.
    pub fn add_image_dimensions(&self, doc: &Document, html: &str) -> String {
        static IMG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<img\s([^>]*?)(\s*/?)>"#).expect("valid regex"));
        static SRC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\bsrc="([^"]*)""#).expect("valid regex"));

        IMG.replace_all(html, |caps: &Captures| {
            let attrs = &caps[1];
            let dimensions = (!attrs.contains("width="))
                .then(|| SRC.captures(attrs))
                .flatten()
                .and_then(|src| match self.resolve(doc, &src[1].replace("&amp;", "&")) {
                    Target::Static(path) => rmx::image::image_dimensions(path).ok(),
                    _ => None,
                });

            match dimensions {
                Some((width, height)) => format!(r#"<img {attrs} width="{width}" height="{height}"{}>"#, &caps[2]),
                None => caps[0].to_string(),
            }
        })
        .into_owned()
    }
}

/// The origin pages are resolved against. Only the path matters.
static SITE: LazyLock<Url> = LazyLock::new(|| Url::parse("http://folio.invalid/").expect("valid URL"));

/// A page path without its trailing slash, so `/a/` and `/a` match.
fn page_key(path: &str) -> &str {
    path.trim_end_matches('/')
}

/// A relative filesystem path as a URL path, with `/` separators.
fn url_path_of(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
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
    out
}

/// Decode `%XX` escapes in a URL path.
///
/// An escape that does not decode to UTF-8 is kept as written.
fn percent_decode(path: &str) -> String {
    let bytes = path.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let escape = (bytes[i] == b'%')
            .then(|| bytes.get(i + 1..i + 3))
            .flatten()
            .and_then(|hex| u8::from_str_radix(std::str::from_utf8(hex).ok()?, 16).ok());
        match escape {
            Some(byte) => {
                out.push(byte);
                i += 3;
            }
            None => {
                out.push(bytes[i]);
                i += 1;
            }
        }
    }
    String::from_utf8(out).unwrap_or_else(|_| path.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmx::tempfile::tempdir;
    use std::fs;

    fn write(root: &Path, path: &str, text: &str) {
        let path = root.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    fn doc(title: &str, extra: &str) -> String {
        format!("---\ntitle = \"{title}\"\ntags = [\"rust\"]\n{extra}---\n\nText.\n")
    }

    /// A collection with nested documents, a custom slug, a draft and a static image.
    fn collection() -> (rmx::tempfile::TempDir, Collection, Config) {
        let dir = tempdir().unwrap();
        let root = dir.path();
        write(root, "folio.toml", "");
        write(root, "content/intro.md", &doc("Intro", ""));
        write(root, "content/guide/setup.md", &doc("Setup", "slug = \"getting-started\"\n"));
        write(root, "content/wip.md", &doc("WIP", "draft = true\n"));

        let png = rmx::image::RgbImage::new(3, 2);
        fs::create_dir_all(root.join("static/img")).unwrap();
        png.save(root.join("static/img/dot.png")).unwrap();

        let config = Config::load(root).unwrap();
        let collection = Collection::load(root, &config).unwrap();
        (dir, collection, config)
    }

    fn find<'a>(collection: &'a Collection, title: &str) -> &'a Document {
        collection.documents.iter().find(|d| d.frontmatter.title == title).unwrap()
    }

    #[test]
    fn markdown_links_resolve_against_the_source_file() {
        let (_dir, collection, config) = collection();
        let site = SiteIndex::new(&collection, &config).unwrap();
        let intro = find(&collection, "Intro");
        let setup = find(&collection, "Setup");

        assert_eq!(site.resolve(intro, "guide/setup.md"), Target::Document { url: "/getting-started/".into() });
        assert_eq!(site.resolve(setup, "../intro.md#top"), Target::Document { url: "/intro/".into() });
        assert_eq!(site.resolve(setup, "/intro.md"), Target::Document { url: "/intro/".into() });
        assert_eq!(site.resolve(intro, "wip.md"), Target::Draft { url: "/wip/".into() });
        assert_eq!(site.resolve(intro, "setup.md"), Target::Missing);
    }

    #[test]
    fn page_links_resolve_against_the_page_url() {
        let (_dir, collection, config) = collection();
        let site = SiteIndex::new(&collection, &config).unwrap();
        let intro = find(&collection, "Intro");

        assert_eq!(site.resolve(intro, "/getting-started/"), Target::Built);
        assert_eq!(site.resolve(intro, "../getting-started"), Target::Built);
        assert_eq!(site.resolve(intro, "/"), Target::Built);
        assert_eq!(site.resolve(intro, "/tags/rust/"), Target::Built);
        assert_eq!(site.resolve(intro, "/rss.xml"), Target::Built);
        assert_eq!(site.resolve(intro, "/search/"), Target::Built);
        assert_eq!(site.resolve(intro, "/intro/index.html"), Target::Built);
        assert_eq!(site.resolve(intro, "/wip/"), Target::Draft { url: "/wip/".into() });
        assert_eq!(site.resolve(intro, "/nowhere/"), Target::Missing);
        assert_eq!(site.resolve(intro, "#section"), Target::SamePage);
        assert_eq!(site.resolve(intro, "https://example.com/x"), Target::External);
        assert_eq!(site.resolve(intro, "mailto:a@example.com"), Target::External);
        assert_eq!(site.resolve(intro, "//cdn.example.com/x.js"), Target::External);
    }

    #[test]
    fn static_files_resolve_to_their_source() {
        let (dir, collection, config) = collection();
        let site = SiteIndex::new(&collection, &config).unwrap();
        let intro = find(&collection, "Intro");

        let expected = Target::Static(dir.path().join("static/img/dot.png"));
        assert_eq!(site.resolve(intro, "/img/dot.png"), expected);
        assert_eq!(site.resolve(intro, "../img/dot.png"), expected);
        assert_eq!(site.resolve(intro, "/img/%64ot.png"), expected);
        assert_eq!(site.resolve(intro, "img/dot.png"), Target::Missing);
    }

    #[test]
    fn drafts_are_pages_when_drafts_are_built() {
        let (_dir, collection, mut config) = collection();
        config.build.features |= BuildFeatures::DRAFTS;
        let site = SiteIndex::new(&collection, &config).unwrap();
        let intro = find(&collection, "Intro");

        assert_eq!(site.resolve(intro, "wip.md"), Target::Document { url: "/wip/".into() });
        assert_eq!(site.resolve(intro, "/wip/"), Target::Built);
    }

    #[test]
    fn source_links_are_rewritten_in_rendered_pages() {
        let (_dir, collection, config) = collection();
        let site = SiteIndex::new(&collection, &config).unwrap();
        let intro = find(&collection, "Intro");

        let html = r#"<a href="guide/setup.md#install">Setup</a> <a href="missing.md">x</a> <a href="/intro/">y</a>"#;
        assert_eq!(
            site.rewrite_source_links(intro, html),
            r#"<a href="/getting-started/#install">Setup</a> <a href="missing.md">x</a> <a href="/intro/">y</a>"#,
        );
    }

    #[test]
    fn local_images_get_their_dimensions() {
        let (_dir, collection, config) = collection();
        let site = SiteIndex::new(&collection, &config).unwrap();
        let intro = find(&collection, "Intro");

        let html = r#"<img src="/img/dot.png" alt="a" /><img src="/img/dot.png" width="9"><img src="https://x.example/a.png">"#;
        assert_eq!(
            site.add_image_dimensions(intro, html),
            r#"<img src="/img/dot.png" alt="a" width="3" height="2" /><img src="/img/dot.png" width="9"><img src="https://x.example/a.png">"#,
        );
    }

    #[test]
    fn paths_normalize_and_decode() {
        assert_eq!(normalize(Path::new("a/b/../c/./d.md")), PathBuf::from("a/c/d.md"));
        assert_eq!(percent_decode("/a%20b/%E2%9C%93"), "/a b/✓");
        assert_eq!(percent_decode("/100%"), "/100%");
    }
}
