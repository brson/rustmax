use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let www_dir = Path::new(&manifest_dir).join("../../www");
    let assets_dir = Path::new(&manifest_dir).join("assets");

    // Only copy if source exists (i.e., building from repo, not from published crate).
    if www_dir.exists() {
        println!("cargo::rerun-if-changed=../../www/rustmax-themes.css");
        println!("cargo::rerun-if-changed=../../www/rustmax-syntax.css");

        fs::create_dir_all(&assets_dir).expect("failed to create assets directory");

        let themes_src = www_dir.join("rustmax-themes.css");
        let themes_dst = assets_dir.join("rustmax-themes.css");
        fs::copy(&themes_src, &themes_dst).expect("failed to copy rustmax-themes.css");

        let syntax_src = www_dir.join("rustmax-syntax.css");
        let syntax_dst = assets_dir.join("rustmax-syntax.css");
        fs::copy(&syntax_src, &syntax_dst).expect("failed to copy rustmax-syntax.css");
    }
}
