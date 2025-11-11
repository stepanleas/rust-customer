use application::ports::output::publishers::CustomerMessagePublisher;
use application::repositories::CustomerRepository;
use application::settings::Settings;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
    pub customer_message_publisher: Arc<dyn CustomerMessagePublisher + Send + Sync>,
}
