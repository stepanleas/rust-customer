mod builders;
mod entities;
mod error;
mod events;

pub use entities::Customer;
pub use error::DomainError;
pub use events::CustomerCreatedEvent;
pub use events::CustomerUpdatedEvent;
