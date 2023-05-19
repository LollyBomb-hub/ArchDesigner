use rocket::serde::json::{Json, json, Value};
use crate::auth::Auth;
use crate::models::projects::{ProjectInfoCreate};
use crate::database::{Db, projects::{NewProject, create, list_by_account_id, count_by_account_id}};

#[post("/project", data="<project_info_create>", format="json")]
pub async fn create_project(auth: Auth, project_info_create: Json<ProjectInfoCreate>, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    let pic: ProjectInfoCreate = project_info_create.into_inner();
                    create(
                        conn,
                        &NewProject {
                            account_id: auth.id,
                            project_name: pic.project_name,
                            project_description: pic.project_description
                        }
                    )
                }
            ).await
        )
    )
}

#[get("/project/list?<limit>&<offset>")]
pub async fn list_projects(auth: Auth, limit: Option<i64>, offset: Option<i64>, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    list_by_account_id(
                        conn,
                        limit,
                        offset,
                        auth.id
                    )
                }
            ).await
        )
    )
}

#[get("/project/count")]
pub async fn count_projects(auth: Auth, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    count_by_account_id(
                        conn,
                        auth.id
                    )
                }
            ).await
        )
    )
}