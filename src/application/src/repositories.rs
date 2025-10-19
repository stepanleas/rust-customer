use domain::Customer;
use uuid::Uuid;

pub trait CustomerRepository {
    fn find_by_id(&self, entity_id: Uuid) -> anyhow::Result<Customer>;
    fn save(&self, customer: Customer) -> anyhow::Result<Customer>;
}
