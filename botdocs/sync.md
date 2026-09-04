Lots of things need to be kept in sync.

- pitch in
  www/index.html,
  book/src/what-is-rustmax.md,
  README.md
- crate names, features in
  crates/rustmax/Cargo.toml,
  crates/rustmax/src/lib.rs,
  crates/rustmax/doc-src/guide.md,
  src/crates.json5,
  src/topics/crates.toml,
  www/sitemap.html,
  crates/rustmax-doctest/src/generate.rs
  (see task-add-crate in tasks.md for the full checklist)
- categories in
  crates/rustmax/doc-src/guide.md,
  book/src/how-do-i.md
- examples in
  crates/rustmax/doc-src/*.md,
  book/src/how-do-i.md
- CSS styles are no longer duplicated:
  www/rustmax-themes.css and www/rustmax-syntax.css are the sources,
  and crates/rustmax-rustdoc/build.rs copies them into the renderer's
  assets so the api docs match the website.
