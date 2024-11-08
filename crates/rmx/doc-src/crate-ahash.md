A fast and DOS-resistent hash function, for use in `HashMap`s.

- Crate [`::ahash`].
- [docs.rs](https://docs.rs/ahash)
- [cargo.io](https://crates.io/crates/ahash)
- [GitHub](https://github.com/tkaitchuck/ahash)

---

The Rust standard [`HashMap`] (and [`HashSet`])
use a pluggable hash function defined by the standard [`Hasher`] trait.
The default hasher used by Rust's `HashMap` is [SipHash 1-3],
which provides strong resistence to
[denial-of-service (DOS) attacks against hash maps][dos].

SipHash is relatively slow though,
particularly for small keys like integers,
so it is common for programs to use a different
implementation of `Hasher`.

[`ahash`] is both DOS-resistent,
and fast enough for almost all uses of hash maps.

It comes with tradeoffs though,
which is probably why it is not used in the standard library.

`ahash` does not have a fixed hash function -
it is version- and platform-specific,
so it is only suitable for use in-memory,
and not for making stable comparisons across systems
(t is primarly for use in Rust's `HashMap` and `HashSet`).
On x86 it makes use of [AES-NI] instructions for performance.

Although `ahash` can be used in no-std contexts,
the standard Rust `HashMap` cannot,
as it lives in the `std` crate.
For `no-std` hash maps use the [`hashbrown`] crate.

## Examples

Construct a standard [`HashMap`] with an [`AHasher`]
by using the [`AHashMap`] type alias.

```
use ahash::AHashMap;

let mut map: AHashMap<i32, i32> = AHashMap::new();
map.insert(12, 34);
map.insert(56, 78);
```




[`HashMap`]: crate::std::collections::HashMap
[`HashSet`]: crate::std::collections::HashSet
[`Hasher`]: crate::std::hash::Hasher
[SipHash 1-3]: todo
[dos]: todo
[AES-NI]: todo
[`hashbrown`]: https://docs.rs/hashbrown
