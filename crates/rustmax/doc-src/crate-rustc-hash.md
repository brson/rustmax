A fast, non-cryptographic hash function, used by `rustc`.

- Crate [`::rustc_hash`].
- [docs.rs](https://docs.rs/rustc-hash)
- [crates.io](https://crates.io/crates/rustc-hash)
- [GitHub](https://github.com/rust-lang/rustc-hash)

---

`rustc-hash` provides [`FxHasher`],
the hash function the Rust compiler uses for its own hash maps,
and the [`FxHashMap`] and [`FxHashSet`] aliases for using it
with the standard collections.

`FxHasher` is faster than the standard [`HashMap`]'s SipHash,
particularly for small keys like integers and short strings.
It is not resistant to [denial-of-service attacks against hash maps][dos],
so it should only hash keys an attacker cannot choose.
Use [`ahash`] when keys come from untrusted input.

Unlike [`ahash`], `FxHasher` is not randomly seeded.
The same keys hash the same way on every run.
That makes iteration order repeatable between runs of a program,
which helps when reproducing bugs,
though programs should not depend on it.
Hashes can still differ between platforms and crate versions.

`rustc-hash` works in no-std;
there, use [`FxBuildHasher`] with [`hashbrown`]'s maps.

## Examples

Using [`FxHashMap`] in place of [`HashMap`].
Construct it with `default`, since `new` is only defined
for the standard hasher:

```rust
use rustc_hash::{FxHashMap, FxHashSet};

let mut map: FxHashMap<u32, &str> = FxHashMap::default();
map.insert(1, "one");
map.insert(2, "two");
assert_eq!(map.get(&1), Some(&"one"));

let set: FxHashSet<&str> = ["a", "b", "a"].into_iter().collect();
assert_eq!(set.len(), 2);
```

Using [`FxBuildHasher`] with a capacity:

```rust
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;

let mut map = HashMap::with_capacity_and_hasher(16, FxBuildHasher);
map.insert("key", 42);
assert_eq!(map["key"], 42);
```


[`FxHasher`]: crate::rustc_hash::FxHasher
[`FxHashMap`]: crate::rustc_hash::FxHashMap
[`FxHashSet`]: crate::rustc_hash::FxHashSet
[`FxBuildHasher`]: crate::rustc_hash::FxBuildHasher
[`HashMap`]: crate::std::collections::HashMap
[`ahash`]: crate::ahash
[dos]: https://lwn.net/Articles/474912/
[`hashbrown`]: https://docs.rs/hashbrown
