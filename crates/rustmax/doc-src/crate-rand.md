Random number generation.

- Crate [`::rand`].
- [docs.rs](https://docs.rs/rand)
- [crates.io](https://crates.io/crates/rand)
- [GitHub](https://github.com/rust-random/rand)

---

`rand` provides utilities for generating random numbers,
converting them to useful types and distributions,
and working with random number generators.
It is the core random number generation library for the Rust ecosystem.

The main entry points are [`rng`] for a thread-local random number generator,
and the [`Rng`] trait which provides methods for generating random values.
The [`random`] function provides a convenient way to generate
a single random value using the thread-local generator.

The crate provides different random number generators:
[`StdRng`] is a cryptographically secure generator suitable for most applications,
while [`SmallRng`] is optimized for speed when security is not required.
All generators implement the [`RngCore`] trait and can be seeded
through the [`SeedableRng`] trait for reproducible results,
though their specific algorithm is not guaranteed.

When a specific RNG algorithm is needed prefer:

- [`ChaCha12Rng`] from the [`rand_chacha`] crate when in doubt.
- [`Pcg64`] from the [`rand_pcg`] crate when speed is needed and the security impact is understood.


## Examples

Generating basic random values:

```
use rand::{random, rng, Rng};

// Generate a random boolean
let coin_flip: bool = random();

// Generate a random number in a range
let mut rng = rng();
let dice_roll = rng.random_range(1..=6);
println!("Rolled: {}", dice_roll);

// Generate random elements from different types
let random_float: f64 = rng.random();
let random_char: char = rng.random_range('a'..='z');
```

Working with collections:

```
use rand::{prelude::*, rng};

let mut rng = rng();
let mut numbers = vec![1, 2, 3, 4, 5];

// Shuffle a collection
numbers.shuffle(&mut rng);

// Choose a random element
if let Some(&chosen) = numbers.choose(&mut rng) {
    println!("Randomly chose: {}", chosen);
}

// Sample multiple elements without replacement
let samples: Vec<&i32> = numbers.choose_multiple(&mut rng, 3).collect();
println!("Random sample: {:?}", samples);
```

Using different generators and seeding for reproducible results:

```
use rand::{Rng, SeedableRng, rngs::{StdRng, SmallRng}};

// Seed generators for reproducible results
let mut std_rng = StdRng::seed_from_u64(42);
let mut small_rng = SmallRng::seed_from_u64(42);

// Both will produce the same sequence when seeded identically
let std_value: u32 = std_rng.random();
let small_value: u32 = small_rng.random();

// StdRng is cryptographically secure but slower
let secure_random: u64 = std_rng.random();

// SmallRng is faster but not cryptographically secure
let fast_random: u64 = small_rng.random();

// Use thread_rng for non-reproducible results
use rand::rng;
let mut entropy_rng = rng();
let unpredictable: f64 = entropy_rng.random();
```

[`rng`]: crate::rand::rng
[`Rng`]: crate::rand::Rng
[`random`]: crate::rand::random
[`RngCore`]: crate::rand::RngCore
[`SeedableRng`]: crate::rand::SeedableRng
[`StdRng`]: crate::rand::rngs::StdRng
[`SmallRng`]: crate::rand::rngs::SmallRng
[`rand_chacha`]: crate::rand_chacha
[`rand_pcg`]: crate::rand_pcg
[`ChaCha12Rng`]: crate::rand_chacha::ChaCha12Rng
[`Pcg64`]: crate::rand_pcg::Pcg64
