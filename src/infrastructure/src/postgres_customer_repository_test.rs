#[cfg(test)]
mod tests {
    use crate::entities::CustomerEntity;
    use crate::postgres_customer_repository::PostgresCustomerRepository;
    use crate::schema::customers::dsl::customers;
    use crate::schema::customers::id;
    use crate::{DbPool, config};
    use application::repositories::CustomerRepository;
    use diesel::ExpressionMethods;
    use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
    use domain::entities::Customer;
    use domain::error::DomainError;
    use shared::domain::value_objects::CustomerId;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    struct TestContext {
        _container: testcontainers::ContainerAsync<Postgres>,
        repository: PostgresCustomerRepository,
        pool: DbPool,
    }

    async fn setup_context() -> anyhow::Result<TestContext> {
        let container = Postgres::default().start().await?;
        let port = container.get_host_port_ipv4(5432).await?;
        let url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

        let db_pool = config::configure(url).await?;
        let repository = PostgresCustomerRepository::new(&db_pool);

        Ok(TestContext {
            _container: container,
            repository,
            pool: db_pool,
        })
    }

    #[tokio::test]
    async fn test_create_customer() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let customer_id = CustomerId::new();
        let first_customer = Customer::new(
            customer_id,
            "user name".into(),
            "first name".into(),
            "last name".into(),
        );
        ctx.repository.save(first_customer)?;

        let mut connection = ctx.pool.get()?;

        let saved_customer: Customer = customers
            .filter(id.eq(customer_id.as_uuid()))
            .first::<CustomerEntity>(&mut connection)
            .optional()?
            .ok_or(DomainError::NotFound {
                message: format!("Could not find category with id: {}", customer_id.as_uuid()),
            })?
            .into();

        assert_eq!(saved_customer.id(), customer_id);
        assert_eq!(saved_customer.user_name(), "user name");
        assert_eq!(saved_customer.first_name(), "first name");
        assert_eq!(saved_customer.last_name(), "last name");

        Ok(())
    }
}
