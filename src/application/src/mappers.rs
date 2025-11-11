use crate::commands::{CreateCustomerCommand, UpdateCustomerCommand};
use domain::entities::Customer;
use shared::domain::value_objects::CustomerId;

pub struct CustomerMapper;

impl CustomerMapper {
    pub fn map_create_customer_command_to_domain_entity(
        command: CreateCustomerCommand,
    ) -> Customer {
        Customer::builder()
            .id(CustomerId::new())
            .user_name(command.user_name().into())
            .first_name(command.first_name().into())
            .last_name(command.last_name().into())
            .build()
    }

    pub fn map_update_customer_command_to_domain_entity(
        command: UpdateCustomerCommand,
    ) -> Customer {
        Customer::builder()
            .id(CustomerId::from_uuid(command.id()))
            .user_name(command.user_name().into())
            .first_name(command.first_name().into())
            .last_name(command.last_name().into())
            .build()
    }
}
