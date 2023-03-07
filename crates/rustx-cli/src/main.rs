use serde::{Serialize, Deserialize};

fn main() {
    println!("Hello, world!");
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CargoPlugins {
    CargoAudit,
    CargoCleanAll,
    CargoEdit,
    CargoExpand,
    CargoFuzz,
    CargoOutdated,
    CargoUdeps,
    CargoTree,
    CargoWatch,
    CargoWorkspace,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CargoExes {
    BasicHttpServer,
    DuDust,
    FdFind,
    Gist,
    Hyperfine,
    Just,
    Parol,
    Ripgrep,
    Tokei,
}
