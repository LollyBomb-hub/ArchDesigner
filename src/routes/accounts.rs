use crate::database::Db;
use crate::auth::Auth;
use crate::database::accounts::{create, list, get_account};
use serde::Deserialize;
use rocket::serde::json::{json, Json, Value};
use crate::errors::{Errors, FieldValidator};

#[derive(Deserialize, Validate)]
pub struct NewAccount {
    #[validate(length(min = 1))]
    name: String,
    #[validate(length(min = 1))]
    password: String
}

#[post("/account", data="<account>", format="json")]
pub async fn create_account(account: Json<NewAccount>, db: Db) -> Result<Value, Errors> {
    let create_info = account.into_inner();
    let extractor = FieldValidator::validate(&create_info);
    extractor.check()?;
    let created = db.run(
        move |conn| {
            create(conn, &create_info.name, &create_info.password)
        }
    ).await;
    Ok(json!(created))
}

#[get("/account/list?<limit>")]
pub async fn list_accounts(_auth: Auth, limit: Option<i64>, db: Db) -> Result<Value, Errors> {
    let list = db.run(
        move |conn| {
            list(conn, limit)
        }
    ).await;

    Ok(json!(list))
}

#[get("/account")]
pub async fn get_account_by_id(auth: Auth, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    get_account(conn, auth.id)
                }
            ).await
        )
    )
}