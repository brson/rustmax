Image processing and manipulation.

- Crate [`::image`].
- [docs.rs](https://docs.rs/image)
- [crates.io](https://crates.io/crates/image)
- [GitHub](https://github.com/image-rs/image)

---

`image` provides basic image processing functions and methods
for loading and saving images in various formats.
Supports PNG, JPEG, GIF, WebP, BMP, TIFF, and more.

The main types are [`DynamicImage`] for format-agnostic image handling
and [`ImageBuffer`] for typed pixel access.
Use [`open`] to load images from files
and [`DynamicImage::save`] to write them.

Common operations include resizing, cropping, rotating,
color conversion, and pixel-level manipulation.

## Examples

Load an image and resize it:

```rust,no_run
use image::{open, imageops::FilterType};

let img = open("input.jpg").expect("Failed to open image");
let resized = img.resize(800, 600, FilterType::Lanczos3);
resized.save("output.jpg").expect("Failed to save image");
```

Create a new image programmatically:

```rust
use image::{ImageBuffer, Rgb};

let img = ImageBuffer::from_fn(256, 256, |x, y| {
    Rgb([x as u8, y as u8, 128])
});

// Access individual pixels.
let pixel = img.get_pixel(10, 10);
assert_eq!(pixel, &Rgb([10, 10, 128]));
```

Convert between formats:

```rust,no_run
use image::open;

let img = open("photo.png").expect("Failed to open");
img.save("photo.webp").expect("Saved as WebP");
```

[`DynamicImage`]: crate::image::DynamicImage
[`ImageBuffer`]: crate::image::ImageBuffer
[`open`]: crate::image::open
