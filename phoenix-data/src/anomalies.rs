use std::fmt::Display;

use rand::{Rng, rngs::ThreadRng};

use crate::{Stat, Emoji};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnomalyType {
    Guardian,
    Orc,
    Ferak,
    Oozeling,
    Nightfall,
}

impl AnomalyType {
    const fn name(&self) -> &'static str {
        match self {
            AnomalyType::Guardian => "Guardião",
            AnomalyType::Orc => "Orc",
            AnomalyType::Ferak => "Ferak",
            AnomalyType::Oozeling => "Oozeling",
            AnomalyType::Nightfall => "Nightfall",
        }
    }

    const fn image(&self) -> &'static str {
        match self {
            AnomalyType::Guardian => "https://i.imgur.com/wWkkOJX.png",
            AnomalyType::Orc => "https://i.imgur.com/wIfjahq.png",
            AnomalyType::Ferak => "https://i.imgur.com/sNnkwxi.png",
            AnomalyType::Oozeling => "https://i.imgur.com/hro2X3W.png",
            AnomalyType::Nightfall => "https://i.imgur.com/ddX6CCu.png"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnomalyVariant {
    Ghost,
    Giant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnomalyDefinition {
    pub anomaly_type: AnomalyType,
    pub health: Stat,
    pub mana: Stat,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnomalyDrops {
    pub xp: i32,
    pub gold: i32
}

impl Display for AnomalyDrops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} **Ouro**: {}\n{} **XP**: {}",
            Emoji::Gold,
            self.gold,
            Emoji::Experience,
            self.xp
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Anomaly {
    pub definition: AnomalyDefinition,
    pub anomaly_type: AnomalyType,
    pub variant: Option<AnomalyVariant>,
    pub health: Stat,
    pub mana: Stat,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub level: i32,
    pub rewards: AnomalyDrops
}

impl Anomaly {
    pub fn name(&self) -> &'static str {
        self.anomaly_type.name()
    }

    pub fn image(&self) -> &'static str {
        self.anomaly_type.image()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ORC: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Orc,
    health: Stat::new(80),
    mana: Stat::new(10),
    strength: 20,
    agility: 5,
    intelligence: 3,
};

pub const GUARDIAN: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Guardian,
    health: Stat::new(80),
    mana: Stat::new(40),
    strength: 12,
    agility: 5,
    intelligence: 10,
};

pub const FERAK: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Ferak,
    health: Stat::new(60),
    mana: Stat::new(5),
    strength: 30,
    agility: 5,
    intelligence: 1,
};

pub const OOZELING: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Oozeling,
    health: Stat::new(60),
    mana: Stat::new(15),
    strength: 10,
    agility: 15,
    intelligence: 2,
};

pub const NIGHTFALL: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Nightfall,
    health: Stat::new(110),
    mana: Stat::new(30),
    strength: 30,
    agility: 10,
    intelligence: 10,
};

pub const ANOMALIES: [AnomalyDefinition; 5] = [ORC, GUARDIAN, FERAK, OOZELING, NIGHTFALL];

pub fn factor(rng: &mut ThreadRng, level: i32, extra_factor: f32) -> f32 {
    let level = level + 1;
    (level + (level / 2)) as f32 * 0.5 * rng.gen_range(0.3..1.25) * (extra_factor / 2.0)
}

pub fn calculate_potency(rng: &mut ThreadRng, health: f32, mana: f32, strength: f32, agility: f32, intelligence: f32) -> f32 {
    (health * 1.1 + mana * 1.1 + (strength + strength / 1.5f32)) + (agility + intelligence + rng.gen_range(1f32..5f32)) / rng.gen_range(6.85..8.0)
}


pub fn generate_random_anomaly(player_level: i32) -> Anomaly {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..ANOMALIES.len());
    let def = ANOMALIES[random_index];

    let level = ((player_level + 1) as f32 * rng.gen_range(0.7..1.3)) as i32;
    let health = Stat::new((factor(&mut rng, level, 1.3) * def.health.max as f32) as i32);
    let mana = Stat::new((factor(&mut rng, level, 1.2) * def.mana.max as f32) as i32);
    let strength = (factor(&mut rng, level, 1.3) * def.strength as f32) as i32;
    let agility = (factor(&mut rng, level, 1.25) * def.agility as f32) as i32;
    let intelligence = (factor(&mut rng, level, 1.0) * def.intelligence as f32) as i32;

    let potency = calculate_potency(&mut rng, health.max as f32, mana.max as f32, strength as f32, agility as f32, intelligence as f32);
    let value = ((potency * rng.gen_range(0.9..1.1)) as i32).max(1);

    let xp_reward = rng.gen_range(3..7) * level * (value / 5);
    let gold_reward = rng.gen_range(1..3) * level * (value / 9);

    Anomaly {
        definition: def,
        anomaly_type: def.anomaly_type,
        variant: None,
        health,
        mana,
        strength: strength.max(1),
        agility: agility.max(1),
        intelligence: intelligence.max(1),
        level: level.max(1),
        rewards: AnomalyDrops { xp: xp_reward, gold: gold_reward }
    }
}