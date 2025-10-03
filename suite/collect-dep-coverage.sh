#!/usr/bin/env bash
set -euo pipefail

# Collect coverage for all rustmax dependencies via the suite

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUSTMAX_TOML="$SCRIPT_DIR/../crates/rustmax/Cargo.toml"
COVERAGE_DIR="$SCRIPT_DIR/coverage-deps"

# Extract dependency names from rustmax Cargo.toml.
# Look for lines like: ahash = { version = "...", ... }
# Extract just the crate name.
extract_deps() {
    rg '^(\w+(-\w+)*) = \{' "$RUSTMAX_TOML" \
        --only-matching \
        --replace '$1' \
        | rg -v '^(_gcc|_thread_local)' # Skip indirect deps with _ prefix
}

main() {
    echo "Extracting dependencies from $RUSTMAX_TOML..."
    mapfile -t all_deps < <(extract_deps)

    # If --sample flag is passed, only process first 5 deps.
    if [[ "${1:-}" == "--sample" ]]; then
        deps=("${all_deps[@]:0:5}")
        echo "SAMPLE MODE: Testing with first 5 dependencies"
    else
        deps=("${all_deps[@]}")
    fi

    echo "Found ${#deps[@]} dependencies to process: ${deps[*]}"

    echo ""
    echo "Cleaning previous coverage data..."
    cargo llvm-cov clean -p rustmax-suite

    echo ""
    echo "Running tests to collect profdata (this only needs to run once)..."
    cargo llvm-cov -p rustmax-suite --no-report --ignore-run-fail

    echo ""
    echo "Creating coverage reports directory..."
    mkdir -p "$COVERAGE_DIR"

    echo ""
    echo "Generating coverage reports for each dependency..."
    for dep in "${deps[@]}"; do
        echo "  - $dep"
        cargo llvm-cov report --dep-coverage "$dep" --summary-only \
            > "$COVERAGE_DIR/$dep.txt" 2>&1 || echo "    (failed, skipping)"
    done

    echo ""
    echo "Coverage reports saved to: $COVERAGE_DIR/"
    echo ""
    echo "Summary of coverage by dependency (line coverage):"
    for dep in "${deps[@]}"; do
        if [ -f "$COVERAGE_DIR/$dep.txt" ]; then
            # Extract line coverage percentage from TOTAL row (column 10).
            coverage=$(rg '^TOTAL' "$COVERAGE_DIR/$dep.txt" | awk '{print $10}')
            if [ -z "$coverage" ]; then
                coverage="N/A"
            fi
            printf "  %-30s %s\n" "$dep:" "$coverage"
        fi
    done
}

main "$@"
