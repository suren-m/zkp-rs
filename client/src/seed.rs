// Random Seed - k

use rand::Rng;

use crate::MAX_SEED_VAL;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Seed {
    pub val: u32,
}

impl Seed {
    /// Random u32 number
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            val: rng.gen_range(1..MAX_SEED_VAL),
        }
    }
}

#[cfg(test)]
mod unittests {
    use super::*;
    use pretty_assertions::assert_ne;

    #[test]
    pub fn seed_k_must_be_random_num() {
        let mut rng = rand::thread_rng();
        let k1 = Seed::new();
        let k2 = Seed::new();
        assert_ne!(k1, k2);
    }
}
