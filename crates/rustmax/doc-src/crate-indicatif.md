Progress bars and spinners for CLI applications.

- Crate [`::indicatif`].
- [docs.rs](https://docs.rs/indicatif)
- [crates.io](https://crates.io/crates/indicatif)
- [GitHub](https://github.com/console-rs/indicatif)

---

`indicatif` provides progress bars, spinners, and other progress indicators
for command-line applications.
It handles terminal width, colors, and smooth updates.

The main types are [`ProgressBar`] for single progress indicators
and [`MultiProgress`] for displaying multiple progress bars simultaneously.
Use [`ProgressStyle`] to customize the appearance.

## Examples

Simple progress bar:

```rust,no_run
use indicatif::ProgressBar;

let pb = ProgressBar::new(100);
for i in 0..100 {
    pb.inc(1);
    // Do work...
}
pb.finish_with_message("done");
```

Styled progress bar:

```rust,no_run
use indicatif::{ProgressBar, ProgressStyle};

let pb = ProgressBar::new(1000);
pb.set_style(ProgressStyle::with_template(
    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})"
).unwrap());

for _ in 0..1000 {
    pb.inc(1);
}
pb.finish();
```

Spinner for indeterminate progress:

```rust,no_run
use indicatif::ProgressBar;
use std::time::Duration;

let spinner = ProgressBar::new_spinner();
spinner.set_message("Loading...");
spinner.enable_steady_tick(Duration::from_millis(100));
// Do work...
spinner.finish_with_message("Done!");
```

[`ProgressBar`]: crate::indicatif::ProgressBar
[`MultiProgress`]: crate::indicatif::MultiProgress
[`ProgressStyle`]: crate::indicatif::ProgressStyle
