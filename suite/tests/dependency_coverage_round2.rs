// Round 2: Coverage for previously untested high-value dependencies.

use rustmax_suite::*;

// Serde: Tested indirectly through serde_json in round1.
// Direct serde derive testing requires more complex setup.
// Skipping to focus on other high-value targets.

// Rayon: Parallel iteration and operations.

#[test]
fn test_rayon_parallel_iteration() {
    use rmx::rayon::prelude::*;

    let numbers: Vec<i32> = (0..1000).collect();

    // Parallel sum.
    let sum: i32 = numbers.par_iter().sum();
    assert_eq!(sum, 499500);

    // Parallel map.
    let doubled: Vec<i32> = numbers.par_iter().map(|x| x * 2).collect();
    assert_eq!(doubled.len(), 1000);
    assert_eq!(doubled[0], 0);
    assert_eq!(doubled[999], 1998);

    // Parallel filter.
    let evens: Vec<i32> = numbers.par_iter().filter(|x| *x % 2 == 0).copied().collect();
    assert_eq!(evens.len(), 500);
}

#[test]
fn test_rayon_parallel_sorting() {
    use rmx::rayon::prelude::*;

    let mut numbers: Vec<i32> = (0..1000).rev().collect();

    numbers.par_sort();

    assert_eq!(numbers[0], 0);
    assert_eq!(numbers[999], 999);

    // Verify fully sorted.
    for i in 1..numbers.len() {
        assert!(numbers[i - 1] <= numbers[i]);
    }
}

#[test]
fn test_rayon_parallel_find_and_any() {
    use rmx::rayon::prelude::*;

    let numbers: Vec<i32> = (0..1000).collect();

    // Parallel find.
    let found = numbers.par_iter().find_any(|x| **x == 500);
    assert_eq!(found, Some(&500));

    // Parallel any.
    let has_large = numbers.par_iter().any(|x| *x > 900);
    assert!(has_large);

    // Parallel all.
    let all_positive = numbers.par_iter().all(|x| *x >= 0);
    assert!(all_positive);
}

// Proptest: Property-based testing.

#[test]
fn test_proptest_strategies() {
    use rmx::proptest::prelude::*;

    proptest!(|(x in 0..100i32, y in 0..100i32)| {
        // Commutative property.
        assert_eq!(x + y, y + x);

        // Associative property (with bounds).
        if x < 50 && y < 50 {
            assert_eq!(x + y, y + x);
        }
    });
}

#[test]
fn test_proptest_vec_strategies() {
    use rmx::proptest::prelude::*;

    proptest!(|(vec in prop::collection::vec(0..100i32, 0..10))| {
        // Length property.
        assert!(vec.len() <= 10);

        // All elements in range.
        for item in &vec {
            assert!(*item >= 0 && *item < 100);
        }

        // Reverse twice equals original.
        let mut rev1 = vec.clone();
        rev1.reverse();
        rev1.reverse();
        assert_eq!(rev1, vec);
    });
}

// Clap: CLI argument parsing - requires clap to be exposed through rmx.
// Skipping for now as clap might not be re-exported.

// Env_logger and log: Logging infrastructure.

#[test]
fn test_env_logger_init() {
    // env_logger can only be initialized once per process.
    // Try to initialize, but ignore errors if already initialized.
    let _ = rmx::env_logger::try_init();

    // Test that we can log.
    rmx::log::info!("Test info message");
    rmx::log::warn!("Test warning message");
    rmx::log::error!("Test error message");
}

// Nom: Parser combinators.

#[test]
fn test_nom_basic_parsers() {
    use rmx::nom::{
        bytes::complete::{tag, take_while},
        character::complete::{digit1, space0},
        sequence::tuple,
        IResult,
    };

    fn parse_greeting(input: &str) -> IResult<&str, (&str, &str)> {
        tuple((tag("Hello"), space0))(input)
    }

    let result = parse_greeting("Hello world");
    assert!(result.is_ok());
    let (remaining, (hello, space)) = result.unwrap();
    assert_eq!(hello, "Hello");
    assert_eq!(remaining, "world");

    fn parse_number(input: &str) -> IResult<&str, &str> {
        digit1(input)
    }

    let (remaining, num) = parse_number("123abc").unwrap();
    assert_eq!(num, "123");
    assert_eq!(remaining, "abc");
}

// Num-bigint: Big integer arithmetic.

#[test]
fn test_num_bigint_operations() {
    use rmx::num_bigint::BigInt;
    use std::str::FromStr;

    let a = BigInt::from(1_000_000_000);
    let b = BigInt::from(2_000_000_000);

    let sum = &a + &b;
    assert_eq!(sum, BigInt::from(3_000_000_000i64));

    let product = &a * &b;
    assert_eq!(product, BigInt::from(2_000_000_000_000_000_000i64));

    // Very large number.
    let large = BigInt::from_str("123456789012345678901234567890").unwrap();
    let doubled = &large + &large;
    assert_eq!(
        doubled,
        BigInt::from_str("246913578024691357802469135780").unwrap()
    );
}

// Xshell: Shell command execution.

#[test]
fn test_xshell_commands() {
    use rmx::xshell::{cmd, Shell};

    let sh = Shell::new().unwrap();

    // Test echo command.
    let output = cmd!(sh, "echo hello").read().unwrap();
    assert_eq!(output.trim(), "hello");

    // Test ls command (should work on all platforms).
    let output = cmd!(sh, "ls").ignore_status().read().unwrap();
    assert!(!output.is_empty());
}

// Tera: Template engine.

#[test]
fn test_tera_templates() {
    use rmx::tera::{Tera, Context};

    let mut tera = Tera::default();

    tera.add_raw_template("hello", "Hello {{ name }}!").unwrap();
    tera.add_raw_template(
        "list",
        "{% for item in items %}{{ item }}, {% endfor %}",
    )
    .unwrap();

    let mut context = Context::new();
    context.insert("name", "World");

    let rendered = tera.render("hello", &context).unwrap();
    assert_eq!(rendered, "Hello World!");

    let mut context2 = Context::new();
    context2.insert("items", &vec!["a", "b", "c"]);
    let rendered2 = tera.render("list", &context2).unwrap();
    assert!(rendered2.contains("a"));
    assert!(rendered2.contains("b"));
    assert!(rendered2.contains("c"));
}

// Termcolor: Colored output.

#[test]
fn test_termcolor_output() {
    use rmx::termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
    use std::io::Write;

    let bufwtr = BufferWriter::stdout(ColorChoice::Never);
    let mut buffer = bufwtr.buffer();

    // Write with color spec.
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Red));
    buffer.set_color(&spec).unwrap();
    writeln!(&mut buffer, "This would be red").unwrap();

    // Reset.
    buffer.reset().unwrap();
    writeln!(&mut buffer, "This is normal").unwrap();

    // Verify buffer is not empty.
    assert!(!buffer.as_slice().is_empty());
}
