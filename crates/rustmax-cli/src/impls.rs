use rmx::prelude::*;

use super::tools::*;
use crate::cargo_install::{
    Assessment, CratesIo, InstalledPackage, Installs, assess, cargo_home, install_root,
    missing_bins, read_default_installs, read_installs,
};
use rmx::semver::Version;
use rmx::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

impl Tool {
    pub fn attrs(&self) -> ToolAttrs {
        use Tool::*;
        match self {
            Rustup => ToolAttrs {
                display_name: "rustup",
                impl_complete: false,
            },

            Cargo => ToolAttrs {
                display_name: "cargo",
                impl_complete: false,
            },
            CargoClippy => ToolAttrs {
                display_name: "cargo-clippy",
                impl_complete: false,
            },
            CargoFmt => ToolAttrs {
                display_name: "cargo-fmt",
                impl_complete: false,
            },
            CargoMiri => ToolAttrs {
                display_name: "cargo-miri",
                impl_complete: false,
            },
            Rustc => ToolAttrs {
                display_name: "rustc",
                impl_complete: false,
            },
            Rustdoc => ToolAttrs {
                display_name: "rustdoc",
                impl_complete: false,
            },
            Rustfmt => ToolAttrs {
                display_name: "rustfmt",
                impl_complete: false,
            },
            RustGdbGui => ToolAttrs {
                display_name: "rust-gdbgui",
                impl_complete: false,
            },
            RustGdb => ToolAttrs {
                display_name: "rust-gdb",
                impl_complete: false,
            },
            RustLldb => ToolAttrs {
                display_name: "rust-lldb",
                impl_complete: false,
            },

            RustAnalyzer => ToolAttrs {
                display_name: "rust-analyzer",
                impl_complete: false,
            },
            Miri => ToolAttrs {
                display_name: "miri",
                impl_complete: false,
            },
            Clippy => ToolAttrs {
                display_name: "clippy",
                impl_complete: false,
            },
            LlvmTools => ToolAttrs {
                display_name: "llvm-tools",
                impl_complete: false,
            },
            LlvmCov => ToolAttrs {
                display_name: "llvm-cov",
                impl_complete: false,
            },

            CargoAudit => ToolAttrs {
                display_name: "cargo-audit",
                impl_complete: true,
            },
            CargoCleanAll => ToolAttrs {
                display_name: "cargo-clean-all",
                impl_complete: true,
            },
            CargoDeny => ToolAttrs {
                display_name: "cargo-deny",
                impl_complete: true,
            },
            CargoDuplicates => ToolAttrs {
                display_name: "cargo-duplicates",
                impl_complete: true,
            },
            CargoEdit => ToolAttrs {
                display_name: "cargo-edit",
                impl_complete: true,
            },
            CargoGenerate => ToolAttrs {
                display_name: "cargo-generate",
                impl_complete: true,
            },
            CargoOutdated => ToolAttrs {
                display_name: "cargo-outdated",
                impl_complete: true,
            },
            CargoSweep => ToolAttrs {
                display_name: "cargo-sweep",
                impl_complete: true,
            },

            /* non-plugin cargo programs */
            BasicHttpServer => ToolAttrs {
                display_name: "basic-http-server",
                impl_complete: true,
            },
            DuDust => ToolAttrs {
                display_name: "dust",
                impl_complete: true,
            },
            FdFind => ToolAttrs {
                display_name: "fd",
                impl_complete: true,
            },
            Gist => ToolAttrs {
                display_name: "gist",
                impl_complete: true,
            },
            Jsonxf => ToolAttrs {
                display_name: "jsonxf",
                impl_complete: true,
            },
            Jaq => ToolAttrs {
                display_name: "jaq",
                impl_complete: true,
            },
            Just => ToolAttrs {
                display_name: "just",
                impl_complete: true,
            },
            Mdbook => ToolAttrs {
                display_name: "mdbook",
                impl_complete: true,
            },
            Ripgrep => ToolAttrs {
                display_name: "rg",
                impl_complete: true,
            },
            Sd => ToolAttrs {
                display_name: "sd",
                impl_complete: true,
            },
            Tokei => ToolAttrs {
                display_name: "tokei",
                impl_complete: true,
            },

            /* non-rust */
            Mold => ToolAttrs {
                display_name: "mold",
                impl_complete: true,
            },
        }
    }
}

/// How rustmax installs a tool.
enum Installer {
    /// `cargo install` of a crates.io package.
    Cargo(&'static str),
    Mold,
}

impl Tool {
    fn installer(&self) -> Installer {
        use Tool::*;
        let package = match self {
            Mold => return Installer::Mold,
            CargoAudit => "cargo-audit",
            CargoCleanAll => "cargo-clean-all",
            CargoDeny => "cargo-deny",
            CargoDuplicates => "cargo-duplicates",
            CargoEdit => "cargo-edit",
            CargoGenerate => "cargo-generate",
            CargoOutdated => "cargo-outdated",
            // `cargo-sweep` keeps no state outside the `target` directories it cleans:
            // the `--stamp` timestamp file lives in the project being swept,
            // so there is nothing to remove on uninstall.
            CargoSweep => "cargo-sweep",
            BasicHttpServer => "basic-http-server",
            DuDust => "du-dust",
            FdFind => "fd-find",
            Gist => "gist",
            Jsonxf => "jsonxf",
            Jaq => "jaq",
            Just => "just",
            Mdbook => "mdbook",
            Ripgrep => "ripgrep",
            Sd => "sd",
            Tokei => "tokei",
            _ => todo!("rustmax does not manage {} yet", self.attrs().display_name),
        };
        Installer::Cargo(package)
    }

    /// The name of the tool as written on the rustmax command line.
    fn cli_name(&self) -> String {
        let value = rmx::clap::ValueEnum::to_possible_value(self).expect("tool has a CLI name");
        value.get_name().to_string()
    }

    pub fn install(&self) -> AnyResult<()> {
        match self.installer() {
            Installer::Mold => crate::moldman::install(),
            Installer::Cargo(package) => cargo_install(self, package),
        }
    }

    pub fn update(&self) -> AnyResult<()> {
        match self.installer() {
            Installer::Mold => crate::moldman::update(),
            Installer::Cargo(package) => cargo_update(self, package),
        }
    }

    pub fn uninstall(&self) -> AnyResult<()> {
        match self.installer() {
            Installer::Mold => crate::moldman::uninstall(),
            Installer::Cargo(package) => cargo_uninstall(self, package),
        }
    }

    fn status(
        &self,
        crates_io: &CratesIo,
        installs: &Installs,
        root: &Path,
    ) -> AnyResult<ToolReport> {
        let package = match self.installer() {
            Installer::Mold => return crate::moldman::status(),
            Installer::Cargo(package) => package,
        };

        let installed = installs.get(package);
        let missing = installed.map(|p| missing_bins(root, p)).unwrap_or_default();
        let assessment = assess(installed, &missing, || {
            crates_io
                .latest_version(package)
                .map_err(|e| format!("{e:#}"))
        });

        let mut notes = Vec::new();
        if let Tool::CargoAudit = self
            && installed.is_some()
            && !advisory_db_path()?.exists()
        {
            notes.push("advisory db missing, run 'cargo audit' to fetch it".to_string());
        }

        Ok(ToolReport {
            name: self.attrs().display_name,
            installed: installed.map(|p| p.version.clone()),
            assessment,
            notes,
        })
    }
}

/// One line of `tools-status` output.
pub struct ToolReport {
    pub name: &'static str,
    pub installed: Option<Version>,
    pub assessment: Assessment,
    pub notes: Vec<String>,
}

/// Check the status of several tools, looking up their latest versions in parallel.
pub fn status_all(tools: &[Tool]) -> AnyResult<Vec<ToolReport>> {
    let root = install_root()?;
    let installs = read_installs(&root)?;
    let crates_io = CratesIo::new()?;
    std::thread::scope(|s| {
        let handles: Vec<_> = tools
            .iter()
            .map(|tool| s.spawn(|| tool.status(&crates_io, &installs, &root)))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("status thread panicked"))
            .collect()
    })
}

pub fn print_reports(reports: &[ToolReport]) -> AnyResult<()> {
    let choice = if std::io::stdout().is_terminal() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };
    let mut out = StandardStream::stdout(choice);

    let versions: Vec<String> = reports
        .iter()
        .map(|r| {
            r.installed
                .as_ref()
                .map_or("-".to_string(), |v| v.to_string())
        })
        .collect();
    let name_width = reports.iter().map(|r| r.name.len()).max().unwrap_or(0);
    let version_width = versions.iter().map(|v| v.len()).max().unwrap_or(0);

    for (report, version) in reports.iter().zip(&versions) {
        write!(
            out,
            "{:name_width$}  {version:version_width$}  ",
            report.name
        )?;
        let (color, text) = describe(&report.assessment);
        out.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(out, "{text}")?;
        out.reset()?;
        if !report.notes.is_empty() {
            out.set_color(ColorSpec::new().set_dimmed(true))?;
            write!(out, "  ({})", report.notes.join("; "))?;
            out.reset()?;
        }
        writeln!(out)?;
    }
    Ok(())
}

fn describe(assessment: &Assessment) -> (Color, String) {
    match assessment {
        Assessment::NotInstalled => (Color::Red, "not installed".to_string()),
        Assessment::MissingBins(bins) => {
            (Color::Red, format!("missing binaries: {}", bins.join(", ")))
        }
        Assessment::OtherSource(source) => (Color::Cyan, format!("installed from {source}")),
        Assessment::UpToDate => (Color::Green, "up to date".to_string()),
        Assessment::Outdated(latest) => (Color::Yellow, format!("update available: {latest}")),
        Assessment::LatestUnknown(e) => (Color::Yellow, format!("could not check latest: {e}")),
    }
}

fn installed_package(package: &str) -> AnyResult<Option<InstalledPackage>> {
    Ok(read_default_installs()?.remove(package))
}

fn run_cargo(args: &[&str]) -> AnyResult<()> {
    println!("Running: cargo {}", args.join(" "));
    let status = Command::new("cargo")
        .args(args)
        .status()
        .context("Failed to run cargo")?;
    if !status.success() {
        bail!("cargo {} failed: {status}", args.join(" "));
    }
    Ok(())
}

fn cargo_install(tool: &Tool, package: &str) -> AnyResult<()> {
    let name = tool.attrs().display_name;
    if let Some(installed) = installed_package(package)? {
        println!(
            "{name} {} is already installed, use 'rustmax update-tool {}' to update it",
            installed.version,
            tool.cli_name()
        );
        return Ok(());
    }

    run_cargo(&["install", package])?;
    let installed = installed_package(package)?
        .with_context(|| format!("cargo installed {package} but did not record it"))?;
    println!("Installed {name} {}", installed.version);

    if let Tool::CargoAudit = tool {
        cargo_audit_download_db()?;
    }
    Ok(())
}

fn cargo_update(tool: &Tool, package: &str) -> AnyResult<()> {
    let name = tool.attrs().display_name;
    let Some(installed) = installed_package(package)? else {
        println!(
            "{name} is not installed, use 'rustmax install-tool {}' to install it",
            tool.cli_name()
        );
        return Ok(());
    };
    if !installed.is_from_crates_io() {
        println!(
            "{name} was installed from {}, not updating it",
            installed.source
        );
        return Ok(());
    }

    let latest = CratesIo::new()?.latest_version(package)?;
    let missing = missing_bins(&install_root()?, &installed);
    let target = format!("{package}@{latest}");
    if !missing.is_empty() {
        println!(
            "{name} is missing binaries {}, reinstalling",
            missing.join(", ")
        );
        run_cargo(&["install", "--force", &target])?;
    } else if installed.version >= latest {
        println!("{name} {} is up to date", installed.version);
        return Ok(());
    } else {
        println!("Updating {name} {} -> {latest}", installed.version);
        run_cargo(&["install", &target])?;
    }

    let updated = installed_package(package)?
        .with_context(|| format!("cargo installed {package} but did not record it"))?;
    if updated.version != latest {
        bail!(
            "cargo installed {package} {}, expected {latest}",
            updated.version
        );
    }
    println!("Updated {name} to {latest}");
    Ok(())
}

fn cargo_uninstall(tool: &Tool, package: &str) -> AnyResult<()> {
    let name = tool.attrs().display_name;
    let Some(installed) = installed_package(package)? else {
        println!("{name} is not installed");
        return Ok(());
    };

    run_cargo(&["uninstall", package])?;
    if installed_package(package)?.is_some() {
        bail!("cargo uninstalled {package} but still records it as installed");
    }
    println!("Uninstalled {name} {}", installed.version);

    if let Tool::CargoAudit = tool {
        cargo_audit_remove_db()?;
    }
    Ok(())
}

fn advisory_db_path() -> AnyResult<PathBuf> {
    Ok(cargo_home()?.join("advisory-db"))
}

fn cargo_audit_download_db() -> AnyResult<()> {
    // The audit itself fails outside a cargo project,
    // so judge by whether the database appeared rather than by exit status.
    println!("Fetching the advisory database...");
    Command::new("cargo")
        .args(["audit", "--stale"])
        .output()
        .context("Failed to run cargo audit")?;
    let path = advisory_db_path()?;
    if path.exists() {
        println!("Advisory database is at {}", path.display());
    } else {
        println!(
            "Could not fetch the advisory database, run 'cargo audit' in a project to fetch it"
        );
    }
    Ok(())
}

fn cargo_audit_remove_db() -> AnyResult<()> {
    let path = advisory_db_path()?;
    if path.exists() {
        println!("Removing advisory database at {}", path.display());
        std::fs::remove_dir_all(&path)
            .with_context(|| format!("Failed to remove {}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_names() {
        assert_eq!(Tool::Ripgrep.cli_name(), "ripgrep");
        assert_eq!(Tool::CargoAudit.cli_name(), "cargo-audit");
    }

    #[test]
    fn every_complete_tool_has_an_installer() {
        for tool in enum_iterator::all::<Tool>() {
            if tool.attrs().impl_complete {
                tool.installer();
            }
        }
    }

    #[test]
    fn describes_outdated() {
        let latest = Version::parse("1.2.3").unwrap();
        let (color, text) = describe(&Assessment::Outdated(latest));
        assert_eq!(color, Color::Yellow);
        assert_eq!(text, "update available: 1.2.3");
    }
}
