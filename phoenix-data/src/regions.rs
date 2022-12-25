use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{Probability, common};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionType {
    #[default]
    Forest,
    City,
    Swamp,
    Grassland,
}

pub const REGIONS: [RegionType; 4] = [
    RegionType::City, RegionType::Swamp, RegionType::Grassland, RegionType::Forest
];

const COMMON_NAMES: [&'static str; 15] = [
    "Negr$", "Verde", "Bel$", "Encantad$", "Sombri$", "Mágic$", "Seren$", "Sagrad$", "Milenar", "Superior", "Lendári$",
    "Fúnebre", "Espetacular", "Fabulos$", "Cristalin$"
];

const ADJECTIVES: [&'static str; 6] = ["Incrível", "dos Sonhos Perdidos", "Inesperad$", "Espectral", "Encantad$", "Místic$"];

const LOCATIONS: [&'static str; 9] = ["do Norte", "do Sul", "do Leste", "do Oeste", "do Noroeste", "do Nordeste", "do Sudeste", "do Sudoeste", "Central"];

impl RegionType {
    pub const fn emoji(&self) -> &'static str {
        match self {
            RegionType::City => "🏙️",
            RegionType::Forest => "🌲",
            RegionType::Swamp => "🍀",
            RegionType::Grassland => "🏞️",
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            RegionType::City => "Cidade",
            RegionType::Forest => "Floresta",
            RegionType::Swamp => "Pântano",
            RegionType::Grassland => "Planície",
        }
    }

    pub fn generate_specific_name(&self) -> String {
        let rng = &mut rand::thread_rng();
        let mut title: Option<String> = None;
        if self == &RegionType::City || rng.gen_bool(0.3) {
            title = Some(
                common::invent_word(
                    rand::thread_rng().gen_range(1..=4)
                ).unwrap_or("UNKNOWN".to_string())
            );
        } 

        let mut string = self.name().to_string();
        if let Some(title) = title {
            string.push_str(format!(" {}", title).as_str());

            if rng.gen_bool(0.95) {
                return string;
            }
        }

        let first_suffix = COMMON_NAMES.to_vec();
        let second_suffix = if rng.gen_bool(0.5) { Some(ADJECTIVES.to_vec()) } else { None };
        let last_suffix = if rng.gen_bool(0.3) { Some(LOCATIONS.to_vec()) } else { None };

        let pronoun_letter = match *self {
            RegionType::Forest | RegionType::Grassland | RegionType::City => "a",
            _ => "o"
        };

        common::generate_name(&string, first_suffix, second_suffix, last_suffix)
            .unwrap_or(string)
            .replace("$", pronoun_letter)
    }
    
    pub const fn rarity(&self) -> Probability {
        match self {
            RegionType::City => Probability::new(0),
            RegionType::Swamp => Probability::new(30),
            _ => Probability::new(50)
        }
    }
}