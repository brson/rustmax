//! Time and date utilities using jiff and chrono.

use rustmax::prelude::*;
use rustmax::jiff::{civil::Date, Zoned};
use rustmax::chrono::{NaiveDate, DateTime, Utc, Local, Datelike};

/// Parse a date string using jiff.
pub fn parse_date(s: &str) -> Option<Date> {
    s.parse().ok()
}

/// Parse a date string using chrono (for compatibility).
pub fn parse_date_chrono(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
}

/// Get today's date using jiff.
pub fn today() -> Date {
    Zoned::now().date()
}

/// Get today's date using chrono.
pub fn today_chrono() -> NaiveDate {
    Local::now().date_naive()
}

/// Get current timestamp.
pub fn now() -> Zoned {
    Zoned::now()
}

/// Get current UTC timestamp using chrono.
pub fn now_utc_chrono() -> DateTime<Utc> {
    Utc::now()
}

/// Format a jiff date as ISO string.
pub fn format_date_iso(date: Date) -> String {
    date.to_string()
}

/// Format a chrono date as ISO string.
pub fn format_date_chrono_iso(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Format a date for display (e.g., "January 15, 2024").
pub fn format_date_long(date: Date) -> String {
    let month = match date.month() {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    };
    format!("{} {}, {}", month, date.day(), date.year())
}

/// Convert jiff Date to chrono NaiveDate.
pub fn jiff_to_chrono(date: Date) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(date.year() as i32, date.month() as u32, date.day() as u32)
}

/// Convert chrono NaiveDate to jiff Date.
pub fn chrono_to_jiff(date: NaiveDate) -> Option<Date> {
    Date::new(date.year() as i16, date.month() as i8, date.day() as i8).ok()
}

/// Calculate days between two dates.
pub fn days_between(start: Date, end: Date) -> i64 {
    let span = end - start;
    span.get_days() as i64
}

/// Calculate days between two chrono dates.
pub fn days_between_chrono(start: NaiveDate, end: NaiveDate) -> i64 {
    (end - start).num_days()
}

/// Check if a date is in the past.
pub fn is_past(date: Date) -> bool {
    date < today()
}

/// Check if a date is in the future.
pub fn is_future(date: Date) -> bool {
    date > today()
}

/// Format a timestamp for RSS feeds (RFC 2822).
pub fn format_rfc2822(timestamp: &Zoned) -> String {
    timestamp.strftime("%a, %d %b %Y %H:%M:%S %z").to_string()
}

/// Format a timestamp for sitemaps (ISO 8601).
pub fn format_iso8601(timestamp: &Zoned) -> String {
    timestamp.strftime("%Y-%m-%dT%H:%M:%S%:z").to_string()
}

/// Parse an RFC 2822 date string.
pub fn parse_rfc2822(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc2822(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

/// Parse an RFC 3339 date string.
pub fn parse_rfc3339(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        let date = parse_date("2024-01-15").unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_parse_date_chrono() {
        let date = parse_date_chrono("2024-01-15").unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_format_date_long() {
        let date = parse_date("2024-01-15").unwrap();
        assert_eq!(format_date_long(date), "January 15, 2024");
    }

    #[test]
    fn test_jiff_chrono_conversion() {
        let jiff_date = parse_date("2024-06-20").unwrap();
        let chrono_date = jiff_to_chrono(jiff_date).unwrap();
        let back = chrono_to_jiff(chrono_date).unwrap();
        assert_eq!(jiff_date, back);
    }

    #[test]
    fn test_days_between() {
        let start = parse_date("2024-01-01").unwrap();
        let end = parse_date("2024-01-15").unwrap();
        assert_eq!(days_between(start, end), 14);
    }

    #[test]
    fn test_days_between_chrono() {
        let start = parse_date_chrono("2024-01-01").unwrap();
        let end = parse_date_chrono("2024-01-15").unwrap();
        assert_eq!(days_between_chrono(start, end), 14);
    }

    #[test]
    fn test_today() {
        let t = today();
        // Just verify it doesn't panic and returns a valid date.
        assert!(t.year() >= 2024);
    }

    #[test]
    fn test_format_rfc2822() {
        let now = now();
        let formatted = format_rfc2822(&now);
        // Should contain day of week abbreviation.
        assert!(formatted.len() > 20);
    }

    #[test]
    fn test_parse_rfc3339() {
        let dt = parse_rfc3339("2024-01-15T12:00:00Z").unwrap();
        assert_eq!(dt.year(), 2024);
    }
}
