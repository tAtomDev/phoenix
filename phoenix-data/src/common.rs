use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Stat {
    #[serde(default = "default")]
    pub max: i32,
    #[serde(default = "default")]
    pub value: i32
}

impl Stat {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            max: value
        }
    }

    pub fn percentage(&self) -> i32 {
        ((self.value as f32 / self.max as f32) * 100f32) as i32
    }

    pub fn add_value(&mut self, amount: i32) {
        self.value = (self.value + amount).min(self.max);
    }

    pub fn subtract_value(&mut self, amount: i32) {
        self.value = (self.value - amount).max(0);
    }

    pub fn add_max_value(&mut self, amount: i32) {
        self.max += amount;
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