//! Identifier generation.
//!
//! Folio wants two things from generated identifiers that pull in opposite
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
/// [`Folio's builds reproducible`](crate::build).
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
        assert_eq!(a.slug(16), b.slug(16));
    }

    #[test]
    fn different_seeds_diverge() {
        assert_ne!(
            IdGenerator::from_seed(1).id(16),
            IdGenerator::from_seed(2).id(16),
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
    }

    #[test]
    fn a_seeded_build_id_is_stable() {
        assert_eq!(build_id(Some(99)), build_id(Some(99)));
        assert_ne!(build_id(Some(99)), build_id(Some(100)));
        assert_eq!(build_id(Some(1)).len(), 12);
    }
}
