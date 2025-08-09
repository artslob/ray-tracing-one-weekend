use std::sync::{Arc, Mutex};

use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng as ChaCha;

#[derive(Debug, Clone)]
pub(crate) struct Random {
    rng: Arc<Mutex<ChaCha>>,
}

impl Random {
    pub(crate) fn from_seed(seed: u64) -> Self {
        let rng = ChaCha::seed_from_u64(seed);
        Self {
            rng: Arc::new(Mutex::new(rng)),
        }
    }

    pub(crate) fn random_f64(&self) -> f64 {
        self.rng.lock().unwrap().random()
    }

    pub fn random_f64_in_range(&self, min: f64, max: f64) -> f64 {
        // returns a random real in [min,max)
        min + (max - min) * self.random_f64()
    }
}
