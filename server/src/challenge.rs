use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
pub struct Challenge(u128);

impl Challenge {
    /// Random u128 number
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self(rng.gen_range(1..u128::MAX))
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
