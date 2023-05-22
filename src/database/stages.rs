use diesel::{PgConnection, RunQueryDsl, ExpressionMethods, QueryDsl, BoolExpressionMethods};

use crate::{schema::{stages, projects, model_to_stage_link, models}, models::{stages::{StageInfoRead, StageToModelLinkInfoRead, PositionedCompleteModelInfoRead, CompleteStageInfoRead}}};

#[derive(Insertable)]
#[diesel(table_name = stages)]
pub struct NewStage {
    pub project_id: i32,
    pub mesh_id: i32,
    pub account_id: i32,
    pub stage_description: Option<String>,
    pub position_z: f64
}

pub fn create(conn: &mut PgConnection, info: &NewStage) -> StageInfoRead {
    projects::table.select(projects::project_id).find(info.project_id).first::<i32>(conn).expect("Not found project by id");
    diesel::insert_into(stages::table)
        .values(info)
        .get_result::<StageInfoRead>(conn)
        .expect("Could not create stage!")
}


pub fn list_by_account_id(conn: &mut PgConnection, account_id: i32, limit: Option<i64>, offset: Option<i64>) -> Vec<StageInfoRead> {
    stages::dsl::stages.filter(stages::account_id.eq(account_id)).limit(limit.unwrap_or(10i64)).offset(offset.unwrap_or(0i64)).get_results::<StageInfoRead>(conn).expect("Error fetching stages by account id")
}

#[derive(Insertable)]
#[diesel(table_name = model_to_stage_link)]
pub struct NewLink {
    pub model_id: i32,
    pub stage_id: i32,
    pub account_id: i32,
    pub link_description: Option<String>,
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: Option<f64>,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
}

pub fn assign_model(conn: &mut PgConnection, info: &NewLink) -> StageToModelLinkInfoRead {
    models::table.select(models::model_id).find(info.model_id).first::<i32>(conn).expect("Could not get model");
    stages::table.select(stages::stage_id).find(info.stage_id).first::<i32>(conn).expect("Could not get stage");
    diesel::insert_into(model_to_stage_link::table)
        .values(info)
        .get_result::<StageToModelLinkInfoRead>(conn)
        .expect("Error creating link!")
}

pub fn get_full_stage_info(conn: &mut PgConnection, account_id: i32, stage_id: i32) -> CompleteStageInfoRead {
    let stage_info: (i32, i32, i32) = stages::table
        .select((stages::account_id, stages::stage_id, stages::mesh_id)).filter(
            stages::stage_id.eq(stage_id).and(stages::account_id.eq(account_id))
        ).get_result::<(i32, i32, i32)>(conn)
        .expect("could not load given stage");
    
    let full_models: Vec<PositionedCompleteModelInfoRead> = model_to_stage_link::table
        .inner_join(models::table)
        .select(
            (
                model_to_stage_link::model_id,
                model_to_stage_link::position_x,
                model_to_stage_link::position_y,
                model_to_stage_link::position_z,
                model_to_stage_link::rotation_x,
                model_to_stage_link::rotation_y,
                model_to_stage_link::rotation_z,
                models::model_name,
                models::model_description,
                models::model_ifc
            )
        )
        .filter(model_to_stage_link::stage_id.eq(stage_id).and(model_to_stage_link::account_id.eq(account_id)))
        .get_results::<PositionedCompleteModelInfoRead>(conn)
        .expect("Could not get models");
    
    CompleteStageInfoRead {
        account_id: stage_info.0,
        stage_id: stage_info.1,
        mesh_id: stage_info.2,
        models: full_models
    }
}