use domain::{CustomerCreatedEvent, CustomerUpdatedEvent};

pub trait CustomerMessagePublisher {
    fn publish_created(&self, event: CustomerCreatedEvent) -> anyhow::Result<()>;
    fn publish_updated(&self, event: CustomerUpdatedEvent) -> anyhow::Result<()>;
}
