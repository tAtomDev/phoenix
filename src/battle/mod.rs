#![allow(unused)]
mod fighter;
pub mod util;

use std::f32::consts::E;

use ::util::Color;
pub use fighter::Fighter;
use format as f;
use rand::Rng;

use crate::{
    commands::prelude::DynamicError,
    discord::{
        embed::{EmbedAuthor, EmbedBuilder},
        extensions::UserExtension,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Attack,
}

pub const ACTIONS: [Action; 1] = [Action::Attack];

impl Action {
    fn from_name(name: &str) -> Option<Action> {
        ACTIONS.iter().copied().find(|a| a.name() == name)
    }

    fn emoji(&self) -> &'static str {
        match self {
            Action::Attack => "👊",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Action::Attack => "Atacar",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Round {
    messages: Vec<String>,
    fighter: Fighter,
    action: Action,
}

impl Round {
    fn new_with_message(fighter: Fighter, action: Action, message: impl Into<String>) -> Self {
        Self {
            messages: vec![message.into()],
            action,
            fighter,
        }
    }
}

impl From<Round> for EmbedBuilder {
    fn from(round: Round) -> Self {
        Self::new()
            .set_author(EmbedAuthor {
                name: f!("{} usou {}", round.fighter.name, round.action.name()),
                icon_url: Some(round.fighter.image()),
            })
            .set_thumbnail(round.fighter.image())
            .set_color(Color::LIGHT_ORANGE)
            .set_description(round.messages.join("\n"))
            .set_current_timestamp()
    }
}

pub struct BattleResult {
    pub winner: Fighter,
    pub defeated_fighters: Vec<Fighter>,
    pub all_fighters: Vec<Fighter>,
    pub battle: Battle,
}

#[derive(Debug, Clone)]
pub struct Battle {
    fighters: Vec<Fighter>,
    current_fighter: usize,
    winner: Option<Fighter>,
    rounds: Vec<Round>,
}

impl Battle {
    pub fn new(mut fighters: Vec<Fighter>) -> Result<Self, DynamicError> {
        let len = fighters.len();
        if len < 2 {
            Err("a")?;
        }

        let fighters: Vec<Fighter> = fighters
            .iter_mut()
            .enumerate()
            .map(|(i, fighter)| {
                fighter.target_index = Some((i + 1) % len);
                fighter.to_owned()
            })
            .collect();

        Ok(Self {
            current_fighter: 0,
            fighters,
            winner: None,
            rounds: Vec::new(),
        })
    }

    pub fn next_fighter(&self) -> usize {
        (self.current_fighter + 1) % self.fighters.len()
    }

    pub fn current_fighter(&self) -> &Fighter {
        self.fighters.get(self.current_fighter).unwrap()
    }

    pub fn current_fighter_mut(&mut self) -> &mut Fighter {
        self.fighters.get_mut(self.current_fighter).unwrap()
    }

    pub fn target_fighter(&self) -> &Fighter {
        let &Fighter { target_index: Some(index), .. } = self.current_fighter() else {
            panic!("Battle needs two or more fighters with a valid target_index");
        };

        self.fighters.get(index).unwrap()
    }

    pub fn target_fighter_mut(&mut self) -> &mut Fighter {
        let &Fighter { target_index: Some(index), .. } = self.current_fighter() else {
            panic!("Battle needs two or more fighters with a valid target_index");
        };

        self.fighters.get_mut(index).unwrap()
    }

    pub fn run_action(&mut self, action: Action) -> Round {
        let fighter = self.current_fighter().clone();
        let target = self.target_fighter_mut();

        let dodged = target
            .calculate_dodge_chance(&fighter)
            .generate_random_bool();
        let critical = fighter
            .calculate_critical_chance(target)
            .generate_random_bool();

        let round = match action {
            Action::Attack => {
                let damage = if dodged {
                    0
                } else {
                    fighter.calculate_damage(critical)
                };

                #[rustfmt::skip]
                let mut round = Round::new_with_message(fighter.clone(), action, f!(
                    "**{}** atacou **{}** com um golpe simples, que causou **{}** de dano.{}",
                    fighter.name, target.name, damage,
                    if critical { "\n(**ACERTO CRÍTICO!** 💥)" } else { "" }
                ));

                if dodged {
                    round.messages.push(f!("🪶 **{}** esquivou!", target.name));
                } else {
                    target.take_damage(damage);
                }

                round
            }
        };

        let alive_fighters: Vec<Fighter> = self
            .fighters
            .iter()
            .cloned()
            .filter(|f| f.health.value > 0)
            .collect();

        if alive_fighters.len() == 1 {
            self.winner = alive_fighters.first().cloned();
        }

        self.rounds.push(round.clone());

        self.current_fighter = self.next_fighter();

        round
    }
}
