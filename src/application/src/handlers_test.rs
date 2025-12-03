#[cfg(test)]
mod tests {
    use crate::commands::{CreateCustomerCommand, UpdateCustomerCommand};
    use crate::handlers::{CreateCustomerCommandHandler, UpdateCustomerCommandHandler};
    use crate::ports::output::publishers::MockCustomerMessagePublisher;
    use crate::repositories::MockCustomerRepository;
    use shared::domain::value_objects::CustomerId;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_customer_command_handler_execute() -> anyhow::Result<()> {
        let mut mock_repository = MockCustomerRepository::new();
        let mut mock_message_publisher = MockCustomerMessagePublisher::new();

        mock_repository
            .expect_save()
            .once()
            .withf(move |customer| {
                customer.first_name() == "John"
                    && customer.last_name() == "Doe"
                    && customer.user_name() == "Artellas"
            })
            .returning(Ok);

        mock_message_publisher
            .expect_publish_created()
            .once()
            .withf(move |event| {
                event.customer().first_name() == "John"
                    && event.customer().last_name() == "Doe"
                    && event.customer().user_name() == "Artellas"
            })
            .returning(|_| Ok(()));

        let command = CreateCustomerCommand::new(
            "Artellas".to_string(),
            "John".to_string(),
            "Doe".to_string(),
        );

        let handler = CreateCustomerCommandHandler::new(
            Arc::new(mock_repository),
            Arc::new(mock_message_publisher),
        );

        let result = handler.execute(command).await?;

        assert_eq!(result.first_name(), "John");
        assert_eq!(result.last_name(), "Doe");
        assert_eq!(result.user_name(), "Artellas");

        Ok(())
    }

    #[tokio::test]
    async fn test_update_customer_command_handler_execute() -> anyhow::Result<()> {
        let mut mock_repository = MockCustomerRepository::new();
        let mut mock_message_publisher = MockCustomerMessagePublisher::new();

        let customer_id = Uuid::new_v4();

        mock_repository
            .expect_save()
            .once()
            .withf(move |customer| {
                customer.id() == CustomerId::from_uuid(customer_id)
                    && customer.first_name() == "John"
                    && customer.last_name() == "Doe"
                    && customer.user_name() == "Artellas"
            })
            .returning(Ok);

        mock_message_publisher
            .expect_publish_updated()
            .once()
            .withf(move |event| {
                event.customer().id() == CustomerId::from_uuid(customer_id)
                    && event.customer().first_name() == "John"
                    && event.customer().last_name() == "Doe"
                    && event.customer().user_name() == "Artellas"
            })
            .returning(|_| Ok(()));

        let command = UpdateCustomerCommand::new(
            customer_id,
            "Artellas".to_string(),
            "John".to_string(),
            "Doe".to_string(),
        );

        let handler = UpdateCustomerCommandHandler::new(
            Arc::new(mock_repository),
            Arc::new(mock_message_publisher),
        );

        let result = handler.execute(command).await?;

        assert_eq!(result.first_name(), "John");
        assert_eq!(result.last_name(), "Doe");
        assert_eq!(result.user_name(), "Artellas");

        Ok(())
    }
}
