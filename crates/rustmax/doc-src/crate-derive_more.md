Derive implementations for common traits.

- Crate [`::derive_more`].
- [docs.rs](https://docs.rs/derive_more)
- [crates.io](https://crates.io/crates/derive_more)
- [GitHub](https://github.com/JelteF/derive_more)

---

`derive_more` extends Rust's derive macro system to automatically implement
commonly-used traits for custom types.

When wrapping types inside custom structs or enums,
the implementations of built-in traits like [`Add`], [`Display`], and [`From`] are lost.
`derive_more` provides derives that restore these implementations with minimal boilerplate,
making the newtype pattern ergonomic.

The crate provides derives across several categories:
conversion traits ([`From`], [`Into`], [`TryFrom`], [`IntoIterator`]),
formatting traits ([`Display`] and related),
operator overloading ([`Add`], [`Mul`], [`Deref`]),
and enum utilities that generate helper methods like `is_variant()`.

Key features include automatic operator implementations for wrapper types,
flexible [`Display`] formatting with format strings,
and comprehensive support for both structs and enums.


## Examples

Deriving arithmetic operators for a newtype:

```rust
use derive_more::{Add, From};

#[derive(Add, From, Debug, PartialEq)]
struct MyInt(i32);

let a = MyInt(10);
let b = MyInt(20);
let result = a + b;
assert_eq!(result, MyInt(30));
```

Deriving [`Display`] with custom formatting:

```rust
use derive_more::Display;

#[derive(Display, Debug)]
#[display("Point({x}, {y})")]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 10, y: 20 };
assert_eq!(format!("{}", p), "Point(10, 20)");
```

[`Add`]: crate::std::ops::Add
[`Mul`]: crate::std::ops::Mul
[`Deref`]: crate::std::ops::Deref
[`Display`]: crate::std::fmt::Display
[`From`]: crate::std::convert::From
[`Into`]: crate::std::convert::Into
[`TryFrom`]: crate::std::convert::TryFrom
[`IntoIterator`]: crate::std::iter::IntoIterator
