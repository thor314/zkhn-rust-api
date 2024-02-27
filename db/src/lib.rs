#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use models::user::User;
use uuid::Uuid as Uid;

pub mod error;
pub mod models;
pub mod schema;
#[cfg(test)] mod tests;
mod utils;


pub type DbPool = Pool<AsyncPgConnection>;

pub async fn get_user_from_id(db_pool: &DbPool, id: Uid) -> Option<User> {
    todo!()
}
