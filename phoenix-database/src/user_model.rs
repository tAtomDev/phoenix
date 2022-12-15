use data::classes::ClassType;
use mongodb::bson::oid::ObjectId;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use data::Stat;

const fn default_class() -> ClassType {
    ClassType::Knight
}
const fn default_gold() -> i32 {
    10
}
const fn default_journey() -> f32 {
    0.0
}
const fn default_strength() -> i32 {
    20
}
const fn default_agi_intel() -> i32 {
    5
}
const fn default_xp() -> i32 {
    0
}
const fn default_level() -> i32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(default)]
    pub user_id: String,
    #[serde(default = "default_class")]
    pub class: ClassType,
    #[serde(default = "default_gold")]
    pub gold: i32,
    #[serde(default)]
    pub health: Stat,
    #[serde(default)]
    pub mana: Stat,
    #[serde(default = "default_journey")]
    pub journey: f32,
    #[serde(default = "default_strength")]
    pub strength: i32,
    #[serde(default = "default_agi_intel")]
    pub agility: i32,
    #[serde(default = "default_agi_intel")]
    pub intelligence: i32,
    #[serde(default = "default_xp")]
    pub xp: i32,
    #[serde(default = "default_level")]
    pub level: i32,
}

impl UserData {
    pub fn new(user_id: String, class: ClassType) -> Self {
        Self {
            id: ObjectId::new(),
            user_id,
            class,
            ..Default::default()
        }
    }

    pub fn add_xp(&mut self, amount: i32) {
        self.xp += amount
    }

    // Returns the new User's level or None if not leveled up
    pub fn level_up(&mut self) -> Option<i32> {
        if self.xp < util::math::calculate_xp_required_for_level_up(self.level) {
            return None;
        }

        let rng = &mut rand::thread_rng();

        let mut attributes_points = 2;

        let (lower_range, upper_range) = ((self.level / 3).max(1), (self.level / 2).max(2));

        while self.xp >= util::math::calculate_xp_required_for_level_up(self.level) {
            self.xp -= util::math::calculate_xp_required_for_level_up(self.level);
            self.level += 1;
            attributes_points += rng.gen_range(lower_range..=upper_range);
        }

        while attributes_points > 0 {
            let upgrades: Vec<Box<dyn Fn(&mut UserData, i32) -> ()>> = vec![
                Box::new(UserData::add_max_health),
                Box::new(UserData::add_max_mana),
                Box::new(UserData::add_strength),
                Box::new(UserData::add_intelligence),
                Box::new(UserData::add_agility),
            ];

            let Some(upgrade) = upgrades.choose(rng) else {
                break;
            };

            upgrade(self, 1);
            attributes_points -= 1;
        }

        Some(self.level)
    }

    pub fn add_gold(&mut self, amount: i32) {
        self.gold += amount
    }

    pub fn remove_gold(&mut self, amount: i32) {
        self.gold = (self.gold - amount).max(0)
    }

    pub fn restore_health(&mut self) {
        self.health.restore_value()
    }

    pub fn add_health(&mut self, amount: i32) {
        self.health.add_value(amount)
    }

    pub fn set_health(&mut self, amount: i32) {
        self.health.set_value(amount)
    }

    pub fn remove_health(&mut self, amount: i32) {
        self.health.subtract_value(amount)
    }

    pub fn restore_mana(&mut self) {
        self.mana.restore_value()
    }

    pub fn add_mana(&mut self, amount: i32) {
        self.mana.add_value(amount)
    }

    pub fn set_mana(&mut self, amount: i32) {
        self.mana.set_value(amount)
    }

    pub fn remove_mana(&mut self, amount: i32) {
        self.mana.subtract_value(amount)
    }

    pub fn add_max_health(&mut self, amount: i32) {
        self.health.add_max_value(amount)
    }

    pub fn add_max_mana(&mut self, amount: i32) {
        self.mana.add_max_value(amount)
    }

    pub fn add_strength(&mut self, amount: i32) {
        self.strength += amount;
    }

    pub fn add_intelligence(&mut self, amount: i32) {
        self.intelligence += amount;
    }

    pub fn add_agility(&mut self, amount: i32) {
        self.agility += amount;
    }
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            id: ObjectId::new(),
            user_id: "".into(),
            class: default_class(),
            gold: default_gold(),
            health: Stat::new(100),
            mana: Stat::new(20),
            journey: default_journey(),
            strength: default_strength(),
            agility: default_agi_intel(),
            intelligence: default_agi_intel(),
            xp: default_xp(),
            level: default_level(),
        }
    }
}
