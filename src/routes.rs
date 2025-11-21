use rocket::{fs::NamedFile, response::Redirect, serde::json::Json, Route};

use crate::{db, mod_data::ModData};

#[get("/new_list")]
fn new_list() -> Redirect {
    let id = db::new_list();
    Redirect::to(uri!(list_html(id)))
}

#[get("/list/<_id>")]
async fn list_html(_id: &str) -> NamedFile {
    NamedFile::open("frontend/static/list.html").await.unwrap()
}

#[get("/list/<id>/data")]
fn list_data(id: &str) -> Json<Vec<(String, Vec<u8>)>> {
    Json(db::get_mods(id))
}

#[put("/list/<id>/data", data="<mods>")]
fn add_rank(id: &str, mods: Json<Vec<(&str, u8)>>) {
    for (mod_name, rank) in mods.into_inner() {
        db::add_mod_rank(id, mod_name, rank);
    }
}

#[delete("/list/<id>/data", data="<mods>")]
fn remove_rank(id: &str, mods: Json<Vec<(&str, u8)>>) {
    for (mod_name, rank) in mods.into_inner() {
        db::remove_mod_rank(id, mod_name, rank);
    }
}

#[get("/mods")]
fn get_mods() -> Json<Vec<ModData>> {
    Json(crate::MODS.read().unwrap().clone())
}

pub fn create_dynamic_routes() -> Vec<Route> {
    routes![new_list, list_html, list_data, add_rank, remove_rank, get_mods]
}
