# Task: teach rustmax-cli to manage the mold linker

The mold linker is often used with Rust on Linux.
I rarely set it up though because I can't just `cargo install` it.
rustmax-cli is intended to manage common Rust tools,
both those installable by cargo and not,
though it's only just sketched out in the `Tool` type.

The `moldman` module is already sketched out,
and wired up to the `install-tool`, `update-tool`,
`uninstall-tool`, and `tool-status` CLI commands;
but I haven't done all the gnarly work of
getting the recent moldman version from GitHub,
managing installation to the user's home directory,
etc.

Fill in the functions in the `moldman` module,
using the crates re-exported from `rmx`
to get the most recent mold from GitHub, etc.
