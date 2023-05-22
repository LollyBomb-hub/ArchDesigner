use crate::{schema::models, models::models::{ModelCompleteInfoRead, ModelMinifiedInfoRead}};

use diesel::{self, PgConnection, RunQueryDsl, ExpressionMethods, prelude::{QueryDsl}, BoolExpressionMethods};

#[derive(Insertable)]
#[diesel(table_name = models)]
pub struct NewModel {
    pub account_id: i32,
    pub model_name: String,
    pub model_description: String,
    pub model_ifc: String
}


pub fn create(conn: &mut PgConnection, info: &NewModel) -> ModelMinifiedInfoRead {
    diesel::insert_into(models::table)
        .values(info)
        .returning(
            (
                models::model_id,
                models::account_id,
                models::model_name,
                models::model_description,
                models::uploaded_at,
                models::stars
            )
        )
        .get_result::<ModelMinifiedInfoRead>(conn)
        .expect("Error creating model")
}

pub fn list_minified_by_account_id(conn: &mut PgConnection, account_id: i32, limit: Option<i64>, offset: Option<i64>) -> Vec<ModelMinifiedInfoRead> {
    models::dsl::models.select(
            (
                models::model_id,
                models::account_id,
                models::model_name,
                models::model_description,
                models::uploaded_at,
                models::stars
            )
        )
        .filter(models::dsl::account_id.eq(account_id)).limit(limit.unwrap_or(10i64))
        .offset(offset.unwrap_or(0i64))
        .get_results::<ModelMinifiedInfoRead>(conn)
        .expect("Could not load models!")
}

pub fn get_model_complete(conn: &mut PgConnection, account_id: i32, model_id: i32) -> ModelCompleteInfoRead {
    models::dsl::models.filter(models::dsl::account_id.eq(account_id).and(models::dsl::model_id.eq(model_id))).get_result::<ModelCompleteInfoRead>(conn).expect("Error loading data!")
}

pub fn get_model_ifc(conn: &mut PgConnection, account_id: i32, model_id: i32) -> String {
    models::dsl::models.select(models::model_ifc).filter(models::account_id.eq(account_id).and(models::model_id.eq(model_id))).get_result::<String>(conn).expect("Could not get ifc model")
}