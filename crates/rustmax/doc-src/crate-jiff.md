Modern date and time library.

- Crate [`::jiff`].
- [docs.rs](https://docs.rs/jiff)
- [crates.io](https://crates.io/crates/jiff)
- [GitHub](https://github.com/BurntSushi/jiff)

---

`jiff` is a modern date and time library for Rust that prioritizes correctness,
ergonomics, and performance. It provides timezone-aware date arithmetic,
high-precision timestamps, and comprehensive parsing and formatting capabilities.

The library offers several core types:
[`Timestamp`] for nanosecond-precision instants in time,
[`Zoned`] for timezone-aware date-times,
[`DateTime`] for calendar date-times without timezone,
[`Date`] for calendar dates,
and [`Time`] for wall clock times.

Jiff excels at timezone-aware operations and automatically handles
daylight saving time transitions correctly.
It includes built-in support for the IANA Time Zone Database
and provides powerful duration arithmetic that respects calendar rules.

The API design emphasizes preventing common datetime bugs through
the type system, making operations like cross-timezone comparisons
and DST-aware arithmetic safe by default.

## Examples

Working with timestamps and basic operations:

```rust
use jiff::{Timestamp, civil::Date};

// Current timestamp
let now = Timestamp::now();
println!("Now: {}", now);

// Create a specific date
let date = Date::new(2024, 3, 15).unwrap();
println!("Date: {}", date);

// Get Unix timestamp
let unix_seconds = now.as_second();
println!("Unix timestamp: {}", unix_seconds);

// Parse an RFC 3339 timestamp
let parsed: Timestamp = "2024-03-15T14:30:00Z".parse().unwrap();
println!("Parsed: {}", parsed);
```

Date arithmetic and spans:

```rust
use jiff::{civil::Date, ToSpan};

// Create dates
let start = Date::new(2024, 3, 15).unwrap();
let end = Date::new(2024, 12, 25).unwrap();

// Calculate span between dates
let span = start.until(end).unwrap();
println!("Span: {}", span);

// Add one month using ToSpan
let next_month = start.checked_add(1.month()).unwrap();
println!("Next month: {}", next_month);

// Add days
let future = start.checked_add(30.days()).unwrap();
println!("30 days later: {}", future);
```

[`Timestamp`]: crate::jiff::Timestamp
[`Zoned`]: crate::jiff::Zoned
[`DateTime`]: crate::jiff::civil::DateTime
[`Date`]: crate::jiff::civil::Date
[`Time`]: crate::jiff::civil::Time