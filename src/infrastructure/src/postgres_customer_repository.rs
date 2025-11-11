use crate::DbPool;
use crate::entities::CustomerEntity;
use crate::schema::customers::dsl::customers;
use crate::schema::customers::id;
use application::repositories::CustomerRepository;
use diesel::RunQueryDsl;
use domain::entities::Customer;

pub struct PostgresCustomerRepository {
    pool: DbPool,
}

impl PostgresCustomerRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl CustomerRepository for PostgresCustomerRepository {
    fn save(&self, entity: Customer) -> anyhow::Result<Customer> {
        let mut connection = self.pool.get()?;

        let persistent_entity = CustomerEntity::from(entity);

        diesel::insert_into(customers)
            .values(&persistent_entity)
            .on_conflict(id)
            .do_update()
            .set(&persistent_entity)
            .execute(&mut connection)?;

        Ok(persistent_entity.into())
    }
}
