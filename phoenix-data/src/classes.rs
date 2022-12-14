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
    description: "Fiéis guerreiros que sempre se sacrificavam para proteger seu povo na linha de frente, independente se sua chance de sucesso fosse alta e baixa, o que importava para eles era a chama de sua deusa Phoenix que dava esperança para continuar lutando. Se você deseja honrar o sacrifício de sua deusa sobre o Sol e reacender a sua força, o cavaleiro é a escolha certa.",
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
    description: "Um grupo de pessoas que buscavam a forma mais segura e inteligente de resolver as ações. Eram a principal fonte de conhecimento que buscava estudar as anomalias do Sol e ajudar Phoenix a garantir a segurança do povo. Se você deseja reacender tal inteligência e estudar esse novo mundo com seus conhecimentos místico, o mago é a escolha certa.",
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
    description: "Sempre independentes, buscam suas próprias soluções de forma rápida e letal. Podem não parecer tão leais a Phoenix, mas possuem uma chama escondida que deseja o bem do povo. Caso seu objetivo seja aniquilar as anomalias desse mundo da forma mais rápida possível, o assasino é a melhor escolha.",
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