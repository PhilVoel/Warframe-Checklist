use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, mlua::FromLua, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    Any,
    Core,
    Madurai,
    Naramon,
    None,
    Penjaga,
    Umbra,
    Unairu,
    Universal,
    Vazarin,
    Zenurik,
}

#[derive(Serialize, Deserialize, mlua::FromLua, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

#[derive(Serialize, Deserialize, mlua::FromLua, Clone, Debug)]
pub struct ModData {
    // #[serde(rename(deserialize = "BaseDrain"), default)]
    // pub base_drain: i8,
    // #[serde(rename(deserialize = "Description"))]
    // pub description: String,
    // #[serde(rename(deserialize = "Icon"))]
    // pub icon: String,
    #[serde(rename(deserialize = "Image"))]
    pub image: String,
    #[serde(rename(deserialize = "Link"))]
    pub link: String,
    #[serde(rename(deserialize = "MaxRank"))]
    pub max_rank: u8,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    // #[serde(rename(deserialize = "Rarity"))]
    // pub rarity: Rarity,
    // #[serde(rename(deserialize = "Polarity"))]
    // pub polarity: Polarity,
    // #[serde(rename(deserialize = "InternalName"))]
    // pub unique_name: Option<String>,
}
