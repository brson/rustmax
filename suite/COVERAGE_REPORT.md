# Rustmax Dependencies Coverage Report

**Generated:** 2025-10-03
**Test Count:** 34 tests (23 round1 + 11 round2)
**Dependencies Tested:** 63

## Executive Summary

- **28/63 dependencies (44%)** now have coverage > 0%
- **9 dependencies** have >20% coverage
- **24 dependencies** have 5-20% coverage
- **35 dependencies** still at 0% (mostly async/web stack and proc-macro tools)

## Top Performers (>20% Coverage)

| Dependency | Coverage | Notes |
|------------|----------|-------|
| byteorder | 100.00% | ✅ Fully covered by our tests |
| env_logger | 43.26% | Logger initialization |
| cc | 39.40% | Build tool (C compilation) |
| hex | 35.81% | Hex encoding/decoding |
| log | 32.37% | Logging macros |
| walkdir | 30.58% | Directory walking |
| anyhow | 23.15% | Error handling |
| base64 | 22.00% | Base64 encoding/decoding |
| unicode-segmentation | 22.12% | Unicode string operations |

## Strong Coverage (10-20%)

| Dependency | Coverage |
|------------|----------|
| url | 17.91% |
| ahash | 16.93% |
| sha2 | 14.51% |
| tempfile | 13.90% |
| serde_json | 11.64% |
| proptest | 10.33% |

## Moderate Coverage (5-10%)

| Dependency | Coverage |
|------------|----------|
| toml | 9.51% |
| blake3 | 8.80% |
| jiff | 7.98% |
| semver | 7.20% |
| regex | 5.95% |
| rand | 5.90% |
| bytes | 5.75% |
| num-bigint | 5.54% |

## Low Coverage (1-5%)

| Dependency | Coverage | Notes |
|------------|----------|-------|
| chrono | 4.22% | Date/time library |
| itertools | 3.46% | Iterator extensions |
| bitflags | 1.99% | Bit flag macros |
| nom | 1.07% | Parser combinators |
| tokio | 0.20% | Async runtime (minimal) |

## Zero Coverage (35 dependencies)

**Async/Web Stack:**
- axum, hyper, reqwest, tower, socket2, http, mime

**Build/Proc-Macro Tools:**
- bindgen, cxx, cxx-build, proc-macro2, quote, syn

**CLI Tools:**
- clap, rustyline, termcolor

**Concurrency:**
- rayon (tested but not showing up?)

**Data/Serialization:**
- serde (trait library, tested indirectly)
- json5

**System:**
- libc, ctrlc

**Other:**
- tera (template engine - tested but 0%?)
- xshell (shell commands - tested but 0%?)
- num_cpus, num_enum, derive_more, static_assertions, extension-trait, cfg-if, futures, rand_pcg, thiserror

## Round 2 Impact Analysis

**Successfully Added Coverage:**
- proptest: 0% → 10.33%
- num-bigint: 0% → 5.54%
- nom: 0% → 1.07%

**Unexpectedly Still at 0%:**
- rayon: Parallel tests run successfully but showing 0% coverage
- tera: Template tests run successfully but showing 0% coverage
- xshell: Shell command tests run successfully but showing 0% coverage
- termcolor: Color tests run successfully but showing 0% coverage

**Possible Causes for 0% Despite Tests:**
1. Dependencies might be feature-gated and not enabled in rustmax-suite build
2. Code might be inlined/optimized away
3. Coverage tooling might not instrument certain types of code
4. Tests might be calling wrapper functions instead of the dep directly

## Recommendations

### Priority 1: Fix Round 2 Zero Coverage Issues
Investigate why rayon, tera, xshell, termcolor show 0% despite having tests:
- Check if dependencies are properly enabled in suite's feature set
- Verify tests actually call the dep code (not just our wrappers)
- Try more direct usage of dep APIs

### Priority 2: Async/Web Stack
Requires async runtime setup:
- tokio: Basic spawn, sleep, channels
- reqwest: HTTP client requests
- hyper: Low-level HTTP
- axum/tower: Web framework

### Priority 3: CLI Tools
- clap: Need to enable derive feature
- rustyline: Interactive readline
- termcolor: Need to verify feature flags

### Priority 4: Proc-Macro/Build Tools
Lower priority as these are compile-time tools:
- proc-macro2, quote, syn
- bindgen, cxx, cxx-build

## Test Files

- `tests/dependency_coverage.rs` - 23 tests (round 1)
- `tests/dependency_coverage_round2.rs` - 11 tests (round 2)

## Coverage Collection

Run: `./suite/collect-dep-coverage.sh`
Sample (5 deps): `./suite/collect-dep-coverage.sh --sample`
Results: `suite/coverage-deps/*.txt`
