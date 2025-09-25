# The Rustmax Test Suite

This is an integration test for the crates of Rustmax.

It is in the form of a single application
that uses as many Rustmax crates and features as it can,
and its test suite.

Development is coverage-driven,
and coverage is easy to generate, track and compare.


## Philosophy

The test suite is a CLI application with multiple subcommands,
each exercising different rustmax crates and features.
The actual application behavior is unimportant - what matters is that it:

- **Exercises real usage patterns** of rustmax crates (not just imports)
- **Uses the rustmax prelude** (`rmx::prelude::*`) as the primary API surface
- **Integrates multiple crates together** to test cross-crate compatibility
- **Has comprehensive integration tests** that actually invoke the binary to ensure code paths are executed
- **Maintains high coverage** (target: 80%+) as measured by `cargo-llvm-cov`

When adding new functionality:
1. Add a new subcommand that exercises a rustmax crate feature
2. Implement the functionality using realistic operations (not trivial calls)
3. Add integration tests that invoke the binary and verify output
4. Run coverage to ensure the new code paths are tested
5. Iterate until coverage is high for the new functionality

The suite should continuously expand to cover more rustmax crates
as they are added or as existing crates gain new features.


## Current Coverage

The suite currently implements **20 commands** exercising various rustmax crates:

### Core Functionality
- **greet** - Basic string operations and formatting
- **count** - Numeric operations and control flow
- **math** - Arithmetic operations and error handling
- **test** - Internal self-testing capabilities

### File & I/O Operations
- **file** - File I/O operations with tempfile integration
- **walk** - Directory traversal using walkdir

### Parsing & Serialization
- **parse** - CLI argument parsing with clap
- **serialize** - JSON/TOML serialization with serde
- **nom** - Parser combinators for structured text parsing
- **regex** - Pattern matching and text processing

### Cryptography & Hashing
- **crypto** - Cryptographic operations (BLAKE3, SHA2)

### Date & Time
- **time** - Date/time handling with chrono and jiff

### Networking & URLs
- **url** - URL parsing, manipulation, and validation

### Concurrency & Parallelism
- **async** - Async/await patterns with tokio and futures
- **parallel** - Parallel processing with rayon
- **crossbeam** - Advanced concurrency primitives

### Utilities & System
- **util** - Utility crates (itertools, bytes, bigint, semver, base64)
- **rand** - Random number generation and shuffling
- **xshell** - Cross-platform shell command execution
- **thiserror** - Custom error type definitions and handling

**Test Coverage**: 48 integration tests with 72%+ line coverage maintained through comprehensive binary invocation testing.


## Running the test suite

```
just test
```


## Generating a coverage report

```
just coverage
```


### Coverage reports

After running coverage generation, you'll have:

- **HTML report**: `target/llvm-cov/html/index.html` - Interactive web-based coverage viewer
- **JSON report**: `target/llvm-cov/cov.json` - Machine-readable format for tooling

The HTML report provides an interactive, line-by-line coverage analysis that helps identify untested code paths and areas for improvement.
