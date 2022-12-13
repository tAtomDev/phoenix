#![allow(unused)]
mod fighter;
pub mod util;

pub use fighter::Fighter;
use format as f;

use crate::{
    discord::{
        embed::{EmbedAuthor, EmbedBuilder},
        extensions::UserExtension,
    },
    util::Color,
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

impl From<Round> for EmbedBuilder {
    fn from(round: Round) -> Self {
        Self::new()
            .set_author(EmbedAuthor {
                name: f!("{} usou {}", round.fighter.name, round.action.name()),
                icon_url: round.fighter.user.map(|u| u.avatar_url()),
            })
            .set_color(Color::LIGHT_ORANGE)
            .set_description(round.messages.join("\n"))
            .set_current_timestamp()
    }
}

pub struct BattleResult {
    winner: Fighter,
    defeated_fighters: Vec<Fighter>,
    battle: Battle,
}

#[derive(Debug, Clone)]
pub struct Battle {
    fighters: Vec<Fighter>,
    current_fighter: usize,
    winner: Option<Fighter>,
    rounds: Vec<Round>,
}

impl Battle {
    pub fn new(fighters: Vec<Fighter>) -> Self {
        Self {
            current_fighter: 0,
            fighters,
            winner: None,
            rounds: Vec::new(),
        }
    }

    pub fn next_fighter(&self) -> usize {
        if self.current_fighter == self.fighters.len() - 1 {
            0
        } else {
            self.current_fighter + 1
        }
    }

    pub fn current_fighter(&self) -> Option<&Fighter> {
        self.fighters.get(self.current_fighter)
    }

    pub fn current_fighter_mut(&mut self) -> Option<&mut Fighter> {
        self.fighters.get_mut(self.current_fighter)
    }

    pub fn target_fighter(&self) -> Option<&Fighter> {
        self.fighters.get(self.next_fighter())
    }

    pub fn target_fighter_mut(&mut self) -> Option<&mut Fighter> {
        let next_fighter = self.next_fighter();
        self.fighters.get_mut(next_fighter)
    }

    pub fn run_action(&mut self, action: Action) -> Round {
        let round = match action {
            Action::Attack => {
                let fighter = self.current_fighter().unwrap().clone();
                let target = self.target_fighter_mut().unwrap();

                let damage = fighter.calculate_damage();

                target.take_damage(damage);

                Round {
                    messages: vec![f!(
                        "**{}** atacou **{}** com um golpe simples, que causou **{}** de dano.",
                        fighter.name,
                        target.name,
                        damage
                    )],
                    action,
                    fighter: self.current_fighter().unwrap().clone(),
                }
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
