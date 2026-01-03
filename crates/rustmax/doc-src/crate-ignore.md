Fast directory traversal respecting gitignore rules.

- Crate [`::ignore`].
- [docs.rs](https://docs.rs/ignore)
- [crates.io](https://crates.io/crates/ignore)
- [GitHub](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore)

---

`ignore` provides recursive directory traversal
that automatically respects `.gitignore` and `.ignore` files.
It is the library that powers file matching in ripgrep.

The main entry point is [`WalkBuilder`],
which configures and creates a directory walker.
For simple cases, [`Walk`] provides an iterator over matching files.
For parallel traversal, use [`WalkParallel`].

This crate is useful when you need to walk a directory tree
while respecting the same ignore rules that git uses.

## Examples

Basic directory traversal respecting gitignore:

```rust
use ignore::Walk;

for result in Walk::new("./") {
    match result {
        Ok(entry) => println!("{}", entry.path().display()),
        Err(err) => eprintln!("error: {}", err),
    }
}
```

Customizing the walker with WalkBuilder:

```rust
use ignore::WalkBuilder;

let walker = WalkBuilder::new("./")
    .hidden(false)        // Include hidden files
    .git_ignore(true)     // Respect .gitignore
    .max_depth(Some(3))   // Limit depth
    .build();

for result in walker {
    if let Ok(entry) = result {
        println!("{}", entry.path().display());
    }
}
```

[`WalkBuilder`]: crate::ignore::WalkBuilder
[`Walk`]: crate::ignore::Walk
[`WalkParallel`]: crate::ignore::WalkParallel
