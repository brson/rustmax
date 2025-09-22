Extra iterator methods and utilities.

- Crate [`::itertools`].
- [docs.rs](https://docs.rs/itertools)
- [crates.io](https://crates.io/crates/itertools)
- [GitHub](https://github.com/rust-itertools/itertools)

---

`itertools` extends Rust's iterator functionality with a rich collection
of additional iterator methods and utilities.

The crate provides the [`Itertools`] trait that adds dozens of useful methods
to any iterator, enabling powerful functional programming patterns.
It includes methods for grouping, batching, sorting, deduplication,
and complex iteration patterns that would otherwise require verbose loops.

Key features include [`group_by`] for grouping consecutive elements,
[`chunk`] for batching elements into fixed-size groups,
[`sorted`] for sorting without collecting first,
[`dedup`] for removing duplicates,
[`intersperse`] for inserting separators,
and [`cartesian_product`] for combining iterators.

The crate also provides standalone functions like [`zip_eq`] for
strict zipping that panics on length mismatches,
and [`repeat_n`] for creating repeating sequences.

## Examples

Grouping consecutive elements:

```rust
use itertools::Itertools;

let data = vec![1, 1, 2, 2, 2, 3, 1, 1];
let groups: Vec<_> = data
    .into_iter()
    .group_by(|&x| x)
    .into_iter()
    .map(|(key, group)| (key, group.count()))
    .collect();

assert_eq!(groups, [(1, 2), (2, 3), (3, 1), (1, 2)]);
```

Batching and processing in chunks:

```rust
use itertools::Itertools;

let data = 1..=10;
let chunks: Vec<Vec<_>> = data.chunks(3).into_iter().map(|chunk| chunk.collect()).collect();

assert_eq!(chunks, [vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10]]);
```

Combining iterators with cartesian product:

```rust
use itertools::Itertools;

let coords: Vec<_> = (0..3)
    .cartesian_product(0..3)
    .collect();

assert_eq!(coords.len(), 9);
assert_eq!(coords[0], (0, 0));
assert_eq!(coords[8], (2, 2));
```

[`Itertools`]: crate::itertools::Itertools
[`group_by`]: crate::itertools::Itertools::group_by
[`chunk`]: crate::itertools::Itertools::chunks
[`sorted`]: crate::itertools::Itertools::sorted
[`dedup`]: crate::itertools::Itertools::dedup
[`intersperse`]: crate::itertools::Itertools::intersperse
[`cartesian_product`]: crate::itertools::Itertools::cartesian_product
[`zip_eq`]: crate::itertools::zip_eq
[`repeat_n`]: crate::itertools::repeat_n