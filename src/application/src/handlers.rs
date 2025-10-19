use crate::commands::{CreateCustomerCommand, UpdateCustomerCommand};
use crate::dtos::CustomerDto;
use crate::mappers::CustomerMapper;
use crate::repositories::CustomerRepository;
use domain::CustomerCreatedEvent;
use std::sync::Arc;

pub struct CreateCustomerCommandHandler {
    repository: Arc<dyn CustomerRepository>,
}

impl CreateCustomerCommandHandler {
    pub fn new(repository: Arc<dyn CustomerRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateCustomerCommand) -> anyhow::Result<CustomerDto> {
        let customer = self
            .repository
            .save(CustomerMapper::map_create_customer_command_to_domain_entity(command))?;

        let event = CustomerCreatedEvent::new(customer);

        Ok(CustomerDto::from(event.customer()))
    }
}

pub struct UpdateCustomerCommandHandler {
    repository: Arc<dyn CustomerRepository>,
}

impl UpdateCustomerCommandHandler {
    pub fn new(repository: Arc<dyn CustomerRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: UpdateCustomerCommand) -> anyhow::Result<CustomerDto> {
        let customer = self
            .repository
            .save(CustomerMapper::map_update_customer_command_to_domain_entity(command))?;

        Ok(CustomerDto::from(customer))
    }
}
