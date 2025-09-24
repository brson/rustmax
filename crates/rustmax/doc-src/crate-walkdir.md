Efficient directory traversal.

- Crate [`::walkdir`].
- [docs.rs](https://docs.rs/walkdir)
- [crates.io](https://crates.io/crates/walkdir)
- [GitHub](https://github.com/BurntSushi/walkdir)

---

`walkdir` provides efficient recursive directory traversal with customizable behavior.
It handles symlinks, sorting, filtering, and error handling,
and is the standard for directory walking in Rust.

## Examples

Basic directory traversal:

```
use walkdir::WalkDir;

for entry in WalkDir::new("src") {
    let entry = entry.unwrap();
    println!("{}", entry.path().display());
}
```

Filtering and controlling traversal:

```
use walkdir::WalkDir;

for entry in WalkDir::new(".")
    .max_depth(2)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
{
    println!("Rust file: {}", entry.path().display());
}
```

[`WalkDir`]: crate::walkdir::WalkDir
[`new`]: crate::walkdir::WalkDir::new
[`max_depth`]: crate::walkdir::WalkDir::max_depth