/*
# ideas

- crate table for readme.md
- crate table for index.html
- crate table for crate docs
- crate list for rustdoc-script whitelist
- examples for readme.md
- examples for index.html
- examples for crate docs
- various parts of crate docs

*/

#![allow(unused)]

use std::{env, fs};
use anyhow::Result as AnyResult;
use anyhow::Context;
use anyhow::anyhow as A;
use std::collections::BTreeMap;

const CRATES_META: &str = "src/crates.json5";
const TOOLS_META: &str = "src/tools.json5";
const RMX_MANIFEST: &str = "crates/rmx/Cargo.toml";
const EXAMPLES_DIR: &str = "crates/rmx/doc-src";
const LINK_SUBS: &str = "src/linksubs.json5";

const OUT_DIR: &str = "work";
const OUT_CRATES_MD: &str = "work/crates.md";
const OUT_CRATES_JSON: &str = "work/crates.json";
const OUT_CRATES_HTML: &str = "work/crates.html";

#[derive(Debug)]
struct CrateInfo {
    name: String,
    category: String,
    version: String,
    short_desc: String,
    oneline_desc: String,
    example: String,
}

mod meta {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    #[derive(Clone, Debug)]
    pub struct Crates {
        pub crates: Vec<Crate>,
    }

    #[derive(Serialize, Deserialize)]
    #[derive(Clone, Debug)]
    pub struct Crate {
        pub name: String,
        pub category: String,
        pub short_desc: String,
        pub oneline_desc: String,
    }

    #[derive(Serialize, Deserialize)]
    #[derive(Clone, Debug)]
    pub struct Tools {
        pub tools: Vec<Tool>,
    }

    #[derive(Serialize, Deserialize)]
    #[derive(Clone, Debug)]
    pub struct Tool {
        pub name: String,
        pub category: String,
        pub short_desc: String,
        pub oneline_desc: String,
    }
}

fn main() -> AnyResult<()> {
    let workspace_dir = env::current_dir()?;

    let crates_meta_file = workspace_dir.join(CRATES_META);
    let tools_meta_file = workspace_dir.join(TOOLS_META);
    let rmx_manifest_file = workspace_dir.join(RMX_MANIFEST);
    let examples_dir = workspace_dir.join(EXAMPLES_DIR);
    let link_subs_file = workspace_dir.join(LINK_SUBS);

    let crates_meta_str = fs::read_to_string(&crates_meta_file)
        .context(crates_meta_file.display().to_string())?;
    let tools_meta_str = fs::read_to_string(&tools_meta_file)
        .context(tools_meta_file.display().to_string())?;
    let rmx_manifest_str = fs::read_to_string(&rmx_manifest_file)
        .context(rmx_manifest_file.display().to_string())?;
    let link_subs_str = fs::read_to_string(&link_subs_file)
        .context(tools_meta_file.display().to_string())?;

    let crates_meta: meta::Crates = json5::from_str(&crates_meta_str)
        .context("crates meta")?;
    let tools_meta: meta::Tools = json5::from_str(&tools_meta_str)
        .context("tools meta")?;
    let rmx_manifest: toml::Value = toml::from_str(&rmx_manifest_str)
        .context("rmx manifest meta")?;
    let examples_dir = fs::read_dir(&examples_dir)
        .context(examples_dir.display().to_string())?;
    let link_subs: BTreeMap<String, String> = json5::from_str(&link_subs_str)
        .context("crates meta")?;

    let crate_info = build_crate_info(
        &crates_meta, &rmx_manifest, examples_dir,
    )?;

    let out_crates_md_file = workspace_dir.join(OUT_CRATES_MD);
    let out_crates_json_file = workspace_dir.join(OUT_CRATES_JSON);
    let out_crates_html_file = workspace_dir.join(OUT_CRATES_HTML);

    let (
        out_crates_md_str,
        out_crates_json_str,
        out_crates_html_str,
    ) = make_crate_lists(&crate_info, &link_subs);

    fs::create_dir_all(OUT_DIR)?;
    write(out_crates_md_file, &out_crates_md_str)?;
    write(out_crates_json_file, &out_crates_json_str)?;
    write(out_crates_html_file, &out_crates_html_str)?;

    Ok(())
}

fn write<P>(p: P, c: &str) -> AnyResult<()>
    where P: AsRef<std::path::Path>,
{
    fs::write(&p, c)?;
    eprintln!("wrote {}", p.as_ref().display());
    Ok(())
}

fn build_crate_info(
    crates_meta: &meta::Crates,
    rmx_manifest: &toml::Value,
    examples_dir: fs::ReadDir,
) -> AnyResult<Vec<CrateInfo>> {
    let manifest_crate_info = get_manifest_crate_info(rmx_manifest)?;
    let examples = get_examples(examples_dir)?;

    let mut infos = Vec::new();

    for crate_ in &manifest_crate_info {
        let meta = crates_meta.crates.iter().find(|c| {
            c.name == crate_.name
        }).ok_or(A!("missing crate meta for {}", crate_.name))?;

        let example = examples.iter().find(|c| {
            c.name == crate_.name
        }).map(|ce| ce.text.to_string()).unwrap_or_default();

        infos.push(CrateInfo {
            name: crate_.name.to_string(),
            category: meta.category.to_string(),
            version: crate_.version.to_string(),
            short_desc: meta.short_desc.to_string(),
            oneline_desc: meta.oneline_desc.to_string(),
            example,
        });
    }

    for crate_ in &crates_meta.crates {
        let _ = manifest_crate_info.iter().find(|c| {
            c.name == crate_.name
        }).ok_or(A!("unused crate meta for {}", crate_.name))?;
    }

    for crate_ in &examples {
        let _ = manifest_crate_info.iter().find(|c| {
            c.name == crate_.name
        }).ok_or(A!("unused example for {}", crate_.name))?;
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

    deps.iter().map(|(name, dep)| {
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
    }).collect()
}


#[derive(Debug)]
struct CrateExample {
    name: String,
    text: String,
}

fn get_examples(
    mut examples_dir: fs::ReadDir,
) -> AnyResult<Vec<CrateExample>> {
    let crate_name_regex = regex::Regex::new(
        r"^crate-(?<name>[a-zA-Z0-9_-]+)\.md$"
    ).expect(".");

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

fn make_crate_lists(
    crates: &[CrateInfo],
    link_subs: &BTreeMap<String, String>,
) -> (
    String, String, String
) {
    let mut md = String::new();
    let mut json = String::new();
    let mut html = String::new();

    md.push_str("| Feature | Crate | |\n");
    md.push_str("|-|-|-|\n");
    json.push_str("[\n");
    html.push_str("<table id='rmx-crate-table'>\n");
    html.push_str("<thead>\n");
    html.push_str("<th>Feature</th>\n");
    html.push_str("<th>Crate</th>\n");
    html.push_str("<th><button type='button'>+</button></th>\n");
    html.push_str("</thead>\n");

    for (i, krate) in crates.iter().enumerate() {
        let example_html = render_example(krate, link_subs, crates);

        md.push_str(&format!(
            "| {} | `{} = \"{}\"` | [📖]({}) |\n",
            krate.short_desc,
            krate.name,
            krate.version,
            "todo",
        ));

        if i + 1 < crates.len() {
            json.push_str(&format!(
                "\"{}\",\n",
                krate.name,
            ));
        } else {
            json.push_str(&format!(
                "\"{}\"\n",
                krate.name,
            ));
        }

        html.push_str(&format!(
            "<tr class='{}'>\n",
            if i % 2 == 0 {
                "row-even"
            } else {
                "row-odd"
            }
        ));
        html.push_str(&format!(
            "<td>{}</td>\n",
            krate.short_desc,
        ));
        html.push_str(&format!(
            "<td><a href='https://docs.rs/{}/{}/{}'><code>{} = \"{}\"</code></a></td>\n",
            krate.name,
            krate.version,
            krate.name.replace("-", "_"),
            krate.name,
            krate.version,
        ));
        if example_html.is_some() {
            html.push_str(&format!(
                "<td><button id='button-{}' class='example-button' data-name='{}' type='button'>+</button></td>\n",
                krate.name,
                krate.name,
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
                    if i % 2 == 0 {
                        "row-even"
                    } else {
                        "row-odd"
                    }
                ));
                html.push_str("<td colspan=3>\n");
                html.push_str(&example_html);
                html.push_str("</td>\n");
                html.push_str("</tr>\n");
            }
        }
    }

    md.push_str("");
    json.push_str("]");
    html.push_str("</thead>\n");

    (md, json, html)
}

fn render_example(
    krate: &CrateInfo,
    link_subs: &BTreeMap<String, String>,
    crates: &[CrateInfo],
) -> Option<String> {
    if !krate.example.is_empty() {
        let md = process_md(&krate.example, link_subs, crates);
        let html = comrak::markdown_to_html(
            &md,
            &Default::default(),
        );
        Some(html)
    } else {
        None
    }
}

fn process_md(
    md: &str,
    link_subs: &BTreeMap<String, String>,
    crates: &[CrateInfo],
) -> String {
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

fn substitute_links(
    md: &str,
    link_subs: &BTreeMap<String, String>,
) -> String {
    let re = regex::Regex::new("^\\[(.+)\\]:(.+)$").expect(".");
    let mut buf = String::new();
    for line in md.lines() {
        if let Some(caps) = re.captures(line) {
            let link_name = caps.get(1).expect(".");
            let link_name = link_name.as_str();
            let link_dest = caps.get(2).expect(".");
            let link_dest = link_dest.as_str().trim();
            if let Some(sub) = link_subs.get(link_dest) {
                buf.push_str(
                    &format!("[{link_name}]: {sub}"),
                );
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

fn substitute_versions(
    md: &str,
    crates: &[CrateInfo],
) -> String {
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
            if let Some(info) = find_crate(crates, crate_name) {
                let version = &info.version;
                buf.push_str(
                    &format!("[{link_name}]: https://docs.rs/{crate_name}/{version}/{link_tail}")
                );
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

fn find_crate<'c>(
    crates: &'c [CrateInfo],
    name: &str,
) -> Option<&'c CrateInfo> {
    crates.iter().find(|c| c.name == name)
}
