use std::time::SystemTime;

use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct MeshInfoCreate {
    pub mesh_name: String,
    pub mesh_description: String,
    pub ply_contents: String
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct MeshMinInfoRead {
    pub mesh_id: i32,
    pub account_id: i32,
    pub mesh_name: String,
    pub mesh_description: String,
    pub uploaded_at: SystemTime,
    pub stars: i32
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct MeshCompleteInfoRead {
    pub mesh_id: i32,
    pub account_id: i32,
    pub mesh_name: String,
    pub mesh_description: String,
    pub ply_contents: String,
    pub uploaded_at: SystemTime,
    pub stars: i32
}