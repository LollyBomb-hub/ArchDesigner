use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::schema::{stages, model_to_stage_link};

use super::models::ModelCompleteInfoRead;

use diesel::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct StageInfoCreate {
    pub project_id: i32,
    pub mesh_id: i32,
    pub stage_description: Option<String>,
    pub position_z: f64
}

#[derive(Queryable, Selectable, Identifiable, Deserialize, Serialize)]
#[diesel(primary_key(stage_id))]
#[diesel(table_name = stages)]
pub struct StageInfoRead {
    pub stage_id: i32,
    pub project_id: i32,
    pub mesh_id: i32,
    pub account_id: i32,
    pub stage_description: Option<String>,
    pub position_z: f64,
    pub created_at: SystemTime,
    pub stars: i32
}

#[derive(Deserialize, Serialize)]
pub struct StageToModelLinkInfoCreate {
    pub link_description: Option<String>,
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: Option<f64>,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Deserialize, Serialize)]
#[diesel(primary_key(link_id))]
#[diesel(belongs_to(StageInfoRead, foreign_key = stage_id))]
#[diesel(belongs_to(ModelCompleteInfoRead, foreign_key = model_id))]
#[diesel(table_name = model_to_stage_link)]
pub struct StageToModelLinkInfoRead {
    pub link_id: i32,
    pub model_id: i32,
    pub stage_id: i32,
    pub account_id: i32,
    pub created_at: SystemTime,
    pub link_description: Option<String>,
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: Option<f64>,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
}


#[derive(Queryable, Serialize, Deserialize)]
pub struct PositionedCompleteModelInfoRead {
    pub model_id: i32,
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: Option<f64>,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
    pub model_name: String,
    pub model_description: String,
    pub model_ifc: String
}


#[derive(Serialize, Deserialize)]
pub struct CompleteStageInfoRead {
    pub account_id: i32,
    pub stage_id: i32,
    pub mesh_id: i32,
    pub models: Vec<PositionedCompleteModelInfoRead>
}