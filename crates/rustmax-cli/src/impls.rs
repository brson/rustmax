use rmx::prelude::*;

use super::tools::*;

impl Tool {
    pub fn attrs(&self) -> ToolAttrs {
        use Tool::*;
        match self {
            Rustup => ToolAttrs {
                display_name: "rustup",
            },

            Cargo => ToolAttrs {
                display_name: "cargo",
            },
            CargoClippy => ToolAttrs {
                display_name: "cargo-clippy",
            },
            CargoFmt => ToolAttrs {
                display_name: "cargo-fmt",
            },
            CargoMiri => ToolAttrs {
                display_name: "cargo-miri",
            },
            Rustc => ToolAttrs {
                display_name: "rustc",
            },
            Rustdoc => ToolAttrs {
                display_name: "rustdoc",
            },
            Rustfmt => ToolAttrs {
                display_name: "rustfmt",
            },
            RustGdbGui => ToolAttrs {
                display_name: "rust-gdbgui",
            },
            RustGdb => ToolAttrs {
                display_name: "rust-gdb",
            },
            RustLldb => ToolAttrs {
                display_name: "rust-lldb",
            },

            RustAnalyzer => ToolAttrs {
                display_name: "rust-analyzer",
            },
            Miri => ToolAttrs {
                display_name: "miri",
            },
            Clippy => ToolAttrs {
                display_name: "clippy",
            },
            LlvmTools => ToolAttrs {
                display_name: "llvm-tools",
            },
            LlvmCov => ToolAttrs {
                display_name: "llvm-cov",
            },

            CargoAudit => ToolAttrs {
                display_name: "cargo-audit",
            },
            CargoCleanAll => ToolAttrs {
                display_name: "cargo-clean-all",
            },
            CargoDeny => ToolAttrs {
                display_name: "cargo-deny",
            },
            CargoEdit => ToolAttrs {
                display_name: "cargo-edit",
            },
            CargoGenerate => ToolAttrs {
                display_name: "cargo-generate",
            },
            CargoOutdated => ToolAttrs {
                display_name: "cargo-outdated",
            },
            CargoTree => ToolAttrs {
                display_name: "cargo-tree",
            },

            /* non-rust */
            Mold => ToolAttrs {
                display_name: "mold",
            },

            _ => ToolAttrs {
                display_name: "<unknown>",
            },
        }
    }
}

impl Tool {
    pub fn install(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::install(),
            Tool::CargoAudit => cargo_audit_install(),
            Tool::CargoCleanAll => cargo_clean_all_install(),
            _ => todo!(),
        }
    }
}

impl Tool {
    pub fn update(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::update(),
            Tool::CargoAudit => cargo_audit_update(),
            Tool::CargoCleanAll => cargo_clean_all_update(),
            _ => todo!(),
        }
    }
}

impl Tool {
    pub fn uninstall(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::uninstall(),
            Tool::CargoAudit => cargo_audit_uninstall(),
            Tool::CargoCleanAll => cargo_clean_all_uninstall(),
            _ => todo!(),
        }
    }
}

impl Tool {
    pub fn status(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::status(),
            Tool::CargoAudit => cargo_audit_status(),
            Tool::CargoCleanAll => cargo_clean_all_status(),
            _ => todo!(),
        }
    }
}

struct CargoToolConfig {
    tool_name: &'static str,
    package_name: &'static str,
    subcommand: &'static str,
    post_install_note: Option<&'static str>,
    post_install_action: Option<fn() -> AnyResult<()>>,
    post_status_action: Option<fn() -> AnyResult<()>>,
}

fn cargo_tool_install(config: &CargoToolConfig) -> AnyResult<()> {
    println!("Installing {}...", config.tool_name);

    // Check if already installed
    if let Ok(output) = std::process::Command::new("cargo")
        .args([config.subcommand, "--version"])
        .output()
    {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!("{} is already installed ({})", config.tool_name, version);
            println!("Use 'rustmax update-tool {}' to update to the latest version", config.tool_name);
            return Ok(());
        }
    }

    // Install using cargo install
    println!("Running: cargo install {}", config.package_name);
    let status = std::process::Command::new("cargo")
        .args(["install", config.package_name])
        .status()
        .context("Failed to execute cargo install command")?;

    if !status.success() {
        bail!(
            "cargo install {} failed with exit code: {}",
            config.package_name,
            status
        );
    }

    // Verify installation
    match std::process::Command::new("cargo")
        .args([config.subcommand, "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!("✓ {} installed successfully ({})", config.tool_name, version);
        }
        _ => {
            println!("⚠️  Installation may have succeeded but could not verify version");
        }
    }

    if let Some(note) = config.post_install_note {
        println!("{}", note);
    }

    if let Some(action) = config.post_install_action {
        action()?;
    }

    println!("{} installation complete!", config.tool_name);
    Ok(())
}

fn cargo_tool_uninstall(config: &CargoToolConfig) -> AnyResult<()> {
    println!("Uninstalling {}...", config.tool_name);

    // Check if installed first
    match std::process::Command::new("cargo")
        .args([config.subcommand, "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!("Found {} ({}), proceeding with uninstall", config.tool_name, version);
        }
        _ => {
            println!("{} is not installed", config.tool_name);
            return Ok(());
        }
    }

    // Uninstall using cargo uninstall
    println!("Running: cargo uninstall {}", config.package_name);
    let status = std::process::Command::new("cargo")
        .args(["uninstall", config.package_name])
        .status()
        .context("Failed to execute cargo uninstall command")?;

    if !status.success() {
        bail!(
            "cargo uninstall {} failed with exit code: {}",
            config.package_name,
            status
        );
    }

    // Verify uninstallation
    match std::process::Command::new("cargo")
        .args([config.subcommand, "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            println!("⚠️  {} may still be installed (uninstall verification failed)", config.tool_name);
        }
        _ => {
            println!("✓ {} uninstalled successfully", config.tool_name);
        }
    }

    if let Some(note) = config.post_install_note {
        println!("{}", note);
    }

    println!("{} uninstallation complete!", config.tool_name);
    Ok(())
}

fn cargo_tool_update(config: &CargoToolConfig) -> AnyResult<()> {
    println!("Updating {}...", config.tool_name);

    // Check if installed first
    let current_version = match std::process::Command::new("cargo")
        .args([config.subcommand, "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!("Current version: {}", version);
            Some(version.to_string())
        }
        _ => {
            println!(
                "{} is not installed, use 'rustmax install-tool {}' instead",
                config.tool_name, config.tool_name
            );
            return Ok(());
        }
    };

    // Update using cargo install --force
    println!("Running: cargo install --force {}", config.package_name);
    let status = std::process::Command::new("cargo")
        .args(["install", "--force", config.package_name])
        .status()
        .context("Failed to execute cargo install command")?;

    if !status.success() {
        bail!(
            "cargo install --force {} failed with exit code: {}",
            config.package_name,
            status
        );
    }

    // Verify update
    match std::process::Command::new("cargo")
        .args([config.subcommand, "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let new_version = String::from_utf8_lossy(&output.stdout);
            let new_version = new_version.trim();
            
            if let Some(old_version) = current_version {
                if old_version != new_version {
                    println!("✓ {} updated: {} → {}", config.tool_name, old_version, new_version);
                } else {
                    println!("✓ {} is already up to date ({})", config.tool_name, new_version);
                }
            } else {
                println!("✓ {} updated to {}", config.tool_name, new_version);
            }
        }
        _ => {
            println!("⚠️  Update may have succeeded but could not verify version");
        }
    }

    println!("{} update complete!", config.tool_name);
    Ok(())
}

fn cargo_tool_status(config: &CargoToolConfig) -> AnyResult<()> {
    println!("{} status:", config.tool_name);

    // Check binary installation and version
    match std::process::Command::new("cargo")
        .args([config.subcommand, "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!("  Binary: Installed ({})", version);
        }
        Ok(_) => {
            println!("  Binary: Installed (version unknown)");
        }
        Err(_) => {
            println!("  Binary: Not installed");
        }
    }

    if let Some(action) = config.post_status_action {
        action()?;
    }

    Ok(())
}

/////////

fn cargo_audit_install() -> AnyResult<()> {
    let config = CargoToolConfig {
        tool_name: "cargo-audit",
        package_name: "cargo-audit",
        subcommand: "audit",
        post_install_note: None,
        post_install_action: Some(cargo_audit_download_db),
        post_status_action: Some(cargo_audit_status_db),
    };
    cargo_tool_install(&config)
}

fn cargo_audit_update() -> AnyResult<()> {
    let config = CargoToolConfig {
        tool_name: "cargo-audit",
        package_name: "cargo-audit",
        subcommand: "audit",
        post_install_note: None,
        post_install_action: None,
        post_status_action: None,
    };
    cargo_tool_update(&config)
}

fn cargo_audit_status() -> AnyResult<()> {
    let config = CargoToolConfig {
        tool_name: "cargo-audit",
        package_name: "cargo-audit",
        subcommand: "audit",
        post_install_note: None,
        post_install_action: None,
        post_status_action: Some(cargo_audit_status_db),
    };
    cargo_tool_status(&config)
}

fn cargo_audit_uninstall() -> AnyResult<()> {
    let config = CargoToolConfig {
        tool_name: "cargo-audit",
        package_name: "cargo-audit",
        subcommand: "audit",
        post_install_note: Some("Note: Advisory database at ~/.cargo/advisory-db was not removed"),
        post_install_action: None,
        post_status_action: None,
    };
    cargo_tool_uninstall(&config)
}

fn cargo_audit_download_db() -> AnyResult<()> {
    // Download advisory database on first install
    println!("Downloading advisory database...");
    let db_status = std::process::Command::new("cargo")
        .args(["audit", "--stale"]) // This will download the DB if not present
        .output();

    match db_status {
        Ok(_) => println!("✓ Advisory database ready"),
        Err(_) => println!("⚠️  Could not initialize advisory database"),
    }
    Ok(())
}

fn cargo_audit_status_db() -> AnyResult<()> {
    // Check advisory database status
    let advisory_db_path = std::env::var("HOME")
        .map(|home| format!("{}/.cargo/advisory-db", home))
        .unwrap_or_else(|_| "~/.cargo/advisory-db".to_string());

    if std::path::Path::new(&advisory_db_path).exists() {
        println!("  Advisory DB: Present at {}", advisory_db_path);
    } else {
        println!("  Advisory DB: Not found, run 'cargo audit' to download");
    }
    Ok(())
}

////////

fn cargo_clean_all_install() -> AnyResult<()> {
    let config = CargoToolConfig {
        tool_name: "cargo-clean-all",
        package_name: "cargo-clean-all",
        subcommand: "clean-all",
        post_install_note: None,
        post_install_action: None,
        post_status_action: None,
    };
    cargo_tool_install(&config)
}

fn cargo_clean_all_update() -> AnyResult<()> {
    let config = CargoToolConfig {
        tool_name: "cargo-clean-all",
        package_name: "cargo-clean-all",
        subcommand: "clean-all",
        post_install_note: None,
        post_install_action: None,
        post_status_action: None,
    };
    cargo_tool_update(&config)
}

fn cargo_clean_all_status() -> AnyResult<()> {
    let config = CargoToolConfig {
        tool_name: "cargo-clean-all",
        package_name: "cargo-clean-all",
        subcommand: "clean-all",
        post_install_note: None,
        post_install_action: None,
        post_status_action: None,
    };
    cargo_tool_status(&config)
}

fn cargo_clean_all_uninstall() -> AnyResult<()> {
    let config = CargoToolConfig {
        tool_name: "cargo-clean-all",
        package_name: "cargo-clean-all",
        subcommand: "clean-all",
        post_install_note: None,
        post_install_action: None,
        post_status_action: None,
    };
    cargo_tool_uninstall(&config)
}
