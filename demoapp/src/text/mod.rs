//! Text processing utilities using memchr and unicode-segmentation.

use rustmax::prelude::*;
use rustmax::memchr::{memchr, memchr2, memchr3, memchr_iter, memmem};
use rustmax::unicode_segmentation::UnicodeSegmentation;

/// Fast byte search using memchr.
pub fn find_byte(haystack: &[u8], needle: u8) -> Option<usize> {
    memchr(needle, haystack)
}

/// Find any of two bytes.
pub fn find_byte2(haystack: &[u8], needle1: u8, needle2: u8) -> Option<usize> {
    memchr2(needle1, needle2, haystack)
}

/// Find any of three bytes.
pub fn find_byte3(haystack: &[u8], needle1: u8, needle2: u8, needle3: u8) -> Option<usize> {
    memchr3(needle1, needle2, needle3, haystack)
}

/// Count occurrences of a byte.
pub fn count_byte(haystack: &[u8], needle: u8) -> usize {
    memchr_iter(needle, haystack).count()
}

/// Find all positions of a byte.
pub fn find_all_bytes(haystack: &[u8], needle: u8) -> Vec<usize> {
    memchr_iter(needle, haystack).collect()
}

/// Fast substring search using memmem.
pub fn find_substring(haystack: &str, needle: &str) -> Option<usize> {
    let finder = memmem::Finder::new(needle.as_bytes());
    finder.find(haystack.as_bytes())
}

/// Find all occurrences of a substring.
pub fn find_all_substrings(haystack: &str, needle: &str) -> Vec<usize> {
    let finder = memmem::Finder::new(needle.as_bytes());
    finder.find_iter(haystack.as_bytes()).collect()
}

/// Count occurrences of a substring.
pub fn count_substring(haystack: &str, needle: &str) -> usize {
    let finder = memmem::Finder::new(needle.as_bytes());
    finder.find_iter(haystack.as_bytes()).count()
}

/// Check if string contains substring (fast).
pub fn contains_fast(haystack: &str, needle: &str) -> bool {
    find_substring(haystack, needle).is_some()
}

/// Count words in text using unicode segmentation.
pub fn count_words(text: &str) -> usize {
    text.unicode_words().count()
}

/// Extract words from text.
pub fn extract_words(text: &str) -> Vec<&str> {
    text.unicode_words().collect()
}

/// Count graphemes (user-perceived characters).
pub fn count_graphemes(text: &str) -> usize {
    text.graphemes(true).count()
}

/// Truncate text to a maximum number of graphemes.
pub fn truncate_graphemes(text: &str, max: usize) -> String {
    text.graphemes(true).take(max).collect()
}

/// Split text into sentences (simple heuristic).
pub fn split_sentences(text: &str) -> Vec<&str> {
    text.unicode_sentences().collect()
}

/// Find line breaks in text.
pub fn find_line_breaks(text: &str) -> Vec<usize> {
    find_all_bytes(text.as_bytes(), b'\n')
}

/// Count lines in text.
pub fn count_lines(text: &str) -> usize {
    count_byte(text.as_bytes(), b'\n') + 1
}

/// Split into lines efficiently.
pub fn split_lines(text: &str) -> Vec<&str> {
    text.lines().collect()
}

/// Normalize whitespace (collapse multiple spaces).
pub fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_byte() {
        assert_eq!(find_byte(b"hello", b'e'), Some(1));
        assert_eq!(find_byte(b"hello", b'x'), None);
    }

    #[test]
    fn test_find_byte2() {
        assert_eq!(find_byte2(b"hello", b'e', b'o'), Some(1));
        assert_eq!(find_byte2(b"hello", b'o', b'e'), Some(1));
    }

    #[test]
    fn test_count_byte() {
        assert_eq!(count_byte(b"hello", b'l'), 2);
        assert_eq!(count_byte(b"hello", b'x'), 0);
    }

    #[test]
    fn test_find_all_bytes() {
        let positions = find_all_bytes(b"hello", b'l');
        assert_eq!(positions, vec![2, 3]);
    }

    #[test]
    fn test_find_substring() {
        assert_eq!(find_substring("hello world", "world"), Some(6));
        assert_eq!(find_substring("hello world", "xyz"), None);
    }

    #[test]
    fn test_find_all_substrings() {
        let positions = find_all_substrings("abcabcabc", "abc");
        assert_eq!(positions, vec![0, 3, 6]);
    }

    #[test]
    fn test_count_substring() {
        assert_eq!(count_substring("abcabcabc", "abc"), 3);
        assert_eq!(count_substring("hello", "ll"), 1);
    }

    #[test]
    fn test_contains_fast() {
        assert!(contains_fast("hello world", "world"));
        assert!(!contains_fast("hello world", "xyz"));
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("one two three four"), 4);
    }

    #[test]
    fn test_extract_words() {
        let words = extract_words("hello, world!");
        assert_eq!(words, vec!["hello", "world"]);
    }

    #[test]
    fn test_count_graphemes() {
        assert_eq!(count_graphemes("hello"), 5);
        // Emoji with skin tone modifier is one grapheme.
        assert_eq!(count_graphemes("a]"), 2);
    }

    #[test]
    fn test_truncate_graphemes() {
        assert_eq!(truncate_graphemes("hello world", 5), "hello");
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("one\ntwo\nthree"), 3);
        assert_eq!(count_lines("single line"), 1);
    }

    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(normalize_whitespace("hello   world"), "hello world");
        assert_eq!(normalize_whitespace("  a  b  c  "), "a b c");
    }
}
