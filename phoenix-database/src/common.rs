use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

impl Default for Stat {
    fn default() -> Self {
        Self::new(100)
    }
}

fn default() -> i32 {
    100
}