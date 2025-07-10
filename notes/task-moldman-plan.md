# Task Moldman Implementation Plan

Based on analysis of the rustmax-cli codebase and mold linker requirements, here's the comprehensive plan to implement the `moldman` module.

## Current State Analysis

- **Task**: Implement mold linker management in rustmax-cli
- **Current Infrastructure**: 
  - CLI commands already wired up (`install-tool`, `update-tool`, `uninstall-tool`, `tool-status`)
  - `Tool::Mold` enum variant exists with routing to `moldman` module functions
  - Basic `moldman.rs` skeleton with placeholder functions
  - Available crates: `tempfile`, `xshell`, `regex`, `thiserror`, but missing `reqwest` and `serde_json`

## Required Dependencies Update

1. **Add missing dependencies** to `rustmax-cli/Cargo.toml`:
   - Need to include `reqwest` and `serde_json` features in the rmx dependency
   - Change features from `["rmx-profile-std", "rmx-feature-derive"]` to `["rmx-profile-net", "rmx-feature-derive"]` 
   - This provides both HTTP capabilities (reqwest) and JSON parsing (serde_json)

## Core Implementation Plan

### 1. GitHub API Integration (`get_current_release`)
- Use `reqwest` to fetch `https://api.github.com/repos/rui314/mold/releases/latest`
- Parse JSON response with `serde_json` to extract latest version tag
- Handle API rate limiting and network errors gracefully
- Return version string (e.g., "v2.35.0")

### 2. Download URL Construction (`get_url`)
- Build download URL using pattern: `https://github.com/rui314/mold/releases/download/{version}/mold-{version}-x86_64-linux.tar.gz`
- Validate target architecture matches supported platforms
- Strip 'v' prefix from version for filename construction

### 3. Installation Directory Management (`get_cargo_bin_dir`)
- Use `std::env::var("CARGO_HOME")` or default to `~/.cargo`
- Create `bin` subdirectory path: `{CARGO_HOME}/bin`
- Ensure directory exists with proper permissions

### 4. Core Installation Logic (`install`)
- **Download**: Use `reqwest` to download tarball to temp directory
- **Extract**: Use `xshell` or `std::process::Command` to run `tar -xzf`
- **Install**: Copy `mold` binary from extracted directory to `{CARGO_HOME}/bin/mold`
- **Configure Cargo**: Set up user-level cargo configuration to use mold
- **Verify**: Check that installed binary is executable and correct version
- **Cleanup**: Remove temporary files

### 5. Update Logic (`update`)
- Check current installed version vs latest GitHub release
- If different, run install process (which will overwrite existing binary)
- Preserve cargo configuration during updates
- Handle case where mold isn't currently installed

### 6. Status Logic (`status`)
- Check if `{CARGO_HOME}/bin/mold` exists and is executable
- Run `mold --version` to get current version
- Compare with latest available version from GitHub
- Check cargo configuration status
- Verify dependencies (clang) are available
- Display comprehensive installation status and version information

### 7. Uninstall Logic (`uninstall`)
- Remove `{CARGO_HOME}/bin/mold` if it exists
- Clean up cargo configuration (remove mold settings)
- Provide feedback on success/failure

## Cargo Configuration Management

### Location
User-level configuration at `~/.cargo/config.toml`

### Configuration Content
```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = [
    "-C", "link-arg=-fuse-ld=mold",
]
```

### Updated Installation Logic (`install`)
After successful mold binary installation:
1. **Check for clang**: Verify `clang` is available in PATH (required for our config approach)
2. **Backup existing config**: If `~/.cargo/config.toml` exists, create backup with timestamp
3. **Create/update config**: 
   - Parse existing config.toml if present
   - Add/update the `[target.x86_64-unknown-linux-gnu]` section with mold configuration
   - Preserve any existing configuration for other targets/settings
4. **Validation**: Test that cargo can find mold through the configuration

### Updated Status Logic (`status`)
Extended status checking:
1. Check if mold binary is installed and executable
2. Check if `~/.cargo/config.toml` exists and contains mold configuration
3. Verify `clang` is available (required for our config)
4. Display comprehensive status:
   - Mold version installed
   - Cargo configuration status (configured/not configured)
   - Dependencies status (clang available/missing)

### Updated Uninstall Logic (`uninstall`)
Complete cleanup process:
1. **Remove mold binary**: Delete `{CARGO_HOME}/bin/mold`
2. **Clean cargo config**: 
   - Parse `~/.cargo/config.toml` 
   - Remove mold-specific configuration from `[target.x86_64-unknown-linux-gnu]` section
   - If section becomes empty, remove the entire section
   - If config file becomes empty, remove the file entirely
   - Restore from backup if user prefers
3. **Cleanup verification**: Ensure cargo no longer tries to use mold

## New Helper Functions to Add

### `get_cargo_config_path() -> AnyResult<PathBuf>`
- Return `~/.cargo/config.toml` path
- Handle home directory resolution across platforms

### `backup_cargo_config() -> AnyResult<Option<PathBuf>>`
- Create timestamped backup of existing config
- Return backup path if backup was created

### `update_cargo_config() -> AnyResult<()>`
- Parse existing TOML or create new structure
- Add mold linker configuration for Linux x86_64
- Write updated config back to file
- Use `toml` crate (available in rmx-profile-std)

### `remove_mold_from_cargo_config() -> AnyResult<()>`
- Parse existing config
- Remove mold-specific settings
- Clean up empty sections
- Write back or remove file if empty

### `check_clang_available() -> bool`
- Check if `clang` command is available in PATH
- Required for our linker configuration approach

### `verify_mold_config() -> AnyResult<bool>`
- Test that cargo can locate mold through configuration
- Could run a simple `cargo check` in a temp project to verify

## Error Handling Strategy

Use `anyhow::Context` for descriptive error messages. Handle specific cases:
- Network failures (GitHub API unreachable)
- Unsupported platforms (non-Linux, non-x86_64)
- Permission errors during installation
- Corrupted downloads
- Missing tools (tar command)
- `clang` not available (provide helpful error message with installation instructions)
- Permission errors writing to `~/.cargo/config.toml`
- TOML parsing errors in existing config
- Backup/restore failures

## Platform Support

- Initially support only Linux x86_64 (as sketched in existing code)
- Future expansion possible for other architectures mold supports

## User Experience Improvements

- **Installation**: Inform user that cargo has been configured to use mold
- **Status**: Show complete setup status including dependencies
- **Uninstall**: Confirm that both mold binary and cargo configuration will be removed
- **Update**: Preserve cargo configuration during mold binary updates

## Implementation Steps

1. Update `Cargo.toml` dependencies 
2. Implement GitHub API functions (`get_current_release`, `get_url`)
3. Implement path management (`get_cargo_bin_dir`, `get_cargo_config_path`)
4. Implement cargo config helpers (`update_cargo_config`, `remove_mold_from_cargo_config`, etc.)
5. Implement `install()` with binary installation + cargo configuration
6. Implement `status()` with comprehensive status checking
7. Implement `update()` (preserves cargo config, updates binary)
8. Implement `uninstall()` with complete cleanup
9. Add dependency checking (`check_clang_available`)
10. Add comprehensive error handling and user feedback
11. Test full workflow including cargo compilation with mold

## Expected User Workflow

```bash
# Install mold and configure cargo automatically
rustmax install-tool mold

# Check installation status
rustmax tool-status mold

# Update to latest version
rustmax update-tool mold

# Remove mold and clean up cargo configuration
rustmax uninstall-tool mold
```

This plan leverages the existing rustmax infrastructure while providing a robust, user-friendly mold linker management system that ensures complete setup and cleanup.