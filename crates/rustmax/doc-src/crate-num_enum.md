Type-safe conversions between enums and primitive numbers.

- Crate [`::num_enum`].
- [docs.rs](https://docs.rs/num_enum)
- [crates.io](https://crates.io/crates/num_enum)
- [GitHub](https://github.com/illicitonion/num_enum)

---

`num_enum` provides procedural macros for deriving type-safe conversions
between Rust enums and primitive numeric types.

While Rust's `as` operator can convert enums to numbers,
it can silently truncate values and doesn't provide safe conversions
in the reverse direction.
`num_enum` fills this gap with derive macros that generate
conversion implementations with proper error handling.

The primary derives are [`IntoPrimitive`] for converting enums to numbers,
and [`TryFromPrimitive`] for fallible conversion from numbers to enums.
For cases where all numeric values map to enum variants,
[`FromPrimitive`] provides infallible conversion,
and [`UnsafeFromPrimitive`] offers unsafe transmutation for performance-critical code.

## Examples

Basic conversion between enums and numbers:

```rust
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum Status {
    Success = 0,
    Warning = 1,
    Error = 2,
}

// Convert enum to number.
let code: u8 = Status::Warning.into();
assert_eq!(code, 1);

// Convert number to enum.
let status = Status::try_from(2).unwrap();
assert_eq!(status, Status::Error);

// Invalid conversions return an error.
assert!(Status::try_from(99).is_err());
```

[`IntoPrimitive`]: https://docs.rs/num_enum/latest/num_enum/derive.IntoPrimitive.html
[`TryFromPrimitive`]: https://docs.rs/num_enum/latest/num_enum/derive.TryFromPrimitive.html
[`FromPrimitive`]: https://docs.rs/num_enum/latest/num_enum/derive.FromPrimitive.html
[`UnsafeFromPrimitive`]: https://docs.rs/num_enum/latest/num_enum/derive.UnsafeFromPrimitive.html
