Base64 encoding and decoding.

- Crate [`::base64`].
- [docs.rs](https://docs.rs/base64)
- [crates.io](https://crates.io/crates/base64)
- [GitHub](https://github.com/marshallpierce/rust-base64)

---

`base64` provides encoding and decoding for Base64,
a binary-to-text encoding scheme that represents binary data
in a printable ASCII string format.
Base64 is commonly used for encoding binary data in contexts
where only text can be stored or transmitted,
such as email attachments, URLs, and data URIs.

The crate supports multiple Base64 alphabets and configurations,
including standard Base64, URL-safe Base64, and custom alphabets.
The main functions are [`encode`] and [`decode`] for simple operations,
and the [`Engine`] trait for more advanced usage with custom configurations.

## Examples

Basic encoding and decoding:

```
use base64::{encode, decode};

let original = "Hello, world!";
let encoded = encode(original);
println!("Encoded: {}", encoded); // "SGVsbG8sIHdvcmxkIQ=="

let decoded = decode(&encoded).unwrap();
let decoded_str = String::from_utf8(decoded).unwrap();
println!("Decoded: {}", decoded_str); // "Hello, world!"
```

Using URL-safe encoding for use in URLs:

```
use base64::{Engine as _, engine::general_purpose};

let data = b"data with/special+chars";
let encoded = general_purpose::URL_SAFE_NO_PAD.encode(data);
println!("URL-safe: {}", encoded); // Uses - and _ instead of + and /

let decoded = general_purpose::URL_SAFE_NO_PAD.decode(&encoded).unwrap();
assert_eq!(decoded, data);
```

[`encode`]: crate::base64::encode
[`decode`]: crate::base64::decode
[`Engine`]: crate::base64::Engine