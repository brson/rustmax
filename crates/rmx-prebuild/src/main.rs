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

use std::env;
use anyhow::Result as AnyResult;

const CRATES_META: &str = "src/crates.json5";
const TOOLS_META: &str = "src/tools.json5";
const RMX_MANIFEST: &str = "crates/rmx/Cargo.toml";
const EXAMPLES_DIR: &str = "crates/doc-src/";

struct CrateInfo {
    name: String,
    category: String,
    version: String,
    short_desc: String,
    oneline_desc: String,
    example: String,
}

fn main() -> AnyResult<()> {
    let workspace_dir = env::current_dir()?;
    let crates_meta_file = workspace_dir.join(CRATES_META);
    let tools_meta_file = workspace_dir.join(TOOLS_META);
    let rmx_manifest_file = workspace_dir.join(RMX_MANIFEST);
    let examples_dir = workspace_dir.join(EXAMPLES_DIR);

    todo!()
}
