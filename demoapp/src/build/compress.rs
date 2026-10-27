//! Asset compression using flate2.

use rmx::prelude::*;
use rmx::flate2::write::GzEncoder;
use rmx::flate2::Compression;
use rmx::walkdir::WalkDir;
use rmx::log::{info, debug};
use std::io::Write;
use std::path::Path;
use std::fs;

use crate::Result;

/// Compress all eligible files in a directory.
///
/// Writes a `.gz` beside each HTML, CSS, JS, JSON, XML, SVG and text file
/// that is big enough to be worth it and actually gets smaller.
pub fn compress_output(output_dir: &Path) -> Result<CompressStats> {
    let mut stats = CompressStats::default();

    for entry in WalkDir::new(output_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && should_compress(path)
            && let Some((original, compressed)) = compress_file(path)?
        {
            debug!("Compressed {} ({original} -> {compressed} bytes)", path.display());
            stats.files_compressed += 1;
            stats.original_size += original;
            stats.compressed_size += compressed;
        }
    }

    if stats.files_compressed > 0 {
        info!(
            "Compressed {} files, saved {} bytes ({:.1}%)",
            stats.files_compressed,
            stats.bytes_saved(),
            stats.bytes_saved() as f64 / stats.original_size as f64 * 100.0,
        );
    }

    Ok(stats)
}

/// Compression statistics, over the files that were compressed.
#[derive(Debug, Default)]
pub struct CompressStats {
    pub files_compressed: usize,
    pub original_size: u64,
    pub compressed_size: u64,
}

impl CompressStats {
    pub fn bytes_saved(&self) -> u64 {
        self.original_size - self.compressed_size
    }
}

/// Check if a file should be compressed.
fn should_compress(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    matches!(ext, "html" | "css" | "js" | "json" | "xml" | "svg" | "txt" | "md")
}

/// Compress a single file, creating a `.gz` version.
///
/// Returns the original and compressed sizes,
/// or `None` if the file was too small or did not shrink.
fn compress_file(path: &Path) -> Result<Option<(u64, u64)>> {
    let content = fs::read(path)?;
    let original_size = content.len() as u64;

    if original_size < 256 {
        return Ok(None);
    }

    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&content)?;
    let compressed = encoder.finish()?;
    let compressed_size = compressed.len() as u64;

    if compressed_size >= original_size {
        return Ok(None);
    }

    let mut gz_path = path.as_os_str().to_owned();
    gz_path.push(".gz");
    fs::write(&gz_path, compressed)?;
    Ok(Some((original_size, compressed_size)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compressed_files_round_trip_and_small_ones_are_skipped() {
        use std::io::Read;

        let dir = rmx::tempfile::tempdir().unwrap();
        let big = "Hello, World! ".repeat(100);
        fs::write(dir.path().join("big.html"), &big).unwrap();
        fs::write(dir.path().join("small.html"), "hi").unwrap();
        fs::write(dir.path().join("photo.png"), big.as_bytes()).unwrap();

        let stats = compress_output(dir.path()).unwrap();
        assert_eq!(stats.files_compressed, 1);
        assert_eq!(stats.original_size, big.len() as u64);
        assert!(stats.bytes_saved() > 0);

        let mut unpacked = String::new();
        rmx::flate2::read::GzDecoder::new(fs::File::open(dir.path().join("big.html.gz")).unwrap())
            .read_to_string(&mut unpacked)
            .unwrap();
        assert_eq!(unpacked, big);
        assert!(!dir.path().join("small.html.gz").exists());
        assert!(!dir.path().join("photo.png.gz").exists());
    }

    #[test]
    fn test_should_compress() {
        assert!(should_compress(Path::new("index.html")));
        assert!(should_compress(Path::new("style.css")));
        assert!(should_compress(Path::new("app.js")));
        assert!(should_compress(Path::new("data.json")));
        assert!(!should_compress(Path::new("image.png")));
        assert!(!should_compress(Path::new("photo.jpg")));
    }

}
