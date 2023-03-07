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
    CargoTree,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CargoExes {
    BasicHttpServer,
    DuDust,
    FdFind,
    Gist,
    Hyperfine,
    Parol,
    Ripgrep,
    Tokei,
}
