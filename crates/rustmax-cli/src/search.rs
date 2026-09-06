//! Topic search.
//!
//! This is a port of `www/search-core.js`, which runs the same algorithm in
//! the browser. The two are kept at parity by `tests/search-corpus.json`:
//! the tests below check this implementation against it, and
//! `tests/search-parity.js` checks the JavaScript one. Change either
//! implementation and you must change the other.
//!
//! Parity is practical here because the algorithm is plain string matching
//! over ASCII data, and because both languages sort stably and use IEEE 754
//! doubles, so equal scores keep their input order in both.

use rmx::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use rmx::serde_json;
use std::path::Path;

/// How a query matched its target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchType {
    Exact,
    Prefix,
    Substring,
}

impl MatchType {
    /// The base score for this kind of match.
    fn score(self) -> f64 {
        match self {
            MatchType::Exact => 1.0,
            MatchType::Prefix => 0.9,
            MatchType::Substring => 0.6,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            MatchType::Exact => "exact",
            MatchType::Prefix => "prefix",
            MatchType::Substring => "substring",
        }
    }
}

/// An entry in the generated search index.
#[derive(Debug, Clone, Deserialize)]
pub struct IndexEntry {
    pub id: String,
    pub name: String,
    /// Name and aliases, pipe-separated.
    pub searchable: String,
    pub category: String,
    pub brief: String,
    #[serde(default)]
    pub path: Option<String>,
}

/// A ranked search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub entry: IndexEntry,
    pub score: f64,
    /// The alias that matched, when it was not the name.
    pub matched_text: Option<String>,
    pub match_type: MatchType,
}

/// Category weights for ranking.
///
/// These only break ties between equally good matches. They stay within a
/// ratio of 1.0/0.9, the smallest gap between two match types, so a weight
/// can never promote a worse match over a better one.
fn category_weight(category: &str) -> f64 {
    match category {
        "std" => 1.10,
        "crate" => 1.05,
        "book" => 1.00,
        _ => 1.0,
    }
}

/// Check how a query matches a target string.
///
/// Substring matches require a word boundary: the match must be at the
/// start of the target or preceded by a non-alphanumeric character. This
/// keeps "time" from matching inside "runtime".
fn get_match(query: &str, target: &str) -> Option<MatchType> {
    let q = query.to_lowercase();
    let t = target.to_lowercase();

    if t == q {
        return Some(MatchType::Exact);
    }
    if t.starts_with(&q) {
        return Some(MatchType::Prefix);
    }

    // An empty query is a prefix of everything, so it never reaches here.
    // Guarding anyway keeps the search below from spinning.
    if q.is_empty() {
        return None;
    }

    let mut pos = 0;
    while pos <= t.len() {
        let Some(offset) = t[pos..].find(&q) else {
            break;
        };
        let idx = pos + offset;

        let preceded_by_word_char = t[..idx]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());

        if !preceded_by_word_char {
            return Some(MatchType::Substring);
        }

        // Advance one character past the start of this match.
        pos = idx + t[idx..].chars().next().map_or(1, char::len_utf8);
    }

    None
}

/// Find the best match for a query against an entry.
///
/// The searchable field is `"name|alias one|alias two"`. Each alias is
/// matched on its own so that multi-word aliases keep their boundaries and
/// a match cannot straddle two of them.
fn find_match(query: &str, entry: &IndexEntry) -> Option<(f64, Option<String>, MatchType)> {
    let mut parts = entry.searchable.split('|');
    let name = parts.next().unwrap_or("");

    let mut best: Option<(f64, Option<String>, MatchType)> = None;

    // The name is displayed already, so a name match carries no alias text.
    if let Some(kind) = get_match(query, name) {
        best = Some((kind.score(), None, kind));
    }

    for alias in parts {
        if alias.is_empty() {
            continue;
        }
        if let Some(kind) = get_match(query, alias) {
            let score = kind.score();
            // Strictly greater, so the earliest of equally good aliases wins.
            if best.as_ref().is_none_or(|(best, _, _)| score > *best) {
                best = Some((score, Some(alias.to_string()), kind));
            }
        }
    }

    best
}

/// Search the index and return results ranked best first, capped at 20.
pub fn search(index: &[IndexEntry], query: &str) -> Vec<SearchResult> {
    if query.trim().is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for entry in index {
        let Some((score, matched_text, match_type)) = find_match(query, entry) else {
            continue;
        };

        if !seen.insert(entry.id.as_str()) {
            continue;
        }

        results.push(SearchResult {
            entry: entry.clone(),
            score: score * category_weight(&entry.category),
            matched_text,
            match_type,
        });
    }

    // A stable sort, so equal scores keep the index's own order.
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).expect("scores are finite"));
    results.truncate(20);

    results
}

/// Load a generated search index from a JSON file.
pub fn load_index(path: &Path) -> AnyResult<Vec<IndexEntry>> {
    let text = std::fs::read_to_string(path).context(path.display().to_string())?;
    serde_json::from_str(&text).context(path.display().to_string())
}

/// Format the alias a result matched on, for display.
pub fn format_match_info(matched_text: Option<&str>) -> Option<String> {
    matched_text.map(|text| format!("aka \"{}\"", text))
}


#[cfg(test)]
mod tests {
    use super::*;

    /// The shared parity corpus, also checked by `www/search-parity.js`.
    #[derive(Deserialize)]
    struct Corpus {
        index: Vec<IndexEntry>,
        cases: Vec<Case>,
    }

    #[derive(Deserialize)]
    struct Case {
        query: String,
        results: Vec<ExpectedResult>,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ExpectedResult {
        id: String,
        score: f64,
        match_type: MatchType,
        matched_text: Option<String>,
    }

    fn corpus() -> Corpus {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/search-corpus.json");
        let text = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&text).unwrap()
    }

    /// Scores are compared to this tolerance rather than bit-for-bit.
    ///
    /// Not because the two implementations disagree: they compute the same
    /// doubles. serde_json is built here without `float_roundtrip`, so its
    /// fast float parser can land one ULP off the literal in the corpus.
    /// Ordering is what is actually observable, and that is compared
    /// exactly through the sequence of ids.
    const SCORE_TOLERANCE: f64 = 1e-9;

    /// The corpus was generated from `www/search-core.js`, so passing it
    /// means the two implementations agree: same results, same order, same
    /// match types, same matched aliases, same scores.
    #[test]
    fn matches_the_javascript_implementation() {
        let corpus = corpus();

        for case in &corpus.cases {
            let got = search(&corpus.index, &case.query);

            // Compare everything discrete first: it gives a readable diff,
            // and a score difference big enough to matter shows up here as
            // a reordering anyway.
            let got_rows: Vec<(&str, MatchType, Option<&str>)> = got
                .iter()
                .map(|r| {
                    (
                        r.entry.id.as_str(),
                        r.match_type,
                        r.matched_text.as_deref(),
                    )
                })
                .collect();
            let want_rows: Vec<(&str, MatchType, Option<&str>)> = case
                .results
                .iter()
                .map(|r| (r.id.as_str(), r.match_type, r.matched_text.as_deref()))
                .collect();

            assert_eq!(
                got_rows, want_rows,
                "diverged from search-core.js on query {:?}",
                case.query
            );

            for (got, want) in got.iter().zip(&case.results) {
                assert!(
                    (got.score - want.score).abs() < SCORE_TOLERANCE,
                    "score for '{}' on query {:?}: {} vs {} from search-core.js",
                    want.id,
                    case.query,
                    got.score,
                    want.score,
                );
            }
        }
    }

    /// The other half of the parity check: the browser implementation
    /// against the same corpus.
    ///
    /// Skipped when node is absent, since node is a convenience for running
    /// the browser's code outside a browser and not a build requirement.
    /// Both CI runners ship it, so CI does enforce this.
    #[test]
    fn the_javascript_implementation_matches_the_corpus() {
        let script = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/search-parity.js");

        let output = match std::process::Command::new("node").arg(script).output() {
            Ok(output) => output,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("skipping: node not found, cannot check search-core.js");
                return;
            }
            Err(e) => panic!("failed to run node: {e}"),
        };

        assert!(
            output.status.success(),
            "search-core.js diverged from the parity corpus:\n{}",
            String::from_utf8_lossy(&output.stderr),
        );
    }

    /// Guards the assumption that makes parity tractable: both languages
    /// lowercase ASCII identically, but not necessarily anything else.
    #[test]
    fn the_corpus_index_is_ascii() {
        for entry in &corpus().index {
            assert!(
                entry.searchable.is_ascii(),
                "non-ascii searchable text in '{}' reopens the question of \
                 whether JS and Rust lowercase it the same way",
                entry.id
            );
        }
    }

    #[test]
    fn a_word_boundary_is_required_for_a_substring_match() {
        assert_eq!(get_match("time", "runtime"), None);
        assert_eq!(get_match("time", "run time"), Some(MatchType::Substring));
        assert_eq!(get_match("time", "run-time"), Some(MatchType::Substring));
        assert_eq!(get_match("time", "time"), Some(MatchType::Exact));
        assert_eq!(get_match("time", "timer"), Some(MatchType::Prefix));
    }

    /// The loop scanning for a word-boundary match must always advance.
    #[test]
    fn a_repeated_match_without_a_boundary_terminates() {
        assert_eq!(get_match("aa", "xaaaaaaaa"), None);
        assert_eq!(get_match("a", "ba"), None);
    }

    #[test]
    fn a_blank_query_finds_nothing() {
        let index = corpus().index;
        assert!(search(&index, "").is_empty());
        assert!(search(&index, "   ").is_empty());
    }
}

