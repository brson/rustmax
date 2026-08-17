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

A digest is a fixed-size byte array, not a string.
Use the [`hex`] crate to render one for display or storage.

## Examples

One-shot SHA-256 hashing:

```rust
use sha2::{Digest, Sha256};

let digest = Sha256::digest(b"hello world");

assert_eq!(
    hex::encode(digest),
    "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
);
```

Incremental hashing, for data that does not arrive all at once:

```rust
use sha2::{Digest, Sha256};

let mut hasher = Sha256::new();
hasher.update(b"hello ");
hasher.update(b"world");
let digest = hasher.finalize();

assert_eq!(digest, Sha256::digest(b"hello world"));
```

The variants differ only in digest length and internal state size:

```rust
use sha2::{Digest, Sha256, Sha384, Sha512};

assert_eq!(Sha256::digest(b"hello world").len(), 32);
assert_eq!(Sha384::digest(b"hello world").len(), 48);
assert_eq!(Sha512::digest(b"hello world").len(), 64);
```

[`Digest`]: crate::sha2::Digest
[`Digest::digest`]: crate::sha2::Digest::digest
[`Digest::new`]: crate::sha2::Digest::new
[`Digest::update`]: crate::sha2::Digest::update
[`Digest::finalize`]: crate::sha2::Digest::finalize
[`hex`]: crate::hex
