use std::fmt::Display;

use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Probability(u8);

impl Probability {
    pub const fn new(probability: u8) -> Self {
        if probability < 1 {
            return Self(0);
        } else if probability > 100 {
            return Self(100);
        }

        Self(probability)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn generate_random_bool(&self) -> bool {
        let probability = self.0.clamp(0, 100) as f64 / 100f64;

        rand::thread_rng().gen_bool(probability.clamp(0f64, 1f64))
    }
}

impl Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stat {
    #[serde(default = "default")]
    pub max: i32,
    #[serde(default = "default")]
    pub value: i32
}

impl Stat {
    pub const fn new(value: i32) -> Self {
        Self {
            value,
            max: value
        }
    }

    pub fn percentage(&self) -> i32 {
        ((self.value as f32 / self.max as f32) * 100f32) as i32
    }

    pub fn restore_value(&mut self) {
        self.value = self.max;
    }

    pub fn set_value(&mut self, amount: i32) {
        self.value = amount.clamp(0, self.max);
    }

    pub fn add_value(&mut self, amount: i32) {
        self.value = (self.value + amount).min(self.max);
    }

    pub fn subtract_value(&mut self, amount: i32) {
        self.value = (self.value - amount).max(0);
    }

    pub fn add_max_value(&mut self, amount: i32) {
        self.max += amount;
        self.value += amount;
    }

    pub fn subtract_max_value(&mut self, amount: i32) {
        self.max = (self.max - amount).max(0);
    }
}

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**{}**/{}", self.value, self.max)
    }
}

impl Default for Stat {
    fn default() -> Self {
        Self::new(100)
    }
}

fn default() -> i32 {
    100
}