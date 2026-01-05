use rmx::prelude::*;
use std::fs;
use std::path::Path;
use crate::types::DocTest;

/// Generate the test crate in the work directory.
pub fn generate_test_crate(examples: &[DocTest], work_dir: &Path) -> AnyResult<()> {
    fs::create_dir_all(work_dir)?;
    fs::create_dir_all(work_dir.join("src"))?;

    generate_cargo_toml(work_dir)?;
    generate_lib_rs(examples, work_dir)?;

    Ok(())
}

/// Generate Cargo.toml for the test crate.
fn generate_cargo_toml(work_dir: &Path) -> AnyResult<()> {
    let cargo_toml = r#"[package]
name = "rustmax-doctest-generated"
edition = "2021"
version = "0.0.0"
publish = false

[workspace]

[dependencies]
rmx.path = "../../crates/rustmax"
rmx.package = "rustmax"
rmx.features = ["rmx-profile-max"]

# Direct dependencies for derive macros that look for crates at root
serde = { version = "1", features = ["derive"] }
thiserror = "2"
clap = { version = "4", features = ["derive"] }
derive_more = { version = "2", features = ["full"] }
num_enum = "0.7"
"#;

    fs::write(work_dir.join("Cargo.toml"), cargo_toml)?;
    Ok(())
}

/// Generate lib.rs with all test functions organized by crate module.
fn generate_lib_rs(examples: &[DocTest], work_dir: &Path) -> AnyResult<()> {
    use std::collections::BTreeMap;

    let mut output = String::new();

    output.push_str("#![allow(unused)]\n\n");

    // Group examples by crate name.
    let mut by_crate: BTreeMap<String, Vec<&DocTest>> = BTreeMap::new();
    for example in examples {
        let crate_name = extract_crate_name(&example.source_file);
        by_crate.entry(crate_name).or_default().push(example);
    }

    // Generate one module per crate.
    for (crate_name, crate_examples) in &by_crate {
        output.push_str(&format!("pub mod {} {{\n", crate_name));
        output.push_str("    #![allow(unused)]\n");
        output.push_str("    use rmx::prelude::*;\n\n");

        for example in crate_examples {
            let test_code = generate_test_function(example)?;
            // Indent the test function.
            for line in test_code.lines() {
                if line.is_empty() {
                    output.push('\n');
                } else {
                    output.push_str("    ");
                    output.push_str(line);
                    output.push('\n');
                }
            }
            output.push('\n');
        }

        output.push_str("}\n\n");
    }

    fs::write(work_dir.join("src/lib.rs"), output)?;
    Ok(())
}

/// Extract crate name from source file path.
///
/// E.g., "crate-anyhow.md" -> "anyhow"
fn extract_crate_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .strip_prefix("crate-")
        .unwrap_or("unknown")
        .replace('-', "_")
}

/// Generate a single test function from a DocTest.
fn generate_test_function(example: &DocTest) -> AnyResult<String> {
    let mut output = String::new();

    // Add doc comment with source information.
    output.push_str(&format!(
        "/// Test from {}\n/// Line {}\n",
        example.source_file.display(),
        example.line
    ));

    // Add test attribute.
    output.push_str("#[test]\n");

    // Check if the example uses patterns that don't work when wrapped in a function.
    // Note: serde, thiserror, clap, derive_more, num_enum derives work because
    // those crates are direct dependencies of the generated test crate.
    let has_problematic_patterns = example.code.contains("proptest!")
        || example.code.contains("crossbeam::select!")
        || example.code.contains("tokio::join!")
        || example.code.contains("#[tokio::main]")  // Tokio async examples are complex
        || example.code.contains("use crossbeam;")  // Simple crossbeam import
        || example.code.contains("use blake3;")     // Simple blake3 import
        || example.code.contains("use nom::");      // nom::bytes conflicts with bytes crate

    // Add ignore attribute if needed.
    if example.ignore || has_problematic_patterns {
        if has_problematic_patterns {
            output.push_str("#[ignore = \"complex patterns\"]\n");
        } else {
            output.push_str("#[ignore]\n");
        }
    } else if example.no_run {
        output.push_str("#[ignore = \"no_run\"]\n");
    }

    // Test function signature - use just example number since tests are inside crate modules.
    let test_name = example.name.rsplit('_').next().unwrap_or(&example.name);
    output.push_str(&format!("fn example_{}() {{\n", test_name));

    // For tests with problematic patterns, just output a stub.
    if has_problematic_patterns {
        output.push_str("    // This test uses patterns that don't work when wrapped.\n");
        output.push('}');
        return Ok(output);
    }

    // Filter rustdoc hidden lines (starting with #) and rewrite imports.
    let filtered_code = filter_rustdoc_hidden_lines(&example.code);
    let filtered_code = rewrite_imports(&filtered_code);

    // Determine if this is an async tokio test.
    let is_tokio_main = filtered_code.contains("#[tokio::main]");

    if is_tokio_main {
        // Extract async main body and wrap in Runtime::block_on.
        output.push_str(&generate_tokio_test(&filtered_code)?);
    } else if filtered_code.trim_start().starts_with("fn main()") {
        // Standalone main function - just call it.
        output.push_str(&indent(&filtered_code, 1));
        output.push_str("\n    main();\n");
    } else {
        // Regular code - wrap in inner function for error handling.
        output.push_str("    fn run_test() -> AnyResult<()> {\n");
        output.push_str(&indent(&filtered_code, 2));
        output.push_str("\n        Ok(())\n");
        output.push_str("    }\n\n");
        output.push_str("    run_test().unwrap();\n");
    }

    output.push('}');

    Ok(output)
}

/// Filter out rustdoc hidden lines (lines starting with `# `).
fn filter_rustdoc_hidden_lines(code: &str) -> String {
    code.lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !trimmed.starts_with("# ") && trimmed != "#"
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Rewrite crate imports to use rmx:: prefix.
fn rewrite_imports(code: &str) -> String {
    // List of crates that should be prefixed with rmx::
    // Note: we don't include "bytes" because it conflicts with nom::bytes
    let crates_to_rewrite = [
        "serde", "serde_json", "tokio", "anyhow", "regex", "ahash",
        "itertools", "rayon", "zip", "indicatif", "notify", "image", "reqwest",
        "base64", "chrono", "uuid", "url", "log", "tracing", "thiserror",
        "clap", "json5", "toml", "tempfile", "xshell", "walkdir", "glob",
        "nom", "memchr", "comrak", "ignore", "bstr", "once_cell", "lazy_static",
        "ctrlc", "rustyline", "jiff", "hex", "http", "hyper", "axum",
        "futures", "tower", "tungstenite", "rav1e", "ravif",
        "unicode_segmentation", "num_bigint", "semver", "mime", "crossbeam",
        "blake3", "sha2", "powerletters", "flate2", "env_logger",
        "derive_more", "num_enum", "rand", "bytes", "proptest",
    ];

    let mut result = code.to_string();

    for crate_name in crates_to_rewrite {
        // Rewrite `use crate_name::` to `use rmx::crate_name::`
        let pattern = format!("use {}::", crate_name);
        let replacement = format!("use rmx::{}::", crate_name);
        result = result.replace(&pattern, &replacement);

        // Also rewrite inline usage like `crate_name::func()` to `rmx::crate_name::func()`
        // Match patterns like ` crate_name::` or `(crate_name::` or `{crate_name::` or `<crate_name::`
        for prefix in [" ", "(", "{", "\n", "=", "<", ","] {
            let inline_pattern = format!("{}{}::", prefix, crate_name);
            let inline_replacement = format!("{}rmx::{}::", prefix, crate_name);
            result = result.replace(&inline_pattern, &inline_replacement);
        }
    }

    result
}

/// Generate code for a tokio async test.
fn generate_tokio_test(code: &str) -> AnyResult<String> {
    let mut output = String::new();

    // Find async fn main body.
    let async_main_body = extract_async_main_body(code);

    output.push_str("    tokio::runtime::Runtime::new()\n");
    output.push_str("        .unwrap()\n");
    output.push_str("        .block_on(async {\n");
    output.push_str(&indent(&async_main_body, 3));
    output.push_str("\n        });\n");

    Ok(output)
}

/// Extract the body of an async main function.
fn extract_async_main_body(code: &str) -> String {
    // Simple extraction: look for the function body between braces.
    // This is a heuristic - may not work for all cases.
    if let Some(start) = code.find("async fn main") {
        if let Some(brace_start) = code[start..].find('{') {
            let body_start = start + brace_start + 1;
            if let Some(brace_end) = find_matching_brace(&code[body_start..]) {
                return code[body_start..body_start + brace_end].trim().to_string();
            }
        }
    }

    // Fallback: return code as-is.
    code.to_string()
}

/// Find the matching closing brace.
fn find_matching_brace(s: &str) -> Option<usize> {
    let mut depth = 1;
    for (i, ch) in s.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Indent code by a number of levels (4 spaces per level).
fn indent(code: &str, levels: usize) -> String {
    let indent_str = "    ".repeat(levels);
    code.lines()
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                format!("{}{}", indent_str, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
