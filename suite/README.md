# The Rustmax Test Suite

This is an integration test for the crates of Rustmax.

It is in the form of a single application
that uses as many Rustmax crates and features as it can,
and its test suite.

Development is coverage-driven,
and coverage is easy to generate, track and compare.


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
