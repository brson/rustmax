use rmx::prelude::*;
use rmx::tempfile::TempDir;
use std::env;
use std::path::PathBuf;


pub fn install() -> AnyResult<()> {
    let tempdir = TempDir::with_prefix_in(
        "mold-install",
        env::current_dir()?,
    )?;

    //let path = download_release_tarball(tempdir.path())?;

    todo!()
}

pub fn update() -> AnyResult<()> {
    todo!()
}

pub fn uninstall() -> AnyResult<()> {
    todo!()
}

pub fn status() -> AnyResult<()> {
    todo!()
}

enum MoldTarget {
    LinuxX86_64,
}

fn get_target() -> AnyResult<MoldTarget> {
    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        Ok(MoldTarget::LinuxX86_64)
    } else {
        Err(anyhow!("unsupperted target"))
    }
}

fn get_current_release() -> AnyResult<String> {
    todo!()
}
  
fn get_url() -> AnyResult<String> {
    todo!()
}

fn get_cargo_bin_dir() -> AnyResult<PathBuf> {
    todo!()
}
