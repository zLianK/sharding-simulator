use crate::{
    model::seed_model::{UniformSeed, ZipfianSeed},
    state::AppState,
};
use axum::{Json, extract::State};

/// Handles the uniform seeding request.
pub async fn uniform_seed(
    State(state): State<AppState>,
    Json(payload): Json<UniformSeed>,
) -> String {
    state.seed_service.uniform(payload.n);
    "The seeding process has started successfully with uniform distribution".to_string()
}

/// Handles the zipfian seeding request.
pub async fn zipfian_seed(
    State(state): State<AppState>,
    Json(payload): Json<ZipfianSeed>,
) -> String {
    state.seed_service.zipfian(payload.n, payload.s);
    "The seeding process has started successfully with zipfian distribution".to_string()
}
