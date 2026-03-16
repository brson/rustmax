Compile-time conditional compilation as `if` / `else` blocks.

- Crate [`::cfg_if`].
- [docs.rs](https://docs.rs/cfg-if)
- [crates.io](https://crates.io/crates/cfg-if)
- [GitHub](https://github.com/rust-lang/cfg-if)

---

`cfg_if` provides the [`cfg_if!`] macro,
which allows writing `#[cfg]` conditional compilation
in a more readable `if` / `else if` / `else` style
instead of repeating `#[cfg]` attributes with negated conditions.

This is especially useful when selecting between more than two configurations,
where manually writing the correct combination of `not()`, `any()`, and `all()`
becomes error-prone.

## Example

```rust
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(unix)] {
        fn platform() -> &'static str { "unix" }
    } else if #[cfg(windows)] {
        fn platform() -> &'static str { "windows" }
    } else {
        fn platform() -> &'static str { "other" }
    }
}

let p = platform();
assert!(p == "unix" || p == "windows" || p == "other");
```

[`cfg_if!`]: crate::cfg_if::cfg_if
