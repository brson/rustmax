use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_dir);

    // Copy template directory for new-project command.
    let src = manifest_dir.join("../../template");
    let dst = manifest_dir.join("assets/template");

    // Only copy if source exists (i.e., building from repo, not from published crate).
    if src.exists() {
        println!("cargo::rerun-if-changed=../../template");

        if dst.exists() {
            fs::remove_dir_all(&dst).expect("failed to remove old assets/template");
        }

        copy_dir_recursive(&src, &dst).expect("failed to copy template directory");
    }

    // Copy shared CSS files for rmxbook.
    let www_dir = manifest_dir.join("../../www");
    let assets_dir = manifest_dir.join("assets");

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

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
