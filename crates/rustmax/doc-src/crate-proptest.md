Property-based testing framework for Rust.

- Crate [`::proptest`].
- [docs.rs](https://docs.rs/proptest)
- [crates.io](https://crates.io/crates/proptest)
- [GitHub](https://github.com/proptest-rs/proptest)

---

`proptest` is a property-based testing framework inspired by QuickCheck and
Hypothesis. Instead of writing tests with specific hardcoded inputs,
proptest generates many random test inputs to verify that properties
of your code hold across a wide range of cases.

The crate automatically generates test data using [`Strategy`] types,
runs tests with the [`proptest!`] macro, and when tests fail it
shrinks the failing input to find the minimal test case that reproduces
the failure. This helps discover edge cases and boundary conditions
that might not be obvious from example-based testing.

Key features include strategies for generating primitives, collections,
and custom types, the [`prop_assert!`] family of assertion macros,
and automatic test case shrinking to minimal failing examples.

Proptest integrates with Rust's standard test harness and works
alongside regular unit tests. It's particularly useful for testing
functions with complex input spaces, validating invariants,
and ensuring code behaves correctly across edge cases.

## Examples

Basic property test verifying that reversing a vector twice returns the original:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_double_reverse(v: Vec<i32>) {
        let reversed_twice: Vec<_> = v.iter().cloned().rev().rev().collect();
        prop_assert_eq!(v, reversed_twice);
    }
}
```

Testing that addition is commutative:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_addition_commutative(a: i32, b: i32) {
        let sum1 = a.saturating_add(b);
        let sum2 = b.saturating_add(a);
        prop_assert_eq!(sum1, sum2);
    }
}
```

Using custom strategies to generate constrained data:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_positive_division(a in 1..1000i32, b in 1..1000i32) {
        let result = a / b;
        prop_assert!(result >= 0);
        prop_assert!(result <= a);
    }
}
```

[`Strategy`]: crate::proptest::strategy::Strategy
[`proptest!`]: crate::proptest::proptest
[`prop_assert!`]: crate::proptest::prop_assert
