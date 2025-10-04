// Round 5: Targeting shallow dependencies with high growth potential.
// Focus: itertools, chrono, nom, bitflags, rand.

#![allow(unused)]

use rustmax_suite::*;

// ============================================================================
// ITERTOOLS - Comprehensive coverage (current: 3.46%, target: 15%+)
// ============================================================================

#[test]
fn test_itertools_interleave() {
    use rmx::itertools::Itertools;

    let a = vec![1, 3, 5];
    let b = vec![2, 4, 6];

    let interleaved: Vec<_> = a.iter().interleave(b.iter()).copied().collect();
    assert_eq!(interleaved, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_itertools_intersperse() {
    use rmx::itertools::Itertools;

    let words = vec!["hello", "world", "rust"];
    let with_sep: Vec<_> = words.iter().copied().intersperse(" ").collect();
    assert_eq!(with_sep, vec!["hello", " ", "world", " ", "rust"]);
}

#[test]
fn test_itertools_cartesian_product() {
    use rmx::itertools::Itertools;

    let a = vec![1, 2];
    let b = vec!['a', 'b', 'c'];

    let product: Vec<_> = a.iter().cartesian_product(b.iter()).collect();
    assert_eq!(product.len(), 6);
    assert_eq!(product[0], (&1, &'a'));
    assert_eq!(product[5], (&2, &'c'));
}

#[test]
fn test_itertools_combinations() {
    use rmx::itertools::Itertools;

    let items = vec![1, 2, 3, 4];
    let combos: Vec<Vec<_>> = items.iter().combinations(2)
        .map(|c| c.into_iter().copied().collect())
        .collect();

    assert_eq!(combos.len(), 6);
    assert!(combos.contains(&vec![1, 2]));
    assert!(combos.contains(&vec![3, 4]));
}

#[test]
fn test_itertools_permutations() {
    use rmx::itertools::Itertools;

    let items = vec![1, 2, 3];
    let perms: Vec<Vec<_>> = items.iter().permutations(2)
        .map(|p| p.into_iter().copied().collect())
        .collect();

    assert_eq!(perms.len(), 6);
    assert!(perms.contains(&vec![1, 2]));
    assert!(perms.contains(&vec![2, 1]));
}

#[test]
fn test_itertools_multi_peek() {
    use rmx::itertools::Itertools;

    let data = vec![1, 2, 3, 4, 5];
    let mut iter = data.iter().multipeek();

    assert_eq!(iter.peek(), Some(&&1));
    assert_eq!(iter.peek(), Some(&&2));
    assert_eq!(iter.peek(), Some(&&3));

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
}

#[test]
fn test_itertools_merge() {
    use rmx::itertools::Itertools;

    let a = vec![1, 3, 5, 7];
    let b = vec![2, 4, 6, 8];

    let merged: Vec<_> = a.iter().merge(b.iter()).copied().collect();
    assert_eq!(merged, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_itertools_zip_longest() {
    use rmx::itertools::{Itertools, EitherOrBoth};

    let a = vec![1, 2, 3];
    let b = vec!['a', 'b'];

    let zipped: Vec<_> = a.iter().zip_longest(b.iter()).collect();
    assert_eq!(zipped.len(), 3);

    match zipped[0] {
        EitherOrBoth::Both(_, _) => {},
        _ => panic!("Expected Both"),
    }

    match zipped[2] {
        EitherOrBoth::Left(_) => {},
        _ => panic!("Expected Left"),
    }
}

#[test]
fn test_itertools_group_by() {
    use rmx::itertools::Itertools;

    let data = vec![1, 1, 2, 2, 2, 3, 3, 1];
    let groups: Vec<(i32, Vec<i32>)> = data.iter()
        .group_by(|&&x| x)
        .into_iter()
        .map(|(key, group)| (key, group.copied().collect()))
        .collect();

    assert_eq!(groups.len(), 4);
    assert_eq!(groups[0], (1, vec![1, 1]));
    assert_eq!(groups[1], (2, vec![2, 2, 2]));
}

#[test]
fn test_itertools_tuple_windows() {
    use rmx::itertools::Itertools;

    let data = vec![1, 2, 3, 4, 5];
    let windows: Vec<(&i32, &i32)> = data.iter().tuple_windows().collect();

    assert_eq!(windows.len(), 4);
    assert_eq!(windows[0], (&1, &2));
    assert_eq!(windows[3], (&4, &5));

    let triple_windows: Vec<(&i32, &i32, &i32)> = data.iter().tuple_windows().collect();
    assert_eq!(triple_windows.len(), 3);
    assert_eq!(triple_windows[0], (&1, &2, &3));
}

#[test]
fn test_itertools_minmax() {
    use rmx::itertools::{Itertools, MinMaxResult};

    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    match data.iter().minmax() {
        MinMaxResult::MinMax(min, max) => {
            assert_eq!(*min, 1);
            assert_eq!(*max, 9);
        }
        _ => panic!("Expected MinMax"),
    }

    let single = vec![42];
    match single.iter().minmax() {
        MinMaxResult::OneElement(val) => assert_eq!(*val, 42),
        _ => panic!("Expected OneElement"),
    }
}

#[test]
fn test_itertools_sorted() {
    use rmx::itertools::Itertools;

    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let sorted: Vec<_> = data.iter().sorted().copied().collect();
    assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);

    let rev_sorted: Vec<_> = data.iter().sorted_by(|a, b| b.cmp(a)).copied().collect();
    assert_eq!(rev_sorted, vec![9, 6, 5, 4, 3, 2, 1, 1]);
}

// ============================================================================
// CHRONO - Deep date/time operations (current: 4.22%, target: 12%+)
// ============================================================================

#[test]
fn test_chrono_naive_date_creation() {
    use rmx::chrono::{NaiveDate, Datelike};

    let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), 3);
    assert_eq!(date.day(), 15);

    let from_ordinal = NaiveDate::from_yo_opt(2024, 75).unwrap();
    assert_eq!(from_ordinal.month(), 3);
}

#[test]
fn test_chrono_naive_date_arithmetic() {
    use rmx::chrono::{NaiveDate, Days, Datelike};

    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let later = date.checked_add_days(Days::new(30)).unwrap();

    assert_eq!(later.month(), 1);
    assert_eq!(later.day(), 31);

    let earlier = later.checked_sub_days(Days::new(10)).unwrap();
    assert_eq!(earlier.day(), 21);
}

#[test]
fn test_chrono_naive_datetime() {
    use rmx::chrono::{NaiveDateTime, Datelike, Timelike};

    let dt = NaiveDateTime::parse_from_str("2024-03-15 14:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.hour(), 14);
    assert_eq!(dt.minute(), 30);

    let formatted = dt.format("%Y-%m-%d").to_string();
    assert_eq!(formatted, "2024-03-15");
}

#[test]
fn test_chrono_time_delta() {
    use rmx::chrono::TimeDelta;

    let delta = TimeDelta::hours(2) + TimeDelta::minutes(30);
    assert_eq!(delta.num_hours(), 2);
    assert_eq!(delta.num_minutes(), 150);
    assert_eq!(delta.num_seconds(), 9000);

    let days = TimeDelta::days(7);
    assert_eq!(days.num_weeks(), 1);
}

#[test]
fn test_chrono_weekday() {
    use rmx::chrono::{NaiveDate, Weekday, Datelike};

    let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    let weekday = date.weekday();

    assert!(matches!(weekday, Weekday::Mon | Weekday::Tue | Weekday::Wed |
                               Weekday::Thu | Weekday::Fri | Weekday::Sat | Weekday::Sun));

    let monday = date.week(Weekday::Mon).first_day();
    assert_eq!(monday.weekday(), Weekday::Mon);
}

#[test]
fn test_chrono_date_comparison() {
    use rmx::chrono::NaiveDate;

    let d1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let d2 = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

    assert!(d1 < d2);
    assert!(d2 > d1);

    let diff = d2.signed_duration_since(d1);
    assert!(diff.num_days() > 360);
}

#[test]
fn test_chrono_naive_time() {
    use rmx::chrono::{NaiveTime, Timelike};

    let time = NaiveTime::from_hms_opt(14, 30, 45).unwrap();
    assert_eq!(time.hour(), 14);
    assert_eq!(time.minute(), 30);
    assert_eq!(time.second(), 45);

    let from_secs = NaiveTime::from_num_seconds_from_midnight_opt(3600, 0).unwrap();
    assert_eq!(from_secs.hour(), 1);
}

#[test]
fn test_chrono_datetime_utc() {
    use rmx::chrono::{Utc, Duration, TimeZone, Datelike};

    let now = Utc::now();
    let later = now + Duration::hours(2);

    assert!(later > now);

    let fixed = Utc.with_ymd_and_hms(2024, 3, 15, 12, 0, 0).unwrap();
    assert_eq!(fixed.year(), 2024);
}

#[test]
fn test_chrono_timestamp() {
    use rmx::chrono::{DateTime, Utc, Datelike};

    let timestamp = 1710504000;
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap();

    assert_eq!(dt.year(), 2024);

    let ts = dt.timestamp();
    assert_eq!(ts, timestamp);
}

#[test]
fn test_chrono_date_iteration() {
    use rmx::chrono::{NaiveDate, Days, Datelike};

    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let mut dates = vec![];

    let mut current = start;
    for _ in 0..5 {
        dates.push(current);
        current = current.checked_add_days(Days::new(1)).unwrap();
    }

    assert_eq!(dates.len(), 5);
    assert_eq!(dates[4].day(), 5);
}

// ============================================================================
// NOM - Parser combinators (current: 1.07%, target: 5%+)
// ============================================================================
// Note: Simplified due to complex type annotation requirements in new nom API.

#[test]
fn test_nom_simple_parsers() {
    use rmx::nom::character::complete::{alpha1, alphanumeric1, digit1};
    use rmx::nom::bytes::complete::{tag, take, take_until};

    // Character parsers
    let (rest, word) = alpha1::<_, ()>("hello123").unwrap();
    assert_eq!(word, "hello");
    assert_eq!(rest, "123");

    let (_, token) = alphanumeric1::<_, ()>("abc123").unwrap();
    assert_eq!(token, "abc123");

    let (_, nums) = digit1::<_, ()>("456xyz").unwrap();
    assert_eq!(nums, "456");

    // Bytes parsers
    let (_, header) = take_until::<_, _, ()>(":")("Content-Type: json").unwrap();
    assert_eq!(header, "Content-Type");

    let (_, chunk) = take::<_, _, ()>(5usize)("hello world").unwrap();
    assert_eq!(chunk, "hello");

    let (_, matched) = tag::<_, _, ()>("test")("test123").unwrap();
    assert_eq!(matched, "test");
}

#[test]
fn test_nom_number_parsers() {
    use rmx::nom::number::complete::be_u32;

    let data = &[0, 0, 0, 42][..];
    let (_, num) = be_u32::<_, ()>(data).unwrap();
    assert_eq!(num, 42);

    let data2 = &[0, 0, 1, 0][..];
    let (_, num2) = be_u32::<_, ()>(data2).unwrap();
    assert_eq!(num2, 256);
}

// ============================================================================
// BITFLAGS - Comprehensive operations (current: 1.99%, target: 10%+)
// ============================================================================

#[test]
fn test_bitflags_iteration() {
    use rmx::bitflags::bitflags;

    bitflags! {
        #[derive(Clone, Copy, PartialEq, Eq)]
        struct Flags: u32 {
            const A = 0b0001;
            const B = 0b0010;
            const C = 0b0100;
            const D = 0b1000;
        }
    }

    let flags = Flags::A | Flags::C | Flags::D;

    let count = flags.iter().count();
    assert_eq!(count, 3);

    assert!(flags.contains(Flags::A));
    assert!(flags.contains(Flags::C));
    assert!(flags.contains(Flags::D));
    assert!(!flags.contains(Flags::B));
}

#[test]
fn test_bitflags_operations() {
    use rmx::bitflags::bitflags;

    bitflags! {
        #[derive(Clone, Copy, PartialEq, Eq)]
        struct Perms: u8 {
            const READ = 0b001;
            const WRITE = 0b010;
            const EXEC = 0b100;
        }
    }

    let rw = Perms::READ | Perms::WRITE;
    assert!(rw.contains(Perms::READ));
    assert!(rw.contains(Perms::WRITE));
    assert!(!rw.contains(Perms::EXEC));

    assert!(rw.intersects(Perms::READ));
    assert!(rw.intersects(Perms::WRITE));
    assert!(!rw.intersects(Perms::EXEC));

    let all = Perms::READ | Perms::WRITE | Perms::EXEC;
    let diff = all - rw;
    assert!(diff.contains(Perms::EXEC));
    assert!(!diff.contains(Perms::READ));
}

#[test]
fn test_bitflags_symmetric_difference() {
    use rmx::bitflags::bitflags;

    bitflags! {
        #[derive(Clone, Copy, PartialEq, Eq)]
        struct Features: u16 {
            const FEATURE_A = 0b0001;
            const FEATURE_B = 0b0010;
            const FEATURE_C = 0b0100;
            const FEATURE_D = 0b1000;
        }
    }

    let set1 = Features::FEATURE_A | Features::FEATURE_B;
    let set2 = Features::FEATURE_B | Features::FEATURE_C;

    let sym_diff = set1 ^ set2;
    assert!(sym_diff.contains(Features::FEATURE_A));
    assert!(!sym_diff.contains(Features::FEATURE_B));
    assert!(sym_diff.contains(Features::FEATURE_C));

    let intersection = set1 & set2;
    assert!(intersection.contains(Features::FEATURE_B));
    assert!(!intersection.contains(Features::FEATURE_A));
}

// ============================================================================
// RAND - Distribution sampling (current: 5.90%, target: 12%+)
// ============================================================================

#[test]
fn test_rand_distributions() {
    use rmx::rand::{Rng, thread_rng};

    let mut rng = thread_rng();

    // Test uniform sampling via gen_range (which uses Uniform internally)
    let sample: i32 = rng.random_range(0..100);
    assert!(sample < 100);

    let samples: Vec<i32> = (0..10).map(|_| rng.random_range(0..100)).collect();
    assert_eq!(samples.len(), 10);

    // Test standard distribution via gen
    let std_sample: f64 = rng.random();
    assert!(std_sample >= 0.0 && std_sample < 1.0);

    // Test bool generation
    let _bool_val: bool = rng.random();
}

#[test]
fn test_rand_range_operations() {
    use rmx::rand::{Rng, thread_rng};

    let mut rng = thread_rng();

    for _ in 0..10 {
        let n: u32 = rng.random_range(0..100);
        assert!(n < 100);
    }

    let floats: Vec<f64> = (0..5).map(|_| rng.random_range(0.0..1.0)).collect();
    assert_eq!(floats.len(), 5);
    assert!(floats.iter().all(|&x| x >= 0.0 && x < 1.0));
}
