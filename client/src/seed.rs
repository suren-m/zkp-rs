// Random Seed - k

use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
pub struct Seed {
    pub val: u32,
}

impl Seed {
    /// Random u128 number
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            val: rng.gen_range(1..25),
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
