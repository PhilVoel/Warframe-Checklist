use std::{cell::LazyCell, collections::HashMap, env, path::PathBuf};

use uuid::Uuid;

const DB_FILE: LazyCell<PathBuf> = LazyCell::new(|| {
    let dir = env::var("WF_STATE_DIR").unwrap_or("/var/lib/wf-checklist/".to_string());
    PathBuf::from(dir).join("wf-checklist.db")
});

pub(crate) fn init() {
    let connection = sqlite::open(&*DB_FILE).unwrap();
    connection.execute("CREATE TABLE IF NOT EXISTS lists(id TEXT PRIMARY KEY)").unwrap();
    connection.execute("CREATE TABLE IF NOT EXISTS mods(
        list_id TEXT NOT NULL,
        mod TEXT NOT NULL,
        rank INTEGER,
        PRIMARY KEY(list_id, mod, rank),
        FOREIGN KEY(list_id) REFERENCES lists(id) ON DELETE CASCADE
    )").unwrap();
}

pub(crate) fn new_list() -> String {
    let connection = sqlite::open(&*DB_FILE).unwrap();
    loop {
        let id = Uuid::new_v4().simple().to_string();
        let mut statement = connection.prepare("INSERT INTO lists VALUES (?)").unwrap();
        statement.bind((1, id.as_str())).unwrap();
        match statement.next() {
            Ok(_) => return id,
            Err(_) => continue,
        }
    }
}

pub(crate) fn get_mods(id: &str) -> Vec<(String, Vec<u8>)> {
    let connection = sqlite::open(&*DB_FILE).unwrap();
    let mut map: HashMap<String, Vec<u8>> = HashMap::new();
    for row in connection.prepare("SELECT mod, rank FROM mods WHERE list_id = ?").unwrap()
        .into_iter()
        .bind((1, id)).unwrap()
    {
        let row = row.unwrap();
        let (name, rank) = (row.read::<&str, _>("mod").to_string(), row.read::<i64, _>("rank") as u8);
        map.entry(name).or_default().push(rank);
    }
    map.into_iter().collect()
}

pub(crate) fn add_mod_rank(id: &str, mod_name: &str, rank: u8) {
    let connection = sqlite::open(&*DB_FILE).unwrap();
    let mut statement = connection.prepare("INSERT INTO mods(list_id, mod, rank) VALUES (:id, :mod_name, :rank) ON CONFLICT IGNORE").unwrap();
    statement.bind((":id", id)).unwrap();
    statement.bind((":mod_name", mod_name)).unwrap();
    statement.bind((":rank", rank as i64)).unwrap();
}

pub(crate) fn remove_mod_rank(id: &str, mod_name: &str, rank: u8) {
    let connection = sqlite::open(&*DB_FILE).unwrap();
    let mut statement = connection.prepare("REMOVE FROM mods WHERE list_id=:id AND mod=:mod_name AND rank=:rank").unwrap();
    statement.bind((":id", id)).unwrap();
    statement.bind((":mod_name", mod_name)).unwrap();
    statement.bind((":rank", rank as i64)).unwrap();
}
