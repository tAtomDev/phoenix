use data::Probability;

pub fn calculate_dodge_chance(agility_a: i32, agility_b: i32) -> Probability {
    let ratio = (agility_a + 1) as f32 / (agility_b + 1) as f32;

    Probability::new((ratio * 1.5 * 5.0) as u8)
}

pub fn calculate_critical_chance(intelligence_a: i32, intelligence_b: i32) -> Probability {
    let ratio = (intelligence_a + 1) as f32 / (intelligence_b + 1) as f32;

    Probability::new((ratio * 1.4 * 5.5) as u8)
}

pub fn calculate_xp_required_for_level_up(level: i32) -> i32 {
    (100f32 * (level as f32 * 1.5)) as i32
}
