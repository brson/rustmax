Floating-point math functions for `no_std`.

- Crate [`::libm`].
- [docs.rs](https://docs.rs/libm)
- [crates.io](https://crates.io/crates/libm)
- [GitHub](https://github.com/rust-lang/compiler-builtins/tree/master/libm)

---

`libm` is a pure-Rust port of the C library's math routines,
maintained by the Rust project alongside `compiler-builtins`.

The interesting math operations on `f32` and `f64` —
[`sin`], [`exp`], [`pow`], [`sqrt`], and friends —
are inherent methods in `std` only.
They are absent from `core` because the standard library
implements them by calling the platform's C `libm`,
which a `no_std` target may not have.
Code written against `core` therefore needs this crate instead.
Its other use is reproducibility:
`libm` computes the same results on every platform,
while the system `libm` may not.

## Examples

Math on `f64` and `f32` without `std`:

```rust
// `core` has no `f64::sqrt`, so call the free function.
let x = libm::sqrt(2.0);
assert!((x - 1.414213562373095).abs() < 1e-15);

// The `f32` variants carry an `f` suffix, as in C.
let y = libm::sqrtf(2.0);
assert!((y - 1.4142135_f32).abs() < 1e-6);
```

Dispatching by float width with [`Libm`]:

```rust
use libm::Libm;

// `Libm<T>` selects the suffixed function matching `T`.
assert_eq!(Libm::<f64>::sqrt(4.0), libm::sqrt(4.0));
assert_eq!(Libm::<f32>::sqrt(4.0), libm::sqrtf(4.0));
```

[`sin`]: crate::libm::sin
[`exp`]: crate::libm::exp
[`pow`]: crate::libm::pow
[`sqrt`]: crate::libm::sqrt
[`sqrtf`]: crate::libm::sqrtf
[`Libm`]: crate::libm::Libm
