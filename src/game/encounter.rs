// Space encounter system (pirates, police, traders)

use crate::types::GameState;

#[derive(Clone, Debug, PartialEq)]
pub enum EncounterType {
    None,
    Trader,
    Pirate,
    Police,
    SpaceMonster,
}

#[derive(Clone, Debug)]
pub struct Encounter {
    pub encounter_type: EncounterType,
    pub description: String,
    pub ship_name: String,
    pub distance_clicks: i32,
    pub system_name: String,
}

impl Encounter {
    pub fn new(
        encounter_type: EncounterType,
        description: String,
        ship_name: String,
        distance_clicks: i32,
        system_name: String,
    ) -> Self {
        Encounter {
            encounter_type,
            description,
            ship_name,
            distance_clicks,
            system_name,
        }
    }
    
    pub fn get_color_rgb(&self) -> (u8, u8, u8) {
        match self.encounter_type {
            EncounterType::Trader => (0, 100, 200),      // Blue
            EncounterType::Pirate => (200, 0, 0),        // Red
            EncounterType::Police => (0, 150, 0),        // Green
            EncounterType::SpaceMonster => (150, 0, 150), // Purple
            EncounterType::None => (100, 100, 100),      // Gray
        }
    }
}

pub fn check_for_encounter(game_state: &GameState) -> Option<Encounter> {
    // Encounter probability increases with:
    // - Higher difficulty setting
    // - More cargo/credits
    // - Lower police record (more pirate encounters)
    // - Current tech level of system
    
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};
    
    let mut hasher = RandomState::new().build_hasher();
    game_state.current_system_id.hash(&mut hasher);
    let day_hash = (game_state.days as u64).wrapping_mul(73856093);
    let seed = hasher.finish() ^ day_hash;
    
    // Simplified encounter probability: ~20% chance
    let encounter_roll = (seed % 100) as i32;
    
    if encounter_roll < 20 {
        let encounter_type_roll = (seed / 100) % 4;
        let encounter_type = match encounter_type_roll {
            0 => EncounterType::Trader,
            1 => EncounterType::Pirate,
            2 => EncounterType::Police,
            _ => EncounterType::SpaceMonster,
        };
        
        let distance = ((seed / 500) % 30) as i32 + 10;
        let system = &game_state.solar_systems[game_state.current_system_id];
        
        let (description, ship_name) = generate_encounter_description(&encounter_type, &system.name);
        
        Some(Encounter::new(
            encounter_type,
            description,
            ship_name,
            distance,
            system.name.clone(),
        ))
    } else {
        None
    }
}

fn generate_encounter_description(
    encounter_type: &EncounterType,
    system_name: &str,
) -> (String, String) {
    let trader_ships = vec![
        "bumblebee", "merchant", "freighter", "hauler", "wheeler",
    ];
    let pirate_ships = vec![
        "corsair", "marauder", "raider", "scourge", "dreadnought",
    ];
    let police_ships = vec![
        "enforcer", "guardian", "sentinel", "patrol", "defender",
    ];
    let monster_ships = vec![
        "creature", "beast", "leviathan", "wyrm", "anomaly",
    ];
    
    match encounter_type {
        EncounterType::Trader => {
            let ship = trader_ships[(system_name.len() % trader_ships.len())];
            (
                format!(
                    "At some distance from {}, you encounter a trader {}.\n\nIt ignores you.",
                    system_name, ship
                ),
                ship.to_string(),
            )
        }
        EncounterType::Pirate => {
            let ship = pirate_ships[(system_name.len() % pirate_ships.len())];
            (
                format!(
                    "A pirate {} appears, weapons charged!\n\nThey demand your cargo or your life!",
                    ship
                ),
                ship.to_string(),
            )
        }
        EncounterType::Police => {
            let ship = police_ships[(system_name.len() % police_ships.len())];
            (
                format!(
                    "A {} vessel approaches.\n\nThey scan your cargo manifest and ask your business here.",
                    ship
                ),
                ship.to_string(),
            )
        }
        EncounterType::SpaceMonster => {
            let ship = monster_ships[(system_name.len() % monster_ships.len())];
            (
                format!(
                    "A massive space {} emerges from the cosmic void!\n\nIt appears to view your ship as either prey or threat.",
                    ship
                ),
                ship.to_string(),
            )
        }
        EncounterType::None => ("No encounter.".to_string(), "None".to_string()),
    }
}

pub fn resolve_encounter(
    game_state: &mut GameState,
    encounter: &Encounter,
    player_choice: EncounterChoice,
) -> String {
    match (&encounter.encounter_type, player_choice) {
        (EncounterType::Trader, EncounterChoice::Ignore) => {
            "The trader continues on its way.".to_string()
        }
        (EncounterType::Trader, EncounterChoice::Attack) => {
            game_state.police_record_score += 10; // Attack increases wanted level
            format!("You attack the {} and take {} credits!", encounter.ship_name, 500)
        }
        
        (EncounterType::Pirate, EncounterChoice::Attack) => {
            let damage = ((game_state.current_system_id as i32) % 20) as i32 + 10;
            game_state.ship.hull -= damage;
            if game_state.ship.hull < 0 {
                game_state.ship.hull = 0;
            }
            format!("Fierce battle! You take {} damage.", damage)
        }
        (EncounterType::Pirate, EncounterChoice::Ignore) => {
            let loss = (game_state.credits / 4).min(5000);
            game_state.credits -= loss;
            format!("You pay {} credits in tribute to escape.", loss)
        }
        
        (EncounterType::Police, EncounterChoice::Attack) => {
            game_state.police_record_score += 50; // Attacking police is serious
            "You are now wanted across known space!".to_string()
        }
        (EncounterType::Police, EncounterChoice::Ignore) => {
            if game_state.police_record_score > 50 {
                let fine = (game_state.police_record_score * 50).min(5000);
                game_state.credits -= fine;
                game_state.police_record_score -= 25;
                format!("You pay a fine of {} credits.", fine)
            } else {
                "They scan your cargo and wave you through.".to_string()
            }
        }
        
        (EncounterType::SpaceMonster, EncounterChoice::Attack) => {
            let damage = 30 + ((game_state.current_system_id as i32) % 40);
            game_state.ship.hull -= damage;
            if game_state.ship.hull < 0 {
                game_state.ship.hull = 0;
            }
            format!("Brutal combat! You take {} damage to your hull!", damage)
        }
        (EncounterType::SpaceMonster, EncounterChoice::Ignore) => {
            if game_state.ship.fuel > 50 {
                game_state.ship.fuel -= 50;
                "You accelerate away at full burn!".to_string()
            } else {
                let damage = 15 + ((game_state.current_system_id as i32) % 20);
                game_state.ship.hull -= damage;
                format!("You manage to escape but take {} damage!", damage)
            }
        }
        
        _ => "Encounter resolved.".to_string(),
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum EncounterChoice {
    Attack,
    Ignore,
}
