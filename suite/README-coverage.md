# Dependency Coverage Collection

This directory contains a script to collect coverage for all rustmax dependencies.

## Usage

### Quick test (first 5 dependencies)
```bash
./collect-dep-coverage.sh --sample
```

### Full run (all ~63 dependencies)
```bash
./collect-dep-coverage.sh
```

## How it works

1. Extracts all dependency names from `crates/rustmax/Cargo.toml`
2. Runs suite tests once with instrumentation (`--no-report`)
3. Generates individual coverage reports for each dependency using `--dep-coverage`
4. Saves reports to `coverage-deps/` directory
5. Displays summary of line coverage percentages

## Output

- Individual reports: `coverage-deps/<dep-name>.txt`
- Summary printed to console showing line coverage % for each dependency

## Example

```
ahash:                         16.93%
anyhow:                        0.00%
axum:                          0.00%
backtrace:                     0.00%
base64:                        0.00%
```

Note: 0.00% means the suite tests don't exercise that dependency.
