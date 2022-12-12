use database::{common::Stat, user_model::User};
use rand::Rng;
use twilight_model::user::User as DiscordUser;

use crate::commands::prelude::DynamicError;

#[derive(Debug, Clone)]
pub struct Fighter {
    pub name: &'static str,
    pub user: Option<DiscordUser>,
    pub health: Stat,
    pub mana: Stat,
    pub strength: i32,
}

impl Fighter {
    pub fn create_from_user_data(
        username: &'static str,
        user: User,
        discord_user: DiscordUser,
    ) -> Result<Self, DynamicError> {
        Ok(Self {
            name: username,
            user: Some(discord_user),
            health: user.health,
            mana: user.mana,
            strength: user.strength,
        })
    }

    pub fn calculate_damage(&self) -> i32 {
        let mut rng = rand::thread_rng();

        let multiplier = rng.gen_range(0.8..=1.2f32);

        (self.strength as f32 * multiplier) as i32
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health.subtract_value(damage);
    }
}
