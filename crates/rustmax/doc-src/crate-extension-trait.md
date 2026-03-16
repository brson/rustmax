A macro for defining extension methods on external types.

- Crate [`::extension_trait`].
- [docs.rs](https://docs.rs/extension-trait)
- [crates.io](https://crates.io/crates/extension-trait)
- [GitHub](https://github.com/dureuill/extension-trait)

---

`extension_trait` provides the [`extension_trait`] attribute macro,
which generates a trait and its implementation from a single `impl` block.
This is the standard Rust pattern for adding methods to types
defined in other crates, but without the boilerplate
of writing a matching trait definition by hand.

## Example

```rust
use extension_trait::extension_trait;

#[extension_trait]
pub impl IntExt for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }

    fn double(&self) -> i32 {
        self * 2
    }
}

assert!(4.is_even());
assert_eq!(3.double(), 6);
```

[`extension_trait`]: crate::extension_trait::extension_trait
