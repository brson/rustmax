Cross-platform terminal colors.

- Crate [`::termcolor`].
- [docs.rs](https://docs.rs/termcolor)
- [crates.io](https://crates.io/crates/termcolor)
- [GitHub](https://github.com/BurntSushi/termcolor)

---

`termcolor` provides cross-platform colored output to terminals.
On Unix it uses ANSI escape codes;
on Windows it uses the console API,
so colors work correctly in `cmd.exe` and PowerShell without extra setup.

In use: create a [`StandardStream`],
set colors with a [`ColorSpec`] via the [`WriteColor`] trait,
write text, then [`reset`] back to defaults.
[`ColorChoice`] controls whether colors are actually emitted,
making it easy to respect `--color=auto/always/never` flags.

For multithreaded programs, [`BufferWriter`] lets each thread
write to an independent [`Buffer`] that is flushed atomically,
preventing interleaved output.

## Examples

Print colored text to stdout:

```rust,no_run
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

let mut stdout = StandardStream::stdout(ColorChoice::Auto);
stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
writeln!(&mut stdout, "success").unwrap();
stdout.reset().unwrap();
```

Print with bold red for errors:

```rust,no_run
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

let mut stderr = StandardStream::stderr(ColorChoice::Auto);
stderr.set_color(
    ColorSpec::new()
        .set_fg(Some(Color::Red))
        .set_bold(true)
).unwrap();
writeln!(&mut stderr, "error: something went wrong").unwrap();
stderr.reset().unwrap();
```

[`StandardStream`]: crate::termcolor::StandardStream
[`ColorSpec`]: crate::termcolor::ColorSpec
[`WriteColor`]: crate::termcolor::WriteColor
[`reset`]: crate::termcolor::WriteColor::reset
[`ColorChoice`]: crate::termcolor::ColorChoice
[`BufferWriter`]: crate::termcolor::BufferWriter
[`Buffer`]: crate::termcolor::Buffer
