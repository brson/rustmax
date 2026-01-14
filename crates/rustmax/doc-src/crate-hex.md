Hexadecimal encoding and decoding.

- Crate [`::hex`].
- [docs.rs](https://docs.rs/hex)
- [crates.io](https://crates.io/crates/hex)
- [GitHub](https://github.com/KokaKiwi/rust-hex)

---

`hex` provides encoding and decoding of binary data
to and from hexadecimal strings.

## Examples

Basic encoding and decoding:

```
use hex::{encode, decode};

let data = b"Hello, world!";
let hex_string = encode(data);
println!("Hex: {}", hex_string); // "48656c6c6f2c20776f726c6421"

let decoded = decode(&hex_string).unwrap();
assert_eq!(decoded, data);
```

Uppercase hex encoding:

```
use hex::encode_upper;

let data = b"ABC";
let hex_upper = encode_upper(data);
println!("Upper: {}", hex_upper); // "414243"
```

[`encode`]: crate::hex::encode
[`decode`]: crate::hex::decode
[`encode_upper`]: crate::hex::encode_upper