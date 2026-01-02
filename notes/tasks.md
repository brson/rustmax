# Task: task-update-library

The library is a collection of mdbooks
we attempt to build in a consistent way and publish;
but they all use different mdbook and plugins
and its a bit of a mess.

Metadata is in src/books.json5.

They are managed with the cli commands `refresh-library`,
`install-library-deps`, and `build-library`.

We usually build with `just doc-library`.

Go through the book repos and update books.json5 to the latest commits
of each;
run `just doc-library`;
examine the output for errors
and try to resolve them.


# Task: task-upgrade-crates

Check for major version bumps of all the rustmax crate's public dependencies
(i.e. not the underscore-prefixed hacks).
Remember in Rust pre-1.0 crates, minor version bumps are considered major.

For each, update the version in Cargo.toml
and run `just test`.

Keep the ones that pass.

Report on the keepers and non-keepers.

The `just maint-outdated` rule may help.


# Task: task-review-unreplaced-links

Our crate docs in `doc-src` are displayed in both rustdocs
and on the website. The links are written for rustdoc,
and need to be replaced in the website build.

Run `just doc-www` and look for "unreplaced link" lines.

The ones that are just rust module paths need to be replaced
with appropriate links to docs.rs, or perhaps to the standard library docs,
by editing linksubs.json5.


# Task: task-review-existing-trusted-maintainers

We have a notion of a "trusted maintainer"
with the list in maintainers.json5,
and the crates they maintain in crates.json5

Review each crates maintainer and make sure the crate is still maintained by them.

If you see a crate mantained by a trusted mantainer, add them to our crate metadata.

Do not add any new trusted maintainers.


# Task: task-sync-sitemap

The sitemap in sitemap.html is hand-written,
and index into every piece of info in rustmax.

Cross-reference it with its sources and make sure
the entries in it are complete and correct.


# Task: task-check-crate-root-docs

The rustmax crate docs are extensive,
with categories and lists and descriptions.
It's easy to fall out of sync with the actual state of the rustmax
crate dependencies and their descriptions and docs.

These docs live in `root-docs.md`.
The crate descriptions live in `crate-XYZ.md` next to it,
with metadata in the `rustmax/Cargo.toml` file as well as `crates.json5`.

Do a pass over `root-docs.md` and fix any inconsistencies.


# Task: task-improve-one-book-section

This project is rustmax,
a learning tool for rust users
derived from my expert experience.

We're working on the rustmax book,
in book/, with text in book/src.

The book has many chapters
and sections sketched out but
is incomplete.

Pick one section of one markdown
file in book/src and improve it.

The best text will be concise,
and impart "tribal knowledge"
about rust that will be non-obvious to new users.


# Task task-document-one-crate

Each crate should have a doc file in crates/rustmax/doc-src/.
Most don't.

Pick one of the crate dependencies of the rustmax crate that is
not documented and document it. Include the docs via lib.rs per previous crates.

**Steps to follow:**

1. **Check existing documentation:** Look in `crates/rustmax/doc-src/` for existing `crate-*.md` files
2. **Pick an undocumented crate:** Choose from dependencies in `crates/rustmax/Cargo.toml`
3. **Create documentation file:** `crates/rustmax/doc-src/crate-NAME.md`
  - follow existing conventions to create content
4. **Update lib.rs:** replace module contents per other crates
5. **Update linksubs.json5:** Add entries for any cross-references in your documentation
6. **Test with `just doc-crates`:** Run and check for "unreplaced link" warnings
7. **Verify crates.json5:** Usually already has the crate entry, but check it has appropriate metadata

Examples should be runnable. Test with

```
cargo test --doc -p rustmax --features=rmx-profile-max
```

The examples should be testing the crates, not the rustmax prelude.
Do not import the rustmax prelude or reference anything under the rustmax namespace.

Keep examples simple, practical, and minimal.
Just one or a few examples.

Note that the examples need to compile within whatever rustmax
profile that crate is in - so e.g. the thiserror example can't use reqwest.
Run `just test` to test in all profiles.


# Task task-add-crate: Add a crate to rustmax

**Files to update:**
1. `crates/rustmax/Cargo.toml` - Add dependencies and feature groups
2. `crates/rustmax/src/lib.rs` - Add module reexports with `#![doc = include_str!("../doc-src/crate-NAME.md")]`
3. `crates/rustmax/doc-src/root-docs.md` - Update category table and profile sections
4. `src/crates.json5` - Add crate metadata (category, descriptions)
5. `README.md` - Add to documentation table
6. `crates/rustmax/doc-src/crate-NAME.md` - Create detailed documentation

**Key steps:**
- Add crate to appropriate feature group (usually `rmx-crates-std`)
- Only add feature flags that actually exist (check docs for `default`, `std` features)
- Remove non-existent features from `rmx-feature-default` and `rmx-feature-std`
- Run `cargo check` to verify no feature conflicts
- Use proper descriptions: "Low-level" vs "High-level", match existing patterns
- If it's by a "trusted maintainer", make sure we add that metadata as appropriate.
- Update linksubs.json5 as appropriate
- If it is listed in radar.md remove it.

**Common gotchas:**
- Not all crates have `default` or `std` features
- Audio crates go in `rmx-crates-std` (require std)
- Some crates have important features to add to `rmx-feature-more`,
  ask the user.
- Keep alphabetical order in all files
- If the crate is maintained by a trusted maintainer in src/maintainers.json5,
  remember to add it to the crates.json5 meta.
- Test with `cargo check --all-features`
- Test with `just test`

See also processes.md.


# Task task-add-cargo-plugin-cli: Add a cargo plugin to rustmax-cli

`rustmax-cli` has four commands that operate on "tools":
`install-tool`, `uninstall-tool`, `update-tool`, `tool-status`;
the source for which begins in `rustmax-cli/src/main.rs`, `tools.rs` and `impls.rs`.

To add a new cargo plugin:

- Add it to the `Tools` enum if it doesn't already exist.
- Create the `CargoPluginConfig` constant for the tool.
- Fill in any associated match arms in `tools.rs` and `impls.rs`
- Follow the coding pattern that already exists for cargo plugins like `cargo-audit` and `cargo-clean-all`,
  defering to `cargo_plugin_install` etc for the primary logic.
- Think about if these tools have any special considerations for post-install/uninstall actions etc.
  Do they store caches in the home directory that can be delete on uninstall?


# Task task-add-cargo-tool-cli: Add a cargo tool to rustmax-cli

`rustmax-cli` has four commands that operate on "tools":
`install-tool`, `uninstall-tool`, `update-tool`, `tool-status`;
the source for which begins in `rustmax-cli/src/main.rs`, `tools.rs` and `impls.rs`.

To add a new cargo-installable tool that is not a cargo plugin:

- Add it to the `Tools` enum if it doesn't already exist.
- Create the `CargoToolConfig` constant for the tool.
- Fill in any associated match arms in `tools.rs` and `impls.rs`
- Follow the coding pattern that already exists for cargo programs like `basic-http-server` and `ripgrep`,
  defering to `cargo_tool_install` etc for the primary logic.
- Think about if these tools have any special considerations for post-install/uninstall actions etc.
  Do they store caches in the home directory that can be delete on uninstall?


# Task task-add-cargo-tool-to-book: Add a cargo-based tool to the rustmax book

Rustmax documents important Rust tools, most of which can be install by cargo.

These are documented in book/src/tools.md

- Decide which category your tool belongs to
- Add the tool to the index at the top of the file
- Add a corresponding section for the tool to the correct location in the file

When writing documentation try to express
why this tool is significant to the Rust ecosystem.
Write one or two examples of how it is most typically used.
