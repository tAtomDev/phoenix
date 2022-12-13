use data::classes::ClassType;
use mongodb::bson::{oid::ObjectId};
use serde::{Serialize, Deserialize};

use data::Stat;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: String,
    pub class: ClassType,
    #[serde(default = "default_gold")]
    pub gold: i32,
    pub health: Stat,
    pub mana: Stat,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    #[serde(default = "default_xp")]
    pub xp: i32,
    #[serde(default = "default_level")]
    pub level: i32
}

impl UserData {
    pub fn new(user_id: String, class: ClassType) -> Self {
        Self {
            id: ObjectId::new(),
            user_id,
            class,
            gold: default_gold(),
            health: Stat::new(100),
            mana: Stat::new(20),
            strength: 20,
            agility: 5,
            intelligence: 5,
            xp: 0,
            level: 1
        }
    }
}

const fn default_gold() -> i32 {
    10
}

const fn default_xp() -> i32 {
    0
}

const fn default_level() -> i32 {
    1
}