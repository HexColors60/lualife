use rand::Rng;

pub fn derive_seed(base_seed: u64, modifier: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();
    base_seed.hash(&mut hasher);
    modifier.hash(&mut hasher);
    hasher.finish()
}

pub fn seed_from_string(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}