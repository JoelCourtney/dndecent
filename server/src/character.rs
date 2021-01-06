use serde::{Deserialize, Serialize};

use crate::modify::*;
use crate::feature::{Featured, Feature};
use crate::misc::*;
use std::fmt::Debug;

#[typetag::serde]
pub trait Race: Modify + Featured + Debug {}

#[typetag::serde]
pub trait Class: Modify + Featured + Debug {}

#[typetag::serde]
pub trait Subclass: Modify + Featured + Debug {}

#[typetag::serde]
pub trait Background: Modify + Featured + Debug {}

#[typetag::serde]
pub trait Feat: Modify + Featured + Debug {}

#[typetag::serde]
pub trait Item: Modify + Featured + Debug {}

#[derive(Debug, Deserialize, Serialize)]
pub struct StoredCharacter {
    name: String,

    health: u64,
    temp_health: u64,

    base_strength: u8,
    base_dexterity: u8,
    base_constitution: u8,
    base_intelligence: u8,
    base_wisdom: u8,
    base_charisma: u8,

    alignment: Alignment,

    pub(crate) race: Box<dyn Race>
}

impl StoredCharacter {
    pub fn resolve(&mut self) -> Character {
        let mut char = Character {
            name: self.name.clone(),
            health: self.health,
            temp_health: self.temp_health,

            strength: self.base_strength,
            dexterity: self.base_dexterity,
            constitution: self.base_constitution,
            intelligence: self.base_intelligence,
            wisdom: self.base_wisdom,
            charisma: self.base_charisma,

            alignment: self.alignment,

            ..Default::default()
        };
        self.race.initialize(&mut char);
        self.race.modify(&mut char);
        self.race.finalize(&mut char);
        char.traits.extend(self.race.features());
        char
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Character<'a> {
    pub name: String,

    // HEALTH
    pub health: u64,
    pub temp_health: u64,
    pub max_health: u64,

    // ABILITIES
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,

    pub strength_modifier: i8,
    pub dexterity_modifier: i8,
    pub constitution_modifier: i8,
    pub intelligence_modifier: i8,
    pub wisdom_modifier: i8,
    pub charisma_modifier: i8,

    // SIZE
    pub size: CreatureSize,

    // ALIGNMENT
    pub alignment: Alignment,

    // PROFICIENCIES AND LANGUAGES
    pub skill_proficiencies: Vec<(Skill, ProficiencyType)>,
    pub languages: Vec<Language>,

    // SPEED
    pub walking_speed: u8,
    pub flying_speed: u8,
    pub climbing_speed: u8,
    pub swimming_speed: u8,
    pub burrowing_speed: u8,

    // FEATURES AND TRAITS
    pub traits: Vec<Feature<'a>>,
    pub features: Vec<Feature<'a>>,
}