use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use diesel::prelude::*;


#[derive(Deserialize, Serialize)]
pub struct ProjectInfoCreate {
    pub project_name: String,
    pub project_description: Option<String>
}


#[derive(Queryable, Deserialize, Serialize)]
#[diesel(table_name = projects)]
pub struct ProjectInfoRead {
    pub project_id: i32,
    pub account_id: i32,
    pub project_name: String,
    pub project_description: Option<String>,
    pub created_at: SystemTime,
    pub stars: i32
}