//! Identifier generation.
//!
//! Anthology wants two things from generated identifiers that pull in opposite
//! directions: they should not collide, and a rebuild of unchanged input should
//! produce a byte-identical site. [`IdGenerator`] resolves that by making the
//! randomness explicit -- seed it and the whole sequence is reproducible, on
//! any platform, across runs.

use rmx::prelude::*;
use rmx::rand::distr::Alphanumeric;
use rmx::rand_chacha::ChaCha8Rng;

/// A source of identifiers for one build.
///
/// The generator is `ChaCha8Rng` rather than the thread RNG because its output
/// is a documented function of its seed: a given seed produces the same
/// identifiers on every platform and every run, which is what makes
/// [`Anthology's builds reproducible`](crate::build).
pub struct IdGenerator {
    rng: ChaCha8Rng,
}

impl IdGenerator {
    /// A generator whose output is fixed by `seed`.
    pub fn from_seed(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// A generator seeded from the operating system.
    ///
    /// Successive builds get different identifiers.
    pub fn from_entropy() -> Self {
        Self {
            rng: ChaCha8Rng::from_rng(&mut rmx::rand::rng()),
        }
    }

    /// A generator seeded by hashing `content`.
    ///
    /// Two collections with the same content get the same identifiers without
    /// anyone having to choose a seed.
    pub fn from_content(content: &[u8]) -> Self {
        let hash = rmx::blake3::hash(content);
        let seed = u64::from_le_bytes(hash.as_bytes()[..8].try_into().expect("8 bytes"));
        Self::from_seed(seed)
    }

    /// A random alphanumeric identifier of `len` characters.
    pub fn id(&mut self, len: usize) -> String {
        (&mut self.rng)
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    /// A lowercase identifier, safe to use in a URL path segment.
    pub fn slug(&mut self, len: usize) -> String {
        self.id(len).to_lowercase()
    }

    /// A random version 4 UUID.
    pub fn uuid(&mut self) -> String {
        let mut bytes: [u8; 16] = self.rng.random();

        // Set the version (4) and variant (RFC 4122) fields, without which
        // this is a random string that merely looks like a UUID.
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;

        let hex = rmx::hex::encode(bytes);
        format!(
            "{}-{}-{}-{}-{}",
            &hex[0..8],
            &hex[8..12],
            &hex[12..16],
            &hex[16..20],
            &hex[20..32],
        )
    }

    /// A filename of the form `prefix-xxxxxxxx.ext`.
    pub fn filename(&mut self, prefix: &str, extension: &str) -> String {
        format!("{}-{}.{}", prefix, self.slug(8), extension)
    }

    /// `len` random bytes.
    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        self.rng.fill(bytes.as_mut_slice());
        bytes
    }

    /// One of `items`, or `None` if it is empty.
    pub fn choice<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            return None;
        }
        Some(&items[self.rng.random_range(0..items.len())])
    }

    /// Shuffle `items` in place.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        use rmx::rand::seq::SliceRandom;
        items.shuffle(&mut self.rng);
    }
}

/// The identifier stamped into a build's output.
///
/// Templates see it as `build_id`. It distinguishes one build of a site from
/// another in caches and error reports, and it is stable whenever the build is.
pub fn build_id(seed: Option<u64>) -> String {
    let mut ids = match seed {
        Some(seed) => IdGenerator::from_seed(seed),
        None => IdGenerator::from_entropy(),
    };
    ids.slug(12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_seed_fixes_the_whole_sequence() {
        let mut a = IdGenerator::from_seed(42);
        let mut b = IdGenerator::from_seed(42);

        assert_eq!(a.id(16), b.id(16));
        assert_eq!(a.uuid(), b.uuid());
        assert_eq!(a.bytes(32), b.bytes(32));
    }

    #[test]
    fn different_seeds_diverge() {
        assert_ne!(
            IdGenerator::from_seed(1).id(16),
            IdGenerator::from_seed(2).id(16),
        );
    }

    #[test]
    fn identical_content_seeds_identically() {
        let content = b"the same documents";
        assert_eq!(
            IdGenerator::from_content(content).id(12),
            IdGenerator::from_content(content).id(12),
        );
        assert_ne!(
            IdGenerator::from_content(content).id(12),
            IdGenerator::from_content(b"different documents").id(12),
        );
    }

    #[test]
    fn entropy_seeded_generators_differ() {
        assert_ne!(
            IdGenerator::from_entropy().id(24),
            IdGenerator::from_entropy().id(24),
        );
    }

    #[test]
    fn ids_have_the_requested_shape() {
        let mut ids = IdGenerator::from_seed(7);

        let id = ids.id(10);
        assert_eq!(id.len(), 10);
        assert!(id.chars().all(|c| c.is_ascii_alphanumeric()));

        let slug = ids.slug(8);
        assert_eq!(slug.len(), 8);
        assert!(slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));

        let name = ids.filename("doc", "md");
        assert!(name.starts_with("doc-") && name.ends_with(".md"));
        assert_eq!(name.len(), "doc-".len() + 8 + ".md".len());
    }

    #[test]
    fn uuids_carry_the_version_and_variant_of_a_v4() {
        let uuid = IdGenerator::from_seed(3).uuid();

        assert_eq!(uuid.len(), 36);
        let fields: Vec<&str> = uuid.split('-').collect();
        assert_eq!(
            fields.iter().map(|f| f.len()).collect::<Vec<_>>(),
            [8, 4, 4, 4, 12],
        );
        assert!(uuid.chars().all(|c| c == '-' || c.is_ascii_hexdigit()));

        // Version nibble, then the two variant bits.
        assert_eq!(fields[2].as_bytes()[0], b'4', "{uuid}");
        assert!(matches!(fields[3].as_bytes()[0], b'8' | b'9' | b'a' | b'b'), "{uuid}");
    }

    #[test]
    fn choice_returns_a_member_and_nothing_from_nothing() {
        let mut ids = IdGenerator::from_seed(11);
        let items = [1, 2, 3, 4, 5];

        let picked = ids.choice(&items).unwrap();
        assert!(items.contains(picked));

        assert!(ids.choice::<i32>(&[]).is_none());
    }

    #[test]
    fn shuffle_permutes_without_losing_elements() {
        let mut ids = IdGenerator::from_seed(5);
        let original: Vec<i32> = (1..=10).collect();

        let mut items = original.clone();
        ids.shuffle(&mut items);
        assert_ne!(items, original, "a shuffle of ten items should reorder them");

        items.sort();
        assert_eq!(items, original);
    }

    #[test]
    fn a_seeded_build_id_is_stable() {
        assert_eq!(build_id(Some(99)), build_id(Some(99)));
        assert_ne!(build_id(Some(99)), build_id(Some(100)));
        assert_eq!(build_id(Some(1)).len(), 12);
    }
}
