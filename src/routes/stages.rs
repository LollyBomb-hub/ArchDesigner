use rocket::serde::json::{Json, json, Value};

use crate::{models::stages::{StageInfoCreate, StageToModelLinkInfoCreate}, auth::Auth, database::{Db, stages::{NewLink, NewStage, create, list_by_account_id, assign_model, get_full_stage_info, count_by_account_id}}};

#[post("/stage", data="<stage>", format="json")]
pub async fn create_stage(auth: Auth, stage: Json<StageInfoCreate>, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    let sic = stage.into_inner();
                    create(
                        conn,
                        &NewStage {
                            account_id: auth.id,
                            project_id: sic.project_id,
                            mesh_id: sic.mesh_id,
                            position_z: sic.position_z,
                            stage_description: sic.stage_description
                        }
                    )
                }
            ).await
        )
    )
}

#[get("/stage/count")]
pub async fn count_stages(auth: Auth, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    count_by_account_id(conn, auth.id)
                }
            ).await
        )
    )
}

#[get("/stage/list?<limit>&<offset>")]
pub async fn list(auth: Auth, limit: Option<i64>, offset: Option<i64>, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    list_by_account_id(conn, auth.id, limit, offset)
                }
            ).await
        )
    )
}

#[post("/stage/<stage_id>/assign/<model_id>", data="<link_info_create>", format="json")]
pub async fn assign_model_to_stage(auth: Auth, stage_id: i32, model_id: i32, link_info_create: Json<StageToModelLinkInfoCreate>, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    let lic: StageToModelLinkInfoCreate = link_info_create.into_inner();
                    assign_model(
                        conn,
                        &NewLink {
                            model_id: model_id,
                            stage_id: stage_id,
                            account_id: auth.id,
                            link_description: lic.link_description,
                            position_x: lic.position_x,
                            position_y: lic.position_y,
                            position_z: lic.position_z,
                            rotation_x: lic.rotation_x,
                            rotation_y: lic.rotation_y,
                            rotation_z: lic.rotation_z,
                        }
                    )
                }
            ).await
        )
    )
}

#[get("/stage/<stage_id>/full/info")]
pub async fn get_full_info(auth: Auth, stage_id: i32, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    get_full_stage_info(
                        conn,
                        auth.id,
                        stage_id
                    )
                }
            ).await
        )
    )
}