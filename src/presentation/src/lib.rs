mod api;
mod app_state;
mod config;
mod error;
mod requests;
mod responses;
mod validation;

pub use app_state::AppState;

pub use api::docs::open_api_docs;
pub use config::configure;
pub use responses::CustomerResponse;
