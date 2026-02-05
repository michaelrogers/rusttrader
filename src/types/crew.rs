// Crew member data structures
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewMember {
    pub name: MercenaryName,
    pub pilot_skill: i32,
    pub fighter_skill: i32,
    pub trader_skill: i32,
    pub engineer_skill: i32,
    pub current_system: usize,
}

impl CrewMember {
    pub fn commander(name: &str, pilot: i32, fighter: i32, trader: i32, engineer: i32) -> Self {
        Self {
            name: MercenaryName::Custom(name.to_string()),
            pilot_skill: pilot,
            fighter_skill: fighter,
            trader_skill: trader,
            engineer_skill: engineer,
            current_system: 0,
        }
    }
    
    pub fn total_skills(&self) -> i32 {
        self.pilot_skill + self.fighter_skill + self.trader_skill + self.engineer_skill
    }
}

/// Mercenary names from the original game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MercenaryName {
    Alyssa,
    Armatur,
    Bentos,
    C2U2,
    ChiTi,
    Crystal,
    Dane,
    Deirdre,
    Doc,
    Drax,
    Jeremiah,
    Jujubal,
    Krydon,
    Luis,
    Mercedez,
    MiriamA,
    MiriamB,
    Muri,
    Mystyc,
    Nandi,
    Orestes,
    Pancho,
    PS37,
    Quarck,
    Sosumi,
    Uma,
    Wesley,
    Wonton,
    Yorvick,
    Zeethibal,
    Custom(String),
}

impl MercenaryName {
    pub fn as_str(&self) -> &str {
        match self {
            MercenaryName::Alyssa => "Alyssa",
            MercenaryName::Armatur => "Armatur",
            MercenaryName::Bentos => "Bentos",
            MercenaryName::C2U2 => "C2U2",
            MercenaryName::ChiTi => "Chi'Ti",
            MercenaryName::Crystal => "Crystal",
            MercenaryName::Dane => "Dane",
            MercenaryName::Deirdre => "Deirdre",
            MercenaryName::Doc => "Doc",
            MercenaryName::Drax => "Drax",
            MercenaryName::Jeremiah => "Jeremiah",
            MercenaryName::Jujubal => "Jujubal",
            MercenaryName::Krydon => "Krydon",
            MercenaryName::Luis => "Luis",
            MercenaryName::Mercedez => "Mercedez",
            MercenaryName::MiriamA => "Miriam",
            MercenaryName::MiriamB => "Miriam",
            MercenaryName::Muri => "Muri",
            MercenaryName::Mystyc => "Mystyc",
            MercenaryName::Nandi => "Nandi",
            MercenaryName::Orestes => "Orestes",
            MercenaryName::Pancho => "Pancho",
            MercenaryName::PS37 => "PS37",
            MercenaryName::Quarck => "Quarck",
            MercenaryName::Sosumi => "Sosumi",
            MercenaryName::Uma => "Uma",
            MercenaryName::Wesley => "Wesley",
            MercenaryName::Wonton => "Wonton",
            MercenaryName::Yorvick => "Yorvick",
            MercenaryName::Zeethibal => "Zeethibal",
            MercenaryName::Custom(name) => name,
        }
    }
}
