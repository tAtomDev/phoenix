use data::Stat;
use database::user_model::UserData;
use rand::Rng;
use twilight_model::user::User as DiscordUser;

use crate::{commands::prelude::DynamicError, util::math};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fighter {
    pub name: String,
    pub user: Option<DiscordUser>,
    pub target_index: Option<usize>,
    pub health: Stat,
    pub mana: Stat,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
}

impl Fighter {
    pub fn create_from_user_data(
        discord_user: DiscordUser,
        user: UserData,
    ) -> Result<Self, DynamicError> {
        Ok(Self {
            name: discord_user.name.clone(),
            user: Some(discord_user),
            target_index: None,
            health: user.health,
            mana: user.mana,
            strength: user.strength,
            agility: user.agility,
            intelligence: user.intelligence,
        })
    }

    pub fn calculate_damage(&self, critical: bool) -> i32 {
        let mut rng = rand::thread_rng();

        let mut multiplier = rng.gen_range(0.8..=1.2f32);

        if critical {
            multiplier += rng.gen_range(0.75f32..1.2f32);
        }

        (self.strength as f32 * multiplier) as i32
    }

    pub fn calculate_dodge_chance(&self, other: &Fighter) -> i32 {
        math::calculate_dodge_chance(self.agility, other.agility)
    }

    pub fn calculate_critical_chance(&self, other: &Fighter) -> i32 {
        math::calculate_critical_chance(self.intelligence, other.intelligence)
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health.subtract_value(damage);
    }
}
