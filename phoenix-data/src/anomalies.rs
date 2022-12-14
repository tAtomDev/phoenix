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
    anomaly_type: AnomalyType::Oozeling,
    health: Stat::new(110),
    mana: Stat::new(30),
    strength: 30,
    agility: 10,
    intelligence: 10,
};

pub const ANOMALIES: [AnomalyDefinition; 5] = [ORC, GUARDIAN, FERAK, OOZELING, NIGHTFALL];

pub fn multiplier_from_level(rng: &mut ThreadRng, factor: i32, level: i32) -> f32 {
    let roll: f32 = rng.gen();
    let base = 1.0 + (roll * (level as f32 / 100.0) * factor as f32);
    let result = base;
    
    result
}

pub fn generate_random_anomaly(player_level: i32) -> Anomaly {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..ANOMALIES.len());
    let anomaly_definition = ANOMALIES[random_index];

    let level = player_level;
    let health = Stat::new((anomaly_definition.health.max as f32 * multiplier_from_level(&mut rng, 10, level)) as i32);
    let mana = Stat::new((anomaly_definition.mana.max as f32 * multiplier_from_level(&mut rng, 5, level)) as i32);
    let strength = (anomaly_definition.strength as f32 * multiplier_from_level(&mut rng, 5, level)) as i32;
    let agility = (anomaly_definition.agility as f32 * multiplier_from_level(&mut rng, 8, level)) as i32;
    let intelligence = (anomaly_definition.intelligence as f32 * multiplier_from_level(&mut rng, 8, level)) as i32;

    let xp = (multiplier_from_level(&mut rng, 100, level) * 50f32) as i32;
    let xp_reward = (level * 100) + rng.gen_range(-xp..xp);
    let gold_reward = 10 + rng.gen_range(-5..=5) * multiplier_from_level(&mut rng, 25, level) as i32;

    Anomaly {
        definition: anomaly_definition,
        anomaly_type: anomaly_definition.anomaly_type,
        variant: None,
        health,
        mana,
        strength,
        agility,
        intelligence,
        level,
        rewards: AnomalyDrops { xp: xp_reward, gold: gold_reward }
    }
}