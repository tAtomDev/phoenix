#![allow(unused)]

use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassType {
    Knight,
    Mage,
    Assassin
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterClass {
    pub name: &'static str,
    pub emoji: &'static str,
    pub description: &'static str,
    pub class_type: ClassType,
    pub health: i32,
    pub strength: i32,
    pub mana: i32,
    pub agility: i32,
    pub intelligence: i32,
}

pub const KNIGHT: CharacterClass = CharacterClass {
    name: "Cavaleiro",
    emoji: "⚔️",
    description: "Um cavaleiro é forte e resistente",
    class_type: ClassType::Knight,
    health: 100,
    strength: 20,
    mana: 10,
    agility: 5,
    intelligence: 5,
};

pub const MAGE: CharacterClass = CharacterClass {
    name: "Mago",
    emoji: "🪄",
    description: "Um mago é mestre das artes místicas",
    class_type: ClassType::Mage,
    health: 80,
    strength: 5,
    mana: 50,
    agility: 8,
    intelligence: 15,
};

pub const ASSASSIN: CharacterClass = CharacterClass {
    name: "Assassino",
    emoji: "🗡️",
    description: "O assassino é um guerreiro sombrio e letal",
    class_type: ClassType::Assassin,
    health: 60,
    strength: 15,
    mana: 15,
    agility: 15,
    intelligence: 10,
};

pub const ALL_CLASSES: [CharacterClass; 3] = [KNIGHT, MAGE, ASSASSIN];

pub fn get_class_by_name(name: String) -> Option<CharacterClass> {
    ALL_CLASSES.iter().find(|c| c.name == name).copied()
}

pub fn get_class_by_type(class_type: ClassType) -> Option<CharacterClass> {
    ALL_CLASSES.iter().find(|c| c.class_type == class_type).copied()
}