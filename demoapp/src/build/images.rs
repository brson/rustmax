//! Image optimization pipeline.
//!
//! Uses the image crate for resizing, format conversion, and optimization.

use rustmax::image::{self, DynamicImage, ImageFormat, imageops::FilterType, GenericImageView};
use std::path::{Path, PathBuf};
use std::fs;
use crate::Result;
use crate::Error;

/// Configuration for image optimization.
#[derive(Debug, Clone)]
pub struct ImageConfig {
    /// Maximum width for images (0 = no limit).
    pub max_width: u32,
    /// Maximum height for images (0 = no limit).
    pub max_height: u32,
    /// Thumbnail width.
    pub thumb_width: u32,
    /// Thumbnail height.
    pub thumb_height: u32,
    /// JPEG quality (1-100).
    pub jpeg_quality: u8,
    /// Generate WebP versions.
    pub generate_webp: bool,
    /// Generate thumbnails.
    pub generate_thumbnails: bool,
    /// Preserve original images.
    pub preserve_originals: bool,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            max_width: 1920,
            max_height: 1080,
            thumb_width: 200,
            thumb_height: 200,
            jpeg_quality: 85,
            generate_webp: true,
            generate_thumbnails: true,
            preserve_originals: true,
        }
    }
}

/// Result of processing an image.
#[derive(Debug)]
pub struct ImageResult {
    /// Original file path.
    pub source: PathBuf,
    /// Output file path.
    pub output: PathBuf,
    /// Thumbnail path (if generated).
    pub thumbnail: Option<PathBuf>,
    /// WebP path (if generated).
    pub webp: Option<PathBuf>,
    /// Original dimensions.
    pub original_size: (u32, u32),
    /// Output dimensions.
    pub output_size: (u32, u32),
    /// Original file size in bytes.
    pub original_bytes: u64,
    /// Output file size in bytes.
    pub output_bytes: u64,
}

/// Supported image formats.
fn is_supported_format(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(ext.to_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "tiff" | "ico"
        ),
        None => false,
    }
}

/// Get the output format for a given extension.
fn get_format(ext: &str) -> Option<ImageFormat> {
    match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
        "png" => Some(ImageFormat::Png),
        "gif" => Some(ImageFormat::Gif),
        "webp" => Some(ImageFormat::WebP),
        "bmp" => Some(ImageFormat::Bmp),
        "ico" => Some(ImageFormat::Ico),
        "tiff" => Some(ImageFormat::Tiff),
        _ => None,
    }
}

/// Calculate dimensions preserving aspect ratio.
fn calculate_dimensions(
    width: u32,
    height: u32,
    max_width: u32,
    max_height: u32,
) -> (u32, u32) {
    if max_width == 0 && max_height == 0 {
        return (width, height);
    }

    let ratio = width as f64 / height as f64;

    let (new_width, new_height) = if max_width > 0 && max_height > 0 {
        if width > max_width || height > max_height {
            let width_ratio = max_width as f64 / width as f64;
            let height_ratio = max_height as f64 / height as f64;
            let ratio = width_ratio.min(height_ratio);
            ((width as f64 * ratio) as u32, (height as f64 * ratio) as u32)
        } else {
            (width, height)
        }
    } else if max_width > 0 && width > max_width {
        (max_width, (max_width as f64 / ratio) as u32)
    } else if max_height > 0 && height > max_height {
        ((max_height as f64 * ratio) as u32, max_height)
    } else {
        (width, height)
    };

    (new_width.max(1), new_height.max(1))
}

/// Process a single image.
pub fn process_image(
    source: &Path,
    output_dir: &Path,
    config: &ImageConfig,
) -> Result<ImageResult> {
    if !is_supported_format(source) {
        return Err(Error::build(format!(
            "Unsupported image format: {}",
            source.display()
        )));
    }

    let original_bytes = fs::metadata(source)?.len();

    // Load the image.
    let img = image::open(source).map_err(|e| {
        Error::build(format!("Failed to open image {}: {}", source.display(), e))
    })?;

    let (orig_width, orig_height) = img.dimensions();

    // Calculate output dimensions.
    let (out_width, out_height) = calculate_dimensions(
        orig_width,
        orig_height,
        config.max_width,
        config.max_height,
    );

    // Resize if needed.
    let processed = if out_width != orig_width || out_height != orig_height {
        img.resize(out_width, out_height, FilterType::Lanczos3)
    } else {
        img
    };

    // Determine output path.
    let filename = source.file_name().unwrap();
    let output_path = output_dir.join(filename);

    // Ensure output directory exists.
    fs::create_dir_all(output_dir)?;

    // Save the processed image.
    let ext = source
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");

    save_image(&processed, &output_path, ext, config.jpeg_quality)?;

    let output_bytes = fs::metadata(&output_path)?.len();

    // Generate thumbnail if configured.
    let thumbnail = if config.generate_thumbnails {
        let thumb = processed.thumbnail(config.thumb_width, config.thumb_height);
        let stem = source.file_stem().unwrap().to_string_lossy();
        let thumb_path = output_dir.join(format!("{}-thumb.{}", stem, ext));
        save_image(&thumb, &thumb_path, ext, config.jpeg_quality)?;
        Some(thumb_path)
    } else {
        None
    };

    // Generate WebP version if configured.
    let webp = if config.generate_webp {
        let stem = source.file_stem().unwrap().to_string_lossy();
        let webp_path = output_dir.join(format!("{}.webp", stem));
        save_image(&processed, &webp_path, "webp", config.jpeg_quality)?;
        Some(webp_path)
    } else {
        None
    };

    Ok(ImageResult {
        source: source.to_path_buf(),
        output: output_path,
        thumbnail,
        webp,
        original_size: (orig_width, orig_height),
        output_size: (out_width, out_height),
        original_bytes,
        output_bytes,
    })
}

/// Save an image with the given format.
fn save_image(img: &DynamicImage, path: &Path, ext: &str, quality: u8) -> Result<()> {
    match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" => {
            let mut file = fs::File::create(path)?;
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, quality);
            img.write_with_encoder(encoder).map_err(|e| {
                Error::build(format!("Failed to save JPEG: {}", e))
            })?;
        }
        "webp" => {
            img.save_with_format(path, ImageFormat::WebP).map_err(|e| {
                Error::build(format!("Failed to save WebP: {}", e))
            })?;
        }
        _ => {
            if let Some(format) = get_format(ext) {
                img.save_with_format(path, format).map_err(|e| {
                    Error::build(format!("Failed to save image: {}", e))
                })?;
            } else {
                img.save(path).map_err(|e| {
                    Error::build(format!("Failed to save image: {}", e))
                })?;
            }
        }
    }
    Ok(())
}

/// Process all images in a directory.
pub fn process_directory(
    source_dir: &Path,
    output_dir: &Path,
    config: &ImageConfig,
) -> Result<Vec<ImageResult>> {
    use rustmax::walkdir::WalkDir;

    let mut results = Vec::new();

    for entry in WalkDir::new(source_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && is_supported_format(path) {
            // Preserve directory structure.
            let relative = path.strip_prefix(source_dir).unwrap();
            let out_subdir = if let Some(parent) = relative.parent() {
                output_dir.join(parent)
            } else {
                output_dir.to_path_buf()
            };

            match process_image(path, &out_subdir, config) {
                Ok(result) => results.push(result),
                Err(e) => {
                    rustmax::log::warn!("Failed to process {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(results)
}

/// Statistics from image processing.
#[derive(Debug, Default)]
pub struct ImageStats {
    /// Number of images processed.
    pub processed: usize,
    /// Number of thumbnails generated.
    pub thumbnails: usize,
    /// Number of WebP images generated.
    pub webp_generated: usize,
    /// Total original bytes.
    pub original_bytes: u64,
    /// Total output bytes.
    pub output_bytes: u64,
}

impl ImageStats {
    /// Calculate bytes saved.
    pub fn bytes_saved(&self) -> i64 {
        self.original_bytes as i64 - self.output_bytes as i64
    }

    /// Calculate savings percentage.
    pub fn savings_percent(&self) -> f64 {
        if self.original_bytes == 0 {
            0.0
        } else {
            (self.bytes_saved() as f64 / self.original_bytes as f64) * 100.0
        }
    }

    /// Add results from a single image.
    pub fn add(&mut self, result: &ImageResult) {
        self.processed += 1;
        self.original_bytes += result.original_bytes;
        self.output_bytes += result.output_bytes;
        if result.thumbnail.is_some() {
            self.thumbnails += 1;
        }
        if result.webp.is_some() {
            self.webp_generated += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustmax::tempfile::tempdir;

    #[test]
    fn test_is_supported_format() {
        assert!(is_supported_format(Path::new("image.jpg")));
        assert!(is_supported_format(Path::new("image.JPEG")));
        assert!(is_supported_format(Path::new("image.png")));
        assert!(is_supported_format(Path::new("image.gif")));
        assert!(is_supported_format(Path::new("image.webp")));
        assert!(!is_supported_format(Path::new("file.txt")));
        assert!(!is_supported_format(Path::new("noextension")));
    }

    #[test]
    fn test_get_format() {
        assert!(matches!(get_format("jpg"), Some(ImageFormat::Jpeg)));
        assert!(matches!(get_format("JPEG"), Some(ImageFormat::Jpeg)));
        assert!(matches!(get_format("png"), Some(ImageFormat::Png)));
        assert!(matches!(get_format("webp"), Some(ImageFormat::WebP)));
        assert!(get_format("txt").is_none());
    }

    #[test]
    fn test_calculate_dimensions_no_limit() {
        let (w, h) = calculate_dimensions(1000, 500, 0, 0);
        assert_eq!((w, h), (1000, 500));
    }

    #[test]
    fn test_calculate_dimensions_width_limit() {
        let (w, h) = calculate_dimensions(2000, 1000, 1000, 0);
        assert_eq!(w, 1000);
        assert_eq!(h, 500);
    }

    #[test]
    fn test_calculate_dimensions_height_limit() {
        let (w, h) = calculate_dimensions(1000, 2000, 0, 1000);
        assert_eq!(w, 500);
        assert_eq!(h, 1000);
    }

    #[test]
    fn test_calculate_dimensions_both_limits() {
        let (w, h) = calculate_dimensions(4000, 3000, 1920, 1080);
        assert!(w <= 1920);
        assert!(h <= 1080);
    }

    #[test]
    fn test_calculate_dimensions_within_limits() {
        let (w, h) = calculate_dimensions(800, 600, 1920, 1080);
        assert_eq!((w, h), (800, 600));
    }

    #[test]
    fn test_image_config_default() {
        let config = ImageConfig::default();
        assert_eq!(config.max_width, 1920);
        assert_eq!(config.max_height, 1080);
        assert_eq!(config.thumb_width, 200);
        assert_eq!(config.jpeg_quality, 85);
        assert!(config.generate_webp);
        assert!(config.generate_thumbnails);
    }

    #[test]
    fn test_image_stats() {
        let mut stats = ImageStats::default();
        assert_eq!(stats.processed, 0);
        assert_eq!(stats.savings_percent(), 0.0);

        // Simulate adding an image result.
        stats.processed = 1;
        stats.original_bytes = 1000;
        stats.output_bytes = 800;

        assert_eq!(stats.bytes_saved(), 200);
        assert!((stats.savings_percent() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_process_image_creates_output() {
        use image::{RgbImage, ImageBuffer};

        let dir = tempdir().unwrap();
        let input_dir = dir.path().join("input");
        let output_dir = dir.path().join("output");
        fs::create_dir_all(&input_dir).unwrap();

        // Create a test image.
        let img: RgbImage = ImageBuffer::from_fn(100, 100, |x, y| {
            image::Rgb([(x % 256) as u8, (y % 256) as u8, 128])
        });
        let input_path = input_dir.join("test.png");
        img.save(&input_path).unwrap();

        let config = ImageConfig {
            max_width: 50,
            max_height: 50,
            generate_thumbnails: false,
            generate_webp: false,
            ..Default::default()
        };

        let result = process_image(&input_path, &output_dir, &config).unwrap();

        assert!(result.output.exists());
        assert_eq!(result.original_size, (100, 100));
        assert_eq!(result.output_size, (50, 50));
    }

    #[test]
    fn test_process_image_with_thumbnail() {
        use image::{RgbImage, ImageBuffer};

        let dir = tempdir().unwrap();
        let input_dir = dir.path().join("input");
        let output_dir = dir.path().join("output");
        fs::create_dir_all(&input_dir).unwrap();

        // Create a test image.
        let img: RgbImage = ImageBuffer::from_fn(400, 300, |x, y| {
            image::Rgb([(x % 256) as u8, (y % 256) as u8, 128])
        });
        let input_path = input_dir.join("photo.jpg");
        img.save(&input_path).unwrap();

        let config = ImageConfig {
            max_width: 1920,
            max_height: 1080,
            thumb_width: 100,
            thumb_height: 100,
            generate_thumbnails: true,
            generate_webp: false,
            ..Default::default()
        };

        let result = process_image(&input_path, &output_dir, &config).unwrap();

        assert!(result.output.exists());
        assert!(result.thumbnail.is_some());
        assert!(result.thumbnail.unwrap().exists());
    }

    #[test]
    fn test_process_image_with_webp() {
        use image::{RgbImage, ImageBuffer};

        let dir = tempdir().unwrap();
        let input_dir = dir.path().join("input");
        let output_dir = dir.path().join("output");
        fs::create_dir_all(&input_dir).unwrap();

        // Create a test image.
        let img: RgbImage = ImageBuffer::from_fn(200, 150, |x, y| {
            image::Rgb([(x % 256) as u8, (y % 256) as u8, 128])
        });
        let input_path = input_dir.join("image.png");
        img.save(&input_path).unwrap();

        let config = ImageConfig {
            generate_thumbnails: false,
            generate_webp: true,
            ..Default::default()
        };

        let result = process_image(&input_path, &output_dir, &config).unwrap();

        assert!(result.output.exists());
        assert!(result.webp.is_some());
        assert!(result.webp.unwrap().exists());
    }
}
