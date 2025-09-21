use rmx::prelude::*;
use std::env;

fn main() -> AnyResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "greet" => {
                let default_name = "World".to_string();
                let name = args.get(2).unwrap_or(&default_name);
                println!("Hello, {}!", greet_user(name));
            }
            "count" => {
                let default_num = "10".to_string();
                let num_str = args.get(2).unwrap_or(&default_num);
                let num: i32 = num_str.parse().unwrap_or(10);
                println!("Counting to {}: {}", num, count_to(num));
            }
            "math" => {
                let default_a = "5".to_string();
                let default_b = "3".to_string();
                let a_str = args.get(2).unwrap_or(&default_a);
                let b_str = args.get(3).unwrap_or(&default_b);
                let a: i32 = a_str.parse().unwrap_or(5);
                let b: i32 = b_str.parse().unwrap_or(3);
                println!("Math operations on {} and {}:", a, b);
                println!("  Add: {}", add_numbers(a, b));
                println!("  Multiply: {}", multiply_numbers(a, b));
                if b != 0 {
                    println!("  Divide: {}", divide_numbers(a, b));
                } else {
                    println!("  Divide: Cannot divide by zero");
                }
            }
            "test" => {
                println!("Running internal tests...");
                run_internal_tests();
            }
            _ => {
                println!("Unknown command. Available commands: greet, count, math, test");
            }
        }
    } else {
        println!("Rustmax Suite - Integration test application");
        println!("Usage: {} <command> [args...]", args[0]);
        println!("Commands:");
        println!("  greet [name]     - Greet someone");
        println!("  count [num]      - Count to a number");
        println!("  math [a] [b]     - Perform math operations");
        println!("  test             - Run internal tests");
    }

    Ok(())
}

fn greet_user(name: &str) -> String {
    if name.is_empty() {
        "Anonymous".to_string()
    } else if name.len() > 20 {
        format!("{}...", &name[..17])
    } else {
        name.to_string()
    }
}

fn count_to(num: i32) -> String {
    if num <= 0 {
        "Nothing to count".to_string()
    } else if num > 100 {
        "Too big to count".to_string()
    } else {
        (1..=num).map(|i| i.to_string()).collect::<Vec<_>>().join(", ")
    }
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply_numbers(a: i32, b: i32) -> i32 {
    a * b
}

fn divide_numbers(a: i32, b: i32) -> f64 {
    a as f64 / b as f64
}

fn run_internal_tests() {
    assert_eq!(greet_user("Alice"), "Alice");
    assert_eq!(greet_user(""), "Anonymous");
    assert_eq!(greet_user("VeryLongNameThatExceedsTwentyCharacters"), "VeryLongNameThatE...");

    assert_eq!(count_to(3), "1, 2, 3");
    assert_eq!(count_to(0), "Nothing to count");
    assert_eq!(count_to(101), "Too big to count");

    assert_eq!(add_numbers(2, 3), 5);
    assert_eq!(multiply_numbers(4, 5), 20);
    assert_eq!(divide_numbers(10, 2), 5.0);

    println!("All internal tests passed!");
}

fn dead_code() {
    eprintln!("dead code");
}
