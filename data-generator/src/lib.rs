use crate::distribution::DistributionStrategy;
use fake::{Fake, faker::lorem::en::Sentence};
use rand::{SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};

pub mod distribution;

/// A constant that defines the start date for generated data objects.
const START_DATE: i64 = 1704067200; // 2024-01-01 00:00:00 UTC
/// A constant that defines the end date for generated data objects.
const END_DATE: i64 = 1711929600; // 2024-04-01 00:00:00 UTC

/// A data object that represents a single unit of data in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataObject {
    pub id: u64,
    pub content: String,
    pub created_at: i64,
    pub number: u32,
}

/// A data generator that uses a distribution strategy
/// to generate data objects.
pub struct DataGenerator<T: DistributionStrategy> {
    strategy: T,
    rng: StdRng,
}

impl<T: DistributionStrategy> DataGenerator<T> {
    pub fn new(strategy: T, seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        Self { strategy, rng }
    }

    /// Generates a data object using the distribution strategy.
    pub fn generate(&mut self) -> DataObject {
        let id = self.strategy.next(&mut self.rng);
        let raw: String = Sentence(5..15).fake_with_rng(&mut self.rng);
        let content: String = raw.chars().take(100).collect();
        let created_at: i64 = (START_DATE..END_DATE).fake_with_rng(&mut self.rng);
        let number: u32 = (0..500).fake_with_rng(&mut self.rng);

        DataObject {
            id,
            content,
            created_at,
            number,
        }
    }
}
