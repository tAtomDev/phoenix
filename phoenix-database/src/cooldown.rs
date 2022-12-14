use std::fmt::Display;

use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum CooldownType {
    #[default]
    Rest
}

impl Display for CooldownType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CooldownData {
    #[serde(rename = "_id")]    pub id: ObjectId,
    #[serde(default)]           pub user_id: String,
    #[serde(default)]           pub cooldown_type: CooldownType,
    #[serde(default)]           pub expires_at: i64,
}

impl CooldownData {
    pub fn remaining_milis(&self) -> i64 {
        self.expires_at - Utc::now().timestamp_millis()
    }
 
    pub fn expired(&self) -> bool {
        self.remaining_milis() <= 0
    }
}