use crate::model::seed::{UniformSeed, ZipfianSeed};
use axum::Json;
use data_generator::{DataGenerator, uniform::UniformStrategy, zipfian::ZipfianStrategy};
use tracing::info;

/// A constant seed value for reproducibility.
const SEED: u64 = 42;

/// Handles the uniform seeding request.
pub async fn uniform_seed(Json(payload): Json<UniformSeed>) -> String {
    let strategy = UniformStrategy::new(payload.n).unwrap();
    let mut generator = DataGenerator::new(strategy, SEED);

    for _ in 0..payload.n {
        info!("Generating uniform value {:?}", generator.generate());
    }

    "The seeding process has started successfully with uniform distribution".to_string()
}

/// Handles the zipfian seeding request.
pub async fn zipfian_seed(Json(payload): Json<ZipfianSeed>) -> String {
    let strategy = ZipfianStrategy::new(payload.n, payload.s).unwrap();
    let mut generator = DataGenerator::new(strategy, SEED);

    for _ in 0..payload.n {
        info!("Generating zipfian value {:?}", generator.generate());
    }

    "The seeding process has started successfully with zipfian distribution".to_string()
}
