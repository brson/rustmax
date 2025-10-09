Unix shell style pattern matching.

- Crate [`::glob`].
- [docs.rs](https://docs.rs/glob)
- [crates.io](https://crates.io/crates/glob)
- [GitHub](https://github.com/rust-lang/glob)

---

`glob` provides support for matching file paths against Unix shell style patterns.
The crate offers filesystem iteration with pattern matching,
similar to the `glob` function in libc,
as well as direct pattern matching against individual paths,
similar to `fnmatch`.

The main entry points are the [`glob`] and [`glob_with`] functions,
which return iterators over filesystem paths matching a pattern.
For direct pattern matching without filesystem access,
the [`Pattern`] type can be compiled and used to test individual paths.

Glob patterns support wildcards like `*` for any sequence of characters,
`?` for any single character,
and `**` for recursive directory matching.
Character sets with `[]` and brace expansion with `{}` are also supported.

## Examples

Finding all Rust source files recursively:

```
use glob::glob;

for entry in glob("**/*.rs").expect("Failed to read glob pattern") {
    match entry {
        Ok(path) => println!("{:?}", path.display()),
        Err(e) => println!("{:?}", e),
    }
}
```

Matching paths against a pattern:

```
use glob::Pattern;

let pattern = Pattern::new("*.txt").unwrap();
assert!(pattern.matches("hello.txt"));
assert!(pattern.matches("document.txt"));
assert!(!pattern.matches("image.png"));
```

Using custom match options:

```
use glob::{glob_with, MatchOptions};

let options = MatchOptions {
    case_sensitive: false,
    ..Default::default()
};

for entry in glob_with("*.TXT", options).expect("Failed to read glob pattern") {
    println!("{:?}", entry);
}
```

[`glob`]: crate::glob::glob
[`glob_with`]: crate::glob::glob_with
[`Pattern`]: crate::glob::Pattern
