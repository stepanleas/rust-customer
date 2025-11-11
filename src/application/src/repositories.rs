use domain::entities::Customer;
use mockall::automock;

#[automock]
pub trait CustomerRepository {
    fn save(&self, customer: Customer) -> anyhow::Result<Customer>;
}
