mod api_customer;
mod api_health_check;
mod api_info;
pub mod docs;

pub use api_customer::create as create_customer;
pub use api_customer::update as update_customer;

pub use api_health_check::live;
pub use api_health_check::ready;
pub use api_health_check::startup;
pub use api_info::info;
