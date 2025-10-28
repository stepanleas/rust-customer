use crate::entities::Customer;
use chrono::{DateTime, Utc};

pub struct CustomerCreatedEvent {
    customer: Customer,
    created_at: DateTime<Utc>,
}

impl CustomerCreatedEvent {
    pub fn new(customer: Customer) -> Self {
        Self {
            customer,
            created_at: Utc::now(),
        }
    }

    pub fn customer(&self) -> &Customer {
        &self.customer
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

pub struct CustomerUpdatedEvent {
    customer: Customer,
    created_at: DateTime<Utc>,
}

impl CustomerUpdatedEvent {
    pub fn new(customer: Customer) -> Self {
        Self {
            customer,
            created_at: Utc::now(),
        }
    }

    pub fn customer(&self) -> &Customer {
        &self.customer
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
