# Agent instructions for {{project-name}}

This project uses rustmax, a crate that re-exports a curated set of
crates.io crates under `rmx::`. Writing code as if those crates were
direct dependencies, or from memory of their older APIs, is the most
common source of errors here.

Before writing Rust code in this project, read the rustmax guide for
coding agents. Run `rustmax agents` to print it, or, if the `rustmax`
CLI is not installed, read <https://rustmax.net/agent-guide.md>.

The guide's essentials:

- Start every module with `use rmx::prelude::*;`.
- Reach third-party crates through `rmx::<crate>`. Do not add a direct
  dependency on a crate that rustmax re-exports. The available crates
  are set by `rmx.features` in the workspace `Cargo.toml`.
- Derive through `#[rmx::derive(..)]` when a derive comes from a
  re-exported crate, such as serde's `Serialize`.
- Return `AnyResult<T>` from fallible code, using `bail!`, `anyhow!`
  and `.context(..)`.


## Layout

- `crates/{{project-name}}`: the library.
- `crates/{{project-name}}-cli`: the command-line binary, the
  workspace's default member.


## Checks

Before reporting a task as done, run:

```sh
cargo build --workspace
cargo test --workspace
```
