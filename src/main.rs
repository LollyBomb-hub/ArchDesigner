#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

extern crate rocket_sync_db_pools;

extern crate rocket_cors;
use rocket_cors::{Cors, CorsOptions};

#[macro_use]
extern crate diesel;
extern crate uuid;

#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;
use rocket::{Build, Rocket};

mod auth;
mod config;
mod database;
mod errors;
mod models;
mod routes;
mod schema;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    CorsOptions::default()
        .allowed_origins(rocket_cors::AllOrSome::All)
        .to_cors()
        .expect("Cors fairing cannot be created")
}

#[launch]
pub fn rocket() -> Rocket<Build> {
    println!("Configuration!");
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api",
            routes![
                routes::accounts::create_account,
                routes::accounts::list_accounts,
                routes::accounts::get_account_by_id,
                routes::models::create_model,
                routes::models::list_models,
                routes::models::get_complete_model,
                routes::models::get_ifc_model,
                routes::meshs::create_mesh,
                routes::meshs::list_meshs,
                routes::meshs::get_minified_mesh,
                routes::meshs::get_complete_mesh,
                routes::meshs::get_ply_mesh,
                routes::stages::create_stage,
                routes::stages::list,
                routes::stages::assign_model_to_stage,
                routes::stages::get_full_info,
                routes::projects::create_project,
                routes::projects::list_projects,
                routes::projects::count_projects
            ],
        )
        .mount(
            "/",
            routes![
                routes::login::authorize
            ],
        )
        .attach(database::Db::fairing())
        .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}