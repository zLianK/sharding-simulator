use crate::generator::strategy::DistributionStrategy;
use anyhow::Result;
use rand::{RngExt, rngs::StdRng};
use rand_distr::Uniform;

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
