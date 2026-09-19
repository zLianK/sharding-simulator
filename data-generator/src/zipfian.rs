use crate::DistributionStrategy;
use anyhow::Result;
use rand::{RngExt, rngs::StdRng};
use rand_distr::Zipf;

/// A distribution strategy that generates values
/// according to a Zipfian distribution.
pub struct ZipfianStrategy {
    zipf: Zipf<f64>,
}

impl ZipfianStrategy {
    pub fn new(n: u64, s: f64) -> Result<Self> {
        Ok(Self {
            zipf: Zipf::new(n as f64, s)?,
        })
    }
}

impl DistributionStrategy for ZipfianStrategy {
    fn next(&self, rng: &mut StdRng) -> u64 {
        rng.sample(self.zipf) as u64
    }
}
