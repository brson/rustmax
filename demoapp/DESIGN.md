# Folio Design

Folio is a static site generator for technical writing,
and the application `rustmax` is tested against.

## Two jobs

1. Be a static site generator worth using.
2. Put weight on `rustmax` until something breaks.

The second job is why Folio exists,
and it only works if the first one is done honestly.
A program that calls one function from each crate
to fill in a checklist does not find anything;
what finds things is a real program
that reaches for whichever crate the task actually calls for
and has to live with the result.
Every crate listed under [Coverage](#coverage) below
is used by a feature that would exist anyway.
Every crate that is not listed is not listed for a reason,
and the reasons are given.

## One dependency

```toml
[dependencies]
rmx.package = "rustmax"
rmx.path = "../crates/rustmax"
rmx.features = ["rmx-profile-max"]
```

That is the whole manifest.
Folio takes no direct dependency on any crate `rustmax` re-exports,
which is the strongest statement it can make about the supercrate:
a twelve-thousand-line program with a CLI, a markdown pipeline,
a search engine and four export formats
needs nothing else.

The dependency is renamed to `rmx`,
matching the project template,
so the macros are exercised against a dependency
that is not named `rustmax` --
`#[rmx::derive]` finds its own path through the depending crate's manifest,
and a rename is the case that can go wrong.

Getting to one dependency required two things to change.

**Derives go through `#[rmx::derive]`.**
Folio previously carried direct dependencies on `serde` and `clap`
purely so their derive macros would resolve.
`#[rmx::derive(..)]` removes that,
and all thirty-odd derive sites use it.

**The error type uses `derive_more`, not `thiserror`.**
`thiserror` generates `::thiserror`,
which resolves only against the extern prelude,
so it is the one derive crate that cannot be reached through a re-export.
`derive_more`'s `Display`, `Error` and `From` cover the same ground
and do work through `rustmax`.
See [`src/error.rs`](src/error.rs).

## Layout

```
src/
  main.rs          entry point
  lib.rs           module exports
  error.rs         error type (derive_more)
  cli/             clap command definitions, REPL
  collection/      config, documents, frontmatter, directory scanning
  build/           the build pipeline
    markdown.rs      comrak
    template.rs      tera
    highlight.rs     syntax highlighting
    toc.rs           table of contents, HeadingLevel
    cache.rs         incremental builds
    images.rs        image processing
    compress.rs      gzip
    encoding.rs      base64/hex
    rewrite.rs       URL rewriting
    search_js.rs     client-side search bundle
    assets.rs        generated CSS/JS and their SRI hashes (sha2)
    progress.rs      progress bars
  features/        `[build] features` (bitflags)
  shortcode/       shortcodes (nom, memchr)
  lint/            content lints, including Rust code block parsing
  search/          BM25 index, Porter stemmer
  export/          EPUB and tar archives
  feeds/           Atom and JSON Feed
  remote/          `fetch` (reqwest)
  watch/           `watch` (notify, crossbeam, ctrlc)
  util/            reproducible identifier generation
```

Every module is reached from a command.
An earlier version carried `crypto/`, `text/`, `time/` and `concurrency/`
modules that nothing called,
each a list of wrappers around one crate with unit tests of its own.
They were checklist-filling, the thing this app exists to avoid,
and were deleted;
the crates that had a real job to do were given one.

## Design decisions

### Reproducible builds

`[build] seed` fixes every generated identifier,
and `SOURCE_DATE_EPOCH`, the [reproducible builds] convention,
fixes the time stamped into pages, feeds, EPUBs and tarballs.
With both, two builds of the same content produce the same bytes,
and the integration tests compare every file of two builds to check it.

[reproducible builds]: https://reproducible-builds.org/specs/source-date-epoch/
[`util::IdGenerator`](src/util/mod.rs) is `ChaCha8Rng` rather than the thread RNG
because its output is a documented function of its seed
on every platform and every run.

Reproducibility turned out to be a stricter constraint than it looked.
The search index is written to disk as JSON from a hash map,
and a hash map iterates in an order that depends on the process's hash seed,
so the index differed between two builds of identical content
before anything else did.
`SearchIndex::word_index` now serializes through a `BTreeMap`,
and `suggest` sorts before truncating to ten
so the same prefix suggests the same words every time.

The exports had their own versions of the problem.
The EPUB identifier was a fresh random UUID,
which also told reading systems every export was a different book;
it is now derived from the collection's base URL and title.
`tar`'s `append_dir_all` writes entries in whatever order the filesystem lists them,
with real owners and mtimes;
the archive is now written from a sorted walk,
with deterministic headers whenever the time is pinned.

The time is read once, in `Config::load`, into `BuildConfig::timestamp`,
rather than wherever a timestamp is wanted,
so tests can pin it without touching the process environment.
Pinned or not, it is UTC; the Atom feed used to format local time
with a `Z` suffix, which was wrong anywhere but Greenwich.

### Two config formats

A collection is configured by `folio.toml`, or by `folio.json5`
if it was generated by something that speaks JSON.
JSON5 reads plain JSON as well as its own extensions,
so one parser covers both.
`[collection] folio_version` is a semver requirement
checked when the config loads.

### Lints over prose

Folio is aimed at technical writing,
where what usually goes wrong in a document is the code, not the sentences.
`folio check` parses every Rust code block with `syn`
and reports the ones that do not parse,
honouring rustdoc's conventions:
hidden `#` lines are part of the program,
`ignore` and `compile_fail` opt out,
and a snippet with no `fn main` is checked as a `main` body.
Lints carry the source line of the opening fence.

### Typed heading levels

Markdown and HTML headings stop at six,
so a level outside one to six is a parse that has gone wrong,
not a deep heading.
`HeadingLevel` (`num_enum`) makes that unrepresentable
and replaced an `unwrap_or(1)` that silently papered over it.

### Build features

`[build] features` lists what a build writes beyond the document pages:
`search`, `feeds`, `sitemap`, `shortcodes`, `compress` and `drafts`.
The default is the first four.
`folio build --drafts` and `--compress` add to the configured set.
`BuildFeatures` is a `bitflags` type,
and its config form is a list of lowercase flag names,
read and written through `from_name` and `iter_names`
so that adding a flag adds its config name with it.
An unknown name is a config error that lists the known ones.

Every flag is honoured by the build.
The earlier set included `minify` and `inline_images`,
which nothing implemented; they were removed rather than left to mislead.

### Shortcodes

`{{< name args >}}` and `{{% name %}}body{{% /name %}}`
are parsed with `nom` and expanded before the markdown is rendered.
The built-ins are `youtube`, `figure`, `note`, `warning`, `tip`, `code` and `toc`.

Because Folio is for technical writing,
documents routinely show shortcode syntax rather than use it,
so anything inside a fenced code block or an inline code span is left alone.
Candidates are found with `memchr::memmem`,
which also fixed a panic:
the scanner used to step through the text one byte at a time
and sliced through the middle of any multibyte character.

An unknown shortcode is left in the page as written
and reported by `folio check`.
`toc` cannot be rendered until the heading ids exist,
so it expands to a marker that is filled in from the rendered HTML.

### Subresource Integrity

The generated stylesheets and scripts are produced before any page is rendered,
so every page can carry their SHA-384 hashes (`sha2`, `base64`)
in the `integrity` template map,
keyed by file name with punctuation replaced by `_`
(`integrity.highlight_css`).
The default template uses them on its stylesheet links.

### Watching

`folio watch` builds once, then rebuilds whenever the collection changes.
It waits on three things at once with `crossbeam`'s `select!`:
`notify` file events, a stop request from `ctrlc`,
and a debounce timer that every event restarts,
since an editor's save is several events in quick succession.

Only `content/`, `templates/` and `static/` are watched recursively.
The root is watched on its own, for the config file,
because watching it recursively would see the build's own writes
to the output directory and the cache, and rebuild forever.

An edit rebuilds incrementally.
A removal, a rename or a config change rebuilds in full,
since an incremental build neither deletes pages whose source is gone
nor notices that settings every page depends on have changed.

Ctrl-C only sends on a channel, which the loop reads between builds,
so stopping never interrupts a build halfway through writing its cache.
`watch_until` takes that channel as a parameter,
which is how the integration test drives a real watch and stops it.

### Queries belong to the documents

`Collection` is not the only thing holding documents:
the REPL narrows them as the user filters.
`DocumentsExt` (`extension_trait`) puts `published`, `by_tag`, `by_slug` and `tags`
on `[Document]`, and `Collection` delegates.

## Coverage

Used by a feature, as of this writing: 53 of the 73 crates `rustmax` re-exports.

| Area | Crates |
| --- | --- |
| CLI | `clap`, `termcolor`, `rustyline`, `indicatif`, `ctrlc` |
| Files | `notify` |
| Web | `reqwest`, `mime`, `url` |
| Async | `tokio`, `futures` |
| Concurrency | `rayon`, `crossbeam` |
| Parsing | `comrak`, `regex`, `nom`, `syn` |
| Templates | `tera` |
| Serialization | `serde`, `serde_json`, `toml`, `json5` |
| Encoding | `flate2`, `base64`, `hex`, `bytes`, `zip`, `tar` |
| Hashing | `blake3`, `sha2` |
| Filesystem | `walkdir`, `ignore`, `glob`, `tempfile` |
| Time | `jiff` |
| Text | `unicode-segmentation`, `memchr` |
| Collections | `itertools`, `ahash` |
| Numbers | `semver`, `num_enum`, `bitflags` |
| Random | `rand`, `rand_chacha` |
| Graphics | `image` |
| Logging | `log`, `env_logger` |
| Errors | `derive_more`, `anyhow` |
| Language | `extension-trait`, `rustmax-macros`, `powerletters` |
| Testing | `proptest` |

### Deliberately not covered

Filling these in would mean inventing work,
which is the failure mode this app is supposed to avoid.

| Crate | Why not |
| --- | --- |
| `bindgen`, `cc`, `cxx`, `cxx-build` | Folio links no C or C++. |
| `libc` | Nothing here needs a raw syscall. |
| `libm` | `no_std` math. Folio is a hosted program. |
| `num-bigint` | No arbitrary-precision arithmetic anywhere in a site generator. |
| `hyper` | `reqwest` is the layer Folio works at. Reaching past it would be pretence. |
| `http`, `http-body-util`, `hyper-util`, `tower`, `socket2` | Used only by the dev server, which was removed when `axum` left `rustmax`. Candidates for a `folio serve`. |
| `xshell` | Folio runs no external commands. |
| `cfg-if` | No platform-specific code. |
| `chrono` | `jiff` covers every date Folio handles. A second date library would be decoration. |
| `rustc-hash` | `ahash` already covers the one hash map where hashing speed matters, the search index. |
| `proc-macro2`, `quote` | Only meaningful inside a proc-macro crate. `syn` is here because parsing Rust is a real feature; generating it is not. |
| `rand_pcg` | `rand_chacha` already covers the reproducibility requirement. A second PRNG would be decoration. |
| `thiserror` | Deliberately avoided, and that is the finding. See [One dependency](#one-dependency). |

`powerletters` is glob-imported by the prelude
rather than named directly.
That it introduces no ambiguity across twelve thousand lines
is itself worth knowing.

## Testing

263 tests: 234 unit, 29 integration.

Reproducibility is tested by building the same collection twice
and comparing the output byte for byte,
plus a direct assertion that the search index's terms come out sorted,
since equal bytes from two builds in one process could be luck.

`proptest` covers hashing, stemming and frontmatter parsing invariants.
