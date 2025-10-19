mod config;
mod customer_postgres_repository;
mod entities;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub use config::configure;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, r2d2};

pub use customer_postgres_repository::PostgresCustomerRepository;
