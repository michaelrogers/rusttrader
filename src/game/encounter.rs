// Space encounter system (pirates, police, traders)

use crate::types::GameState;

pub enum EncounterType {
    None,
    Trader,
    Pirate,
    Police,
    SpaceMonster,
}

pub fn check_for_encounter(game_state: &GameState) -> EncounterType {
    // TODO: Implement encounter probability logic
    // Based on police_record_score, reputation, difficulty, etc.
    EncounterType::None
}

pub fn execute_encounter(game_state: &mut GameState, encounter_type: EncounterType) {
    match encounter_type {
        EncounterType::None => {},
        EncounterType::Trader => {
            println!("You encounter a trader!");
        },
        EncounterType::Pirate => {
            println!("Pirates attack!");
        },
        EncounterType::Police => {
            println!("Police scan your ship!");
        },
        EncounterType::SpaceMonster => {
            println!("A space monster appears!");
        },
    }
}
