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

const CRATES_META: &str = "src/crates.json5";
const TOOLS_META: &str = "src/tools.json5";
const RMX_MANIFEST: &str = "crates/rmx/Cargo.toml";
const EXAMPLES_DIR: &str = "crates/rmx/doc-src";

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

    let crates_meta_str = fs::read_to_string(&crates_meta_file)
        .context(crates_meta_file.display().to_string())?;
    let tools_meta_str = fs::read_to_string(&tools_meta_file)
        .context(tools_meta_file.display().to_string())?;
    let rmx_manifest_str = fs::read_to_string(&rmx_manifest_file)
        .context(rmx_manifest_file.display().to_string())?;

    let crates_meta: meta::Crates = json5::from_str(&crates_meta_str)
        .context("crates meta")?;
    let tools_meta: meta::Tools = json5::from_str(&tools_meta_str)
        .context("tools meta")?;
    let rmx_manifest: toml::Value = toml::from_str(&rmx_manifest_str)
        .context("rmx manifest meta")?;
    let examples_dir = fs::read_dir(&examples_dir)
        .context(examples_dir.display().to_string())?;

    let crate_info = build_crate_info(
        &crates_meta, &rmx_manifest, examples_dir,
    )?;

    todo!()
}

fn build_crate_info(
    crates_meta: &meta::Crates,
    rmx_manifest: &toml::Value,
    examples_dir: fs::ReadDir,
) -> AnyResult<Vec<CrateInfo>> {
    let manifest_crate_info = get_manifest_crate_info(rmx_manifest)?;
    let examples = get_examples(examples_dir)?;

    let mut infos = Vec::new();
    
    for crate_ in manifest_crate_info {
        let meta = crates_meta.crates.iter().find(|cm| {
            cm.name == crate_.name
        }).ok_or(A!("missing crate meta for {}", crate_.name))?;

        let example = examples.iter().find(|ce| {
            ce.name == crate_.name
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

    todo!()
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
