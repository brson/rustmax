Concise single-letter methods for common operations.

- Crate [`::powerletters`].
- [docs.rs](https://docs.rs/powerletters)
- [crates.io](https://crates.io/crates/powerletters)
- [GitHub](https://github.com/brson/powerletters)

---

`powerletters` provides single-letter methods for common Rust operations,
offering concise alternatives to standard trait methods.

The crate defines extension traits that add methods like [`C`] for cloning,
[`O`] for converting to owned types, [`S`] for converting to strings,
[`I`] for ignoring results, and [`X`] for expecting values.

These methods can be used both as method calls on values
and as standalone functions,
providing flexibility in different coding contexts.

## Examples

Cloning with `C`:

```rust
use powerletters::C;

let vec = vec![1, 2, 3];
let cloned = vec.C();
assert_eq!(vec, cloned);
```

Converting to string with `S`:

```rust
use powerletters::S;

let num = 42;
let s = num.S();
assert_eq!(s, "42");
```

Ignoring a result with `I`:

```rust
use powerletters::I;
use std::io::Write;

let mut vec = Vec::new();
write!(vec, "hello").I();
```

[`C`]: crate::powerletters::C
[`O`]: crate::powerletters::O
[`S`]: crate::powerletters::S
[`I`]: crate::powerletters::I
[`X`]: crate::powerletters::X
