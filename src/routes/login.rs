use chrono::prelude::*;
use rocket::serde::json::{Json};
use serde::Deserialize;

use crate::database::accounts::{get_account_by_auth_data};

use crate::auth::Auth;
use crate::database::Db;
use crate::errors::{FieldValidator, Errors};
use std::env;

#[derive(Deserialize, Validate)]
pub struct AccountAuth {
    #[validate(length(min = 1))]
    username: String,
    #[validate(length(min = 1))]
    password: String,
}

#[post("/auth", data = "<account_auth_j>", format = "json")]
pub async fn authorize(
    account_auth_j: Json<AccountAuth>,
    db: Db,
) -> Result<String, Errors> {
    let account_auth = account_auth_j.into_inner();
    FieldValidator::validate(&account_auth).check()?;
    let AccountAuth {username, password} = account_auth;
    let entities = db.run(
        move |conn| {
            get_account_by_auth_data(conn, &username, &password)
        }
    ).await;
    if entities.len() != 1 {
        return Err(
            Errors::new(&[("Entity not found!", "Account not found!")])
        )
    }
    let account = &entities[0];
    Ok(
        Auth {
            exp: 60 * 60 + Utc::now().timestamp(),
            id: account.account_id,
            username: account.name.to_string(),
        }.token(env::var("SECRET_KEY").unwrap_or("".to_string()).as_bytes())
    )
}
