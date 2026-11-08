## How `rustmax` manages tools

Code: `crates/rustmax-cli/src/{tools,impls,cargo_install,moldman}.rs`.

Every tool except mold is a crates.io package installed with `cargo install`.
Installed versions are read from `.crates2.json` in the install root
(`$CARGO_INSTALL_ROOT`, else `$CARGO_HOME`; the `install.root` config key is ignored),
never by running `<tool> --version`, which is unreliable
(gist exits 1, jsonxf has no flag, rg prints build info).
Latest versions come from the crates.io sparse index,
newest stable unyanked release.
`update-tool` installs `<pkg>@<latest>` only when the installed version is older,
and reinstalls with `--force` when cargo records binaries that are gone.
Packages installed from git or a path are reported and left alone.

Tests: unit tests in `cargo_install.rs` and `impls.rs` run with `cargo test`.
`just test-tools` runs an ignored end-to-end test that installs, updates and uninstalls
`jsonxf` into a temporary `CARGO_INSTALL_ROOT`; it needs the network.


### todo

- [cargo-deny](https://crates.io/crates/cargo-deny) - disallow specific crates, licenses, etc
- [cargo-generate](https://crates.io/crates/cargo-generate) - create cargo projecs from temtplates
- [clippy-control](https://crates.io/crates/clippy-control) - todo
- miri
- [cargo-clean-all](https://crates.io/crates/cargo-clean-all) - clean all projects, leaving recent files
- [cargo-license](https://crates.io/crates/cargo-license) - show licenses of dependencies
- [basic-http-server](https://crates.io/crates/basic-http-server) - serve static HTML and rendered Markdown
- [cargo-audit](https://crates.io/crates/cargo-audit) - find known security vulnerabilities
- [cargo-expand](https://github.com/dtolnay/cargo-expand) - expand macros
- [cargo-hack](https://crates.io/crates/cargo-hack) - build / test with all combinations of cargo features


## Other Tools

- [`basic-http-server`](https://crates.io/crates/basic-http-server) - serve static HTML and rendered Markdown
- [du-dust](https://crates.io/crates/du-dust) - lik du, disk usage
- [fd-find](https://crates.io/crates/fd-find) - convenient `find` replacement
- [gist](https://crates.io/crates/gist) - post gists
- [ripgrep](https://crates.io/crates/ripgrep) - fast and convenient code grep
- [tokei](https://crates.io/crates/tokei) - count lines of code
- just
- [`jaq`](https://crates.io/crates/jaq) - jq clone for querying JSON
- sd
- jsonxf
- jaq
- intentrace?
- jj-cli
- mdbook
- wabt
- cargo-zigbuild
