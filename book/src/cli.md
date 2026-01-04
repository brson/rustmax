# The Rustmax CLI

```
cargo install rustmax-cli
rustmax --help
```

The Rustmax CLI
includes commands for managing Rust tools,
building the Rustmax Library,
the `rmxbook` tool used to build the library,
configurations for linters.

```
Usage: rustmax <COMMAND>

Commands:
  list-tools                   List all available tools
  install-tools                Install all tools
  update-tools                 Update all tools
  uninstall-tools              Uninstall all tools
  tools-status                 Show status of all tools
  install-tool                 Install a specific tool
  update-tool                  Update a specific tool
  uninstall-tool               Uninstall a specific tool
  tool-status                  Show status of a specific tool
  list-library                 List books in the library
  build-library                Build the library or a specific book
  refresh-library              Refresh the library or a specific book
  rmxbook                      Build a book using the rmxbook renderer
  new-project                  Create a new project from template
  write-fmt-config             Write rustfmt.toml configuration file
  write-cargo-deny-config      Write deny.toml configuration file
  write-clippy-control-config  Write clippy-control.toml configuration file
  run-all-checks               Run all code quality checks
  help                         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

