use serde::Deserialize;

/// Represents the parameters for uniform seeding.
#[derive(Debug, Deserialize)]
pub struct UniformSeed {
    pub n: u64,
}

/// Represents the parameters for zipfian seeding.
#[derive(Debug, Deserialize)]
pub struct ZipfianSeed {
    pub n: u64,
    pub s: f64,
}
