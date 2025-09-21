# Task task-add-crate: Add a crate to rustmax

**Files to update:**
1. `crates/rustmax/Cargo.toml` - Add dependencies and feature groups
2. `crates/rustmax/src/lib.rs` - Add module reexports with `#![doc = include_str!("../doc-src/crate-NAME.md")]`
3. `crates/rustmax/doc-src/root-docs.md` - Update category table and profile sections
4. `src/crates.json5` - Add crate metadata (category, descriptions)
5. `README.md` - Add to documentation table
6. `crates/rustmax/doc-src/crate-NAME.md` - Create detailed documentation

**Key steps:**
- Add crate to appropriate feature group (usually `rmx-crates-std`)
- Only add feature flags that actually exist (check docs for `default`, `std` features)
- Remove non-existent features from `rmx-feature-default` and `rmx-feature-std`
- Run `cargo check` to verify no feature conflicts
- Use proper descriptions: "Low-level" vs "High-level", match existing patterns

**Common gotchas:**
- Not all crates have `default` or `std` features
- Audio crates go in `rmx-crates-std` (require std)
- Some crates have important features to add to `rmx-feature-more`,
  ask the user.
- Keep alphabetical order in all files
- Test with `cargo check --all-features`
- Test with `just test`

See also processes.md.


# Task task-add-cargo-plugin-cli: Add a cargo plugin to rustmax-cli

`rustmax-cli` has four commands that operate on "tools":
`install-tool`, `uninstall-tool`, `update-tool`, `tool-status`;
the source for which begins in `rustmax-cli/src/main.rs`, `tools.rs` and `impls.rs`.

To add a new cargo plugin:

- Add it to the `Tools` enum if it doesn't already exist.
- Create the `CargoPluginConfig` constant for the tool.
- Fill in any associated match arms in `tools.rs` and `impls.rs`
- Follow the coding pattern that already exists for cargo plugins like `cargo-audit` and `cargo-clean-all`,
  defering to `cargo_plugin_install` etc for the primary logic.
- Think about if these tools have any special considerations for post-install/uninstall actions etc.
  Do they store caches in the home directory that can be delete on uninstall?


# Task task-add-cargo-tool-cli: Add a cargo tool to rustmax-cli

`rustmax-cli` has four commands that operate on "tools":
`install-tool`, `uninstall-tool`, `update-tool`, `tool-status`;
the source for which begins in `rustmax-cli/src/main.rs`, `tools.rs` and `impls.rs`.

To add a new cargo-installable tool that is not a cargo plugin:

- Add it to the `Tools` enum if it doesn't already exist.
- Create the `CargoToolConfig` constant for the tool.
- Fill in any associated match arms in `tools.rs` and `impls.rs`
- Follow the coding pattern that already exists for cargo programs like `basic-http-server` and `ripgrep`,
  defering to `cargo_tool_install` etc for the primary logic.
- Think about if these tools have any special considerations for post-install/uninstall actions etc.
  Do they store caches in the home directory that can be delete on uninstall?


# Task task-add-cargo-tool-to-book: Add a cargo-based tool to the rustmax book

Rust Max documents important Rust tools, most of which can be install by cargo.

These are documented in book/src/tools.md

- Decide which category your tool belongs to
- Add the tool to the index at the top of the file
- Add a corresponding section for the tool to the correct location in the file

When writing documentation try to express
why this tool is significant to the Rust ecosystem.
Write one or two examples of how it is most typically used.
