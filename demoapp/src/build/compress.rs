//! Asset compression using flate2.

use rustmax::prelude::*;
use rustmax::flate2::write::GzEncoder;
use rustmax::flate2::Compression;
use rustmax::walkdir::WalkDir;
use rustmax::log::{info, debug};
use std::io::Write;
use std::path::Path;
use std::fs;

use crate::Result;

/// Compress all eligible files in a directory.
///
/// Creates `.gz` versions of HTML, CSS, JS, JSON, and XML files.
pub fn compress_output(output_dir: &Path) -> Result<CompressStats> {
    let mut stats = CompressStats::default();

    for entry in WalkDir::new(output_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && should_compress(path) {
            match compress_file(path) {
                Ok(saved) => {
                    stats.files_compressed += 1;
                    stats.bytes_saved += saved;
                    debug!("Compressed: {} (saved {} bytes)", path.display(), saved);
                }
                Err(e) => {
                    debug!("Failed to compress {}: {}", path.display(), e);
                }
            }
        }
    }

    if stats.files_compressed > 0 {
        info!(
            "Compressed {} files, saved {} bytes ({:.1}%)",
            stats.files_compressed,
            stats.bytes_saved,
            if stats.original_size > 0 {
                (stats.bytes_saved as f64 / stats.original_size as f64) * 100.0
            } else {
                0.0
            }
        );
    }

    Ok(stats)
}

/// Compression statistics.
#[derive(Debug, Default)]
pub struct CompressStats {
    pub files_compressed: usize,
    pub original_size: u64,
    pub compressed_size: u64,
    pub bytes_saved: u64,
}

impl CompressStats {
    pub fn compression_ratio(&self) -> f64 {
        if self.original_size == 0 {
            0.0
        } else {
            1.0 - (self.compressed_size as f64 / self.original_size as f64)
        }
    }
}

/// Check if a file should be compressed.
fn should_compress(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    matches!(ext, "html" | "css" | "js" | "json" | "xml" | "svg" | "txt" | "md")
}

/// Compress a single file, creating a `.gz` version.
///
/// Returns the number of bytes saved.
fn compress_file(path: &Path) -> Result<u64> {
    let content = fs::read(path)?;
    let original_size = content.len() as u64;

    // Skip small files.
    if original_size < 256 {
        return Ok(0);
    }

    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&content)?;
    let compressed = encoder.finish()?;
    let compressed_size = compressed.len() as u64;

    // Only save if we actually achieved compression.
    if compressed_size < original_size {
        let gz_path = path.with_extension(format!(
            "{}.gz",
            path.extension().and_then(|e| e.to_str()).unwrap_or("")
        ));
        fs::write(&gz_path, compressed)?;
        Ok(original_size - compressed_size)
    } else {
        Ok(0)
    }
}

/// Decompress gzip content.
pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    use rustmax::flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

/// Compress data to gzip format.
pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

/// Compress data with a specific compression level.
pub fn compress_with_level(data: &[u8], level: u32) -> Result<Vec<u8>> {
    let level = Compression::new(level);
    let mut encoder = GzEncoder::new(Vec::new(), level);
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let original = b"Hello, World! ".repeat(100);
        let compressed = compress(&original).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert!(compressed.len() < original.len());
        assert_eq!(decompressed, original);
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

    #[test]
    fn test_compression_levels() {
        let data = b"Test data for compression. ".repeat(50);

        let fast = compress_with_level(&data, 1).unwrap();
        let best = compress_with_level(&data, 9).unwrap();

        // Best compression should be smaller or equal to fast.
        assert!(best.len() <= fast.len());

        // Both should decompress to original.
        assert_eq!(decompress(&fast).unwrap(), data);
        assert_eq!(decompress(&best).unwrap(), data);
    }
}
