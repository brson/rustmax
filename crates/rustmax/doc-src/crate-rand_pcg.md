PCG family of fast, non-cryptographic random number generators.

- Crate [`::rand_pcg`].
- [docs.rs](https://docs.rs/rand_pcg)
- [crates.io](https://crates.io/crates/rand_pcg)
- [GitHub](https://github.com/rust-random/rand)

---

`rand_pcg` provides random number generators based on the
[PCG family](https://www.pcg-random.org/) of algorithms.
These are fast, have good statistical quality, and are portable &mdash;
given the same seed they produce identical output on all platforms.

They are **not** cryptographically secure.
Use [`rand_chacha`] when security matters.

The main types are:

- [`Pcg32`] -- 32-bit output, 64-bit state. Good general-purpose default.
- [`Pcg64`] -- 64-bit output, 128-bit state. Better for generating 64-bit values.

## Example

```rust
use rand::SeedableRng;
use rand::Rng;
use rand_pcg::Pcg32;

let mut rng = Pcg32::seed_from_u64(42);

let value: u32 = rng.random();
let in_range: f64 = rng.random_range(0.0..1.0);
let coin: bool = rng.random();
```

[`Pcg32`]: crate::rand_pcg::Pcg32
[`Pcg64`]: crate::rand_pcg::Pcg64
[`rand_chacha`]: crate::rand_chacha
