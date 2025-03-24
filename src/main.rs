use mod_data::ModData;

mod mod_data;

fn main() {
    let v = std::process::Command::new("bash").arg("get_new_data.sh").output().expect("Error");
    let deserialized: Vec<ModData> = serde_json::from_slice(&v.stdout).unwrap();
}
