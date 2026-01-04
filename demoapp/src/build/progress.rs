//! Progress bar support for build operations.
//!
//! Uses indicatif to provide visual feedback during builds.

use rustmax::indicatif::{ProgressBar, ProgressStyle, MultiProgress};
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

/// Multi-progress container for complex builds.
pub struct BuildProgress {
    mp: MultiProgress,
    documents: Option<ProgressBar>,
    current_task: Option<ProgressBar>,
}

impl BuildProgress {
    /// Create a new build progress tracker.
    pub fn new() -> Self {
        Self {
            mp: MultiProgress::new(),
            documents: None,
            current_task: None,
        }
    }

    /// Start tracking document progress.
    pub fn start_documents(&mut self, total: usize) {
        let pb = self.mp.add(document_progress(total));
        self.documents = Some(pb);
    }

    /// Increment document progress.
    pub fn inc_document(&self) {
        if let Some(ref pb) = self.documents {
            pb.inc(1);
        }
    }

    /// Finish document progress.
    pub fn finish_documents(&mut self, message: &str) {
        if let Some(ref pb) = self.documents {
            pb.set_message(message.to_string());
            pb.finish();
        }
        self.documents = None;
    }

    /// Start a spinner task.
    pub fn start_task(&mut self, message: &str) {
        self.finish_task();
        let pb = ProgressBar::new_spinner();
        pb.set_style(spinner_style());
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        self.current_task = Some(self.mp.add(pb));
    }

    /// Finish the current spinner task.
    pub fn finish_task(&mut self) {
        if let Some(ref pb) = self.current_task {
            pb.finish_and_clear();
        }
        self.current_task = None;
    }

    /// Finish all progress tracking.
    pub fn finish(&mut self) {
        self.finish_documents("Done");
        self.finish_task();
    }
}

impl Default for BuildProgress {
    fn default() -> Self {
        Self::new()
    }
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
    fn test_build_progress() {
        let mut bp = BuildProgress::new();

        bp.start_documents(10);
        bp.inc_document();
        bp.inc_document();
        bp.finish_documents("Built 2 documents");

        bp.start_task("Testing...");
        bp.finish_task();

        bp.finish();
    }

    #[test]
    fn test_finish_with_check() {
        let pb = ProgressBar::new_spinner();
        finish_with_check(&pb, "Completed");
        // Should not panic.
    }
}
