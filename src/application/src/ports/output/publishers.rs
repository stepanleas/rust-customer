use domain::events::{CustomerCreatedEvent, CustomerUpdatedEvent};
use mockall::automock;

#[automock]
pub trait CustomerMessagePublisher {
    fn publish_created(&self, event: CustomerCreatedEvent) -> anyhow::Result<()>;
    fn publish_updated(&self, event: CustomerUpdatedEvent) -> anyhow::Result<()>;
}
