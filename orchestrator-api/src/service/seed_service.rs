use crate::{
    error::AppResult,
    generator::{DataGenerator, uniform::UniformStrategy, zipfian::ZipfianStrategy},
};
use tracing::info;

/// A constant seed value for reproducibility.
const SEED: u64 = 42;

/// Service responsible for handling seeding operations.
#[derive(Debug, Clone, Default)]
pub struct SeedService;

impl SeedService {
    /// Handles the uniform seeding request.
    pub fn uniform(&self, n: u64) -> AppResult<()> {
        let strategy = UniformStrategy::new(n)?;
        let mut generator = DataGenerator::new(strategy, SEED);
        for _ in 0..n {
            info!("Generating uniform value {:?}", generator.generate());
        }
        Ok(())
    }

    /// Handles the zipfian seeding request.
    pub fn zipfian(&self, n: u64, s: f64) -> AppResult<()> {
        let strategy = ZipfianStrategy::new(n, s)?;
        let mut generator = DataGenerator::new(strategy, SEED);
        for _ in 0..n {
            info!("Generating zipfian value {:?}", generator.generate());
        }
        Ok(())
    }
}
