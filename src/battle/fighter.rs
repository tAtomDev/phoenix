use data::{anomalies::Anomaly, Emoji, Probability, Stat};
use database::user_model::UserData;
use rand::Rng;
use twilight_model::user::User as DiscordUser;

use super::{ActionType, Battle};
use crate::commands::prelude::{DynamicError, UserExtension};
use util::math;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fighter {
    pub name: String,
    pub user: Option<DiscordUser>,
    pub anomaly: Option<Anomaly>,
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
            anomaly: None,
            target_index: None,
            health: user.health,
            mana: user.mana,
            strength: user.strength,
            agility: user.agility,
            intelligence: user.intelligence,
        })
    }

    pub fn create_from_anomaly(anomaly: Anomaly) -> Result<Self, DynamicError> {
        Ok(Self {
            name: anomaly.name().to_string(),
            user: None,
            anomaly: Some(anomaly),
            target_index: None,
            health: anomaly.health,
            mana: anomaly.mana,
            strength: anomaly.strength,
            agility: anomaly.agility,
            intelligence: anomaly.intelligence,
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

    pub fn calculate_dodge_chance(&self, other: &Fighter) -> Probability {
        math::calculate_dodge_chance(self.agility, other.agility)
    }

    pub fn calculate_critical_chance(&self, other: &Fighter) -> Probability {
        math::calculate_critical_chance(self.intelligence, other.intelligence)
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health.subtract_value(damage);
    }

    pub fn image(&self) -> String {
        if let Some(anomaly) = self.anomaly {
            anomaly.image().into()
        } else {
            self.user
                .clone()
                .map(|u| u.avatar_url())
                .unwrap_or("https://i.imgur.com/Kl2qRLF.png".into())
        }
    }

    pub fn display_full_stats(&self) -> String {
        format!(
            "{} Vida: {} (`{}%`)\n{} Mana: {} (`{}%`)\n{} Força: {}\n{} Inteligência: `{}`\n{} Agilidade: `{}`",
            Emoji::Health, self.health, self.health.percentage(),
            Emoji::Mana, self.mana, self.mana.percentage(),
            Emoji::Strength, self.strength,
            Emoji::Intelligence, self.intelligence,
            Emoji::Agility, self.agility
        )
    }

    pub fn display_full_stats_with_target(&self, target: &Fighter) -> String {
        format!(
            "{} Vida: {} (`{}%`)\n{} Mana: {} (`{}%`)\n{} Força: {}\nChance de crítico: `{}`\nChance de esquiva: `{}`",
            Emoji::Health, self.health, self.health.percentage(),
            Emoji::Mana, self.mana, self.mana.percentage(),
            Emoji::Strength, self.strength,
            self.calculate_critical_chance(target),
            self.calculate_dodge_chance(target)
        )
    }

    pub fn choose_action_type(&self, battle: &Battle) -> ActionType {
        let _fighter = battle.current_fighter();
        let _target = battle.target_fighter();

        ActionType::Attack
    }
}
