# Task add-cargo-plugin: Add a cargo plugin to rustmax-cli

`rustmax-cli` has four commands that operate on "tools":
`install-tool`, `uninstall-tool`, `update-tool`, `tool-status`;
the source for which begins in `rustmax-cli/src/main.rs`, `tools.rs` and `impls.rs`.

To add a new cargo plugin:

- Add it to the `Tools` enum if it doesn't already exist.
- Fill in any associated match arms in `tools.rs` and `impls.rs`
- Follow the coding pattern that already exists for cargo plugins like `cargo-audit` and `cargo-clean-all`,
  defering to `cargo_plugin_install` etc for the primary logic.
- Think about if these tools have any special considerations for post-install/uninstall actions etc.
  Do they store caches in the home directory that can be delete on uninstall?

