use rmx::prelude::*;
use rmx::{tempfile::TempDir, reqwest, serde_json, toml, xshell};
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;

pub fn install() -> AnyResult<()> {
    println!("Installing mold linker...");
    
    // Check if we're on a supported platform
    let target = get_target().context("Unsupported platform")?;
    
    // Check if clang is available (required for our cargo config)
    if !check_clang_available() {
        bail!("clang is required for mold integration but not found in PATH. Please install clang first.");
    }
    
    // Get the latest release version
    let version = get_current_release().context("Failed to get latest mold release")?;
    println!("Latest mold version: {}", version);
    
    // Create temporary directory for download
    let tempdir = TempDir::with_prefix_in("mold-install", env::current_dir()?)?;
    let temp_path = tempdir.path();
    
    // Download and extract mold
    let url = get_url(&version, &target).context("Failed to build download URL")?;
    println!("Downloading from: {}", url);
    
    let tarball_path = download_release_tarball(temp_path, &url).context("Failed to download mold")?;
    let extracted_dir = extract_tarball(&tarball_path).context("Failed to extract mold")?;
    
    // Install the binary
    let mold_binary = find_mold_binary(&extracted_dir).context("Failed to find mold binary in archive")?;
    let cargo_bin_dir = get_cargo_bin_dir()?;
    let install_path = mold_install_path()?;
    
    fs::create_dir_all(cargo_bin_dir)?;
    fs::copy(&mold_binary, &install_path).context("Failed to copy mold binary")?;
    
    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&install_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&install_path, perms)?;
    }
    
    println!("Mold binary installed to: {}", install_path.display());
    
    // Configure cargo to use mold
    backup_cargo_config().context("Failed to backup existing cargo config")?;
    update_cargo_config().context("Failed to update cargo configuration")?;
    
    println!("Cargo configured to use mold linker");
    println!("Installation complete! You can now use mold for faster linking.");
    
    Ok(())
}

pub fn update() -> AnyResult<()> {
    println!("Checking for mold updates...");
    
    // Check current installed version
    let current_version = get_installed_version();
    let latest_version = get_current_release().context("Failed to get latest mold release")?;
    
    match current_version {
        Ok(current) if current == latest_version => {
            println!("Mold is already up to date ({})", current);
            return Ok(());
        }
        Ok(current) => {
            println!("Updating mold from {} to {}", current, latest_version);
        }
        Err(_) => {
            println!("Mold not currently installed, installing {}", latest_version);
        }
    }
    
    // Run installation (which will overwrite existing binary)
    install()
}

pub fn uninstall() -> AnyResult<()> {
    println!("Uninstalling mold linker...");
    
    let install_path = mold_install_path()?;
    let mut removed_binary = false;
    
    // Remove the binary
    if install_path.exists() {
        fs::remove_file(&install_path).context("Failed to remove mold binary")?;
        println!("Removed mold binary from: {}", install_path.display());
        removed_binary = true;
    }
    
    // Remove cargo configuration
    let config_removed = remove_mold_from_cargo_config().context("Failed to clean up cargo configuration")?;
    if config_removed {
        println!("Removed mold configuration from cargo");
    }
    
    if removed_binary || config_removed {
        println!("Mold uninstallation complete");
    } else {
        println!("Mold was not installed");
    }
    
    Ok(())
}

pub fn status() -> AnyResult<()> {
    println!("Mold linker status:");
    
    // Check binary installation
    let install_path = mold_install_path()?;
    let binary_installed = install_path.exists();
    
    if binary_installed {
        match get_installed_version() {
            Ok(version) => println!("  Binary: Installed ({})", version),
            Err(_) => println!("  Binary: Installed (version unknown)"),
        }
        println!("  Location: {}", install_path.display());
    } else {
        println!("  Binary: Not installed");
    }
    
    // Check cargo configuration
    let config_path = get_cargo_config_path()?;
    let config_status = check_cargo_config_status(&config_path)?;
    
    match config_status {
        CargoConfigStatus::Configured => println!("  Cargo: Configured to use mold"),
        CargoConfigStatus::NotConfigured => println!("  Cargo: Not configured"),
        CargoConfigStatus::NoConfigFile => println!("  Cargo: No config file"),
    }
    
    // Check dependencies
    if check_clang_available() {
        println!("  Dependencies: clang available");
    } else {
        println!("  Dependencies: clang missing (required for mold integration)");
    }
    
    // Check latest version
    match get_current_release() {
        Ok(latest) => {
            if binary_installed {
                match get_installed_version() {
                    Ok(current) if current == latest => println!("  Status: Up to date"),
                    Ok(current) => println!("  Status: Update available ({} -> {})", current, latest),
                    Err(_) => println!("  Status: Latest version is {}", latest),
                }
            } else {
                println!("  Latest available: {}", latest);
            }
        }
        Err(_) => println!("  Status: Could not check for updates"),
    }
    
    Ok(())
}

#[derive(Debug)]
enum MoldTarget {
    LinuxX86_64,
}

impl MoldTarget {
    fn arch_string(&self) -> &'static str {
        match self {
            MoldTarget::LinuxX86_64 => "x86_64-linux",
        }
    }
}

fn get_target() -> AnyResult<MoldTarget> {
    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        Ok(MoldTarget::LinuxX86_64)
    } else {
        bail!("Unsupported target. Mold installer currently only supports Linux x86_64.");
    }
}

fn get_current_release() -> AnyResult<String> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.github.com/repos/rui314/mold/releases/latest")
        .header("User-Agent", "rustmax-cli")
        .send()
        .context("Failed to fetch latest release from GitHub")?;
    
    if !response.status().is_success() {
        bail!("GitHub API request failed with status: {}", response.status());
    }
    
    let release_data: serde_json::Value = response
        .json()
        .context("Failed to parse GitHub API response")?;
    
    let tag_name = release_data["tag_name"]
        .as_str()
        .context("Could not find tag_name in GitHub API response")?;
    
    Ok(tag_name.to_string())
}

fn get_url(version: &str, target: &MoldTarget) -> AnyResult<String> {
    // Strip 'v' prefix from version for filename
    let version_clean = version.strip_prefix('v').unwrap_or(version);
    
    let url = format!(
        "https://github.com/rui314/mold/releases/download/{}/mold-{}-{}.tar.gz",
        version,
        version_clean,
        target.arch_string()
    );
    
    Ok(url)
}

fn get_cargo_bin_dir() -> AnyResult<PathBuf> {
    let cargo_home = match env::var("CARGO_HOME") {
        Ok(home) => PathBuf::from(home),
        Err(_) => {
            let home_dir = env::var("HOME").context("Could not determine home directory")?;
            PathBuf::from(home_dir).join(".cargo")
        }
    };
    
    Ok(cargo_home.join("bin"))
}

fn get_cargo_config_path() -> AnyResult<PathBuf> {
    let cargo_home = match env::var("CARGO_HOME") {
        Ok(home) => PathBuf::from(home),
        Err(_) => {
            let home_dir = env::var("HOME").context("Could not determine home directory")?;
            PathBuf::from(home_dir).join(".cargo")
        }
    };
    
    Ok(cargo_home.join("config.toml"))
}

fn download_release_tarball(temp_dir: &Path, url: &str) -> AnyResult<PathBuf> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "rustmax-cli")
        .send()
        .context("Failed to download mold tarball")?;
    
    if !response.status().is_success() {
        bail!("Download failed with status: {}", response.status());
    }
    
    let filename = url.split('/').last().unwrap_or("mold.tar.gz");
    let tarball_path = temp_dir.join(filename);
    
    let content = response.bytes().context("Failed to read download content")?;
    fs::write(&tarball_path, content).context("Failed to write tarball to disk")?;
    
    Ok(tarball_path)
}

fn extract_tarball(tarball_path: &Path) -> AnyResult<PathBuf> {
    let extract_dir = tarball_path.parent().unwrap().join("extracted");
    fs::create_dir_all(&extract_dir)?;
    
    let output = Command::new("tar")
        .args(["-xzf", tarball_path.to_str().unwrap(), "-C", extract_dir.to_str().unwrap()])
        .output()
        .context("Failed to run tar command. Is tar installed?")?;
    
    if !output.status.success() {
        bail!("tar extraction failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(extract_dir)
}

fn find_mold_binary(extracted_dir: &Path) -> AnyResult<PathBuf> {
    // Look for mold binary in common locations within the extracted directory
    let common_paths = [
        "bin/mold",
        "mold",
    ];
    
    // First, try the common paths
    for path in &common_paths {
        let candidate = extracted_dir.join(path);
        if candidate.exists() {
            return Ok(candidate);
        }
    }
    
    // If not found, search recursively
    fn find_mold_recursive(dir: &Path) -> Option<PathBuf> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(found) = find_mold_recursive(&path) {
                        return Some(found);
                    }
                } else if path.file_name().and_then(|n| n.to_str()) == Some("mold") {
                    return Some(path);
                }
            }
        }
        None
    }
    
    find_mold_recursive(extracted_dir)
        .ok_or_else(|| anyhow!("Could not find mold binary in extracted archive"))
}

fn get_installed_version() -> AnyResult<String> {
    let mold_path = mold_install_path()?;
    
    let output = Command::new(&mold_path)
        .arg("--version")
        .output()
        .context("Failed to run mold --version")?;
    
    if !output.status.success() {
        bail!("mold --version failed");
    }
    
    let version_output = String::from_utf8(output.stdout)
        .context("mold --version output is not valid UTF-8")?;
    
    // Extract version from output like "mold 2.4.0 (compatible with GNU ld)"
    let version = version_output
        .split_whitespace()
        .nth(1)
        .context("Could not parse version from mold --version output")?;
    
    Ok(format!("v{}", version))
}

fn check_clang_available() -> bool {
    Command::new("clang")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[derive(Debug, PartialEq)]
enum CargoConfigStatus {
    Configured,
    NotConfigured,
    NoConfigFile,
}

fn check_cargo_config_status(config_path: &Path) -> AnyResult<CargoConfigStatus> {
    let install_path = mold_install_path()?;

    if !config_path.exists() {
        return Ok(CargoConfigStatus::NoConfigFile);
    }
    
    let config_content = fs::read_to_string(config_path)
        .context("Failed to read cargo config file")?;
    
    let config: toml::Value = toml::from_str(&config_content)
        .context("Failed to parse cargo config as TOML")?;
    
    // Check if the mold configuration exists
    if let Some(target_section) = config.get("target") {
        if let Some(linux_section) = target_section.get("x86_64-unknown-linux-gnu") {
            if let Some(linker) = linux_section.get("linker") {
                if linker.as_str() == Some("clang") {
                    if let Some(rustflags) = linux_section.get("rustflags") {
                        if let Some(flags_array) = rustflags.as_array() {
                            // Check if our mold configuration is present
                            let has_link_arg = flags_array.iter().any(|v| v.as_str() == Some("-C"));
                            let has_mold = flags_array.iter().any(|v| {
                                if let Some(s) = v.as_str() {
                                    let mold_arg = format!("link-arg=-fuse-ld={}", install_path.display());
                                    s.contains(&mold_arg)
                                } else {
                                    false
                                }
                            });
                            
                            if has_link_arg && has_mold {
                                return Ok(CargoConfigStatus::Configured);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(CargoConfigStatus::NotConfigured)
}

fn backup_cargo_config() -> AnyResult<Option<PathBuf>> {
    let config_path = get_cargo_config_path()?;
    
    if !config_path.exists() {
        return Ok(None);
    }
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    
    let backup_path = config_path.with_extension(format!("toml.backup.{}", timestamp));
    
    fs::copy(&config_path, &backup_path)
        .context("Failed to create backup of cargo config")?;
    
    println!("Backed up existing cargo config to: {}", backup_path.display());
    Ok(Some(backup_path))
}

fn mold_install_path() -> AnyResult<PathBuf> {
    let cargo_bin_dir = get_cargo_bin_dir()?;
    let install_path = cargo_bin_dir.join("mold");
    Ok(install_path)
}

fn update_cargo_config() -> AnyResult<()> {
    let config_path = get_cargo_config_path()?;
    let install_path = mold_install_path()?;
    
    // Create .cargo directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Load existing config or create new one
    let mut config: toml::Value = if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .context("Failed to read existing cargo config")?;
        content.parse().context("Failed to parse existing cargo config")?
    } else {
        toml::Value::Table(Default::default())
    };
    
    // Ensure we have a table at the root
    let config_table = config.as_table_mut()
        .context("Config root is not a table")?;
    
    // Get or create target section
    let target_section = config_table
        .entry("target".to_string())
        .or_insert_with(|| toml::Value::Table(Default::default()))
        .as_table_mut()
        .context("target section is not a table")?;
    
    // Get or create linux target section
    let linux_section = target_section
        .entry("x86_64-unknown-linux-gnu".to_string())
        .or_insert_with(|| toml::Value::Table(Default::default()))
        .as_table_mut()
        .context("linux target section is not a table")?;
    
    // Set linker
    linux_section.insert("linker".to_string(), toml::Value::String("clang".to_string()));
    
    // Set rustflags
    let mold_arg = format!("link-arg=-fuse-ld={}", install_path.display());
    let rustflags = toml::Value::Array(vec![
        toml::Value::String("-C".to_string()),
        toml::Value::String(mold_arg),
    ]);
    linux_section.insert("rustflags".to_string(), rustflags);
    
    // Write the updated config
    let updated_content = toml::to_string_pretty(&config)
        .context("Failed to serialize updated config")?;
    
    fs::write(&config_path, updated_content)
        .context("Failed to write updated cargo config")?;
    
    Ok(())
}

fn remove_mold_from_cargo_config() -> AnyResult<bool> {
    let config_path = get_cargo_config_path()?;
    let install_path = mold_install_path()?;
    
    if !config_path.exists() {
        return Ok(false);
    }
    
    let config_content = fs::read_to_string(&config_path)
        .context("Failed to read cargo config")?;
    
    let mut config: toml::Value = config_content.parse()
        .context("Failed to parse cargo config")?;
    
    let mut config_modified = false;
    
    // Navigate to the target section
    if let Some(target_section) = config.get_mut("target") {
        if let Some(target_table) = target_section.as_table_mut() {
            if let Some(linux_section) = target_table.get_mut("x86_64-unknown-linux-gnu") {
                if let Some(linux_table) = linux_section.as_table_mut() {
                    // Remove mold-specific configuration
                    if linux_table.get("linker") == Some(&toml::Value::String("clang".to_string())) {
                        // Check if rustflags contains our mold configuration
                        if let Some(rustflags) = linux_table.get("rustflags") {
                            if let Some(flags_array) = rustflags.as_array() {
                                let has_mold = flags_array.iter().any(|v| {
                                    if let Some(s) = v.as_str() {
                                        let mold_arg = format!("link-arg=-fuse-ld={}", install_path.display());
                                        s.contains(&mold_arg)
                                    } else {
                                        false
                                    }
                                });
                                
                                if has_mold {
                                    linux_table.remove("linker");
                                    linux_table.remove("rustflags");
                                    config_modified = true;
                                }
                            }
                        }
                    }
                    
                    // If the linux section is now empty, remove it
                    if linux_table.is_empty() {
                        target_table.remove("x86_64-unknown-linux-gnu");
                    }
                }
            }
            
            // If the target section is now empty, remove it
            if target_table.is_empty() {
                if let Some(config_table) = config.as_table_mut() {
                    config_table.remove("target");
                }
            }
        }
    }
    
    if config_modified {
        // Check if the entire config is now empty
        if let Some(config_table) = config.as_table() {
            if config_table.is_empty() {
                // Remove the entire config file
                fs::remove_file(&config_path)
                    .context("Failed to remove empty cargo config file")?;
            } else {
                // Write the updated config
                let updated_content = toml::to_string_pretty(&config)
                    .context("Failed to serialize updated config")?;
                
                fs::write(&config_path, updated_content)
                    .context("Failed to write updated cargo config")?;
            }
        }
    }
    
    Ok(config_modified)
}
