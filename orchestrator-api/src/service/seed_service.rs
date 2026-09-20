use crate::{
    error::{AppError, AppResult},
    generator::{DataGenerator, uniform::UniformStrategy, zipfian::ZipfianStrategy},
};
use error_stack::{IntoReportCompat, ResultExt};
use tracing::info;

/// A constant seed value for reproducibility.
const SEED: u64 = 42;

/// Service responsible for handling seeding operations.
#[derive(Debug, Clone, Default)]
pub struct SeedService;

impl SeedService {
    /// Handles the uniform seeding request.
    pub fn uniform(&self, n: u64) -> AppResult<()> {
        let strategy = UniformStrategy::new(n).into_report().change_context(
            AppError::SeedDistributionError("failed to create uniform strategy".to_string()),
        )?;

        let mut generator = DataGenerator::new(strategy, SEED);
        for _ in 0..n {
            info!("Generating uniform value {:?}", generator.generate());
        }

        Ok(())
    }

    /// Handles the zipfian seeding request.
    pub fn zipfian(&self, n: u64, s: f64) -> AppResult<()> {
        let strategy = ZipfianStrategy::new(n, s).into_report().change_context(
            AppError::SeedDistributionError("failed to create zipfian strategy".to_string()),
        )?;

        let mut generator = DataGenerator::new(strategy, SEED);
        for _ in 0..n {
            info!("Generating zipfian value {:?}", generator.generate());
        }

        Ok(())
    }
}
