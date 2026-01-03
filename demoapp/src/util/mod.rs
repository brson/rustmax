//! Utility functions.

use rustmax::prelude::*;
use rustmax::rand::{Rng, rng, distr::Alphanumeric};

/// Generate a random alphanumeric ID.
pub fn random_id(len: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

/// Generate a random slug-friendly ID (lowercase alphanumeric).
pub fn random_slug(len: usize) -> String {
    random_id(len).to_lowercase()
}

/// Generate a UUID-like random identifier.
pub fn random_uuid() -> String {
    let mut r = rng();
    let bytes: [u8; 16] = r.random();

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

/// Generate a short random identifier suitable for filenames.
pub fn random_filename(prefix: &str, extension: &str) -> String {
    format!("{}-{}.{}", prefix, random_slug(8), extension)
}

/// Generate random bytes.
pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut r = rng();
    (0..len).map(|_| r.random()).collect()
}

/// Pick a random item from a slice.
pub fn random_choice<T>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        None
    } else {
        let idx = rng().random_range(0..items.len());
        Some(&items[idx])
    }
}

/// Shuffle a vector in place.
pub fn shuffle<T>(items: &mut [T]) {
    use rustmax::rand::seq::SliceRandom;
    items.shuffle(&mut rng());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_id() {
        let id = random_id(10);
        assert_eq!(id.len(), 10);
        assert!(id.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_random_slug() {
        let slug = random_slug(8);
        assert_eq!(slug.len(), 8);
        assert!(slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_random_uuid() {
        let uuid = random_uuid();
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
    }

    #[test]
    fn test_random_filename() {
        let name = random_filename("doc", "md");
        assert!(name.starts_with("doc-"));
        assert!(name.ends_with(".md"));
        assert_eq!(name.len(), 4 + 8 + 1 + 2); // "doc-" + 8 chars + "." + "md"
    }

    #[test]
    fn test_random_bytes() {
        let bytes = random_bytes(16);
        assert_eq!(bytes.len(), 16);
    }

    #[test]
    fn test_random_choice() {
        let items = vec![1, 2, 3, 4, 5];
        let choice = random_choice(&items);
        assert!(choice.is_some());
        assert!(items.contains(choice.unwrap()));

        let empty: Vec<i32> = vec![];
        assert!(random_choice(&empty).is_none());
    }

    #[test]
    fn test_shuffle() {
        let mut items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let original = items.clone();
        shuffle(&mut items);
        // Verify same elements (though order likely changed).
        items.sort();
        assert_eq!(items, original);
    }
}
