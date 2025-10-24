use domain::CustomerCreatedEvent;

pub trait CustomerMessagePublisher {
    fn publish(&self, event: CustomerCreatedEvent) -> anyhow::Result<()>;
}
