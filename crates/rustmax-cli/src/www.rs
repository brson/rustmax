//! The rustmax.net home page.
//!
//! Copies `www/` and generates the parts of the page that come from the
//! repo's metadata: the crate table, the news posts, and the build info.

mod news;

use rmx::prelude::*;
use rmx::{json5, serde, serde_json, tera};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::highlight::{HighlightAdapter, Highlighter};

const CRATES_META: &str = "src/crates.json5";
const MAINTAINERS_META: &str = "src/maintainers.json5";
const RMX_MANIFEST: &str = "crates/rustmax/Cargo.toml";
const EXAMPLES_DIR: &str = "crates/rustmax/doc-src";
const LINK_SUBS: &str = "src/linksubs.json5";
const POSTS_DIR: &str = "src/posts";
const NEWS_DIR: &str = "src/news";
const WWW_DIR: &str = "www";

const LATEST_POST_TEMPLATE: &str = "src/latest-post.template.html";

// Pipeliner clears its output directory, so the news is staged here.
const NEWS_STAGING_DIR: &str = "work/news";

// Monochrome inline icons, drawn in `currentColor` so CSS can theme them.
// Shapes are from Lucide (ISC license).
const ICON_BOOK_CLOSED: &str = "<svg class='icon icon-book-closed' viewBox='0 0 24 24' aria-hidden='true'><path d='M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20'/></svg>";
const ICON_BOOK_OPEN: &str = "<svg class='icon icon-book-open' viewBox='0 0 24 24' aria-hidden='true'><path d='M12 7v14'/><path d='M3 18a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5a4 4 0 0 1 4 4 4 4 0 0 1 4-4h5a1 1 0 0 1 1 1v13a1 1 0 0 1-1 1h-6a3 3 0 0 0-3 3 3 3 0 0 0-3-3z'/></svg>";
const ICON_USER: &str = "<svg class='icon icon-user' viewBox='0 0 24 24' aria-hidden='true'><circle cx='12' cy='8' r='5'/><path d='M20 21a8 8 0 0 0-16 0'/></svg>";

#[derive(Debug)]
struct CrateInfo {
    name: String,
    version: String,
    short_desc: String,
    example: String,
    maintainer: Option<MaintainerInfo>,
}

#[derive(Debug, Clone)]
struct MaintainerInfo {
    id: String,
    display_name: String,
}

mod meta {
    use rmx::serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Crates {
        pub crates: Vec<Crate>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Crate {
        pub name: String,
        pub category: String,
        pub short_desc: String,
        #[serde(default)]
        pub maintainer: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Maintainers {
        pub maintainers: Vec<Maintainer>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Maintainer {
        pub id: String,
        pub display_name: String,
        pub description: String,
        pub url: String,
    }
}

/// Build the home page into `out_dir`, run from the repo root.
pub fn build(out_dir: &Path) -> AnyResult<()> {
    let crates_meta: meta::Crates = read_json5(CRATES_META)?;
    let maintainers_meta: meta::Maintainers = read_json5(MAINTAINERS_META)?;
    let link_subs: BTreeMap<String, String> = read_json5(LINK_SUBS)?;
    let rmx_manifest: toml::Value = toml::from_str(&read(RMX_MANIFEST)?)
        .with_context(|| format!("parsing {RMX_MANIFEST}"))?;
    let examples_dir = fs::read_dir(EXAMPLES_DIR).with_context(|| format!("reading {EXAMPLES_DIR}"))?;

    let crate_info = build_crate_info(&crates_meta, &maintainers_meta, &rmx_manifest, examples_dir)?;
    let highlighter = Highlighter::new();

    fs::create_dir_all(out_dir)?;
    crate::books::copy_dir_contents(Path::new(WWW_DIR), out_dir)?;

    write(&out_dir.join("crates.html"), &make_crate_table(&crate_info, &link_subs, &highlighter))?;
    write(&out_dir.join("build-info.json"), &make_build_info()?)?;

    let news_dir = Path::new(NEWS_STAGING_DIR);
    news::build_posts(Path::new(NEWS_DIR), Path::new(POSTS_DIR), Path::new(WWW_DIR), news_dir)?;
    crate::books::copy_dir_contents(news_dir, out_dir)?;

    let mut tera = tera::Tera::new();
    tera.add_template_file(LATEST_POST_TEMPLATE, Some("latest-post"))?;
    news::generate_latest_post(&news_dir.join("posts.json"), &tera, &out_dir.join("latest-post.html"))?;

    Ok(())
}

fn read(path: &str) -> AnyResult<String> {
    fs::read_to_string(path).with_context(|| format!("reading {path}"))
}

fn read_json5<T: serde::de::DeserializeOwned>(path: &str) -> AnyResult<T> {
    json5::from_str(&read(path)?).with_context(|| format!("parsing {path}"))
}

fn write(path: &Path, contents: &str) -> AnyResult<()> {
    fs::write(path, contents)?;
    eprintln!("wrote {}", path.display());
    Ok(())
}

fn build_crate_info(
    crates_meta: &meta::Crates,
    maintainers_meta: &meta::Maintainers,
    rmx_manifest: &toml::Value,
    examples_dir: fs::ReadDir,
) -> AnyResult<Vec<CrateInfo>> {
    let manifest_crate_info = get_manifest_crate_info(rmx_manifest)?;
    let examples = get_examples(examples_dir)?;

    let mut infos = Vec::new();

    for crate_ in &manifest_crate_info {
        let meta = crates_meta
            .crates
            .iter()
            .find(|c| c.name == crate_.name)
            .ok_or(A!("missing crate meta for {}", crate_.name))?;

        let example = examples
            .iter()
            .find(|c| c.name == crate_.name)
            .map(|ce| ce.text.to_string())
            .unwrap_or_default();

        let maintainer = meta.maintainer.as_ref().and_then(|maintainer_id| {
            maintainers_meta.maintainers.iter().find(|m| &m.id == maintainer_id).map(|m| {
                MaintainerInfo {
                    id: m.id.clone(),
                    display_name: m.display_name.clone(),
                }
            })
        });

        infos.push(CrateInfo {
            name: crate_.name.to_string(),
            version: crate_.version.to_string(),
            short_desc: meta.short_desc.to_string(),
            example,
            maintainer,
        });
    }

    for crate_ in &crates_meta.crates {
        let _ = manifest_crate_info
            .iter()
            .find(|c| c.name == crate_.name)
            .ok_or(A!("unused crate meta for {}", crate_.name))?;
    }

    for crate_ in &examples {
        let _ = manifest_crate_info
            .iter()
            .find(|c| c.name == crate_.name)
            .ok_or(A!("unused example for {}", crate_.name))?;
    }

    Ok(infos)
}

#[derive(Debug)]
struct ManifestCrate {
    name: String,
    version: String,
}

fn get_manifest_crate_info(manifest: &toml::Value) -> AnyResult<Vec<ManifestCrate>> {
    let deps = manifest
        .as_table()
        .ok_or(A!("toml: manifest table"))?
        .get("dependencies")
        .ok_or(A!("toml: dependencies"))?
        .as_table()
        .ok_or(A!("toml: dependencies table"))?;

    deps.iter()
        // Ignore path dependencies.
        // These are rustmax's own workspace crates, not curated crates.
        .filter(|(_, dep)| dep.get("path").is_none())
        .map(|(name, dep)| {
            let version = dep
                .as_table()
                .ok_or(A!("toml: dep table"))?
                .get("version")
                .ok_or(A!("toml: dep version"))?
                .as_str()
                .ok_or(A!("toml: dep version string"))?;
            Ok(ManifestCrate {
                name: name.to_owned(),
                version: version.to_owned(),
            })
        })
        .filter_map(|crate_| match crate_ {
            Ok(crate_) => {
                // Ignore underscore-prefixed crates.
                // These are used for internal purposes.
                if !crate_.name.starts_with("_") {
                    Some(Ok(crate_))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        })
        .collect()
}

#[derive(Debug)]
struct CrateExample {
    name: String,
    text: String,
}

fn get_examples(examples_dir: fs::ReadDir) -> AnyResult<Vec<CrateExample>> {
    let crate_name_regex = regex::Regex::new(r"^crate-(?<name>[a-zA-Z0-9_-]+)\.md$").expect(".");

    let mut examples = Vec::new();

    for dir_entry in examples_dir {
        let dir_entry = dir_entry?;
        let filename = dir_entry.path();
        let filename = filename
            .file_name()
            .ok_or(A!("file name"))?
            .to_str()
            .ok_or(A!("file name"))?;

        let Some(captures) = crate_name_regex.captures(filename) else {
            continue;
        };

        let name = captures.name("name").expect(".");
        let text = fs::read_to_string(dir_entry.path())?;

        examples.push(CrateExample {
            name: name.as_str().to_string(),
            text,
        });
    }

    Ok(examples)
}

fn make_crate_table(
    crates: &[CrateInfo],
    link_subs: &BTreeMap<String, String>,
    highlighter: &Highlighter,
) -> String {
    let mut html = String::new();

    html.push_str("<table id='rmx-crate-table'>\n");
    html.push_str("<thead>\n");
    html.push_str("<th></th>\n");
    html.push_str("<th>Crate</th>\n");
    html.push_str("<th>Feature</th>\n");
    html.push_str("<th>Ex</th>\n");
    html.push_str("</thead>\n");

    for (i, krate) in crates.iter().enumerate() {
        let api_link = format!(
            "api/rustmax/{}/index.html",
            krate.name.replace("-", "_"),
        );
        let example_html = render_example(krate, link_subs, crates, highlighter);

        html.push_str(&format!(
            "<tr class='{}'>\n",
            if i % 2 == 0 { "row-even" } else { "row-odd" }
        ));

        let maintainer_badge = if let Some(ref maintainer) = krate.maintainer {
            format!(
                "<span class='maintainer-badge'><a href='book/maintainers.html#{}' title='Trusted maintainer: {}' aria-label='Trusted maintainer: {}'>{}</a></span>",
                maintainer.id, maintainer.display_name, maintainer.display_name, ICON_USER,
            )
        } else {
            String::new()
        };

        html.push_str(&format!("<td>{}</td>\n", maintainer_badge));
        html.push_str(&format!(
            "<td><a href='{}'><code>{} = \"{}\"</code></a></td>\n",
            api_link, krate.name, krate.version,
        ));
        html.push_str(&format!("<td>{}</td>\n", krate.short_desc));
        if example_html.is_some() {
            html.push_str(&format!(
                "<td><button id='button-{}' class='example-button' data-name='{}' type='button' aria-expanded='false' aria-label='Toggle {} example'>{}{}</button></td>\n",
                krate.name,
                krate.name,
                krate.name,
                ICON_BOOK_CLOSED,
                ICON_BOOK_OPEN,
            ));
        } else {
            html.push_str("<td></td>");
        }
        html.push_str("</tr>\n");
        {
            if let Some(example_html) = example_html {
                html.push_str(&format!(
                    "<tr id='example-row-{}' class='example-row {}'>\n",
                    krate.name,
                    if i % 2 == 0 { "row-even" } else { "row-odd" }
                ));
                html.push_str("<td colspan=4>\n");
                html.push_str(&example_html);
                html.push_str("</td>\n");
                html.push_str("</tr>\n");
            }
        }
    }

    html.push_str("</thead>\n");

    html
}

fn render_example(
    krate: &CrateInfo,
    link_subs: &BTreeMap<String, String>,
    crates: &[CrateInfo],
    highlighter: &Highlighter,
) -> Option<String> {
    if !krate.example.is_empty() {
        let md = process_md(&krate.example, link_subs, crates);

        // Use comrak with syntax highlighting.
        let adapter = HighlightAdapter { highlighter };
        let plugins = comrak::options::Plugins {
            render: comrak::options::RenderPlugins {
                codefence_syntax_highlighter: Some(&adapter),
                ..Default::default()
            },
        };
        let html = comrak::markdown_to_html_with_plugins(&md, &Default::default(), &plugins);
        Some(html)
    } else {
        None
    }
}

fn process_md(md: &str, link_subs: &BTreeMap<String, String>, crates: &[CrateInfo]) -> String {
    let md = remove_crate_link(md);
    let md = substitute_links(&md, link_subs);
    let md = substitute_versions(&md, crates);
    md
}

fn remove_crate_link(md: &str) -> String {
    let re = regex::Regex::new("^- Crate \\[").expect(".");
    let mut buf = String::new();
    for line in md.lines() {
        if !re.is_match(line) {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    buf
}

fn substitute_links(md: &str, link_subs: &BTreeMap<String, String>) -> String {
    let re = regex::Regex::new("^\\[(.+)\\]:(.+)$").expect(".");
    let mut buf = String::new();
    for line in md.lines() {
        if let Some(caps) = re.captures(line) {
            let link_name = caps.get(1).expect(".");
            let link_name = link_name.as_str();
            let link_dest = caps.get(2).expect(".");
            let link_dest = link_dest.as_str().trim();
            if let Some(sub) = link_subs.get(link_dest) {
                buf.push_str(&format!("[{link_name}]: {sub}"));
            } else {
                eprintln!("unreplaced link: {link_dest}");
                buf.push_str(line);
            }
        } else {
            buf.push_str(line);
        }
        buf.push('\n');
    }
    buf
}

fn substitute_versions(md: &str, crates: &[CrateInfo]) -> String {
    let re = regex::Regex::new("^\\[(.+)\\]: *https://docs.rs/(.+)/latest/(.+)$").expect(".");
    let mut buf = String::new();
    for line in md.lines() {
        if let Some(caps) = re.captures(line) {
            let link_name = caps.get(1).expect(".");
            let link_name = link_name.as_str();
            let crate_name = caps.get(2).expect(".");
            let crate_name = crate_name.as_str();
            let link_tail = caps.get(3).expect(".");
            let link_tail = link_tail.as_str().trim();
            if find_crate(crates, crate_name).is_some() {
                buf.push_str(&format!(
                    "[{link_name}]: api/{link_tail}"
                ));
            } else {
                buf.push_str(line);
            }
        } else {
            buf.push_str(line);
        }
        buf.push('\n');
    }
    buf
}

fn find_crate<'c>(crates: &'c [CrateInfo], name: &str) -> Option<&'c CrateInfo> {
    crates.iter().find(|c| c.name == name)
}

fn make_build_info() -> AnyResult<String> {
    let commit_sha = get_git_commit_sha()?;
    let build_info = serde_json::json!({
        "commit_sha": commit_sha
    });
    Ok(serde_json::to_string_pretty(&build_info)?)
}

fn get_git_commit_sha() -> AnyResult<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .context("Failed to execute git rev-parse HEAD")?;

    if !output.status.success() {
        return Err(A!("git rev-parse HEAD failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let sha = String::from_utf8(output.stdout)
        .context("git output is not valid UTF-8")?
        .trim()
        .to_string();

    Ok(sha)
}
