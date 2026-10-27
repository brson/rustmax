//! Progress bar support for build operations.
//!
//! Uses indicatif to provide visual feedback during builds.

use rmx::indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Style for main build progress bar.
fn build_style() -> ProgressStyle {
    ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .expect("valid template")
        .progress_chars("#>-")
}

/// Style for spinner operations.
fn spinner_style() -> ProgressStyle {
    ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .expect("valid template")
}

/// Create a progress bar for document building.
pub fn document_progress(total: usize) -> ProgressBar {
    let pb = ProgressBar::new(total as u64);
    pb.set_style(build_style());
    pb.set_message("Building documents...");
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Create a progress bar for incremental builds.
pub fn incremental_progress(total: usize) -> ProgressBar {
    let pb = ProgressBar::new(total as u64);
    pb.set_style(build_style());
    pb.set_message("Checking documents...");
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Create a spinner for asset copying.
pub fn asset_spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style());
    pb.set_message("Copying static assets...");
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Create a spinner for template compilation.
pub fn template_spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style());
    pb.set_message("Compiling templates...");
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Create a spinner for search index generation.
pub fn search_spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style());
    pb.set_message("Building search index...");
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Create a spinner for feed generation.
pub fn feed_spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style());
    pb.set_message("Generating feeds...");
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Create a spinner for compression.
pub fn compress_spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style());
    pb.set_message("Compressing output...");
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Finish a progress bar with a checkmark.
pub fn finish_with_check(pb: &ProgressBar, message: &str) {
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{msg}")
            .expect("valid template")
    );
    pb.finish_with_message(format!("✓ {}", message));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_progress() {
        let pb = document_progress(10);
        assert_eq!(pb.length(), Some(10));
        pb.finish();
    }

    #[test]
    fn test_incremental_progress() {
        let pb = incremental_progress(5);
        assert_eq!(pb.length(), Some(5));
        pb.finish();
    }

    #[test]
    fn test_spinners() {
        let asset = asset_spinner();
        let template = template_spinner();
        let search = search_spinner();
        let feed = feed_spinner();
        let compress = compress_spinner();

        // All spinners should be indeterminate (no length).
        assert!(asset.length().is_none());
        assert!(template.length().is_none());
        assert!(search.length().is_none());
        assert!(feed.length().is_none());
        assert!(compress.length().is_none());

        asset.finish();
        template.finish();
        search.finish();
        feed.finish();
        compress.finish();
    }

    #[test]
    fn test_finish_with_check() {
        let pb = ProgressBar::new_spinner();
        finish_with_check(&pb, "Completed");
        // Should not panic.
    }
}
