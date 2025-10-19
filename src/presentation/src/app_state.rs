use application::{CustomerRepository, Settings};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
}
