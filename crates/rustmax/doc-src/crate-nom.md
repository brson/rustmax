Parser combinator library for building zero-copy parsers.

- Crate [`::nom`].
- [docs.rs](https://docs.rs/nom)
- [crates.io](https://crates.io/crates/nom)
- [GitHub](https://github.com/rust-bakery/nom)

---

`nom` is a parser combinator library that enables building parsers
by combining small, reusable parsing functions.

Rather than using separate grammar definition tools or generating code,
nom parsers are written directly in Rust as composable functions.
Each parser function takes an input slice and returns either
the remaining input with the parsed value,
or an error.
These small parsers can be combined using combinator functions
to build complex parsers that match the structure of your grammar.

Parser combinators have tradeoffs vs. other parser
architectures, but it is nice to have one at hand when you need it,
and `nom` is well-tested and well-maintained.
And it is fast.

Key features include:
- Zero-copy parsing that works with input slices directly
- Streaming support for incomplete input (useful for network protocols)
- Both byte-oriented and character-oriented parsing
- Rich set of built-in parsers and combinators
- Strong type safety with compile-time validation

## Examples

Parsing a simple numeric value:

```rust
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::digit1,
};

fn parse_number(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

let result = parse_number("123abc");
assert_eq!(result, Ok(("abc", "123")));
```

Combining parsers with sequence operations:

```rust
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::digit1,
    sequence::tuple,
};

fn parse_date(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((
        digit1,
        tag("-"),
        digit1,
        tag("-"),
        digit1,
    ))(input)
    .map(|(rest, (year, _, month, _, day))| (rest, (year, month, day)))
}

let result = parse_date("2023-12-25 text");
assert_eq!(result, Ok((" text", ("2023", "12", "25"))));
```
