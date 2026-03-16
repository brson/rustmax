ChaCha-based cryptographically secure random number generators.

- Crate [`::rand_chacha`].
- [docs.rs](https://docs.rs/rand_chacha)
- [crates.io](https://crates.io/crates/rand_chacha)
- [GitHub](https://github.com/rust-random/rand)

---

`rand_chacha` provides random number generators based on the [ChaCha stream cipher](https://cr.yp.to/chacha.html).
These are cryptographically secure and portable --
given the same seed they produce identical output on all platforms.

Three variants are available, differing in the number of ChaCha rounds:

- [`ChaCha20Rng`] -- 20 rounds, the standard choice for cryptographic security.
- [`ChaCha12Rng`] -- 12 rounds, a good default balancing security and speed.
- [`ChaCha8Rng`] -- 8 rounds, fastest but with a lower security margin.

*If you neeed a specific, stable, secure
random number generator pick ChaCha12 by default.*

ChaCha20 is tha standard strength algorithm and considered secure.
ChaCha12 is weaker, but expert consensus is that it is also secure,
with no known attacks
(it is the the default algorithm of the `rand` crate's [`StdRng`],
though its algorithm is not specified to be stable).

## Example

```rust
use rand::SeedableRng;
use rand::Rng;
use rand_chacha::ChaCha12Rng;

let mut rng = ChaCha12Rng::seed_from_u64(42);

let value: u32 = rng.random();
let in_range: f64 = rng.random_range(0.0..1.0);
let coin: bool = rng.random();
```

[`ChaCha20Rng`]: crate::rand_chacha::ChaCha20Rng
[`ChaCha12Rng`]: crate::rand_chacha::ChaCha12Rng
[`ChaCha8Rng`]: crate::rand_chacha::ChaCha8Rng
[`StdRng`]: crate::rand::rngs::StdRng
