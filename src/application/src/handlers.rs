use crate::commands::{CreateCustomerCommand, UpdateCustomerCommand};
use crate::dtos::CustomerDto;
use crate::ports::output::publishers::CustomerMessagePublisher;
use crate::repositories::CustomerRepository;
use domain::entities::Customer;
use domain::events::{CustomerCreatedEvent, CustomerUpdatedEvent};
use shared::domain::value_objects::CustomerId;
use std::sync::Arc;

pub struct CreateCustomerCommandHandler {
    repository: Arc<dyn CustomerRepository>,
    message_publisher: Arc<dyn CustomerMessagePublisher>,
}

impl CreateCustomerCommandHandler {
    pub fn new(
        repository: Arc<dyn CustomerRepository>,
        message_publisher: Arc<dyn CustomerMessagePublisher>,
    ) -> Self {
        Self {
            repository,
            message_publisher,
        }
    }

    // TODO: Add transactional outbox
    pub async fn execute(&self, command: CreateCustomerCommand) -> anyhow::Result<CustomerDto> {
        let customer = Customer::builder()
            .id(CustomerId::new())
            .user_name(command.user_name().into())
            .first_name(command.first_name().into())
            .last_name(command.last_name().into())
            .build();

        let saved_customer = self.repository.save(customer)?;
        tracing::info!(
            "Customer with id: {} created",
            saved_customer.id().as_uuid().to_string(),
        );

        self.message_publisher
            .publish_created(CustomerCreatedEvent::new(saved_customer.clone()))?;

        Ok(CustomerDto::from(saved_customer))
    }
}

pub struct UpdateCustomerCommandHandler {
    repository: Arc<dyn CustomerRepository>,
    message_publisher: Arc<dyn CustomerMessagePublisher>,
}

impl UpdateCustomerCommandHandler {
    pub fn new(
        repository: Arc<dyn CustomerRepository>,
        message_publisher: Arc<dyn CustomerMessagePublisher>,
    ) -> Self {
        Self {
            repository,
            message_publisher,
        }
    }

    // TODO: Add transactional outbox
    pub async fn execute(&self, command: UpdateCustomerCommand) -> anyhow::Result<CustomerDto> {
        let customer = Customer::builder()
            .id(CustomerId::from_uuid(command.id()))
            .user_name(command.user_name().into())
            .first_name(command.first_name().into())
            .last_name(command.last_name().into())
            .build();

        let saved_customer = self.repository.save(customer)?;
        tracing::info!(
            "Customer with id: {} updated",
            saved_customer.id().as_uuid().to_string()
        );

        self.message_publisher
            .publish_updated(CustomerUpdatedEvent::new(saved_customer.clone()))?;

        Ok(CustomerDto::from(saved_customer))
    }
}
