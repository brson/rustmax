Read and write tar archives.

- Crate [`::tar`].
- [docs.rs](https://docs.rs/tar)
- [crates.io](https://crates.io/crates/tar)
- [GitHub](https://github.com/composefs/tar-rs)

---

`tar` reads and writes the tar archive format,
which bundles files and their metadata into a single stream.
It does not compress:
a `.tar.gz` is a tar archive passed through gzip,
so pair this crate with [`flate2`] to read or write one.

The API is abstract over [`Read`] and [`Write`]
rather than tied to files,
and it is streaming throughout —
an archive of any size can be processed
without holding it in memory.

[`Archive`] reads an existing archive,
yielding an [`Entry`] per member,
each of which is itself a [`Read`].
[`Builder`] writes a new one.
[`Header`] carries the per-entry metadata:
path, size, mode, timestamps, and [`EntryType`],
which distinguishes regular files from directories,
symlinks, and hard links.

Unpacking an untrusted archive can be insecure.
Entries can name absolute paths, paths containing `..`,
and symlinks or hard links pointing anywhere on the filesystem.
[`Archive::unpack`] and [`Entry::unpack_in`] exist to contain that,
refusing to write outside the destination directory.

## Examples

Writing an archive and reading it back, in memory:

```rust
use std::io::Read;
use tar::{Archive, Builder, Header};

let mut header = Header::new_gnu();
header.set_path("hello.txt").unwrap();
header.set_size(5);
header.set_mode(0o644);
header.set_cksum();

let mut builder = Builder::new(Vec::new());
builder.append(&header, &b"world"[..]).unwrap();
let bytes = builder.into_inner().unwrap();

let mut archive = Archive::new(&bytes[..]);
for entry in archive.entries().unwrap() {
    let mut entry = entry.unwrap();
    assert_eq!(entry.path().unwrap().to_str().unwrap(), "hello.txt");

    let mut contents = String::new();
    entry.read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "world");
}
```

Making a `.tar.gz` by layering a compressor over the archive:

```rust
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use tar::{Archive, Builder, Header};

let mut header = Header::new_gnu();
header.set_path("notes.txt").unwrap();
header.set_size(4);
header.set_cksum();

let mut builder = Builder::new(GzEncoder::new(Vec::new(), Compression::default()));
builder.append(&header, &b"data"[..]).unwrap();
let targz = builder.into_inner().unwrap().finish().unwrap();

let mut archive = Archive::new(GzDecoder::new(&targz[..]));
assert_eq!(archive.entries().unwrap().count(), 1);
```

Unpacking to a directory:

```rust
use tar::{Archive, Builder, Header};

let mut header = Header::new_gnu();
header.set_path("hello.txt").unwrap();
header.set_size(5);
header.set_mode(0o644);
header.set_cksum();

let mut builder = Builder::new(Vec::new());
builder.append(&header, &b"world"[..]).unwrap();
let bytes = builder.into_inner().unwrap();

let dir = tempfile::tempdir().unwrap();
Archive::new(&bytes[..]).unpack(dir.path()).unwrap();

assert_eq!(std::fs::read_to_string(dir.path().join("hello.txt")).unwrap(), "world");
```

[`Archive`]: crate::tar::Archive
[`Archive::unpack`]: crate::tar::Archive::unpack
[`Builder`]: crate::tar::Builder
[`Entry`]: crate::tar::Entry
[`Entry::unpack_in`]: crate::tar::Entry::unpack_in
[`EntryType`]: crate::tar::EntryType
[`Header`]: crate::tar::Header
[`flate2`]: crate::flate2
[`Read`]: std::io::Read
[`Write`]: std::io::Write
