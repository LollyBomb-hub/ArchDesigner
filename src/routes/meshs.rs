use rocket::serde::json::{json, Json, Value};

use crate::models::meshs::MeshInfoCreate;
use crate::{auth::Auth, database::Db};
use crate::database::meshs::{create, read_complete, read_minified, list_by_account_id, NewMesh};

#[post("/mesh", data="<mesh_info_create>", format="json")]
pub async fn create_mesh(auth: Auth, mesh_info_create: Json<MeshInfoCreate>, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    let inner_info_create: MeshInfoCreate = mesh_info_create.into_inner();
                    create(conn, &NewMesh {
                        account_id: auth.id,
                        mesh_name: inner_info_create.mesh_name,
                        mesh_description: inner_info_create.mesh_description,
                        ply_contents: inner_info_create.ply_contents
                    }
                )
                }
            ).await
        )
    )
}

#[get("/mesh/list?<limit>&<offset>")]
pub async fn list_meshs(auth: Auth, limit: Option<i64>, offset: Option<i64>, db: Db) -> Result<Value, ()> {
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

#[get("/mesh/<mesh_id>")]
pub async fn get_minified_mesh(_auth: Auth, mesh_id: i32, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    read_minified(conn, mesh_id)
                }
            ).await
        )
    )
}

#[get("/mesh/complete/<mesh_id>")]
pub async fn get_complete_mesh(_auth: Auth, mesh_id: i32, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    read_complete(conn, mesh_id)
                }
            ).await
        )
    )
}

#[get("/mesh/ply/<mesh_id>")]
pub async fn get_ply_mesh(_auth: Auth, mesh_id: i32, db: Db) -> Result<String, ()> {
    Ok(
        db.run(
            move |conn| {
                read_complete(conn, mesh_id)
            }
        ).await.ply_contents
    )
}