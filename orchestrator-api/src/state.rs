use crate::service::seed_service::SeedService;
use std::sync::Arc;

/// Application state shared across the orchestrator API.
#[derive(Debug, Clone)]
pub struct AppState {
    pub seed_service: Arc<SeedService>,
}
