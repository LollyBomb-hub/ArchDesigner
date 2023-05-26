use rocket::{Data, response};
use rocket::response::status::BadRequest;
use rocket::serde::json::{json, Value, from_str};

use rocket::http::{ContentType};

use crate::{auth::Auth, models::models::ModelInfoCreate, database::Db};
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition};
use crate::database::models::{create, get_model_complete, get_model_ifc, list_minified_by_account_id, count_by_account_id, NewModel};

#[post("/model", data="<data>")]
pub async fn create_model(auth: Auth, content_type: &ContentType, data: Data<'_>, db: Db) -> Result<Value, BadRequest<&'static str>> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            MultipartFormDataField::text("info_create").repetition(Repetition::fixed(1)),
            MultipartFormDataField::raw("ifc_file").size_limit(1024*1024*128).repetition(Repetition::fixed(1))
        ]
    );
    let multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();
    let info_create = multipart_form_data.texts.get("info_create");
    if info_create.is_none() {
        // return status::BadRequest(Some("Info create not provided!"))
        return Result::Err(response::status::BadRequest("Info create not passed".into()));
    }
    let ifc_file = multipart_form_data.raw.get("ifc_file");
    if ifc_file.is_none() {
        return Result::Err(response::status::BadRequest("Ifc file not passed".into()));
    }
    let unwrapped_ifc_file = ifc_file.unwrap();
    let unwrapped_info_create = info_create.unwrap();
    let parsed_info_create = from_str::<ModelInfoCreate>(unwrapped_info_create[0].text.as_str()).expect("Could not parse info create!");
    let ifc_file_contents = String::from_utf8(unwrapped_ifc_file[0].raw.to_owned()).expect("Could not get String instance");
    Ok(
        json!(
            db.run(
                move |conn| {
                    create(conn, &NewModel {
                        account_id: auth.id,
                        model_name: parsed_info_create.model_name,
                        model_description: parsed_info_create.model_description,
                        model_ifc: ifc_file_contents
                    })
                }
            ).await
        )
    )
}

#[get("/model/count")]
pub async fn count_models(auth: Auth, db: Db) -> Result<Value, ()> {
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

#[get("/model/list?<limit>&<offset>")]
pub async fn list_models(auth: Auth, limit: Option<i64>, offset: Option<i64>, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    list_minified_by_account_id(conn, auth.id, limit, offset)
                }
            ).await
        )
    )
}

#[get("/model/<model_id>")]
pub async fn get_complete_model(auth: Auth, model_id: i32, db: Db) -> Result<Value, ()> {
    Ok(
        json!(
            db.run(
                move |conn| {
                    get_model_complete(conn, auth.id, model_id)
                }
            ).await
        )
    )
}

#[get("/model/ifc/<model_id>")]
pub async fn get_ifc_model(auth: Auth, model_id: i32, db: Db) -> Result<String, ()> {
    Ok(
        db.run(
            move |conn| {
                get_model_ifc(conn, auth.id, model_id)
            }
        ).await
    )
}

// #[get("/model/ifc/web-ifc.wasm")]