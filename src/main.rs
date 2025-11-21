use std::{collections::HashMap, process::Command, sync::{Arc, LazyLock, RwLock}};

use mlua::{prelude::{Lua, LuaTable, LuaValue}, LuaSerdeExt};
use mod_data::ModData;
use rocket::fs::FileServer;
use tokio::spawn;
use tokio_schedule::{every, Job};

#[macro_use] extern crate rocket;

mod db;
mod mod_data;
mod routes;

static MODS: LazyLock<Arc<RwLock<Vec<ModData>>>> = LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));

async fn update_mods() {
    let src = Command::new("curl").arg("https://wiki.warframe.com/w/Module:Mods/data?action=raw").output().expect("Error").stdout;
    let src = std::str::from_utf8(&src).unwrap();
    let lua = Lua::new();
    let table: LuaTable = lua.load(src).eval::<LuaTable>().unwrap().get("Mods").unwrap();

    let mut to_remove = Vec::new();
    for pair in table.pairs::<String, LuaValue>() {
        let key = pair.unwrap().0;
        if key.contains("Fusion Core") || key.contains("Legendary Core") || key.contains("Riven") || key.contains("Transmute Core") {
            to_remove.push(key);
        }
    }
    for key in to_remove {
        table.raw_remove(key).unwrap();
    }

    *MODS.write().unwrap() = lua.from_value::<HashMap<String, ModData>>(mlua::Value::Table(table)).unwrap().into_values().collect();
}

#[launch]
async fn rocket() -> _ {
    db::init();

    update_mods().await;

    let schedule = every(20).minutes().perform(update_mods);
    spawn(schedule);

    rocket::build()
        .mount("/", FileServer::from("frontend/static"))
        .mount("/", routes::create_dynamic_routes())
}
