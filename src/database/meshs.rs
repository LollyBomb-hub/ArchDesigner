use diesel::{PgConnection, RunQueryDsl};
use serde::Deserialize;

use crate::schema::meshs;
use crate::models::meshs::{MeshCompleteInfoRead, MeshMinInfoRead};

use diesel::prelude::*;


#[derive(Insertable, Deserialize)]
#[diesel(table_name = meshs)]
pub struct NewMesh {
    pub account_id: i32,
    pub mesh_name: String,
    pub mesh_description: String,
    pub ply_contents: String
}

pub fn create(conn: &mut PgConnection, info: &NewMesh) -> MeshMinInfoRead {
    diesel::insert_into(meshs::table)
        .values(info)
        .returning(
            (
                meshs::mesh_id,
                meshs::account_id,
                meshs::mesh_name,
                meshs::mesh_description,
                meshs::uploaded_at,
                meshs::stars
            )
        )
        .get_result::<MeshMinInfoRead>(conn)
        .expect("Could not create mesh!")
}

pub fn read_minified(conn: &mut PgConnection, mesh_id: i32) -> MeshMinInfoRead {
    meshs::dsl::meshs.select(
        (
            meshs::mesh_id,
            meshs::account_id,
            meshs::mesh_name,
            meshs::mesh_description,
            meshs::uploaded_at,
            meshs::stars
        )
    )
        .filter(meshs::dsl::mesh_id.eq(mesh_id)).limit(1)
        .get_result::<MeshMinInfoRead>(conn)
        .expect("Could not load!")
}

pub fn read_complete(conn: &mut PgConnection, mesh_id: i32) -> MeshCompleteInfoRead {
    meshs::dsl::meshs.filter(meshs::dsl::mesh_id.eq(mesh_id)).limit(1)
        .get_result::<MeshCompleteInfoRead>(conn)
        .expect("Could not load!")
}

pub fn list_by_account_id(conn: &mut PgConnection, account_id: i32, limit: Option<i64>, offset: Option<i64>) -> Vec<MeshMinInfoRead> {
    meshs::dsl::meshs
        .select(
            (
                meshs::mesh_id,
                meshs::account_id,
                meshs::mesh_name,
                meshs::mesh_description,
                meshs::uploaded_at,
                meshs::stars
            )
        )
        .filter(meshs::account_id.eq(account_id))
        .limit(limit.unwrap_or(10i64))
        .offset(offset.unwrap_or(0i64))
        .get_results::<MeshMinInfoRead>(conn)
        .expect("Could not list meshs")
}

pub fn count_by_account_id(conn: &mut PgConnection, account_id: i32) -> i64 {
    meshs::table
        .filter(meshs::account_id.eq(account_id))
        .count()
        .get_result::<i64>(conn)
        .expect("Could not count meshs")
}