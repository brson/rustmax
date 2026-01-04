Read and write ZIP archives.

- Crate [`::zip`].
- [docs.rs](https://docs.rs/zip)
- [crates.io](https://crates.io/crates/zip)
- [GitHub](https://github.com/zip-rs/zip2)

---

`zip` provides support for reading and writing ZIP archives,
including compressed entries using DEFLATE and other algorithms.
Useful for bundling files, creating backups, or working with
formats that are ZIP-based (like EPUB, DOCX, JAR).

The main types are [`ZipArchive`] for reading existing archives
and [`ZipWriter`] for creating new ones.
Use [`ZipArchive::new`] to open a ZIP file for reading.

## Examples

Read files from a ZIP archive:

```rust,no_run
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

let file = File::open("archive.zip").unwrap();
let mut archive = ZipArchive::new(file).unwrap();

for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();
    println!("File: {}", file.name());

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
}
```

Create a new ZIP archive:

```rust,no_run
use std::fs::File;
use std::io::Write;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

let file = File::create("output.zip").unwrap();
let mut zip = ZipWriter::new(file);

let options = SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated);

zip.start_file("hello.txt", options).unwrap();
zip.write_all(b"Hello, World!").unwrap();

zip.finish().unwrap();
```

[`ZipArchive`]: crate::zip::ZipArchive
[`ZipWriter`]: crate::zip::ZipWriter
[`ZipArchive::new`]: crate::zip::ZipArchive::new
