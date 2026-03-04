Interactive line editor for CLI applications.

- Crate [`::rustyline`].
- [docs.rs](https://docs.rs/rustyline)
- [crates.io](https://crates.io/crates/rustyline)
- [GitHub](https://github.com/kkawakam/rustyline)

---

`rustyline` is a Rust implementation of [readline](https://en.wikipedia.org/wiki/GNU_Readline),
providing line editing, history, and completion
for interactive command-line programs.
It supports Emacs and Vi keybinding modes,
persistent file-based history,
and extensible completion and validation through helper traits.

The main type is [`DefaultEditor`],
which reads lines from the terminal with editing support.

## Examples

Reading lines in a loop with history:

```rust,no_run
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

let mut rl = DefaultEditor::new().unwrap();

loop {
    match rl.readline(">> ") {
        Ok(line) => {
            rl.add_history_entry(&line).unwrap();
            println!("You said: {}", line);
        }
        Err(ReadlineError::Interrupted) => {
            println!("Ctrl-C");
            break;
        }
        Err(ReadlineError::Eof) => {
            println!("Ctrl-D");
            break;
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            break;
        }
    }
}
```

[`DefaultEditor`]: crate::rustyline::DefaultEditor
