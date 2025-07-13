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
            _ => todo!(),
        }
    }
}

fn cargo_audit_install() -> AnyResult<()> {
    println!("Installing cargo-audit...");

    // Check if already installed
    if let Ok(output) = std::process::Command::new("cargo")
        .args(["audit", "--version"])
        .output()
    {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!("cargo-audit is already installed ({})", version);
            println!("Use 'rustmax update-tool cargo-audit' to update to the latest version");
            return Ok(());
        }
    }

    // Install using cargo install
    println!("Running: cargo install cargo-audit");
    let status = std::process::Command::new("cargo")
        .args(["install", "cargo-audit"])
        .status()
        .context("Failed to execute cargo install command")?;

    if !status.success() {
        bail!(
            "cargo install cargo-audit failed with exit code: {}",
            status
        );
    }

    // Verify installation
    match std::process::Command::new("cargo")
        .args(["audit", "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!("✓ cargo-audit installed successfully ({})", version);
        }
        _ => {
            println!("⚠️  Installation may have succeeded but could not verify version");
        }
    }

    println!("cargo-audit installation complete!");
    Ok(())
}

impl Tool {
    pub fn update(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::update(),
            _ => todo!(),
        }
    }
}

impl Tool {
    pub fn uninstall(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::uninstall(),
            Tool::CargoAudit => cargo_audit_uninstall(),
            _ => todo!(),
        }
    }
}

impl Tool {
    pub fn status(&self) -> AnyResult<()> {
        match self {
            Tool::Mold => crate::moldman::status(),
            Tool::CargoAudit => cargo_audit_status(),
            _ => todo!(),
        }
    }
}

fn cargo_audit_status() -> AnyResult<()> {
    println!("cargo-audit status:");

    // Check binary installation and version
    match std::process::Command::new("cargo")
        .args(["audit", "--version"])
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
            return Ok(());
        }
    }

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

fn cargo_audit_uninstall() -> AnyResult<()> {
    println!("Uninstalling cargo-audit...");

    // Check if installed first
    match std::process::Command::new("cargo")
        .args(["audit", "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            println!("Found cargo-audit ({}), proceeding with uninstall", version);
        }
        _ => {
            println!("cargo-audit is not installed");
            return Ok(());
        }
    }

    // Uninstall using cargo uninstall
    println!("Running: cargo uninstall cargo-audit");
    let status = std::process::Command::new("cargo")
        .args(["uninstall", "cargo-audit"])
        .status()
        .context("Failed to execute cargo uninstall command")?;

    if !status.success() {
        bail!(
            "cargo uninstall cargo-audit failed with exit code: {}",
            status
        );
    }

    // Verify uninstallation
    match std::process::Command::new("cargo")
        .args(["audit", "--version"])
        .output()
    {
        Ok(output) if output.status.success() => {
            println!("⚠️  cargo-audit may still be installed (uninstall verification failed)");
        }
        _ => {
            println!("✓ cargo-audit uninstalled successfully");
        }
    }

    println!("cargo-audit uninstallation complete!");
    println!("Note: Advisory database at ~/.cargo/advisory-db was not removed");
    Ok(())
}
