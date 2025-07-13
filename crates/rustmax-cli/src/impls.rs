use rmx::prelude::*;

use super::tools::*;

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

            _ => ToolAttrs {
                display_name: "<unknown>",
                impl_complete: false,
            },
        }
    }
}

impl Tool {
    pub fn install(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::install(),
            Tool::CargoAudit => cargo_plugin_install(&CARGO_AUDIT_CONFIG),
            Tool::CargoCleanAll => cargo_plugin_install(&CARGO_CLEAN_ALL_CONFIG),
            Tool::CargoDeny => cargo_plugin_install(&CARGO_DENY_CONFIG),
            Tool::CargoDuplicates => cargo_plugin_install(&CARGO_DUPLICATES_CONFIG),
            Tool::CargoEdit => cargo_plugin_install(&CARGO_EDIT_CONFIG),
            Tool::CargoGenerate => cargo_plugin_install(&CARGO_GENERATE_CONFIG),
            Tool::CargoOutdated => cargo_plugin_install(&CARGO_OUTDATED_CONFIG),
            Tool::BasicHttpServer => cargo_tool_install(&BASIC_HTTP_SERVER_CONFIG),
            Tool::DuDust => cargo_tool_install(&DU_DUST_CONFIG),
            Tool::FdFind => cargo_tool_install(&FD_FIND_CONFIG),
            Tool::Gist => cargo_tool_install(&GIST_CONFIG),
            Tool::Jsonxf => cargo_tool_install(&JSONXF_CONFIG),
            Tool::Jaq => cargo_tool_install(&JAQ_CONFIG),
            Tool::Just => cargo_tool_install(&JUST_CONFIG),
            Tool::Mdbook => cargo_tool_install(&MDBOOK_CONFIG),
            Tool::Ripgrep => cargo_tool_install(&RIPGREP_CONFIG),
            Tool::Sd => cargo_tool_install(&SD_CONFIG),
            Tool::Tokei => cargo_tool_install(&TOKEI_CONFIG),
            _ => todo!(),
        }
    }
}

impl Tool {
    pub fn update(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::update(),
            Tool::CargoAudit => cargo_plugin_update(&CARGO_AUDIT_CONFIG),
            Tool::CargoCleanAll => cargo_plugin_update(&CARGO_CLEAN_ALL_CONFIG),
            Tool::CargoDeny => cargo_plugin_update(&CARGO_DENY_CONFIG),
            Tool::CargoDuplicates => cargo_plugin_update(&CARGO_DUPLICATES_CONFIG),
            Tool::CargoEdit => cargo_plugin_update(&CARGO_EDIT_CONFIG),
            Tool::CargoGenerate => cargo_plugin_update(&CARGO_GENERATE_CONFIG),
            Tool::CargoOutdated => cargo_plugin_update(&CARGO_OUTDATED_CONFIG),
            Tool::BasicHttpServer => cargo_tool_update(&BASIC_HTTP_SERVER_CONFIG),
            Tool::DuDust => cargo_tool_update(&DU_DUST_CONFIG),
            Tool::FdFind => cargo_tool_update(&FD_FIND_CONFIG),
            Tool::Gist => cargo_tool_update(&GIST_CONFIG),
            Tool::Jsonxf => cargo_tool_update(&JSONXF_CONFIG),
            Tool::Jaq => cargo_tool_update(&JAQ_CONFIG),
            Tool::Just => cargo_tool_update(&JUST_CONFIG),
            Tool::Mdbook => cargo_tool_update(&MDBOOK_CONFIG),
            Tool::Ripgrep => cargo_tool_update(&RIPGREP_CONFIG),
            Tool::Sd => cargo_tool_update(&SD_CONFIG),
            Tool::Tokei => cargo_tool_update(&TOKEI_CONFIG),
            _ => todo!(),
        }
    }
}

impl Tool {
    pub fn uninstall(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::uninstall(),
            Tool::CargoAudit => cargo_plugin_uninstall(&CARGO_AUDIT_CONFIG),
            Tool::CargoCleanAll => cargo_plugin_uninstall(&CARGO_CLEAN_ALL_CONFIG),
            Tool::CargoDeny => cargo_plugin_uninstall(&CARGO_DENY_CONFIG),
            Tool::CargoDuplicates => cargo_plugin_uninstall(&CARGO_DUPLICATES_CONFIG),
            Tool::CargoEdit => cargo_plugin_uninstall(&CARGO_EDIT_CONFIG),
            Tool::CargoGenerate => cargo_plugin_uninstall(&CARGO_GENERATE_CONFIG),
            Tool::CargoOutdated => cargo_plugin_uninstall(&CARGO_OUTDATED_CONFIG),
            Tool::BasicHttpServer => cargo_tool_uninstall(&BASIC_HTTP_SERVER_CONFIG),
            Tool::DuDust => cargo_tool_uninstall(&DU_DUST_CONFIG),
            Tool::FdFind => cargo_tool_uninstall(&FD_FIND_CONFIG),
            Tool::Gist => cargo_tool_uninstall(&GIST_CONFIG),
            Tool::Jsonxf => cargo_tool_uninstall(&JSONXF_CONFIG),
            Tool::Jaq => cargo_tool_uninstall(&JAQ_CONFIG),
            Tool::Just => cargo_tool_uninstall(&JUST_CONFIG),
            Tool::Mdbook => cargo_tool_uninstall(&MDBOOK_CONFIG),
            Tool::Ripgrep => cargo_tool_uninstall(&RIPGREP_CONFIG),
            Tool::Sd => cargo_tool_uninstall(&SD_CONFIG),
            Tool::Tokei => cargo_tool_uninstall(&TOKEI_CONFIG),
            _ => todo!(),
        }
    }
}

impl Tool {
    pub fn status(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::status(),
            Tool::CargoAudit => cargo_plugin_status(&CARGO_AUDIT_CONFIG),
            Tool::CargoCleanAll => cargo_plugin_status(&CARGO_CLEAN_ALL_CONFIG),
            Tool::CargoDeny => cargo_plugin_status(&CARGO_DENY_CONFIG),
            Tool::CargoDuplicates => cargo_plugin_status(&CARGO_DUPLICATES_CONFIG),
            Tool::CargoEdit => cargo_plugin_status(&CARGO_EDIT_CONFIG),
            Tool::CargoGenerate => cargo_plugin_status(&CARGO_GENERATE_CONFIG),
            Tool::CargoOutdated => cargo_plugin_status(&CARGO_OUTDATED_CONFIG),
            Tool::BasicHttpServer => cargo_tool_status(&BASIC_HTTP_SERVER_CONFIG),
            Tool::DuDust => cargo_tool_status(&DU_DUST_CONFIG),
            Tool::FdFind => cargo_tool_status(&FD_FIND_CONFIG),
            Tool::Gist => cargo_tool_status(&GIST_CONFIG),
            Tool::Jsonxf => cargo_tool_status(&JSONXF_CONFIG),
            Tool::Jaq => cargo_tool_status(&JAQ_CONFIG),
            Tool::Just => cargo_tool_status(&JUST_CONFIG),
            Tool::Mdbook => cargo_tool_status(&MDBOOK_CONFIG),
            Tool::Ripgrep => cargo_tool_status(&RIPGREP_CONFIG),
            Tool::Sd => cargo_tool_status(&SD_CONFIG),
            Tool::Tokei => cargo_tool_status(&TOKEI_CONFIG),
            _ => todo!(),
        }
    }
}

struct CargoPluginConfig {
    tool_name: &'static str,
    package_name: &'static str,
    subcommand: &'static str,
    post_install_note: Option<&'static str>,
    post_install_action: Option<fn() -> AnyResult<()>>,
    post_status_action: Option<fn() -> AnyResult<()>>,
    post_uninstall_action: Option<fn() -> AnyResult<()>>,
}

const CARGO_AUDIT_CONFIG: CargoPluginConfig = CargoPluginConfig {
    tool_name: "cargo-audit",
    package_name: "cargo-audit",
    subcommand: "audit",
    post_install_note: None,
    post_install_action: Some(cargo_audit_download_db),
    post_status_action: Some(cargo_audit_status_db),
    post_uninstall_action: Some(cargo_audit_remove_db),
};

const CARGO_CLEAN_ALL_CONFIG: CargoPluginConfig = CargoPluginConfig {
    tool_name: "cargo-clean-all",
    package_name: "cargo-clean-all",
    subcommand: "clean-all",
    post_install_note: None,
    post_install_action: None,
    post_status_action: None,
    post_uninstall_action: None,
};

const CARGO_DENY_CONFIG: CargoPluginConfig = CargoPluginConfig {
    tool_name: "cargo-deny",
    package_name: "cargo-deny",
    subcommand: "deny",
    post_install_note: None,
    post_install_action: None,
    post_status_action: None,
    post_uninstall_action: None,
};

const CARGO_DUPLICATES_CONFIG: CargoPluginConfig = CargoPluginConfig {
    tool_name: "cargo-duplicates",
    package_name: "cargo-duplicates",
    subcommand: "duplicates",
    post_install_note: None,
    post_install_action: None,
    post_status_action: None,
    post_uninstall_action: None,
};

const CARGO_EDIT_CONFIG: CargoPluginConfig = CargoPluginConfig {
    tool_name: "cargo-edit",
    package_name: "cargo-edit",
    subcommand: "upgrade",
    post_install_note: None,
    post_install_action: None,
    post_status_action: None,
    post_uninstall_action: None,
};

const CARGO_GENERATE_CONFIG: CargoPluginConfig = CargoPluginConfig {
    tool_name: "cargo-generate",
    package_name: "cargo-generate",
    subcommand: "generate",
    post_install_note: None,
    post_install_action: None,
    post_status_action: None,
    post_uninstall_action: None,
};

const CARGO_OUTDATED_CONFIG: CargoPluginConfig = CargoPluginConfig {
    tool_name: "cargo-outdated",
    package_name: "cargo-outdated",
    subcommand: "outdated",
    post_install_note: None,
    post_install_action: None,
    post_status_action: None,
    post_uninstall_action: None,
};

fn cargo_plugin_install(config: &CargoPluginConfig) -> AnyResult<()> {
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
            println!(
                "Use 'rustmax update-tool {}' to update to the latest version",
                config.tool_name
            );
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
            println!(
                "✓ {} installed successfully ({})",
                config.tool_name, version
            );
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

fn cargo_plugin_uninstall(config: &CargoPluginConfig) -> AnyResult<()> {
    println!("Uninstalling {}...", config.tool_name);

    // Check if installed first
    match std::process::Command::new("cargo")
        .args([config.subcommand, "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!(
                "Found {} ({}), proceeding with uninstall",
                config.tool_name, version
            );
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
            println!(
                "⚠️  {} may still be installed (uninstall verification failed)",
                config.tool_name
            );
        }
        _ => {
            println!("✓ {} uninstalled successfully", config.tool_name);
        }
    }

    if let Some(note) = config.post_install_note {
        println!("{}", note);
    }

    if let Some(action) = config.post_uninstall_action {
        action()?;
    }

    println!("{} uninstallation complete!", config.tool_name);
    Ok(())
}

fn cargo_plugin_update(config: &CargoPluginConfig) -> AnyResult<()> {
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
                    println!(
                        "✓ {} updated: {} → {}",
                        config.tool_name, old_version, new_version
                    );
                } else {
                    println!(
                        "✓ {} is already up to date ({})",
                        config.tool_name, new_version
                    );
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

fn cargo_plugin_status(config: &CargoPluginConfig) -> AnyResult<()> {
    println!("{} status:", config.tool_name);

    // Check binary installation and version
    match std::process::Command::new("cargo")
        .args([config.subcommand, "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            if version.is_empty() {
                // Empty stdout but successful exit - this shouldn't happen for cargo plugins
                println!("  Binary: Installed (version unknown)");
            } else {
                println!("  Binary: Installed ({})", version);
            }
        }
        Ok(output) => {
            // Command ran but failed - check stderr for "no such command"
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("no such command") {
                println!("  Binary: Not installed");
            } else {
                println!("  Binary: Installed (version unknown)");
            }
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

fn cargo_audit_remove_db() -> AnyResult<()> {
    // Remove advisory database on uninstall
    let advisory_db_path = std::env::var("HOME")
        .map(|home| format!("{}/.cargo/advisory-db", home))
        .unwrap_or_else(|_| "~/.cargo/advisory-db".to_string());

    if std::path::Path::new(&advisory_db_path).exists() {
        println!("Removing advisory database at {}...", advisory_db_path);
        match std::fs::remove_dir_all(&advisory_db_path) {
            Ok(_) => println!("✓ Advisory database removed"),
            Err(e) => println!("⚠️  Failed to remove advisory database: {}", e),
        }
    } else {
        println!("Advisory database not found, nothing to remove");
    }
    Ok(())
}

struct CargoToolConfig {
    tool_name: &'static str,
    package_name: &'static str,
    post_install_note: Option<&'static str>,
    post_install_action: Option<fn() -> AnyResult<()>>,
    post_uninstall_action: Option<fn() -> AnyResult<()>>,
}

fn cargo_tool_install(config: &CargoToolConfig) -> AnyResult<()> {
    println!("Installing {}...", config.tool_name);

    // Check if already installed
    if let Ok(output) = std::process::Command::new(config.tool_name)
        .args(["--version"])
        .output()
    {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!("{} is already installed ({})", config.tool_name, version);
            println!(
                "Use 'rustmax update-tool {}' to update to the latest version",
                config.tool_name
            );
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
    match std::process::Command::new(config.tool_name)
        .args(["--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!(
                "✓ {} installed successfully ({})",
                config.tool_name, version
            );
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
    match std::process::Command::new(config.tool_name)
        .args(["--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!(
                "Found {} ({}), proceeding with uninstall",
                config.tool_name, version
            );
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
    match std::process::Command::new(config.tool_name)
        .args(["--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            println!(
                "⚠️  {} may still be installed (uninstall verification failed)",
                config.tool_name
            );
        }
        _ => {
            println!("✓ {} uninstalled successfully", config.tool_name);
        }
    }

    if let Some(note) = config.post_install_note {
        println!("{}", note);
    }

    if let Some(action) = config.post_uninstall_action {
        action()?;
    }

    println!("{} uninstallation complete!", config.tool_name);
    Ok(())
}

fn cargo_tool_update(config: &CargoToolConfig) -> AnyResult<()> {
    println!("Updating {}...", config.tool_name);

    // Check if installed first
    let current_version = match std::process::Command::new(config.tool_name)
        .args(["--version"])
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
    match std::process::Command::new(config.tool_name)
        .args(["--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let new_version = String::from_utf8_lossy(&output.stdout);
            let new_version = new_version.trim();

            if let Some(old_version) = current_version {
                if old_version != new_version {
                    println!(
                        "✓ {} updated: {} → {}",
                        config.tool_name, old_version, new_version
                    );
                } else {
                    println!(
                        "✓ {} is already up to date ({})",
                        config.tool_name, new_version
                    );
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
    match std::process::Command::new(config.tool_name)
        .args(["--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            if version.is_empty() {
                println!("  Binary: Installed (version unknown)");
            } else {
                println!("  Binary: Installed ({})", version);
            }
        }
        Ok(_) => {
            println!("  Binary: Not installed or version command failed");
        }
        Err(_) => {
            println!("  Binary: Not installed");
        }
    }

    Ok(())
}

const BASIC_HTTP_SERVER_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "basic-http-server",
    package_name: "basic-http-server",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};

const DU_DUST_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "dust",
    package_name: "du-dust",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};

const FD_FIND_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "fd",
    package_name: "fd-find",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};

const GIST_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "gist",
    package_name: "gist",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};

const JSONXF_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "jsonxf",
    package_name: "jsonxf",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};

const JAQ_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "jaq",
    package_name: "jaq",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};

const JUST_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "just",
    package_name: "just",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};

const MDBOOK_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "mdbook",
    package_name: "mdbook",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};

const RIPGREP_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "rg",
    package_name: "ripgrep",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};

const SD_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "sd",
    package_name: "sd",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};

const TOKEI_CONFIG: CargoToolConfig = CargoToolConfig {
    tool_name: "tokei",
    package_name: "tokei",
    post_install_note: None,
    post_install_action: None,
    post_uninstall_action: None,
};
