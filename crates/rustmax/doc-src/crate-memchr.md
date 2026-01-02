Fast byte search primitives with SIMD acceleration.

- Crate [`::memchr`].
- [docs.rs](https://docs.rs/memchr)
- [crates.io](https://crates.io/crates/memchr)
- [GitHub](https://github.com/BurntSushi/memchr)

---

`memchr` provides optimized routines for searching bytes in slices,
using SIMD acceleration on supported architectures.
It is significantly faster than naive byte-by-byte searching,
often by an order of magnitude for larger inputs.

The primary functions are [`memchr`] for finding a single byte,
[`memchr2`] and [`memchr3`] for finding one of 2-3 bytes,
and the [`memmem`] module for substring search.

This crate is a foundational building block used by
higher-level crates like [`regex`].

## Examples

Finding a single byte:

```rust
use memchr::memchr;

let haystack = b"hello world";
let needle = b'o';

assert_eq!(memchr(needle, haystack), Some(4));
```

Finding any of multiple bytes:

```rust
use memchr::memchr2;

let haystack = b"hello world";
// Find the first 'l' or 'w'
assert_eq!(memchr2(b'l', b'w', haystack), Some(2));
```

Substring search with the memmem module:

```rust
use memchr::memmem;

let haystack = b"the quick brown fox";
let needle = b"quick";

let finder = memmem::Finder::new(needle);
assert_eq!(finder.find(haystack), Some(4));
```

Finding all occurrences:

```rust
use memchr::memchr_iter;

let haystack = b"hello";
let positions: Vec<_> = memchr_iter(b'l', haystack).collect();
assert_eq!(positions, vec![2, 3]);
```

[`memchr`]: crate::memchr::memchr
[`memchr2`]: crate::memchr::memchr2
[`memchr3`]: crate::memchr::memchr3
[`memmem`]: crate::memchr::memmem
[`regex`]: crate::regex