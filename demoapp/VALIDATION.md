# What Anthology found in rustmax

Anthology exists to put weight on `rustmax` until something gives.
This is what gave.

Findings are kept here after they are fixed,
because the interesting part is usually the shape of the mistake
rather than the mistake.

## Fixed

### `rmx-profile-proc-macro` could not write a proc macro

`syn` is declared `default-features = false`,
and no ecosystem feature turned any of them back on.
The profile therefore shipped a `syn` with no
`parsing`, `printing`, `derive` or `full`,
which leaves `syn::parse2`, `syn::parse_file` and `DeriveInput` all absent.
That is every way a derive macro would use the crate.
`quote` and `proc-macro2` were both getting their defaults,
so the profile looked complete and was not.

What kept it hidden is that it works under `rmx-profile-max`:
`thiserror-impl` and `proptest` are in that graph
and unify `syn`'s features back on.
Anthology hit it because it reached for `syn` under `max`, where it worked,
and the bug only appeared when the profile was isolated.

Fixed by `fe9cfc5`.

*The general shape:* every crate declared `default-features = false`
whose features are not restored by an `rmx-feature-*` list
is a candidate for the same bug,
and feature unification will hide it in any profile
that happens to pull the crate in twice.

Nineteen crates are in that position.
Checking each against its own manifest,
all but two declare no default features at all,
so there is nothing to lose:
`cc`, `cfg-if`, `ctrlc`, `cxx-build`, `extension-trait`, `glob`,
`ignore`, `json5`, `mime`, `powerletters`, `rayon`, `socket2`,
`termcolor`, `unicode-segmentation`, `walkdir`, `xshell`.
`flate2`'s sole default, `rust_backend`, is set explicitly.
`thiserror` loses `std`, which does not matter
because it cannot be reached through the re-export anyway.

That leaves `comrak`, which loses `cli`, `syntect` and `bon`.
Dropping `cli` and `syntect` is a real saving and clearly wanted.
Dropping `bon` removes `Options::builder()`,
so `rmx::comrak` is configured by `Options::default()` and field assignment,
which is what Anthology does and is no hardship.
Noted rather than filed: it is a smaller API than upstream's,
and the guide does not say so.

`syn` was the only one that was actually broken.

### `bug!` with a message did not compile

The message-taking arm expanded to `$crate::format_args!`,
and `rustmax` re-exports no `format_args` at its root,
so any use of `bug!("...")` failed to resolve.
The no-argument arm worked only because it named `core` bare
and `core` is always in the extern prelude.

Neither arm was tested and the macro had no doctest.
Anthology hit it the first time it wrote `bug!("binary export format")`.

Fixed by `7961e8b`, routing both arms through `$crate::core`
and adding a doctest over the arm that was broken.

## Confirmed working

Things that were expected to be hard and were not.

**`#[rmx::derive]` under a renamed dependency.**
Anthology depends on `rustmax` as `rmx` and derives
`Serialize`, `Deserialize`, `Parser`, `Subcommand`, `ValueEnum`,
`Display`, `From`, `derive_more::Error`,
`TryFromPrimitive` and `IntoPrimitive`
across some thirty sites, with container and field attributes throughout,
including `#[serde(with = ..)]`, `#[serde(flatten)]`, `#[serde(default = ..)]`
and `#[from(ignore)]`.
None of it needed a workaround.

**The `thiserror` diagnostic.**
Writing `#[rmx::derive(Error)]` without a direct `thiserror` dependency
produced an error naming the problem and the alternative,
which is exactly what was needed --
the fix (`derive_more::Error`) came straight from the message
with no detour into the guide.

**The prelude in a large program.**
`use rmx::prelude::*` appears in every module.
Its glob of `powerletters::*`, which brings
single-uppercase-letter method names into scope,
produced no ambiguity across twelve thousand lines.

## Rough edges, not bugs

**`#[tokio::test]` needs `use rmx::tokio;`.**
Documented -- `tokio` names its crate relative to the use site,
like `clap` and `derive_more` --
but the failure mode is a wall of
`cannot find module or crate `tokio``
pointing at every test in the file,
which does not obviously suggest the one-line fix.
`#[rmx::derive]` handles the derives in this class;
attribute macros like `tokio::test` and `tokio::main` are left to the reader.
An `#[rmx::attr(tokio::test)]`, or a note in the guide's macro section
naming the error text, would close the gap.

**`tempfile` and `comrak` run with no default features.**
Both work, but for the same structural reason `syn` did not:
nothing restores what `default-features = false` removed.
They happen to need nothing.

## Not covered, and why

`bindgen`, `cc`, `cxx`, `cxx-build`, `libc`, `libm`, `num-bigint`,
`hyper`, `proc-macro2`, `quote`, `rand_pcg`, `thiserror`.

Reasons are in [DESIGN.md](DESIGN.md#deliberately-not-covered).
The short version is that a static site generator
has no honest use for a C compiler driver or a bignum,
and inventing one would test nothing.
Covering the build-script and FFI profiles
needs a different demo application, not a bigger one.
