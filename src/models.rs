use crate::schema::users;
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{self, ConnectionManager},
    Queryable,
};
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbCon = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn create_db_pool(size: u32) -> DbPool {
    r2d2::Pool::builder()
        .max_size(size)
        .build(ConnectionManager::<PgConnection>::new(database_url()))
        .expect("failed to create db connection pool")
}

#[derive(Debug, Clone, Queryable)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}