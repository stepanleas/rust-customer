#[cfg(test)]
mod tests {
    use crate::commands::{CreateCustomerCommand, UpdateCustomerCommand};
    use crate::mappers::CustomerMapper;
    use shared::domain::value_objects::CustomerId;
    use uuid::Uuid;

    #[test]
    fn test_create_customer_command_to_domain_entity() {
        let command = CreateCustomerCommand::new(
            "Artellas".to_string(),
            "John".to_string(),
            "Doe".to_string(),
        );
        let customer = CustomerMapper::map_create_customer_command_to_domain_entity(command);

        assert_ne!(customer.id().as_uuid().to_string(), Uuid::nil().to_string());
        assert_eq!(customer.user_name(), "Artellas");
        assert_eq!(customer.first_name(), "John");
        assert_eq!(customer.last_name(), "Doe");
    }

    #[test]
    fn test_update_customer_command_to_domain_entity() {
        let customer_id = CustomerId::new();
        let command = UpdateCustomerCommand::new(
            customer_id.into(),
            "Artellas".to_string(),
            "John".to_string(),
            "Doe".to_string(),
        );

        let customer = CustomerMapper::map_update_customer_command_to_domain_entity(command);

        assert_eq!(customer.id(), customer_id);
        assert_eq!(customer.user_name(), "Artellas");
        assert_eq!(customer.first_name(), "John");
        assert_eq!(customer.last_name(), "Doe");
    }
}
