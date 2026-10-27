# What Folio found in rustmax

Folio exists to put weight on `rustmax` until something gives.
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
Folio hit it because it reached for `syn` under `max`, where it worked,
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
which is what Folio does and is no hardship.
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
Folio hit it the first time it wrote `bug!("binary export format")`.

Fixed by `7961e8b`, routing both arms through `$crate::core`
and adding a doctest over the arm that was broken.

## Confirmed working

**`#[rmx::tokio::test]` and `#[rmx::tokio::main]`.**
These used to fail with a wall of `cannot find module or crate `tokio``,
whose suggested fix was `cargo add tokio`,
because `tokio`'s macros name `tokio` relative to the use site.
The workaround was `use rmx::tokio;` at each use site.
`rustmax` now shadows both with stand-ins that pass `tokio`
its `crate = ".."` argument, and Folio's fetch test uses
`#[rmx::tokio::test]` with nothing imported.
They were the only attribute macros among the re-exported crates that needed it:
`extension_trait` works as it is,
and the function-like proc macros all sit behind `$crate`-using `macro_rules!`.

Things that were expected to be hard and were not.

**`#[rmx::derive]` under a renamed dependency.**
Folio depends on `rustmax` as `rmx` and derives
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

**Exported macros through the re-export.**
`crossbeam`'s `select!` and `bitflags!` expand to paths into their own crates,
the same thing that defeats `thiserror`,
but both name those paths through `$crate`,
which resolves to wherever the macro was reached from.
`rmx::crossbeam::channel::select!` and `rmx::bitflags::bitflags!`
work with no direct dependency.

## Rough edges, not bugs

**`tempfile` and `comrak` run with no default features.**
Both work, but for the same structural reason `syn` did not:
nothing restores what `default-features = false` removed.
They happen to need nothing.

## Not covered, and why

`bindgen`, `cc`, `libc`, `libm`, `num-bigint`,
`proc-macro2`, `quote`, `rand_pcg`, `thiserror`,
`chrono`, `cfg-if`, `xshell`, `rustc-hash`, `tower`, `socket2`.

Reasons are in [DESIGN.md](DESIGN.md#deliberately-not-covered).
The short version is that a static site generator
has no honest use for a C compiler driver or a bignum,
and inventing one would test nothing.
Covering the build-script and FFI profiles
needs a different demo application, not a bigger one.
