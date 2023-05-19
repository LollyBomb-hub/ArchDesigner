use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::models::projects::{ProjectInfoRead};
use crate::schema::{projects};

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub account_id: i32,
    pub project_name: String,
    pub project_description: Option<String>
}

pub fn create(conn: &mut PgConnection, new_project: &NewProject) -> ProjectInfoRead {
    diesel::insert_into(projects::table)
        .values(new_project)
        .get_result::<ProjectInfoRead>(conn)
        .expect("Could not create project!")
}

pub fn list_by_account_id(conn: &mut PgConnection, limit: Option<i64>, offset: Option<i64>, account_id: i32) -> Vec<ProjectInfoRead> {
    projects::table
        .filter(projects::account_id.eq(account_id))
        .limit(limit.unwrap_or(10i64))
        .offset(offset.unwrap_or(0i64))
        .get_results::<ProjectInfoRead>(conn)
        .expect("Could not load projects!")
}

pub fn count_by_account_id(conn: &mut PgConnection, account_id: i32) -> i64 {
    projects::table
        .filter(projects::account_id.eq(account_id))
        .count()
        .get_result::<i64>(conn)
        .expect("Could not count projects")
}