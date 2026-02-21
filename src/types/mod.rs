// Core data types ported from the original C structures

pub mod ship;
pub mod solar_system;
pub mod crew;
pub mod equipment;
pub mod trade;
pub mod constants;

pub use ship::Ship;
pub use solar_system::SolarSystem;
pub use crew::CrewMember;
pub use trade::TradeGood;

use serde::{Deserialize, Serialize};
use crate::save;

/// Main game state structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub commander_name: String,
    pub credits: i32,
    pub debt: i32,
    pub days: i32,
    pub police_record_score: i32,
    pub reputation_score: i32,
    pub difficulty: Difficulty,
    
    pub ship: Ship,
    pub crew: Vec<CrewMember>,
    pub solar_systems: Vec<SolarSystem>,
    pub current_system_id: usize,
    pub wormholes: Vec<(usize, usize)>,
    
    // Flags
    pub escape_pod: bool,
    pub insurance: bool,
    pub moon_purchased: bool,
    pub jarek_status: u8,
    pub invasion_status: u8,
    pub reactor_status: u8,
    pub scarab_status: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
    Impossible = 4,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            commander_name: "Shelby".to_string(),
            credits: 1000,
            debt: 0,
            days: 0,
            police_record_score: 0,
            reputation_score: 0,
            difficulty: Difficulty::Normal,
            
            ship: Ship::new_flea(),
            crew: vec![],
            solar_systems: vec![],
            current_system_id: 0,
            wormholes: vec![],
            
            escape_pod: false,
            insurance: false,
            moon_purchased: false,
            jarek_status: 0,
            invasion_status: 0,
            reactor_status: 0,
            scarab_status: 0,
        }
    }
    
    pub fn start_new_game(&mut self) {
        // Generate the galaxy
        self.solar_systems = SolarSystem::generate_galaxy();
        
        // Start at a random habitable system
        self.current_system_id = 0; // Will be set properly during galaxy generation
        
        println!("Galaxy generated with {} systems", self.solar_systems.len());
    }
    
    pub fn load_game(&mut self) -> bool {
        match save::load_game() {
            Ok(loaded_state) => {
                *self = loaded_state;
                true
            }
            Err(e) => {
                eprintln!("Failed to load game: {}", e);
                false
            }
        }
    }
    
    pub fn save_game(&self) -> bool {
        match save::save_game(self) {
            Ok(()) => true,
            Err(e) => {
                eprintln!("Failed to save game: {}", e);
                false
            }
        }
    }
    
    pub fn current_system(&self) -> &SolarSystem {
        &self.solar_systems[self.current_system_id]
    }
    
    pub fn current_system_name(&self) -> &str {
        if self.solar_systems.is_empty() {
            "Unknown"
        } else {
            &self.current_system().name
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
