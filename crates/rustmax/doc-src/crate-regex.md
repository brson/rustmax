Regular expression engine with support for Unicode.

- Crate [`::regex`].
- [docs.rs](https://docs.rs/regex)
- [crates.io](https://crates.io/crates/regex)
- [GitHub](https://github.com/rust-lang/regex)

---

`regex` is a high-performance regular expression engine for Rust
that provides safe, Unicode-aware pattern matching.

The primary interface is the [`Regex`] type,
which represents a compiled regular expression
that can be used to match patterns in text.
The engine is built on finite automata
and provides linear time matching guarantees.

Key features include:
- Full Unicode support by default
- Linear time matching (no exponential backtracking)
- Rich capture group support with named captures
- Multi-line and case-insensitive matching
- Zero-copy string splitting and replacement

For repeated use it's more efficient to compile
the pattern once with [`Regex::new`] and reuse it,
calling methods like [`Regex::is_match`] on the compiled pattern.

## Examples

Basic pattern matching:

```rust
use regex::Regex;

let re = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
let text = "Today's date is 2023-12-25";

assert!(re.is_match(text));

if let Some(mat) = re.find(text) {
    println!("Found date: {}", mat.as_str()); // "2023-12-25"
}
```

Using capture groups to extract parts:

```rust
use regex::Regex;

let re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
let text = "Birthday: 1985-06-15";

if let Some(caps) = re.captures(text) {
    println!("Year: {}", &caps["year"]);   // "1985"
    println!("Month: {}", &caps["month"]); // "06"
    println!("Day: {}", &caps["day"]);     // "15"
}
```

Finding all matches in a string:

```rust
use regex::Regex;

let re = Regex::new(r"\b\w+@\w+\.\w+\b").unwrap();
let text = "Contact us at support@example.com or admin@test.org";

for mat in re.find_iter(text) {
    println!("Email: {}", mat.as_str());
}
// Output:
// Email: support@example.com
// Email: admin@test.org
```

String replacement with capture groups:

```rust
use regex::Regex;

let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
let text = "Date: 2023-12-25";

let result = re.replace(text, "$3/$2/$1");
assert_eq!(result, "Date: 25/12/2023");

// Replace all occurrences
let text = "Dates: 2023-12-25 and 2024-01-01";
let result = re.replace_all(text, "$3/$2/$1");
assert_eq!(result, "Dates: 25/12/2023 and 01/01/2024");
```

Case-insensitive matching:

```rust
use regex::RegexBuilder;

let re = RegexBuilder::new(r"hello")
    .case_insensitive(true)
    .build()
    .unwrap();

assert!(re.is_match("Hello World"));
assert!(re.is_match("HELLO there"));
assert!(re.is_match("hello"));
```

[`Regex`]: crate::regex::Regex
[`Regex::new`]: crate::regex::Regex::new
[`Regex::is_match`]: crate::regex::Regex::is_match