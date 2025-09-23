Date and time library.

- Crate [`::chrono`].
- [docs.rs](https://docs.rs/chrono)
- [crates.io](https://crates.io/crates/chrono)
- [GitHub](https://github.com/chronotope/chrono)

---

`chrono` provides comprehensive date and time functionality for Rust.
It supports timezone-aware and timezone-naive date and time types,
parsing and formatting with standard formats like RFC 3339 and ISO 8601,
and arithmetic operations on dates and times.

Note that while widely used in the Rust ecosystem its API is unwieldy.
Most users that just need to work with dates should prefer [`jiff`].

The library centers around several core types:
[`NaiveDate`] for dates without timezones,
[`NaiveTime`] for times without timezones,
[`NaiveDateTime`] for combined date-time without timezones,
and [`DateTime<Tz>`] for timezone-aware date-times.

Chrono uses the [`TimeZone`] trait to handle timezone conversions,
with built-in support for UTC and local time,
plus optional support for the IANA timezone database through the `chrono-tz` crate.

The design emphasizes correctness and type safety,
making invalid states unrepresentable where possible.
All types are designed to be space-efficient and performant.

## Examples

Working with dates and times:

```rust
use chrono::{DateTime, Utc, Local, NaiveDate, Duration};

// Current time in UTC
let now_utc: DateTime<Utc> = Utc::now();
println!("UTC now: {}", now_utc);

// Current local time
let now_local: DateTime<Local> = Local::now();
println!("Local now: {}", now_local);

// Create a specific date
let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
let datetime = date.and_hms_opt(14, 30, 0).unwrap();
println!("Specific datetime: {}", datetime);

// Add duration
let future = now_utc + Duration::days(7);
println!("One week from now: {}", future);
```

Parsing and formatting dates:

```rust
use chrono::{DateTime, Utc, NaiveDateTime};

// Parse RFC 3339 / ISO 8601
let parsed = "2024-03-15T14:30:00Z".parse::<DateTime<Utc>>().unwrap();
println!("Parsed: {}", parsed);

// Custom formatting
let formatted = parsed.format("%Y-%m-%d %H:%M:%S");
println!("Formatted: {}", formatted);

// Parse custom format
let naive = NaiveDateTime::parse_from_str(
    "2024-03-15 14:30:00",
    "%Y-%m-%d %H:%M:%S"
).unwrap();
println!("Parsed naive: {}", naive);
```

[`NaiveDate`]: crate::chrono::NaiveDate
[`NaiveTime`]: crate::chrono::NaiveTime
[`NaiveDateTime`]: crate::chrono::NaiveDateTime
[`DateTime<Tz>`]: crate::chrono::DateTime
[`TimeZone`]: crate::chrono::TimeZone
[`jiff`]: crate::jiff
