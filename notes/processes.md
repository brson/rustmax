Processes for making typical updates to rustmax.

# Document a rustmax crate

Each crate in rustmax is re-exported by the rustmax crate,
with some additional rustmax-specific documentation,
including a short description that is insightful
about the Rust ecosystem, and copy-pastable examples
for typical tasks.

Assuming the crate is already a working dependency
of rustmax, that is it is listed in the rustmax manifest,
features wired up following the existing pattern in the manifest,
re-exported following the pattern in `lib.rs`.

- Add crate documentation.
  To a per-crate markdown file in `crates/rustmax/doc-src`,
  following the existing pattern in `crate-ahash.md` and others there.
- Include it in the API docs.
  Add it to `lib.rs` via `include!` per existing patterns.
- Possibly update `src/crates.json5`.
  This contains metadata used by the build process.
- Update `linksubs.json5`.
  Any Rust links in the crate Markdown docs need to be
  rewritten to link to `docs.rs` or `doc.rust-lang.org`.
  These can be verified by running `just prebuild` and checking
  stderr for "unreplaced link" lines.

  