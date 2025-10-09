Deflate, gzip, and zlib compression.

- Crate [`::flate2`].
- [docs.rs](https://docs.rs/flate2)
- [crates.io](https://crates.io/crates/flate2)
- [GitHub](https://github.com/rust-lang/flate2-rs)

---

`flate2` provides compression and decompression for the DEFLATE algorithm,
along with support for zlib and gzip formats.
The crate offers both streaming and one-shot APIs for compression,
making it suitable for a wide range of use cases.

By default, `flate2` uses the `miniz_oxide` backend,
a pure Rust implementation that requires no C compiler
and uses only safe Rust code.
For maximum performance, the `zlib-rs` backend is available,
which typically outperforms C implementations.

The crate provides separate types for reading and writing compressed data:
[`Encoder`] and [`Decoder`] for raw DEFLATE,
[`GzEncoder`] and [`GzDecoder`] for gzip format,
and [`ZlibEncoder`] and [`ZlibDecoder`] for zlib format.

## Examples

Compressing data with gzip:

```
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::Write;

let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
encoder.write_all(b"Hello, world!").unwrap();
let compressed = encoder.finish().unwrap();
```

Decompressing gzip data:

```
use flate2::read::GzDecoder;
use std::io::Read;

fn decompress_example(compressed: &[u8]) {
    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).unwrap();
}
```

Compression with the deflate algorithm:

```
use flate2::Compression;
use flate2::write::DeflateEncoder;
use std::io::Write;

let original = b"The quick brown fox jumps over the lazy dog";
let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
encoder.write_all(original).unwrap();
let compressed = encoder.finish().unwrap();
```

[`Encoder`]: crate::flate2::write::DeflateEncoder
[`Decoder`]: crate::flate2::read::DeflateDecoder
[`GzEncoder`]: crate::flate2::write::GzEncoder
[`GzDecoder`]: crate::flate2::read::GzDecoder
[`ZlibEncoder`]: crate::flate2::write::ZlibEncoder
[`ZlibDecoder`]: crate::flate2::read::ZlibDecoder
