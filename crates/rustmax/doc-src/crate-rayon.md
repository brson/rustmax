Data parallelism library for Rust.

- Crate [`::rayon`].
- [docs.rs](https://docs.rs/rayon)
- [crates.io](https://crates.io/crates/rayon)
- [GitHub](https://github.com/rayon-rs/rayon)

---

`rayon` is a data-parallelism library for Rust that makes it easy to convert
sequential computations into parallel ones with minimal changes to existing code.

The crate provides parallel versions of common iterator methods through the
[`ParallelIterator`] trait, allowing you to simply change `.iter()` to `.par_iter()`
to parallelize operations. Rayon uses work-stealing to efficiently distribute
computations across CPU cores.

Key features include parallel iteration with [`par_iter`], parallel collection
operations like [`map`], [`filter`], and [`reduce`], parallel sorting with
[`par_sort`], and parallel searching. The library also provides [`join`] for
fork-join parallelism and [`scope`] for structured parallelism with lifetimes.

Rayon automatically manages thread pools and work distribution, making
parallelism accessible without manual thread management. It's particularly
effective for CPU-bound tasks that can be decomposed into independent units.

Rayon's global [`ThreadPool`] is the recommended threadpool implementation
in rustmax for general-purpose parallel computation. It provides excellent
work-stealing performance and integrates seamlessly with rayon's parallel
iterators.

## Examples

Basic parallel iteration:

```rust
use rayon::prelude::*;

let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
let sum: i32 = data.par_iter().map(|&x| x * x).sum();

assert_eq!(sum, 204); // 1 + 4 + 9 + 16 + 25 + 36 + 49 + 64
```

Parallel filtering and collection:

```rust
use rayon::prelude::*;

let numbers = (1..100).collect::<Vec<_>>();
let evens: Vec<_> = numbers
    .par_iter()
    .filter(|&&n| n % 2 == 0)
    .map(|&n| n * 2)
    .collect();

assert_eq!(evens[0], 4);  // 2 * 2
assert_eq!(evens[1], 8);  // 4 * 2
```

Parallel reduction with custom operation:

```rust
use rayon::prelude::*;

let data = vec![1, 5, 3, 9, 2, 8, 4];
let max = data.par_iter().cloned().reduce(|| 0, |a, b| a.max(b));

assert_eq!(max, 9);
```

Using rayon's threadpool for custom parallel work:

```rust
use rayon::ThreadPoolBuilder;

let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

let result = pool.install(|| {
    // This closure runs on the custom threadpool
    (0..1000).into_iter().map(|i| i * i).sum::<i32>()
});

assert_eq!(result, 332833500);
```

[`ParallelIterator`]: crate::rayon::iter::ParallelIterator
[`par_iter`]: crate::rayon::slice::ParallelSlice
[`map`]: crate::rayon::iter::ParallelIterator::map
[`filter`]: crate::rayon::iter::ParallelIterator::filter
[`reduce`]: crate::rayon::iter::ParallelIterator::reduce
[`par_sort`]: crate::rayon::slice::ParallelSliceMut::par_sort
[`join`]: crate::rayon::join
[`scope`]: crate::rayon::scope
[`ThreadPool`]: crate::rayon::ThreadPool