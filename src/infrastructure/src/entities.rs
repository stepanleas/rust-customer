use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use domain::entities::Customer;
use shared::domain::value_objects::CustomerId;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset, PartialEq, Debug)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct CustomerEntity {
    id: Uuid,
    user_name: String,
    first_name: String,
    last_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Customer> for CustomerEntity {
    fn from(customer: Customer) -> Self {
        Self {
            id: customer.id().into(),
            user_name: customer.user_name().into(),
            first_name: customer.first_name().into(),
            last_name: customer.last_name().into(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<CustomerEntity> for Customer {
    fn from(entity: CustomerEntity) -> Self {
        Customer::builder()
            .id(CustomerId::from_uuid(entity.id))
            .user_name(entity.user_name)
            .first_name(entity.first_name)
            .last_name(entity.last_name)
            .build()
    }
}
