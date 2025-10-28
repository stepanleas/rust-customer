use crate::CustomerMessagePublisher;
use crate::commands::{CreateCustomerCommand, UpdateCustomerCommand};
use crate::dtos::CustomerDto;
use crate::mappers::CustomerMapper;
use crate::repositories::CustomerRepository;
use domain::{CustomerCreatedEvent, CustomerUpdatedEvent};
use log::info;
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

    pub async fn execute(&self, command: CreateCustomerCommand) -> anyhow::Result<CustomerDto> {
        let customer = self
            .repository
            .save(CustomerMapper::map_create_customer_command_to_domain_entity(command))?;
        info!(
            "Customer with id: {} created",
            customer.id().as_uuid().to_string()
        );

        self.message_publisher
            .publish_created(CustomerCreatedEvent::new(customer.clone()))?;

        Ok(CustomerDto::from(customer))
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

    pub async fn execute(&self, command: UpdateCustomerCommand) -> anyhow::Result<CustomerDto> {
        let customer = self
            .repository
            .save(CustomerMapper::map_update_customer_command_to_domain_entity(command))?;
        info!(
            "Customer with id: {} updated",
            customer.id().as_uuid().to_string()
        );

        self.message_publisher
            .publish_updated(CustomerUpdatedEvent::new(customer.clone()))?;

        Ok(CustomerDto::from(customer))
    }
}
