use std::fmt::Display;

use rand::{rngs::ThreadRng, Rng};
use serde::{Serialize, Deserialize};

use crate::{Emoji, Stat, regions::RegionType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyType {
    Treant,
    Orc,
    Vexbug,
    Guardian,
    Ferak,
    Oozeling,
    Nightfall,
    Timberwraith,
}

impl AnomalyType {
    const fn name(&self) -> &'static str {
        match self {
            AnomalyType::Treant => "Treant",
            AnomalyType::Orc => "Orc",
            AnomalyType::Vexbug => "Vexbug",
            AnomalyType::Timberwraith => "Timberwraith",
            AnomalyType::Guardian => "Guardião",
            AnomalyType::Ferak => "Ferak",
            AnomalyType::Oozeling => "Oozeling",
            AnomalyType::Nightfall => "Nightfall",
        }
    }

    const fn image(&self) -> &'static str {
        match self {
            AnomalyType::Treant => "https://i.imgur.com/QuWuU0j.png",
            AnomalyType::Orc => "https://i.imgur.com/wIfjahq.png",
            AnomalyType::Vexbug => "https://i.imgur.com/PWpbL8r.png",
            AnomalyType::Guardian => "https://i.imgur.com/wWkkOJX.png",
            AnomalyType::Ferak => "https://i.imgur.com/sNnkwxi.png",
            AnomalyType::Oozeling => "https://i.imgur.com/hro2X3W.png",
            AnomalyType::Nightfall => "https://i.imgur.com/ddX6CCu.png",
            AnomalyType::Timberwraith => "https://i.imgur.com/w6xnlJf.png",
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
    pub valid_regions: &'static [RegionType]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnomalyDrops {
    pub xp: i32,
    pub gold: i32,
}

impl Display for AnomalyDrops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} **Ouro**: {}\n{} **XP**: {}",
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
    pub rewards: AnomalyDrops,
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

pub const TREANT: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Treant,
    health: Stat::new(60),
    mana: Stat::new(10),
    strength: 10,
    agility: 1,
    intelligence: 3,
    valid_regions: &[RegionType::Grassland]
};

pub const ORC: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Orc,
    health: Stat::new(100),
    mana: Stat::new(10),
    strength: 20,
    agility: 5,
    intelligence: 3,
    valid_regions: &[RegionType::Forest]
};

pub const VEXBUG: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Vexbug,
    health: Stat::new(60),
    mana: Stat::new(15),
    strength: 10,
    agility: 3,
    intelligence: 15,
    valid_regions: &[RegionType::Forest]
};

pub const GUARDIAN: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Guardian,
    health: Stat::new(80),
    mana: Stat::new(40),
    strength: 12,
    agility: 5,
    intelligence: 10,
    valid_regions: &[RegionType::Forest]
};

pub const FERAK: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Ferak,
    health: Stat::new(60),
    mana: Stat::new(5),
    strength: 30,
    agility: 5,
    intelligence: 1,
    valid_regions: &[RegionType::Swamp, RegionType::Forest]
};

pub const OOZELING: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Oozeling,
    health: Stat::new(60),
    mana: Stat::new(15),
    strength: 10,
    agility: 15,
    intelligence: 2,
    valid_regions: &[RegionType::Grassland]
};

pub const NIGHTFALL: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Nightfall,
    health: Stat::new(110),
    mana: Stat::new(30),
    strength: 30,
    agility: 10,
    intelligence: 10,
    valid_regions: &[RegionType::Grassland]
};

pub const TIMBERWRAITH: AnomalyDefinition = AnomalyDefinition {
    anomaly_type: AnomalyType::Timberwraith,
    health: Stat::new(225),
    mana: Stat::new(50),
    strength: 20,
    agility: 15,
    intelligence: 10,
    valid_regions: &[RegionType::Forest]
};

pub const ANOMALIES: [AnomalyDefinition; 8] = [
    TREANT,
    ORC,
    VEXBUG,
    GUARDIAN,
    FERAK,
    OOZELING,
    NIGHTFALL,
    TIMBERWRAITH,
];

pub fn get_anomaly_from_type(anomaly: AnomalyType) -> Option<Anomaly> {
    let anomaly = ANOMALIES.iter().copied().find(|a| a.anomaly_type == anomaly)?;

    Some(Anomaly {
        definition: anomaly,
        anomaly_type: anomaly.anomaly_type,
        variant: None,
        health: anomaly.health,
        mana: anomaly.mana,
        strength: anomaly.strength,
        agility: anomaly.agility,
        intelligence: anomaly.intelligence,
        level: 1,
        rewards: AnomalyDrops {
            xp: 0,
            gold: 0,
        },
    })
}

pub fn factor(level: i32, factor: f32) -> i32 {
    let level = level as f32;
    ((level * factor) as i32).max(1)
}

pub fn calculate_potency(
    rng: &mut ThreadRng,
    health: f32,
    mana: f32,
    strength: f32,
    agility: f32,
    intelligence: f32,
) -> f32 {
    (health * 1.1 + mana * 1.1 + (strength + strength / 1.5f32))
        + (agility + intelligence + rng.gen_range(2f32..7f32)) 
        / rng.gen_range(10.0..12.0)
}

pub fn generate_random_anomaly(player_level: i32, region_type: RegionType) -> Anomaly {
    let valid_anomalies: Vec<&AnomalyDefinition> = ANOMALIES.iter().filter(|a| a.valid_regions.contains(&region_type)).collect();

    let rng = &mut rand::thread_rng();
    let random_index = rng.gen_range(0..valid_anomalies.len());
    let def = valid_anomalies[random_index];

    let level = (player_level as f32 * rng.gen_range(0.8..1.3)).max(1.0) as i32;
    let health = Stat::new((factor(level, 1.3) * def.health.max) as i32);
    let mana = Stat::new((factor(level, 1.3) * def.mana.max) as i32);
    let strength = (factor(level, 1.3) * def.strength) as i32;
    let agility = (factor(level, 1.3) * def.agility) as i32;
    let intelligence = (factor(level, 1.3) * def.intelligence) as i32;

    let potency = calculate_potency(
        rng,
        health.max as f32,
        mana.max as f32,
        strength as f32,
        agility as f32,
        intelligence as f32,
    );
    let value = ((potency * rng.gen_range(0.08..0.1)) as i32).max(1);

    let reward_multiplier = (level as f32 / 5.0) + value as f32;
    let xp_reward = rng.gen_range(5..7) * reward_multiplier as i32;
    let gold_reward = rng.gen_range(2..3) * (reward_multiplier * 0.8) as i32;

    Anomaly {
        definition: *def,
        anomaly_type: def.anomaly_type,
        variant: None,
        health,
        mana,
        strength: strength.max(1),
        agility: agility.max(1),
        intelligence: intelligence.max(1),
        level: level.max(1),
        rewards: AnomalyDrops {
            xp: xp_reward,
            gold: gold_reward,
        },
    }
}
