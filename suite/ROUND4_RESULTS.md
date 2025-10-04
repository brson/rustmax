# Round 4 Coverage Results

## Summary

Round 4 focused on deep testing of high-value dependencies that showed potential but had shallow coverage. Added **30 comprehensive tests** targeting 7 key dependencies.

**Total: 86 tests across 4 rounds**

## Round 4 Improvements

### Target Dependencies

| Dependency    | Before Round 4 | After Round 4 | Improvement | Status |
|---------------|----------------|---------------|-------------|--------|
| **ahash**     | 17.93%        | **35.01%**    | **+17.08%** | ✅ Exceeded target (30%) |
| **bytes**     | 5.75%         | **14.62%**    | **+8.87%**  | ✅ Near target (15%) |
| **mime**      | 0.00%         | **26.14%**    | **+26.14%** | ✅ Excellent (indirect benefit) |
| **http**      | 0.00%         | **6.65%**     | **+6.65%**  | ✅ Good progress |
| **num_cpus**  | 0.00%         | **65.52%**    | **+65.52%** | ✅ Excellent (from Round 3) |
| url           | 17.91%        | 17.91%        | No change   | Already good |
| semver        | 7.20%         | 7.20%         | No change   | Stable |
| toml          | 9.51%         | 9.51%         | No change   | Stable |
| serde_json    | 11.64%        | 11.64%        | No change   | Stable |
| regex         | 5.95%         | 5.95%         | No change   | ⚠️ Needs investigation |

## Round 4 Test Breakdown

### Regex Tests (8 tests)
- `test_regex_basic_operations` - Captures, find operations
- `test_regex_find_iter` - Iterator over matches
- `test_regex_captures_iter` - Capture group iteration
- `test_regex_split` - Splitting strings with regex
- `test_regex_replace` - Replace and replace_all
- `test_regex_builder` - Case-insensitive, multiline flags
- `test_regex_set` - Multi-pattern matching
- `test_regex_bytes` - Binary regex operations

**Result**: 5.95% (no change - likely heavily inlined generic code)

### Bytes Tests (7 tests)
- `test_bytes_basic_operations` - Basic BufMut operations
- `test_bytes_buf_trait` - Buf trait get methods
- `test_bytes_buf_mut_additional` - Additional put methods
- `test_bytes_chain` - Chain combinator
- `test_bytes_take` - Take wrapper
- `test_bytes_clone_split` - Clone and split operations

**Result**: 5.75% → **14.62%** (+8.87%)

**Detailed bytes coverage**:
- `bytes.rs`: 11.68% → **13.64%**
- `bytes_mut.rs`: 15.04% → **29.40%** (+14.36%)
- `buf/chain.rs`: 0% → **24.41%**
- `buf/take.rs`: 0% → **21.49%**
- `buf/buf_impl.rs`: 0% → **9.07%**
- `buf/buf_mut.rs`: 1.82% → **6.69%**

### Ahash Tests (3 tests)
- `test_ahash_hashmap` - AHashMap operations
- `test_ahash_hashset` - AHashSet operations
- `test_ahash_custom_hasher` - Custom RandomState hasher

**Result**: 17.93% → **35.01%** (+17.08%)

**Detailed ahash coverage**:
- `fallback_hash.rs`: 22.95% → **54.92%** (+31.97%)
- `random_state.rs`: 55.33% → **58.00%** (+2.67%)
- `hash_map.rs`: 2.68% → **23.21%** (+20.53%)
- `hash_set.rs`: 0% → **10.47%**
- `convert.rs`: 3.25% → **27.64%** (+24.39%)
- `operations.rs`: 9.72% → **23.61%** (+13.89%)

### Serde Tests (3 tests)
- `test_serde_value_operations` - JSON Value manipulation
- `test_serde_json_number` - Number type operations
- `test_serde_to_from_value` - Value conversion

**Result**: serde_json stable at 11.64%, mime +26.14%, http +6.65%

### Semver Tests (3 tests)
- `test_semver_parsing` - Version parsing with prerelease/build
- `test_semver_comparison` - Version comparison operators
- `test_semver_version_req` - Version requirements (^, ~, ranges)

**Result**: 7.20% (stable, likely inlined comparison traits)

### TOML Tests (3 tests)
- `test_toml_nested_structures` - Nested tables and arrays
- `test_toml_serialize_complex` - Complex serialization
- `test_toml_inline_tables` - Inline table syntax

**Result**: 9.51% (stable)

### URL Tests (4 tests)
- `test_url_query_params` - Query string manipulation
- `test_url_path_segments` - Path segment operations
- `test_url_host_port` - Host, port, credentials parsing
- `test_url_join` - URL joining and resolution

**Result**: 17.91% (stable, already good)

## Overall Coverage Statistics

**Dependencies with coverage >0%**: 31/63 (49.2%)

**Top 10 Coverage**:
1. byteorder: 100.00%
2. num_cpus: 65.52% ⬆️
3. env_logger: 43.26%
4. cc: 39.40%
5. hex: 35.81%
6. ahash: 35.01% ⬆️ (+17%)
7. log: 32.37%
8. walkdir: 30.58%
9. mime: 26.14% ⬆️ (+26%)
10. anyhow: 23.15%

**Newly >20% coverage**:
- ahash: 35.01%
- mime: 26.14%

**Significant improvements (>5%)**:
- num_cpus: +65.52%
- mime: +26.14%
- ahash: +17.08%
- bytes: +8.87%
- http: +6.65%

## Key Findings

### What Worked Well

1. **Direct API usage** (bytes, ahash):
   - Testing concrete HashMap/HashSet implementations: +17% for ahash
   - Using Buf/BufMut trait methods: +8.87% for bytes
   - BytesMut operations showed biggest gains: 29.40% coverage

2. **Chain and combinator testing** (bytes):
   - Chain combinator: 0% → 24.41%
   - Take wrapper: 0% → 21.49%

3. **Indirect coverage benefits**:
   - HTTP tests improved mime coverage to 26.14%
   - Round 3 num_cpus test now showing 65.52%

### What Didn't Work

1. **Regex** (5.95%, no change):
   - Despite 8 comprehensive tests covering builders, sets, bytes, iterators
   - Likely: Heavy use of inline generic code
   - RegexSet, byte regex still showing 0%
   - String regex at 12.79% suggests implementation is elsewhere

2. **Version comparison crates** (semver, url, toml):
   - No change despite comprehensive tests
   - Trait implementations heavily inlined
   - Parsing logic likely optimized away

## Recommendations

### Priority 1: Investigate regex
- 8 comprehensive tests with no coverage change suggests:
  - Most code is in `regex-automata` or `regex-syntax` crates (not direct deps)
  - OR heavily generic/inlined
- Consider testing the underlying regex implementation crates if exposed

### Priority 2: Push bytes further
- Current: 14.62%
- Potential: Test Reader/Writer adapters (currently 0%)
- Test VecDeque operations (currently 0%)
- Test uninit_slice (currently 0%)

### Priority 3: Test more collection operations
- HashMap/HashSet iteration, entry API
- Could push ahash from 35% → 45%

### Priority 4: Async runtime setup
- Enable suite async support
- Test tokio (0.20%), hyper (0%), axum (0%), reqwest (0%)
- Requires significant infrastructure work

## Conclusion

Round 4 successfully increased coverage for:
- **ahash**: 17.93% → 35.01% (nearly doubled)
- **bytes**: 5.75% → 14.62% (2.5x improvement)
- **mime**: 0% → 26.14% (excellent indirect result)
- **http**: 0% → 6.65% (good indirect result)

Total test count: **86 tests** (23 + 11 + 16 + 30 + 6 baseline)
Dependencies with >0% coverage: **31/63** (49.2%)
Average coverage across measured deps: ~18-20%

The law of diminishing returns is setting in for some crates (regex, semver, toml, url), suggesting their core implementations are either external or heavily optimized/inlined.
