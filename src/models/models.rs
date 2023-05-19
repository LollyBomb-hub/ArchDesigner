use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::schema::models;

#[derive(Deserialize, Serialize)]
pub struct ModelInfoCreate {
    pub model_name: String,
    pub model_description: String
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct ModelMinifiedInfoRead {
    pub model_id: i32,
    pub account_id: i32,
    pub model_name: String,
    pub model_description: String,
    pub uploaded_at: SystemTime,
    pub stars: i32
}

#[derive(Queryable, Selectable, Identifiable, Deserialize, Serialize)]
#[diesel(primary_key(model_id))]
#[diesel(table_name=models)]
pub struct ModelCompleteInfoRead {
    pub model_id: i32,
    pub account_id: i32,
    pub model_name: String,
    pub model_description: String,
    pub model_ifc: String,
    pub uploaded_at: SystemTime,
    pub stars: i32
}