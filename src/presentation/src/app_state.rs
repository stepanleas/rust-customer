use application::{CustomerMessagePublisher, CustomerRepository, Settings};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
    pub customer_message_publisher: Arc<dyn CustomerMessagePublisher + Send + Sync>,
}
