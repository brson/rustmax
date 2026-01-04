//! Full-text search indexing with stemming and stop words.

use rustmax::prelude::*;
use serde::{Deserialize, Serialize};
use rustmax::unicode_segmentation::UnicodeSegmentation;
use rustmax::log::info;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::collection::Collection;
use crate::Result;

/// Common English stop words to exclude from indexing.
const STOP_WORDS: &[&str] = &[
    "a", "an", "and", "are", "as", "at", "be", "by", "for", "from",
    "has", "he", "in", "is", "it", "its", "of", "on", "that", "the",
    "to", "was", "were", "will", "with", "the", "this", "but", "they",
    "have", "had", "what", "when", "where", "who", "which", "why", "how",
    "all", "each", "every", "both", "few", "more", "most", "other", "some",
    "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too",
    "very", "just", "can", "should", "now", "also", "into", "could", "would",
    "there", "their", "then", "these", "those", "been", "being", "about",
    "after", "before", "between", "under", "over", "again", "further",
    "once", "here", "there", "any", "because", "does", "doing", "during",
    "until", "while", "out", "up", "down", "off", "above", "below",
];

/// Porter stemmer suffix rules.
struct PorterStemmer;

impl PorterStemmer {
    /// Stem a word using simplified Porter stemmer rules.
    fn stem(word: &str) -> String {
        let mut s = word.to_lowercase();

        // Step 1a: plurals.
        if s.ends_with("sses") {
            s.truncate(s.len() - 2);
        } else if s.ends_with("ies") {
            s.truncate(s.len() - 2);
        } else if !s.ends_with("ss") && s.ends_with('s') {
            s.pop();
        }

        // Step 1b: -ed, -ing.
        if s.ends_with("eed") {
            if Self::measure(&s[..s.len() - 3]) > 0 {
                s.truncate(s.len() - 1);
            }
        } else if s.ends_with("ed") && Self::has_vowel(&s[..s.len() - 2]) {
            s.truncate(s.len() - 2);
            s = Self::step1b_fixup(s);
        } else if s.ends_with("ing") && Self::has_vowel(&s[..s.len() - 3]) {
            s.truncate(s.len() - 3);
            s = Self::step1b_fixup(s);
        }

        // Step 1c: y -> i.
        if s.ends_with('y') && Self::has_vowel(&s[..s.len() - 1]) {
            s.pop();
            s.push('i');
        }

        // Step 2: common suffixes.
        s = Self::step2(s);

        // Step 3: more suffixes.
        s = Self::step3(s);

        // Step 4: even more suffixes.
        s = Self::step4(s);

        // Step 5a: remove trailing e.
        if s.ends_with('e') {
            let m = Self::measure(&s[..s.len() - 1]);
            if m > 1 || (m == 1 && !Self::cvc(&s[..s.len() - 1])) {
                s.pop();
            }
        }

        // Step 5b: double consonant + l.
        if s.ends_with("ll") && Self::measure(&s) > 1 {
            s.pop();
        }

        s
    }

    /// Count consonant sequences (measure).
    fn measure(s: &str) -> usize {
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        let mut i = 0;

        // Skip initial consonants.
        while i < chars.len() && !Self::is_vowel_at(&chars, i) {
            i += 1;
        }

        while i < chars.len() {
            // Skip vowels.
            while i < chars.len() && Self::is_vowel_at(&chars, i) {
                i += 1;
            }
            if i >= chars.len() {
                break;
            }
            count += 1;
            // Skip consonants.
            while i < chars.len() && !Self::is_vowel_at(&chars, i) {
                i += 1;
            }
        }

        count
    }

    fn is_vowel_at(chars: &[char], i: usize) -> bool {
        match chars.get(i) {
            Some('a' | 'e' | 'i' | 'o' | 'u') => true,
            Some('y') => i > 0 && !Self::is_vowel_at(chars, i - 1),
            _ => false,
        }
    }

    fn has_vowel(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        (0..chars.len()).any(|i| Self::is_vowel_at(&chars, i))
    }

    /// Check consonant-vowel-consonant pattern at end.
    fn cvc(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        if len < 3 {
            return false;
        }
        let c1 = !Self::is_vowel_at(&chars, len - 3);
        let v = Self::is_vowel_at(&chars, len - 2);
        let c2 = !Self::is_vowel_at(&chars, len - 1);
        let last = chars[len - 1];
        c1 && v && c2 && last != 'w' && last != 'x' && last != 'y'
    }

    fn step1b_fixup(mut s: String) -> String {
        if s.ends_with("at") || s.ends_with("bl") || s.ends_with("iz") {
            s.push('e');
        } else if Self::double_consonant(&s) {
            let last = s.chars().last().unwrap();
            if last != 'l' && last != 's' && last != 'z' {
                s.pop();
            }
        } else if Self::measure(&s) == 1 && Self::cvc(&s) {
            s.push('e');
        }
        s
    }

    fn double_consonant(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 2 {
            return false;
        }
        let last = chars[chars.len() - 1];
        let prev = chars[chars.len() - 2];
        last == prev && !Self::is_vowel_at(&chars, chars.len() - 1)
    }

    fn step2(mut s: String) -> String {
        let replacements = [
            ("ational", "ate"), ("tional", "tion"), ("enci", "ence"),
            ("anci", "ance"), ("izer", "ize"), ("abli", "able"),
            ("alli", "al"), ("entli", "ent"), ("eli", "e"),
            ("ousli", "ous"), ("ization", "ize"), ("ation", "ate"),
            ("ator", "ate"), ("alism", "al"), ("iveness", "ive"),
            ("fulness", "ful"), ("ousness", "ous"), ("aliti", "al"),
            ("iviti", "ive"), ("biliti", "ble"),
        ];

        for (suffix, replacement) in replacements {
            if s.ends_with(suffix) {
                let stem = &s[..s.len() - suffix.len()];
                if Self::measure(stem) > 0 {
                    s.truncate(s.len() - suffix.len());
                    s.push_str(replacement);
                    break;
                }
            }
        }
        s
    }

    fn step3(mut s: String) -> String {
        let replacements = [
            ("icate", "ic"), ("ative", ""), ("alize", "al"),
            ("iciti", "ic"), ("ical", "ic"), ("ful", ""), ("ness", ""),
        ];

        for (suffix, replacement) in replacements {
            if s.ends_with(suffix) {
                let stem = &s[..s.len() - suffix.len()];
                if Self::measure(stem) > 0 {
                    s.truncate(s.len() - suffix.len());
                    s.push_str(replacement);
                    break;
                }
            }
        }
        s
    }

    fn step4(mut s: String) -> String {
        let suffixes = [
            "al", "ance", "ence", "er", "ic", "able", "ible", "ant",
            "ement", "ment", "ent", "ion", "ou", "ism", "ate", "iti",
            "ous", "ive", "ize",
        ];

        for suffix in suffixes {
            if s.ends_with(suffix) {
                let stem = &s[..s.len() - suffix.len()];
                if Self::measure(stem) > 1 {
                    // Special case for -ion.
                    if suffix == "ion" {
                        let chars: Vec<char> = stem.chars().collect();
                        if let Some(&last) = chars.last() {
                            if last == 's' || last == 't' {
                                s.truncate(s.len() - suffix.len());
                            }
                        }
                    } else {
                        s.truncate(s.len() - suffix.len());
                    }
                    break;
                }
            }
        }
        s
    }
}

/// Search index for a collection.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchIndex {
    /// Document entries.
    pub documents: Vec<IndexEntry>,
    /// Inverted index: stemmed word -> document indices with term frequency.
    pub word_index: HashMap<String, Vec<(usize, u16)>>,
    /// Total word count per document.
    pub doc_lengths: Vec<usize>,
}

/// Entry for a document in the search index.
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexEntry {
    pub slug: String,
    pub title: String,
    pub content_preview: String,
    pub tags: Vec<String>,
    pub word_count: usize,
}

impl SearchIndex {
    /// Build an index from a collection.
    pub fn build(collection: &Collection) -> Self {
        let stop_words: HashSet<&str> = STOP_WORDS.iter().copied().collect();
        let mut documents = Vec::new();
        let mut word_index: HashMap<String, Vec<(usize, u16)>> = HashMap::new();
        let mut doc_lengths = Vec::new();

        for (idx, doc) in collection.documents.iter().enumerate() {
            // Create entry.
            let entry = IndexEntry {
                slug: doc.slug(),
                title: doc.frontmatter.title.clone(),
                content_preview: doc.excerpt("<!--more-->", 200),
                tags: doc.frontmatter.tags.clone(),
                word_count: doc.word_count(),
            };

            // Index words from title (boosted) and content.
            let title_text = &doc.frontmatter.title;
            let content_text = format!("{} {}", doc.frontmatter.tags.join(" "), doc.content);

            let mut term_counts: HashMap<String, u16> = HashMap::new();
            let mut total_words = 0;

            // Title words get extra weight.
            for word in title_text.unicode_words() {
                let normalized = word.to_lowercase();
                if normalized.len() >= 2 && !stop_words.contains(normalized.as_str()) {
                    let stemmed = PorterStemmer::stem(&normalized);
                    *term_counts.entry(stemmed).or_default() += 3; // Title boost.
                    total_words += 1;
                }
            }

            // Content words.
            for word in content_text.unicode_words() {
                let normalized = word.to_lowercase();
                if normalized.len() >= 2 && !stop_words.contains(normalized.as_str()) {
                    let stemmed = PorterStemmer::stem(&normalized);
                    *term_counts.entry(stemmed).or_default() += 1;
                    total_words += 1;
                }
            }

            // Add to inverted index.
            for (term, count) in term_counts {
                word_index.entry(term).or_default().push((idx, count));
            }

            documents.push(entry);
            doc_lengths.push(total_words);
        }

        Self {
            documents,
            word_index,
            doc_lengths,
        }
    }

    /// Search for documents matching a query.
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let stop_words: HashSet<&str> = STOP_WORDS.iter().copied().collect();

        let query_terms: Vec<String> = query
            .unicode_words()
            .map(|w| w.to_lowercase())
            .filter(|w| w.len() >= 2 && !stop_words.contains(w.as_str()))
            .map(|w| PorterStemmer::stem(&w))
            .collect();

        if query_terms.is_empty() {
            return Vec::new();
        }

        // Calculate BM25-like scores.
        let avg_doc_len = if self.doc_lengths.is_empty() {
            1.0
        } else {
            self.doc_lengths.iter().sum::<usize>() as f64 / self.doc_lengths.len() as f64
        };

        let k1 = 1.2;
        let b = 0.75;
        let num_docs = self.documents.len() as f64;

        let mut doc_scores: HashMap<usize, f64> = HashMap::new();

        for term in &query_terms {
            // Exact stem match.
            if let Some(postings) = self.word_index.get(term) {
                let df = postings.len() as f64;
                let idf = ((num_docs - df + 0.5) / (df + 0.5) + 1.0).ln();

                for &(doc_idx, tf) in postings {
                    let doc_len = self.doc_lengths.get(doc_idx).copied().unwrap_or(1) as f64;
                    let tf = tf as f64;
                    let score = idf * (tf * (k1 + 1.0)) / (tf + k1 * (1.0 - b + b * doc_len / avg_doc_len));
                    *doc_scores.entry(doc_idx).or_default() += score;
                }
            }

            // Prefix matching for partial terms.
            for (indexed_term, postings) in &self.word_index {
                if indexed_term.starts_with(term) && indexed_term != term {
                    let df = postings.len() as f64;
                    let idf = ((num_docs - df + 0.5) / (df + 0.5) + 1.0).ln();

                    for &(doc_idx, tf) in postings {
                        let doc_len = self.doc_lengths.get(doc_idx).copied().unwrap_or(1) as f64;
                        let tf = tf as f64 * 0.5; // Reduced weight for prefix match.
                        let score = idf * (tf * (k1 + 1.0)) / (tf + k1 * (1.0 - b + b * doc_len / avg_doc_len));
                        *doc_scores.entry(doc_idx).or_default() += score;
                    }
                }
            }
        }

        use rustmax::itertools::Itertools;

        // Sort by score descending.
        doc_scores
            .into_iter()
            .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal))
            .take(20)
            .filter_map(|(idx, score)| {
                self.documents.get(idx).map(|entry| SearchResult {
                    slug: entry.slug.clone(),
                    title: entry.title.clone(),
                    preview: entry.content_preview.clone(),
                    score: (score * 100.0) as usize,
                })
            })
            .collect()
    }

    /// Get suggestions for a partial query (autocomplete).
    pub fn suggest(&self, prefix: &str) -> Vec<String> {
        let prefix = PorterStemmer::stem(&prefix.to_lowercase());
        if prefix.len() < 2 {
            return Vec::new();
        }

        self.word_index
            .keys()
            .filter(|term| term.starts_with(&prefix))
            .take(10)
            .cloned()
            .collect()
    }
}

/// A search result.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub slug: String,
    pub title: String,
    pub preview: String,
    pub score: usize,
}

/// Build and save search index for a collection.
pub fn build_index(collection: &Collection, root: &Path) -> Result<()> {
    info!("Building search index for {} documents", collection.documents.len());

    let index = SearchIndex::build(collection);
    let index_path = root.join("search-index.json");

    let json = rustmax::serde_json::to_string_pretty(&index)?;
    std::fs::write(&index_path, json)?;

    info!("Search index saved to {}", index_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collection::Document;
    use std::path::PathBuf;

    fn make_doc(title: &str, content: &str) -> Document {
        let raw = format!(
            "---\ntitle = \"{}\"\n---\n{}",
            title, content
        );
        Document::parse(PathBuf::from("test.md"), &raw).unwrap()
    }

    #[test]
    fn test_search_index() {
        let docs = vec![
            make_doc("Rust Programming", "Learn about Rust and memory safety."),
            make_doc("Python Basics", "An introduction to Python programming."),
            make_doc("Advanced Rust", "Deep dive into Rust async and lifetimes."),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let index = SearchIndex::build(&collection);
        assert_eq!(index.documents.len(), 3);

        let results = index.search("rust");
        assert_eq!(results.len(), 2);
        assert!(results[0].title.contains("Rust"));

        let results = index.search("python");
        assert_eq!(results.len(), 1);
        assert!(results[0].title.contains("Python"));
    }

    #[test]
    fn test_stemming() {
        assert_eq!(PorterStemmer::stem("running"), "run");
        assert_eq!(PorterStemmer::stem("runs"), "run");
        assert_eq!(PorterStemmer::stem("programmed"), "program");
        assert_eq!(PorterStemmer::stem("programming"), "program");
        assert_eq!(PorterStemmer::stem("programs"), "program");
    }

    #[test]
    fn test_stemming_plurals() {
        assert_eq!(PorterStemmer::stem("cats"), "cat");
        assert_eq!(PorterStemmer::stem("ponies"), "poni");
        assert_eq!(PorterStemmer::stem("caresses"), "caress");
    }

    #[test]
    fn test_stemming_ed_ing() {
        assert_eq!(PorterStemmer::stem("agreed"), "agre");
        assert_eq!(PorterStemmer::stem("disabled"), "disabl");
        assert_eq!(PorterStemmer::stem("matting"), "mat");
        assert_eq!(PorterStemmer::stem("mating"), "mate");
    }

    #[test]
    fn test_stop_words_filtered() {
        let docs = vec![
            make_doc("The Quick Fox", "The quick brown fox jumps over the lazy dog."),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let index = SearchIndex::build(&collection);

        // "the" should not be indexed.
        assert!(!index.word_index.contains_key("the"));

        // "quick" should be indexed (stemmed).
        assert!(index.word_index.contains_key("quick"));

        // "fox" should be indexed.
        assert!(index.word_index.contains_key("fox"));
    }

    #[test]
    fn test_search_with_stemming() {
        let docs = vec![
            make_doc("Running Fast", "I was running quickly through the park."),
            make_doc("Slow Walk", "A leisurely walk in the garden."),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let index = SearchIndex::build(&collection);

        // "run" should match "running".
        let results = index.search("run");
        assert_eq!(results.len(), 1);
        assert!(results[0].title.contains("Running"));

        // "runs" should also match "running".
        let results = index.search("runs");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_suggest() {
        let docs = vec![
            make_doc("Programming", "Learn programming basics."),
            make_doc("Projects", "Start new projects."),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let index = SearchIndex::build(&collection);
        let suggestions = index.suggest("pro");
        assert!(suggestions.len() >= 1);
    }

    #[test]
    fn test_title_boost() {
        let docs = vec![
            make_doc("Rust Language", "A systems programming language."),
            make_doc("Systems Programming", "Learn about Rust and other languages."),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let index = SearchIndex::build(&collection);
        let results = index.search("rust");

        // "Rust Language" should rank higher due to title boost.
        assert_eq!(results.len(), 2);
        assert!(results[0].title.contains("Rust"));
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use rustmax::proptest::prelude::*;

    proptest! {
        #[test]
        fn stemmer_produces_nonempty_output(word in "[a-z]{3,20}") {
            let stemmed = PorterStemmer::stem(&word);
            prop_assert!(!stemmed.is_empty());
        }

        #[test]
        fn stemmer_output_is_shorter_or_equal(word in "[a-z]{3,15}") {
            // Stemmed word should not be longer than original.
            let stemmed = PorterStemmer::stem(&word);
            prop_assert!(stemmed.len() <= word.len());
        }

        #[test]
        fn stemmer_is_lowercase(word in "[a-zA-Z]{3,20}") {
            let stemmed = PorterStemmer::stem(&word);
            prop_assert!(stemmed.chars().all(|c| c.is_ascii_lowercase()));
        }

        #[test]
        fn search_returns_sorted_by_score(
            title1 in "[a-zA-Z ]{5,20}",
            title2 in "[a-zA-Z ]{5,20}",
            content1 in "[a-zA-Z ]{20,100}",
            content2 in "[a-zA-Z ]{20,100}"
        ) {
            use crate::collection::Document;

            let make_doc = |title: &str, content: &str| -> Document {
                let raw = format!("---\ntitle = \"{}\"\n---\n{}", title, content);
                Document::parse(std::path::PathBuf::from("test.md"), &raw).unwrap()
            };

            let docs = vec![
                make_doc(&title1, &content1),
                make_doc(&title2, &content2),
            ];

            let collection = Collection {
                root: std::path::PathBuf::from("."),
                documents: docs,
            };

            let index = SearchIndex::build(&collection);

            // Search for a word that might be in the content.
            if let Some(word) = content1.split_whitespace().next() {
                let results = index.search(word);
                // Results should be sorted by score descending.
                for window in results.windows(2) {
                    prop_assert!(window[0].score >= window[1].score);
                }
            }
        }

        #[test]
        fn suggest_returns_matching_prefixes(word in "[a-z]{4,10}") {
            use crate::collection::Document;

            let make_doc = |title: &str, content: &str| -> Document {
                let raw = format!("---\ntitle = \"{}\"\n---\n{}", title, content);
                Document::parse(std::path::PathBuf::from("test.md"), &raw).unwrap()
            };

            let docs = vec![
                make_doc("Test Document", &format!("This contains the word {}.", word)),
            ];

            let collection = Collection {
                root: std::path::PathBuf::from("."),
                documents: docs,
            };

            let index = SearchIndex::build(&collection);

            // Suggest with first 2 chars should include the stemmed word.
            let prefix = &word[..2.min(word.len())];
            let suggestions = index.suggest(prefix);
            // All suggestions should start with the stemmed prefix.
            let stemmed_prefix = PorterStemmer::stem(prefix);
            for suggestion in &suggestions {
                prop_assert!(suggestion.starts_with(&stemmed_prefix));
            }
        }
    }
}
