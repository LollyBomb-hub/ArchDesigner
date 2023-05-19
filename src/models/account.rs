use serde::Serialize;
use std::time::SystemTime;

#[derive(Queryable, Serialize)]
pub struct Account {
    pub account_id: i32,
    pub name: String,
    pub created_at: SystemTime,
    pub stars: i32,
    pub about: Option<String>
}
