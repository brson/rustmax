//! `folio watch`: rebuild a collection whenever its sources change.
//!
//! Three kinds of event are waited on together with crossbeam's `select!`:
//! file changes from `notify`, a stop request (Ctrl-C, via `ctrlc`),
//! and a debounce timer.
//! Editors save in bursts -- a temporary file, a rename, a metadata touch --
//! so a rebuild waits until the sources have been quiet for [`QUIET`].
//!
//! A stop request is only acted on between builds,
//! so an interrupted watch never leaves a half-written build cache behind.

use rmx::crossbeam::channel::{self, Receiver, after, never, select};
use rmx::log::{error, info, warn};
use rmx::notify::{self, Event, EventKind, RecursiveMode, Watcher};
use rmx::prelude::*;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::collection::{Collection, Config, ConfigFormat};
use crate::features::BuildFeatures;
use crate::Result;

/// How long the sources must be quiet before a rebuild starts.
pub const QUIET: Duration = Duration::from_millis(150);

/// The collection directories whose contents are watched.
const SOURCE_DIRS: &[&str] = &["content", "templates", "static"];

/// How a watched collection is built.
#[derive(Debug, Clone, Default)]
pub struct WatchOptions {
    /// Output directory, overriding the config's.
    pub output: Option<PathBuf>,
    /// Build drafts, in addition to what the config enables.
    pub drafts: bool,
}

/// How much has to be rebuilt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rebuild {
    /// Content changed: rebuild the documents that changed.
    Incremental,
    /// The config changed or a file went away, which an incremental build
    /// cannot see: start the output again from nothing.
    Full,
}

/// Watch a collection until Ctrl-C, rebuilding as it changes.
pub fn watch(root: &Path, options: &WatchOptions) -> Result<()> {
    let (stop_tx, stop_rx) = channel::bounded(1);
    rmx::ctrlc::set_handler(move || {
        // A second Ctrl-C while the first is pending has nothing to add.
        let _ = stop_tx.try_send(());
    })?;

    watch_until(root, options, stop_rx)?;
    info!("Stopped watching");
    Ok(())
}

/// Watch a collection until `stop` receives, rebuilding as it changes.
///
/// The collection is built once up front, in full.
pub fn watch_until(root: &Path, options: &WatchOptions, stop: Receiver<()>) -> Result<()> {
    // Some platforms report events against the canonical path,
    // and `classify` compares paths by prefix.
    let root = &root.canonicalize()?;

    let (event_tx, event_rx) = channel::unbounded();
    let mut watcher = notify::recommended_watcher(move |event| {
        // The receiver only goes away when the watch is over.
        let _ = event_tx.send(event);
    })?;

    for dir in SOURCE_DIRS {
        let dir = root.join(dir);
        if dir.is_dir() {
            watcher.watch(&dir, RecursiveMode::Recursive)?;
        }
    }
    // The root itself only for its config file. Watching it recursively would
    // see the build's own writes to the output directory and the cache.
    watcher.watch(root, RecursiveMode::NonRecursive)?;

    // Watching starts before the first build, so nothing saved during it is missed.
    rebuild(root, options, Rebuild::Full);
    info!("Watching {} for changes", root.display());

    let mut pending: Option<Rebuild> = None;
    loop {
        // Every event restarts the timer, so it only fires once things are quiet.
        let quiet = if pending.is_some() { after(QUIET) } else { never() };

        select! {
            recv(stop) -> _ => return Ok(()),
            recv(event_rx) -> event => {
                match event.expect("the watcher outlives the loop") {
                    Ok(event) => {
                        if let Some(needed) = classify(root, &event) {
                            pending = pending.max(Some(needed));
                        }
                    }
                    Err(e) => warn!("watch error: {e}"),
                }
            }
            recv(quiet) -> _ => {
                let needed = pending.take().expect("the timer only runs with a rebuild pending");
                rebuild(root, options, needed);
            }
        }
    }
}

/// What a file event means for the build, if anything.
pub fn classify(root: &Path, event: &Event) -> Option<Rebuild> {
    if matches!(event.kind, EventKind::Access(_)) {
        return None;
    }

    event
        .paths
        .iter()
        .filter_map(|path| {
            if path.parent() == Some(root) {
                let name = path.file_name()?;
                let is_config = ConfigFormat::search_order()
                    .iter()
                    .any(|format| name == format.file_name());
                return is_config.then_some(Rebuild::Full);
            }

            let in_sources = SOURCE_DIRS.iter().any(|dir| path.starts_with(root.join(dir)));
            if !in_sources {
                return None;
            }

            // A rename is reported as a pair of name events, and either half
            // can leave an output page with no source behind it.
            match event.kind {
                EventKind::Remove(_) | EventKind::Modify(notify::event::ModifyKind::Name(_)) => {
                    Some(Rebuild::Full)
                }
                _ => Some(Rebuild::Incremental),
            }
        })
        .max()
}

/// Build the collection, reporting the outcome instead of returning it.
///
/// A failed build is something to fix and save again,
/// not a reason to stop watching.
fn rebuild(root: &Path, options: &WatchOptions, needed: Rebuild) {
    let started = Instant::now();
    match build(root, options, needed) {
        Ok(summary) => info!("{summary} in {} ms", started.elapsed().as_millis()),
        Err(e) => error!("build failed: {e}"),
    }
}

fn build(root: &Path, options: &WatchOptions, needed: Rebuild) -> Result<String> {
    // Reloaded every time, since the config is one of the things that changes.
    let mut config = Config::load(root)?;
    if options.drafts {
        config.build.features |= BuildFeatures::DRAFTS;
    }

    let collection = Collection::load(root, &config)?;
    let output = options
        .output
        .clone()
        .unwrap_or_else(|| root.join(&config.build.output_dir));

    match needed {
        Rebuild::Full => {
            crate::build::build(&collection, &config, &output)?;
            Ok(format!("Built {} documents", collection.documents.len()))
        }
        Rebuild::Incremental => {
            let result = crate::build::build_incremental(&collection, &config, &output)?;
            Ok(format!("Rebuilt {} of {} documents", result.rebuilt, result.total))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::{CreateKind, DataChange, ModifyKind, RemoveKind, AccessKind};

    fn event(kind: EventKind, path: &str) -> Event {
        Event::new(kind).add_path(PathBuf::from("/site").join(path))
    }

    fn classify_one(kind: EventKind, path: &str) -> Option<Rebuild> {
        classify(Path::new("/site"), &event(kind, path))
    }

    #[test]
    fn edits_to_sources_rebuild_incrementally() {
        let edit = EventKind::Modify(ModifyKind::Data(DataChange::Content));
        assert_eq!(classify_one(edit, "content/post.md"), Some(Rebuild::Incremental));
        assert_eq!(classify_one(edit, "templates/default.html"), Some(Rebuild::Incremental));
        assert_eq!(
            classify_one(EventKind::Create(CreateKind::File), "static/img/a.png"),
            Some(Rebuild::Incremental),
        );
    }

    #[test]
    fn removals_renames_and_config_changes_rebuild_in_full() {
        assert_eq!(
            classify_one(EventKind::Remove(RemoveKind::File), "content/post.md"),
            Some(Rebuild::Full),
        );
        assert_eq!(
            classify_one(EventKind::Modify(ModifyKind::Name(notify::event::RenameMode::From)), "content/post.md"),
            Some(Rebuild::Full),
        );
        assert_eq!(
            classify_one(EventKind::Modify(ModifyKind::Any), "folio.toml"),
            Some(Rebuild::Full),
        );
    }

    #[test]
    fn the_builds_own_writes_are_ignored() {
        let edit = EventKind::Modify(ModifyKind::Data(DataChange::Content));
        assert_eq!(classify_one(edit, "output/index.html"), None);
        assert_eq!(classify_one(edit, "folio-cache.json"), None);
        assert_eq!(classify_one(EventKind::Create(CreateKind::Folder), "output"), None);
    }

    #[test]
    fn reads_are_ignored() {
        assert_eq!(classify_one(EventKind::Access(AccessKind::Any), "content/post.md"), None);
    }

    #[test]
    fn the_biggest_rebuild_wins() {
        let mixed = Event::new(EventKind::Modify(ModifyKind::Any))
            .add_path(PathBuf::from("/site/content/post.md"))
            .add_path(PathBuf::from("/site/folio.toml"));
        assert_eq!(classify(Path::new("/site"), &mixed), Some(Rebuild::Full));
        assert!(Some(Rebuild::Full) > Some(Rebuild::Incremental));
        assert!(Some(Rebuild::Incremental) > None);
    }
}
