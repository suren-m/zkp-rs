// Random Seed - k

use log::info;
use rand::Rng;

use crate::{MAX_SEED_VAL, MIN_SEED_VAL};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Seed {
    pub val: u32,
}

impl Seed {
    /// Random u32 number
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(MIN_SEED_VAL..MAX_SEED_VAL);
        info!("Seed val: {}", val);
        Self { val }
    }
}

#[cfg(test)]
mod unittests {
    use super::*;
    use pretty_assertions::assert_ne;

    #[test]
    pub fn seed_k_must_be_random_num() {
        let _rng = rand::thread_rng();
        let k1 = Seed::new();
        let k2 = Seed::new();
        assert_ne!(k1, k2);
    }
}
