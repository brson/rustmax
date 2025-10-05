The BLAKE3 cryptographic hash function.

- Crate [`::blake3`].
- [docs.rs](https://docs.rs/blake3)
- [crates.io](https://crates.io/crates/blake3)
- [GitHub](https://github.com/BLAKE3-team/BLAKE3)

---

BLAKE3 is a modern cryptographic hash function that is fast, secure, and highly parallelizable.
When you need a cryptographic hash function and otherwise have no constraints, pick BLAKE3.

The primary hashing interface is the [`hash`] function for simple one-shot hashing,
and the [`Hasher`] type for incremental hashing.
BLAKE3 also supports keyed hashing via [`keyed_hash`] for message authentication,
and key derivation via [`derive_key`] for generating cryptographic keys from context strings.

Unlike traditional hash functions that produce a fixed output size,
BLAKE3 is an extendable-output function (XOF).
While it defaults to 32-byte output like SHA-256,
it can produce arbitrarily long output through [`finalize_xof`],
which returns an [`OutputReader`] that can generate unlimited hash bytes.

BLAKE3 achieves high performance through its tree-based structure,
which allows parallel hashing of large inputs.
When compiled with SIMD support (enabled by default),
it can be significantly faster than SHA-256 and SHA-512.

The hash function is designed to be secure as:
- A general-purpose cryptographic hash (collision resistance, preimage resistance)
- A message authentication code (MAC) when used with a key
- A key derivation function (KDF)
- A pseudorandom function (PRF)

## Examples

Basic hashing of data:

```rust
use blake3;

let hash = blake3::hash(b"hello world");
println!("Hash: {}", hash.to_hex());

assert_eq!(hash.as_bytes().len(), 32);
```

Incremental hashing with the Hasher type:

```rust
use blake3::Hasher;

let mut hasher = Hasher::new();
hasher.update(b"hello");
hasher.update(b" ");
hasher.update(b"world");
let hash = hasher.finalize();

assert_eq!(hash, blake3::hash(b"hello world"));
```

Using keyed hashing for message authentication:

```rust
use blake3;

let key = [42u8; 32];
let mac = blake3::keyed_hash(&key, b"authenticated message");

// Verify the MAC
let verification = blake3::keyed_hash(&key, b"authenticated message");
assert_eq!(mac, verification);
```

Extended output for generating arbitrary-length hash output:

```rust
use blake3::Hasher;

let mut hasher = Hasher::new();
hasher.update(b"input data");

let mut extended_output = [0u8; 100];
let mut xof = hasher.finalize_xof();
xof.fill(&mut extended_output);

// Can generate more output from the same XOF reader
let mut more_output = [0u8; 50];
xof.fill(&mut more_output);
```

[`hash`]: crate::blake3::hash
[`Hasher`]: crate::blake3::Hasher
[`keyed_hash`]: crate::blake3::keyed_hash
[`derive_key`]: crate::blake3::derive_key
[`finalize_xof`]: crate::blake3::Hasher::finalize_xof
[`OutputReader`]: crate::blake3::OutputReader
