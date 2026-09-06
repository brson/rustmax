//! Tar archives of a built site.
//!
//! A built site is a directory of files, which is what most static hosts want
//! uploaded. A tarball is the portable way to hand one over, and unlike the
//! EPUB export this keeps the tree exactly as built.

use rmx::flate2::Compression;
use rmx::flate2::write::GzEncoder;
use rmx::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use crate::{Error, Result};

/// How to build an archive.
#[derive(Debug, Clone)]
pub struct ArchiveOptions {
    /// Compress the archive with gzip.
    pub gzip: bool,
    /// Directory prefix every entry is stored under.
    ///
    /// Unpacking then yields one directory rather than loose files.
    pub prefix: PathBuf,
}

impl Default for ArchiveOptions {
    fn default() -> Self {
        Self {
            gzip: true,
            prefix: PathBuf::from("site"),
        }
    }
}

/// What an archive run produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveStats {
    pub files: usize,
    /// Size of the archive on disk, after compression if enabled.
    pub archive_bytes: u64,
}

/// One entry read back out of an archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveEntry {
    pub path: PathBuf,
    pub size: u64,
}

/// Suggest an archive file name for the given options.
pub fn default_archive_name(options: &ArchiveOptions) -> String {
    let stem = options.prefix.display();
    if options.gzip {
        format!("{stem}.tar.gz")
    } else {
        format!("{stem}.tar")
    }
}

/// Archive a built site directory into a tarball.
pub fn archive_directory(source: &Path, dest: &Path, options: &ArchiveOptions) -> Result<ArchiveStats> {
    if !source.is_dir() {
        return Err(Error::build(format!(
            "cannot archive {}: not a directory (build the site first)",
            source.display(),
        )));
    }

    let files = count_files(source)?;

    let out = BufWriter::new(File::create(dest)?);
    if options.gzip {
        let encoder = GzEncoder::new(out, Compression::default());
        let mut builder = rmx::tar::Builder::new(encoder);
        builder.append_dir_all(&options.prefix, source)?;
        // Both layers have to be finished in order, or the archive is
        // truncated: `into_inner` flushes the tar, `finish` the gzip stream.
        builder.into_inner()?.finish()?;
    } else {
        let mut builder = rmx::tar::Builder::new(out);
        builder.append_dir_all(&options.prefix, source)?;
        builder.into_inner()?;
    }

    Ok(ArchiveStats {
        files,
        archive_bytes: std::fs::metadata(dest)?.len(),
    })
}

/// List the files in an archive, in the order they were written.
///
/// Directory entries are skipped, so this matches [`ArchiveStats::files`].
pub fn list_archive(path: &Path) -> Result<Vec<ArchiveEntry>> {
    let file = File::open(path)?;
    let gzipped = path.extension().is_some_and(|ext| ext == "gz");

    if gzipped {
        let decoder = rmx::flate2::read::GzDecoder::new(file);
        read_entries(rmx::tar::Archive::new(decoder))
    } else {
        read_entries(rmx::tar::Archive::new(file))
    }
}

fn read_entries<R: std::io::Read>(mut archive: rmx::tar::Archive<R>) -> Result<Vec<ArchiveEntry>> {
    let mut entries = Vec::new();
    for entry in archive.entries()? {
        let entry = entry?;
        if entry.header().entry_type().is_dir() {
            continue;
        }
        entries.push(ArchiveEntry {
            path: entry.path()?.into_owned(),
            size: entry.header().size()?,
        });
    }
    Ok(entries)
}

fn count_files(dir: &Path) -> Result<usize> {
    let count = rmx::walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .count();
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmx::tempfile::tempdir;

    fn site() -> rmx::tempfile::TempDir {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("posts")).unwrap();
        std::fs::write(dir.path().join("index.html"), "<h1>Home</h1>").unwrap();
        std::fs::write(dir.path().join("posts/first.html"), "<h1>First</h1>").unwrap();
        dir
    }

    #[test]
    fn archives_a_site_and_reads_it_back() {
        let source = site();
        let out = tempdir().unwrap();
        let dest = out.path().join("site.tar");

        let stats = archive_directory(
            source.path(),
            &dest,
            &ArchiveOptions { gzip: false, ..default() },
        )
        .unwrap();

        assert_eq!(stats.files, 2);
        assert!(stats.archive_bytes > 0);

        let mut paths: Vec<_> = list_archive(&dest)
            .unwrap()
            .into_iter()
            .map(|e| e.path.display().to_string())
            .collect();
        paths.sort();

        assert_eq!(paths, ["site/index.html", "site/posts/first.html"]);
    }

    #[test]
    fn gzip_round_trips_and_preserves_sizes() {
        let source = site();
        let out = tempdir().unwrap();
        let dest = out.path().join("site.tar.gz");

        let stats = archive_directory(source.path(), &dest, &default()).unwrap();
        assert_eq!(stats.files, 2);

        let entries = list_archive(&dest).unwrap();
        assert_eq!(entries.len(), 2);

        let index = entries
            .iter()
            .find(|e| e.path.ends_with("index.html"))
            .unwrap();
        assert_eq!(index.size, "<h1>Home</h1>".len() as u64);
    }

    #[test]
    fn the_prefix_names_the_unpacked_directory() {
        let source = site();
        let out = tempdir().unwrap();
        let dest = out.path().join("release.tar");

        archive_directory(
            source.path(),
            &dest,
            &ArchiveOptions {
                gzip: false,
                prefix: PathBuf::from("my-blog-1.0"),
            },
        )
        .unwrap();

        let entries = list_archive(&dest).unwrap();
        assert!(
            entries.iter().all(|e| e.path.starts_with("my-blog-1.0")),
            "{entries:?}",
        );
    }

    #[test]
    fn gzip_is_smaller_than_plain_tar_for_repetitive_content() {
        let source = tempdir().unwrap();
        std::fs::write(source.path().join("a.html"), "<p>hello</p>".repeat(500)).unwrap();
        let out = tempdir().unwrap();

        let plain = out.path().join("s.tar");
        let gzipped = out.path().join("s.tar.gz");
        let plain_stats =
            archive_directory(source.path(), &plain, &ArchiveOptions { gzip: false, ..default() })
                .unwrap();
        let gz_stats = archive_directory(source.path(), &gzipped, &default()).unwrap();

        assert!(
            gz_stats.archive_bytes < plain_stats.archive_bytes,
            "{gz_stats:?} vs {plain_stats:?}",
        );
    }

    #[test]
    fn archiving_a_missing_directory_says_to_build_first() {
        let out = tempdir().unwrap();
        let err = archive_directory(
            &out.path().join("no-such-output"),
            &out.path().join("s.tar"),
            &default(),
        )
        .unwrap_err();

        assert!(err.to_string().contains("build the site first"), "{err}");
    }

    #[test]
    fn the_archive_name_follows_the_compression_setting() {
        assert_eq!(default_archive_name(&default()), "site.tar.gz");
        assert_eq!(
            default_archive_name(&ArchiveOptions { gzip: false, ..default() }),
            "site.tar",
        );
    }
}
