//! Knowledge of what `cargo install` has installed, and what crates.io offers.
//!
//! Installed versions come from the `.crates2.json` file cargo maintains in its
//! install root, so no tool binaries need to be run to find out their version.
//! Latest versions come from the crates.io sparse index.

use rmx::prelude::*;
use rmx::serde::Deserialize;
use rmx::{reqwest, semver::Version, serde_json};
use std::collections::BTreeMap;
use std::env;
use std::path::{Path, PathBuf};

/// The source id cargo records for packages installed from crates.io.
const CRATES_IO_SOURCE: &str = "registry+https://github.com/rust-lang/crates.io-index";

const SPARSE_INDEX_URL: &str = "https://index.crates.io";

/// The cargo home directory, `$CARGO_HOME` or `~/.cargo`.
pub fn cargo_home() -> AnyResult<PathBuf> {
    match env::var_os("CARGO_HOME") {
        Some(home) => Ok(PathBuf::from(home)),
        None => {
            let home = env::var_os("HOME").context("Could not determine home directory")?;
            Ok(PathBuf::from(home).join(".cargo"))
        }
    }
}

/// The directory `cargo install` installs into.
///
/// This honors `$CARGO_INSTALL_ROOT` but not the `install.root` cargo config key.
pub fn install_root() -> AnyResult<PathBuf> {
    match env::var_os("CARGO_INSTALL_ROOT") {
        Some(root) => Ok(PathBuf::from(root)),
        None => cargo_home(),
    }
}

/// A package recorded by cargo as installed.
#[derive(Debug, Clone, PartialEq)]
pub struct InstalledPackage {
    pub version: Version,
    pub source: String,
    pub bins: Vec<String>,
}

impl InstalledPackage {
    pub fn is_from_crates_io(&self) -> bool {
        self.source == CRATES_IO_SOURCE
    }
}

/// The packages cargo has installed, by package name.
pub type Installs = BTreeMap<String, InstalledPackage>;

/// Read the packages installed in `root`.
///
/// A missing `.crates2.json` means nothing has been installed yet.
pub fn read_installs(root: &Path) -> AnyResult<Installs> {
    let path = root.join(".crates2.json");
    match std::fs::read_to_string(&path) {
        Ok(json) => {
            parse_crates2(&json).with_context(|| format!("Failed to parse {}", path.display()))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Installs::new()),
        Err(e) => Err(e).with_context(|| format!("Failed to read {}", path.display())),
    }
}

/// Read the packages installed in the default install root.
pub fn read_default_installs() -> AnyResult<Installs> {
    read_installs(&install_root()?)
}

#[derive(Deserialize)]
struct Crates2 {
    installs: BTreeMap<String, Crates2Entry>,
}

#[derive(Deserialize)]
struct Crates2Entry {
    bins: Vec<String>,
}

/// Parse the contents of `.crates2.json`.
pub fn parse_crates2(json: &str) -> AnyResult<Installs> {
    let crates2: Crates2 = serde_json::from_str(json)?;
    let mut installs = Installs::new();
    for (key, entry) in crates2.installs {
        let (name, version, source) = parse_package_id(&key)?;
        let package = InstalledPackage {
            version,
            source: source.to_string(),
            bins: entry.bins,
        };
        if installs.insert(name.to_string(), package).is_some() {
            bail!("Package {name} is recorded as installed more than once");
        }
    }
    Ok(installs)
}

/// Split a package id of the form `name version (source)`.
fn parse_package_id(id: &str) -> AnyResult<(&str, Version, &str)> {
    let (name, rest) = id
        .split_once(' ')
        .with_context(|| format!("Bad package id {id:?}"))?;
    let (version, source) = rest
        .split_once(' ')
        .with_context(|| format!("Bad package id {id:?}"))?;
    let source = source
        .strip_prefix('(')
        .and_then(|s| s.strip_suffix(')'))
        .with_context(|| format!("Bad package id {id:?}"))?;
    let version =
        Version::parse(version).with_context(|| format!("Bad version in package id {id:?}"))?;
    Ok((name, version, source))
}

/// The binaries of a package that are absent from `root/bin`.
pub fn missing_bins(root: &Path, package: &InstalledPackage) -> Vec<String> {
    package
        .bins
        .iter()
        .filter(|bin| !root.join("bin").join(bin).exists())
        .cloned()
        .collect()
}

/// The path of a crate's file within the sparse index.
pub fn index_path(name: &str) -> String {
    let name = name.to_lowercase();
    match name.len() {
        1 => format!("1/{name}"),
        2 => format!("2/{name}"),
        3 => format!("3/{}/{name}", &name[..1]),
        _ => format!("{}/{}/{name}", &name[..2], &name[2..4]),
    }
}

#[derive(Deserialize)]
struct IndexEntry {
    vers: Version,
    yanked: bool,
}

/// Find the newest stable, unyanked version in a sparse index file.
pub fn parse_index_latest(body: &str) -> AnyResult<Version> {
    let mut latest: Option<Version> = None;
    for line in body.lines().filter(|l| !l.trim().is_empty()) {
        let entry: IndexEntry =
            serde_json::from_str(line).context("Failed to parse index entry")?;
        if entry.yanked || !entry.vers.pre.is_empty() {
            continue;
        }
        if latest.as_ref().is_none_or(|l| entry.vers > *l) {
            latest = Some(entry.vers);
        }
    }
    latest.context("No stable, unyanked versions in index")
}

/// A client for looking up crate versions on crates.io.
#[derive(Clone)]
pub struct CratesIo {
    client: reqwest::blocking::Client,
}

impl CratesIo {
    pub fn new() -> AnyResult<CratesIo> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(concat!("rustmax-cli/", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(CratesIo { client })
    }

    /// The newest stable, unyanked version of a crate.
    pub fn latest_version(&self, name: &str) -> AnyResult<Version> {
        let url = format!("{SPARSE_INDEX_URL}/{}", index_path(name));
        let response = self
            .client
            .get(&url)
            .send()
            .with_context(|| format!("Failed to fetch {url}"))?;
        if !response.status().is_success() {
            bail!("Fetching {url} failed with status {}", response.status());
        }
        let body = response.text()?;
        parse_index_latest(&body).with_context(|| format!("Bad index file {url}"))
    }
}

/// The condition of a tool, compared against the newest release.
#[derive(Debug, Clone, PartialEq)]
pub enum Assessment {
    NotInstalled,
    /// Cargo records the package but some of its binaries are gone.
    MissingBins(Vec<String>),
    /// Installed from git, a path, or another registry.
    OtherSource(String),
    UpToDate,
    Outdated(Version),
    /// Installed, but the newest version could not be determined.
    LatestUnknown(String),
}

/// Decide the condition of a cargo-installed package.
///
/// The latest version is only looked up when it matters.
pub fn assess(
    installed: Option<&InstalledPackage>,
    missing_bins: &[String],
    latest: impl FnOnce() -> Result<Version, String>,
) -> Assessment {
    let Some(installed) = installed else {
        return Assessment::NotInstalled;
    };
    if !missing_bins.is_empty() {
        return Assessment::MissingBins(missing_bins.to_vec());
    }
    if !installed.is_from_crates_io() {
        return Assessment::OtherSource(installed.source.clone());
    }
    assess_version(&installed.version, &latest())
}

/// Decide whether an installed version is current.
pub fn assess_version(installed: &Version, latest: &Result<Version, String>) -> Assessment {
    match latest {
        Ok(latest) if installed >= latest => Assessment::UpToDate,
        Ok(latest) => Assessment::Outdated(latest.clone()),
        Err(e) => Assessment::LatestUnknown(e.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CRATES2: &str = r#"{
      "installs": {
        "gist 0.9.2 (registry+https://github.com/rust-lang/crates.io-index)": {
          "version_req": null,
          "bins": ["gist"],
          "features": [],
          "all_features": false,
          "no_default_features": false,
          "profile": "release",
          "target": "x86_64-unknown-linux-gnu",
          "rustc": "rustc 1.87.0"
        },
        "ripgrep 14.1.1 (registry+https://github.com/rust-lang/crates.io-index)": {
          "bins": ["rg"]
        },
        "cargo-edit 0.13.7 (registry+https://github.com/rust-lang/crates.io-index)": {
          "bins": ["cargo-add", "cargo-rm", "cargo-set-version", "cargo-upgrade"]
        },
        "crown 0.0.1 (path+file:///home/brian/servo/support/crown)": {
          "bins": ["crown"]
        },
        "jj-cli 0.35.0-dev (git+https://github.com/jj-vcs/jj#abc123)": {
          "bins": ["jj"]
        }
      }
    }"#;

    fn v(s: &str) -> Version {
        Version::parse(s).unwrap()
    }

    #[test]
    fn parses_crates2() {
        let installs = parse_crates2(CRATES2).unwrap();
        assert_eq!(installs.len(), 5);

        let rg = &installs["ripgrep"];
        assert_eq!(rg.version, v("14.1.1"));
        assert_eq!(rg.bins, ["rg"]);
        assert!(rg.is_from_crates_io());

        assert_eq!(installs["cargo-edit"].bins.len(), 4);

        let crown = &installs["crown"];
        assert_eq!(crown.source, "path+file:///home/brian/servo/support/crown");
        assert!(!crown.is_from_crates_io());

        assert_eq!(installs["jj-cli"].version, v("0.35.0-dev"));
    }

    #[test]
    fn rejects_bad_package_id() {
        let json = r#"{"installs": {"gist 0.9.2": {"bins": []}}}"#;
        assert!(parse_crates2(json).is_err());
    }

    #[test]
    fn missing_crates2_is_empty() {
        let dir = rmx::tempfile::tempdir().unwrap();
        assert!(read_installs(dir.path()).unwrap().is_empty());
    }

    #[test]
    fn index_paths() {
        assert_eq!(index_path("a"), "1/a");
        assert_eq!(index_path("sd"), "2/sd");
        assert_eq!(index_path("jaq"), "3/j/jaq");
        assert_eq!(index_path("just"), "ju/st/just");
        assert_eq!(index_path("Cargo-Audit"), "ca/rg/cargo-audit");
    }

    #[test]
    fn index_latest_skips_yanked_and_prerelease() {
        let body = r#"
{"name":"x","vers":"1.0.0","deps":[],"cksum":"","features":{},"yanked":false}
{"name":"x","vers":"1.2.0","deps":[],"cksum":"","features":{},"yanked":false}
{"name":"x","vers":"1.3.0","deps":[],"cksum":"","features":{},"yanked":true}
{"name":"x","vers":"2.0.0-rc.1","deps":[],"cksum":"","features":{},"yanked":false}
{"name":"x","vers":"1.1.5","deps":[],"cksum":"","features":{},"yanked":false}
"#;
        assert_eq!(parse_index_latest(body).unwrap(), v("1.2.0"));
    }

    #[test]
    fn index_latest_requires_a_release() {
        let body =
            r#"{"name":"x","vers":"1.0.0","deps":[],"cksum":"","features":{},"yanked":true}"#;
        assert!(parse_index_latest(body).is_err());
    }

    fn crates_io_package(version: &str) -> InstalledPackage {
        InstalledPackage {
            version: v(version),
            source: CRATES_IO_SOURCE.to_string(),
            bins: vec!["tool".to_string()],
        }
    }

    #[test]
    fn assessments() {
        let pkg = crates_io_package("1.2.0");
        let latest_same = || Ok(v("1.2.0"));
        let latest_newer = || Ok(v("1.3.0"));
        let latest_older = || Ok(v("1.1.0"));
        let latest_err = || Err("offline".to_string());
        let unreachable = || -> Result<Version, String> { panic!("latest version looked up") };

        assert_eq!(assess(None, &[], unreachable), Assessment::NotInstalled);
        assert_eq!(assess(Some(&pkg), &[], latest_same), Assessment::UpToDate);
        assert_eq!(assess(Some(&pkg), &[], latest_older), Assessment::UpToDate);
        assert_eq!(
            assess(Some(&pkg), &[], latest_newer),
            Assessment::Outdated(v("1.3.0"))
        );
        assert_eq!(
            assess(Some(&pkg), &[], latest_err),
            Assessment::LatestUnknown("offline".to_string())
        );
        assert_eq!(
            assess(Some(&pkg), &["tool".to_string()], unreachable),
            Assessment::MissingBins(vec!["tool".to_string()])
        );

        let git = InstalledPackage {
            source: "git+https://example.com/x".to_string(),
            ..pkg
        };
        assert_eq!(
            assess(Some(&git), &[], unreachable),
            Assessment::OtherSource("git+https://example.com/x".to_string())
        );
    }

    #[test]
    fn detects_missing_bins() {
        let dir = rmx::tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("bin")).unwrap();
        std::fs::write(dir.path().join("bin/cargo-add"), "").unwrap();
        let pkg = InstalledPackage {
            version: v("0.13.7"),
            source: CRATES_IO_SOURCE.to_string(),
            bins: vec!["cargo-add".to_string(), "cargo-upgrade".to_string()],
        };
        assert_eq!(missing_bins(dir.path(), &pkg), ["cargo-upgrade"]);
    }
}
