use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = Path::new(&manifest_dir).join("../../template");
    let dst = Path::new(&manifest_dir).join("assets/template");

    // Only copy if source exists (i.e., building from repo, not from published crate).
    if src.exists() {
        println!("cargo::rerun-if-changed=../../template");

        if dst.exists() {
            fs::remove_dir_all(&dst).expect("failed to remove old assets/template");
        }

        copy_dir_recursive(&src, &dst).expect("failed to copy template directory");
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
