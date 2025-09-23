Arbitrary precision integers.

- Crate [`::num_bigint`].
- [docs.rs](https://docs.rs/num-bigint)
- [crates.io](https://crates.io/crates/num-bigint)
- [GitHub](https://github.com/rust-num/num-bigint)

---

`num-bigint` provides [`BigInt`] and [`BigUint`] types
for arbitrary precision integer arithmetic.
These types can represent integers of any size,
limited only by available memory.

[`BigInt`] is a signed arbitrary precision integer,
while [`BigUint`] is an unsigned arbitrary precision integer.
Both support the standard arithmetic operations
and can be converted to and from standard integer types
and strings.

The types implement many of the same traits
as the built-in integer types,
making them easy to use as drop-in replacements
when you need larger integers.

## Examples

Basic arithmetic with big integers:

```
use num_bigint::{BigInt, BigUint, ToBigInt};

// Create big integers from standard types
let a = BigInt::from(42);
let b = "123456789012345678901234567890".parse::<BigInt>().unwrap();

// Arithmetic operations work as expected
let sum = &a + &b;
let product = &a * &b;

// Convert back to strings for display
println!("Sum: {}", sum);
println!("Product: {}", product);
```

Computing large factorials:

```
use num_bigint::BigUint;

fn factorial(n: u32) -> BigUint {
    (1..=n).map(BigUint::from).fold(BigUint::from(1u32), |acc, x| acc * x)
}

// Compute 100! (which is very large)
let result = factorial(100);
println!("100! = {}", result);
```

[`BigInt`]: crate::num_bigint::BigInt
[`BigUint`]: crate::num_bigint::BigUint