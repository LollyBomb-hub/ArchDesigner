use rocket_sync_db_pools::{database, diesel};

#[database("diesel_postgres_pool")]
pub struct Db(diesel::pg::PgConnection);

pub mod accounts;
pub mod meshs;
pub mod models;
pub mod stages;
pub mod projects;