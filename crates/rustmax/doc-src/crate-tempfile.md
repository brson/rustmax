Temporary files and directories with automatic cleanup.

- Crate [`::tempfile`].
- [docs.rs](https://docs.rs/tempfile)
- [crates.io](https://crates.io/crates/tempfile)
- [GitHub](https://github.com/Stebalien/tempfile)

---

`tempfile` provides secure, cross-platform temporary file and directory management
with automatic cleanup when the handles are dropped.

Managing temporary files correctly on many platforms involves a lot of technical
tradeoffs and there are many ways to do it poorly.
This crate is ancient and battle tested and well documented.

## Examples

Creating and using a temporary directory:

```rust
use tempfile::TempDir;
use std::fs::File;
use std::io::Write;

let dir = TempDir::with_prefix("myapp-tests")?;

// Create files inside the temporary directory
let file_path = dir.path().join("test.txt");
let mut file = File::create(&file_path)?;
writeln!(file, "temporary data")?;

assert!(file_path.exists());

// Directory and all contents are deleted when dropped
drop(dir);
assert!(!file_path.exists());
# Ok::<(), std::io::Error>(())
```
