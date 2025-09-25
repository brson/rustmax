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

- **HTML report**: `out/coverage-html/index.html` - Interactive web-based coverage viewer
- **LCOV report**: `out/coverage.lcov` - Standard format for CI/CD integration
- **JSON report**: `out/coverage.json` - Machine-readable format for tooling
