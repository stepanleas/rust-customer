pub mod config;
pub mod entities;
pub mod postgres_customer_repository;
mod postgres_customer_repository_test;
mod schema;

use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, r2d2};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
