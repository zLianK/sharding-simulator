use anyhow::Result;
use rand::{RngExt, rngs::StdRng};
use rand_distr::{Uniform, Zipf};

/// A trait that defines a distribution strategy.
pub trait DistributionStrategy {
    /// Returns the next value in the distribution.
    fn next(&self, rng: &mut StdRng) -> u64;
}

/// A distribution strategy that generates values
/// according to a uniform distribution.
pub struct UniformStrategy {
    uniform: Uniform<u64>,
}

impl UniformStrategy {
    pub fn new(n: u64) -> Result<Self> {
        Ok(Self {
            uniform: Uniform::new_inclusive(1, n)?,
        })
    }
}

impl DistributionStrategy for UniformStrategy {
    fn next(&self, rng: &mut StdRng) -> u64 {
        rng.sample(self.uniform)
    }
}

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
