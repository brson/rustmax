# Task add-cargo-plugin-cli: Add a cargo plugin to rustmax-cli

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


# Task add-cargo-tool-cli: Add a cargo tool to rustmax-cli

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


# Task add-cargo-tool-to-book: Add a cargo-based tool to the rustmax book

Rust Max documents important Rust tools, most of which can be install by cargo.

These are documented in book/src/tools.md

- Decide which category your tool belongs to
- Add the tool to the index at the top of the file
- Add a corresponding section for the tool to the correct location in the file

When writing documentation try to express
why this tool is significant to the Rust ecosystem.
Write one or two examples of how it is most typically used.
