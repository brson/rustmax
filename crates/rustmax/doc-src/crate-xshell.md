Shell-like scripting in Rust without invoking a shell.

- Crate [`::xshell`].
- [docs.rs](https://docs.rs/xshell)
- [crates.io](https://crates.io/crates/xshell)
- [GitHub](https://github.com/matklad/xshell)

---

`xshell` provides ergonomic subprocess execution with shell-like syntax.
It executes commands directly without spawning a shell process,
preventing shell injection vulnerabilities while providing
variable interpolation in the `cmd!` macro.

Useful for build scripts, CLI tools, and automation tasks.

## Examples

Running commands with variable interpolation:

```
use xshell::{cmd, Shell};

let sh = Shell::new()?;

let name = "world";
let output = cmd!(sh, "echo hello {name}").read()?;
assert!(output.contains("hello world"));
# Ok::<(), xshell::Error>(())
```

Reading files relative to the shell's working directory:

```
use xshell::Shell;
use std::path::PathBuf;

let sh = Shell::new()?;

// Read a file relative to cwd
let cargo_toml = sh.read_file("Cargo.toml")?;
assert!(cargo_toml.contains("[package]"));
# Ok::<(), xshell::Error>(())
```
