use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Challenge {
    pub val: u32,
}

impl Challenge {
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
    pub fn challenge_must_generate_random_num() {
        let c1 = Challenge::new();
        let c2 = Challenge::new();
        assert_ne!(c1, c2);
    }
}
