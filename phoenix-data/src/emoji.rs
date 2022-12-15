use std::fmt::Display;

pub enum Emoji {
    Gold,
    Health,
    Mana,
    Strength,
    Intelligence,
    Agility,
    Experience,
}

impl Emoji {
    pub const fn to_str(&self) -> &'static str {
        match self {
            Emoji::Gold => "🪙",
            Emoji::Health => "❤️",
            Emoji::Mana => "🌀",
            Emoji::Intelligence => "🧠",
            Emoji::Strength => "💪",
            Emoji::Agility => "🪶",
            Emoji::Experience => "🔹",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl From<Emoji> for String {
    fn from(emoji: Emoji) -> Self {
        emoji.to_str().to_string()
    }
}

impl Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
