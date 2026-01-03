//! Encoding utilities using bytes, base64, and hex.

use rustmax::prelude::*;
use rustmax::bytes::{Bytes, BytesMut, BufMut};
use rustmax::base64::{Engine, prelude::BASE64_STANDARD};
use rustmax::hex;
use std::path::Path;

use crate::Result;

/// Encode binary data as base64.
pub fn to_base64(data: &[u8]) -> String {
    BASE64_STANDARD.encode(data)
}

/// Decode base64 string to bytes.
pub fn from_base64(encoded: &str) -> Result<Vec<u8>> {
    BASE64_STANDARD
        .decode(encoded)
        .map_err(|e| crate::Error::Other(e.into()))
}

/// Encode binary data as hex string.
pub fn to_hex(data: &[u8]) -> String {
    hex::encode(data)
}

/// Decode hex string to bytes.
pub fn from_hex(encoded: &str) -> Result<Vec<u8>> {
    hex::decode(encoded)
        .map_err(|e| crate::Error::Other(e.into()))
}

/// Create a data URL for inline embedding of assets.
///
/// Returns a URL like `data:image/png;base64,<encoded_data>`.
pub fn create_data_url(data: &[u8], mime_type: &str) -> String {
    format!("data:{};base64,{}", mime_type, to_base64(data))
}

/// Read a file and encode it as a data URL.
pub fn file_to_data_url(path: &Path) -> Result<String> {
    let data = std::fs::read(path)?;
    let mime_type = guess_mime_type(path);
    Ok(create_data_url(&data, mime_type))
}

/// Guess MIME type from file extension.
pub fn guess_mime_type(path: &Path) -> &'static str {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "html" | "htm" => "text/html",
        "xml" => "application/xml",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "pdf" => "application/pdf",
        _ => "application/octet-stream",
    }
}

/// Binary buffer for efficient asset handling.
pub struct AssetBuffer {
    inner: BytesMut,
}

impl AssetBuffer {
    /// Create a new empty buffer.
    pub fn new() -> Self {
        Self {
            inner: BytesMut::new(),
        }
    }

    /// Create a buffer with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: BytesMut::with_capacity(capacity),
        }
    }

    /// Append bytes to the buffer.
    pub fn extend(&mut self, data: &[u8]) {
        self.inner.put_slice(data);
    }

    /// Get the buffer contents as bytes.
    pub fn freeze(self) -> Bytes {
        self.inner.freeze()
    }

    /// Get the buffer length.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get a slice of the buffer.
    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }
}

impl Default for AssetBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Format a hash (e.g., blake3) as a short hex string.
pub fn format_hash_short(hash: &str, len: usize) -> &str {
    if hash.len() > len {
        &hash[..len]
    } else {
        hash
    }
}

/// Format bytes as a human-readable size string.
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_roundtrip() {
        let data = b"Hello, World!";
        let encoded = to_base64(data);
        let decoded = from_base64(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_hex_roundtrip() {
        let data = b"\xde\xad\xbe\xef";
        let encoded = to_hex(data);
        assert_eq!(encoded, "deadbeef");
        let decoded = from_hex(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_data_url() {
        let data = b"test";
        let url = create_data_url(data, "text/plain");
        assert!(url.starts_with("data:text/plain;base64,"));
    }

    #[test]
    fn test_guess_mime_type() {
        assert_eq!(guess_mime_type(Path::new("image.png")), "image/png");
        assert_eq!(guess_mime_type(Path::new("style.css")), "text/css");
        assert_eq!(guess_mime_type(Path::new("app.js")), "application/javascript");
        assert_eq!(guess_mime_type(Path::new("data.json")), "application/json");
        assert_eq!(guess_mime_type(Path::new("unknown.xyz")), "application/octet-stream");
    }

    #[test]
    fn test_asset_buffer() {
        let mut buf = AssetBuffer::new();
        buf.extend(b"Hello");
        buf.extend(b", ");
        buf.extend(b"World!");

        assert_eq!(buf.len(), 13);
        assert_eq!(buf.as_slice(), b"Hello, World!");

        let frozen = buf.freeze();
        assert_eq!(&frozen[..], b"Hello, World!");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(100), "100 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
        assert_eq!(format_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_format_hash_short() {
        let hash = "abcdef1234567890";
        assert_eq!(format_hash_short(hash, 8), "abcdef12");
        assert_eq!(format_hash_short(hash, 4), "abcd");
        assert_eq!(format_hash_short("abc", 10), "abc");
    }
}
