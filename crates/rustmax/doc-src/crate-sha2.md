The SHA-2 cryptographic hash functions.

- Crate [`::sha2`].
- [docs.rs](https://docs.rs/sha2)
- [crates.io](https://crates.io/crates/sha2)
- [GitHub](https://github.com/RustCrypto/hashes)

---

`sha2` provides implementations of the SHA-2 family of cryptographic hash functions.

The SHA-2 family includes six hash functions: SHA-224, SHA-256, SHA-384, SHA-512, SHA-512/224, and SHA-512/256.
These are cryptographic hash functions standardized by NIST,
producing fixed-size digests from arbitrary input data.
SHA-256 and SHA-512 are the most commonly used variants.

The crate implements the [`Digest`] trait,
providing both one-shot hashing via [`Digest::digest`]
and incremental hashing via [`Digest::new`], [`Digest::update`], and [`Digest::finalize`].

## Examples

Basic SHA-256 hashing:

```rust
use sha2::{Sha256, Digest};

let result = Sha256::digest(b"hello world");
println!("SHA-256: {:x}", result);
```

Incremental hashing:

```rust
use sha2::{Sha256, Digest};

let mut hasher = Sha256::new();
hasher.update(b"hello ");
hasher.update(b"world");
let result = hasher.finalize();

// Verify it matches one-shot hashing
assert_eq!(result[..], Sha256::digest(b"hello world")[..]);
```

Using different SHA-2 variants:

```rust
use sha2::{Sha512, Digest};

let sha512_result = Sha512::digest(b"hello world");
println!("SHA-512: {:x}", sha512_result);
```

[`Digest`]: crate::sha2::Digest
[`Digest::digest`]: crate::sha2::Digest::digest
[`Digest::new`]: crate::sha2::Digest::new
[`Digest::update`]: crate::sha2::Digest::update
[`Digest::finalize`]: crate::sha2::Digest::finalize
