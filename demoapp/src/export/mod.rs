//! Export functionality for collections.
//!
//! Supports packaging a collection as an EPUB book
//! or a built site as a tar archive.

mod archive;
mod epub;

pub use archive::{
    ArchiveEntry, ArchiveOptions, ArchiveStats, archive_directory, default_archive_name,
    list_archive,
};
pub use epub::{EpubBuilder, EpubConfig, generate_epub};
