use std::cell::RefCell;

use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng as ChaCha;

#[derive(Debug)]
pub struct Random {
    rng: RefCell<ChaCha>,
}

impl Clone for Random {
    fn clone(&self) -> Self {
        // drop mutable ref to avoid panic
        let chacha = self.rng.borrow_mut().clone();
        let rng = RefCell::new(chacha);
        Self { rng }
    }
}

impl Random {
    pub fn from_seed(seed: u64) -> Self {
        let rng = ChaCha::seed_from_u64(seed);
        Self {
            rng: RefCell::new(rng),
        }
    }

    pub fn random_f64(&self) -> f64 {
        // drop mutable ref to avoid panic
        self.rng.borrow_mut().random()
    }

    pub fn random_f64_in_range(&self, min: f64, max: f64) -> f64 {
        // returns a random real in [min,max)
        min + (max - min) * self.random_f64()
    }
}
