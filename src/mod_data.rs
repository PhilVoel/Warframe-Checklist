use std::fmt;
use serde::{de::{self, MapAccess, Visitor}, Deserialize, Deserializer};

#[derive(Debug)]
pub enum Polarity {
    Any,
    Attack,
    Defense,
    Power,
    Precept,
    Tactic,
    Umbra,
    Universal,
    Ward,
}

#[derive(Debug)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

#[derive(Debug)]
pub enum EquipmentType {
    ArchGun,
    ArchMelee,
    Archwing,
    Aura,
    HelminthCharger,
    Kavat,
    Kubrow,
    Melee,
    Parazon,
    Primary,
    Secondary,
    Sentinel,
    Stance,
    Warframe,
    None,
}

#[derive(Debug)]
pub struct ModData {
    pub unique_name: String,
    pub name: String,
    pub polarity: Polarity,
    pub rarity: Rarity,
    pub codex_secret: bool,
    pub base_drain: i8,
    pub fusion_limit: u8,
    pub exclude_from_codex: bool,
    pub is_utility: bool,
    pub compat_name: Option<String>,
    pub equipment_type: EquipmentType,
    pub description: Option<String>,
    pub subtype: Option<String>,
    pub level_stats: Option<Vec<String>>,
    pub modset: Option<String>,
    pub modset_values: Option<Vec<f32>>,
}

impl<'de> Deserialize<'de> for ModData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        struct ModDataVisitor;

        impl<'de> Visitor<'de> for ModDataVisitor {
            type Value = ModData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid Upgrades Schema")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>, {
                let mut unique_name = None;
                let mut name = None;
                let mut polarity = None;
                let mut rarity = None;
                let mut codex_secret = None;
                let mut base_drain = None;
                let mut fusion_limit = None;
                let mut exclude_from_codex = None;
                let mut is_utility = None;
                let mut compat_name = None;
                let mut equipment_type = None;
                let mut description = None;
                let mut subtype = None;
                let mut level_stats = None;
                let mut modset = None;
                let mut modset_values = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "uniqueName" => unique_name = Some(map.next_value()?),
                        "name" => name = Some(map.next_value()?),
                        "polarity" => polarity = match map.next_value()? {
                            "AP_ANY" => Some(Polarity::Any),
                            "AP_ATTACK" => Some(Polarity::Attack),
                            "AP_DEFENSE" => Some(Polarity::Defense),
                            "AP_POWER" => Some(Polarity::Power),
                            "AP_PRECEPT" => Some(Polarity::Precept),
                            "AP_TACTIC" => Some(Polarity::Tactic),
                            "AP_UMBRA" => Some(Polarity::Umbra),
                            "AP_UNIVERSAL" => Some(Polarity::Universal),
                            "AP_WARD" => Some(Polarity::Ward),
                            val => return Err(de::Error::unknown_variant(val, &["AP_ANY", "AP_ATTACK", "AP_DEFENSE", "AP_POWER", "AP_PRECEPT", "AP_TACTIC", "AP_UMBRA", "AP_UNIVERSAL", "AP_WARD"]))
                        },
                        "rarity" => rarity = match map.next_value()? {
                            "COMMON" => Some(Rarity::Common),
                            "UNCOMMON" => Some(Rarity::Uncommon),
                            "RARE" => Some(Rarity::Rare),
                            "LEGENDARY" => Some(Rarity::Legendary),
                            val => return Err(de::Error::unknown_variant(val, &["COMMON", "UNCOMMON", "RARE", "LEGENDARY"]))
                        },
                        "codexSecret" => codex_secret = Some(map.next_value()?),
                        "baseDrain" => base_drain = Some(map.next_value()?),
                        "fusionLimit" => fusion_limit = Some(map.next_value()?),
                        "excludeFromCodex" => exclude_from_codex = Some(map.next_value()?),
                        "isUtility" => is_utility = Some(map.next_value()?),
                        "compatName" => compat_name = Some(map.next_value()?),
                        "type" => equipment_type = match map.next_value()? {
                            "ARCH-GUN" => Some(EquipmentType::ArchGun),
                            "ARCH-MELEE" => Some(EquipmentType::ArchMelee),
                            "ARCHWING" => Some(EquipmentType::Archwing),
                            "AURA" => Some(EquipmentType::Aura),
                            "HELMINTH CHARGER" => Some(EquipmentType::HelminthCharger),
                            "KAVAT" => Some(EquipmentType::Kavat),
                            "KUBROW" => Some(EquipmentType::Kubrow),
                            "MELEE" => Some(EquipmentType::Melee),
                            "PARAZON" => Some(EquipmentType::Parazon),
                            "PRIMARY" => Some(EquipmentType::Primary),
                            "SECONDARY" => Some(EquipmentType::Secondary),
                            "SENTINEL" => Some(EquipmentType::Sentinel),
                            "STANCE" => Some(EquipmentType::Stance),
                            "WARFRAME" => Some(EquipmentType::Warframe),
                            "---" => Some(EquipmentType::None),
                            val => return Err(de::Error::unknown_variant(val, &["ARCH-GUN", "ARCH-MELEE", "ARCHWING", "AURA", "HELMINTH CHARGER", "KAVAT", "KUBROW", "MELEE", "PARAZON", "PRIMARY", "SECONDARY", "SENTINEL", "STANCE", "WARFRAME", "---"]))
                        },
                        "description" => description = Some(
                            map.next_value::<Vec<String>>()?
                                .first()
                                .ok_or(de::Error::custom("description array is empty"))?
                                .clone()
                        ),
                        "subtype" => subtype = Some(map.next_value()?),
                        "levelStats" => {
                            let mut extracted = Vec::new();
                            for stat in map.next_value::<Vec<serde_json::Value>>()? {
                                extracted.push(
                                    stat.get("stats")
                                        .ok_or(de::Error::missing_field("levelStats[].stats"))?
                                        .as_array()
                                        .ok_or(de::Error::custom("stats should be an array"))?
                                        .first()
                                        .ok_or(de::Error::custom("stats array is empty"))?
                                        .to_string()
                                );
                            }
                            level_stats = Some(extracted);
                        },
                        "modSet" => modset = Some(map.next_value()?),
                        "modSetValues" => modset_values = Some(map.next_value()?),
                        _ => return Err(de::Error::unknown_field(&key, &["uniqueName", "name", "polarity", "rarity", "codexSecret", "baseDrain", "fusionLimit", "excludeFromCodex", "isUtility", "compatName", "type", "description", "subtype", "levelStats", "modset", "modset_values"]))
                    }
                }

                let unique_name = unique_name.ok_or(de::Error::missing_field("uniqueName"))?;
                let name = name.ok_or(de::Error::missing_field("name"))?;
                let polarity = polarity.ok_or(de::Error::missing_field("polarity"))?;
                let rarity = rarity.ok_or(de::Error::missing_field("rarity"))?;
                let codex_secret = codex_secret.ok_or(de::Error::missing_field("codexSecret"))?;
                let base_drain = base_drain.ok_or(de::Error::missing_field("baseDrain"))?;
                let fusion_limit = fusion_limit.ok_or(de::Error::missing_field("fusionLimit"))?;
                let exclude_from_codex = exclude_from_codex.unwrap_or(false);
                let is_utility = is_utility.unwrap_or(false);
                let compat_name = compat_name.ok_or(de::Error::missing_field("compat_name"))?;
                let equipment_type = equipment_type.ok_or(de::Error::missing_field("type"))?;

                Ok(ModData{unique_name, name, polarity, rarity, codex_secret, base_drain, fusion_limit, exclude_from_codex, is_utility, compat_name, equipment_type, description, subtype, level_stats, modset, modset_values})
            }
        }

        deserializer.deserialize_map(ModDataVisitor)
    }
}
