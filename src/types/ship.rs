// Ship data structures
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use super::equipment::{Weapon, Shield, Gadget};

/// Ship type definition (from original SHIPTYPE struct)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipType {
    pub name: &'static str,
    pub cargo_bays: i32,
    pub weapon_slots: i32,
    pub shield_slots: i32,
    pub gadget_slots: i32,
    pub crew_quarters: i32,
    pub fuel_tanks: i32,
    pub min_tech_level: i32,
    pub cost_of_fuel: i32,
    pub price: i32,
    pub bounty: i32,
    pub occurrence: i32,
    pub hull_strength: i32,
    pub police: i32,
    pub pirates: i32,
    pub traders: i32,
    pub min_tech_level_repair: i32,
    pub rep_for_bounty: i32,
}

/// Active ship instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub ship_type: usize, // Index into SHIP_TYPES
    pub name: String,
    
    // Cargo
    pub cargo: [i32; 10], // Amount of each trade good
    pub tribbles: i32,
    
    // Equipment
    pub weapons: Vec<Option<Weapon>>,
    pub shields: Vec<Option<Shield>>,
    pub gadgets: Vec<Option<Gadget>>,
    
    // Status
    pub fuel: i32,
    pub hull: i32,
    pub crew: Vec<usize>, // Indices into crew member array
    
    // Upgrades (cumulative levels/counts)
    pub cargo_expansion: u8,        // 0-2: +5 bays per level
    pub fuel_expansion: u8,         // 0-2: +50 capacity per level
    pub weapon_rating: u8,          // 0-5: combat effectiveness
    pub shield_installed: bool,     // Shield generator active
    pub hull_reinforcement: u8,     // 0-2: +10 hull per level
}impl Ship {
    /// Create a new Flea (starting ship)
    pub fn new_flea() -> Self {
        Self {
            ship_type: 0, // Flea
            name: "Flea".to_string(),
            cargo: [0; 10],
            tribbles: 0,
            weapons: vec![None; 0], // No weapon slots on Flea
            shields: vec![None; 0], // No shield slots on Flea
            gadgets: vec![None; 0], // No gadget slots on Flea
            fuel: 14, // Flea has 14 fuel
            hull: 25, // Flea hull strength
            crew: vec![0], // Just the commander
            cargo_expansion: 0,
            fuel_expansion: 0,
            weapon_rating: 0,
            shield_installed: false,
            hull_reinforcement: 0,
        }
    }
    
    pub fn total_cargo(&self) -> i32 {
        self.cargo.iter().sum()
    }
    
    pub fn cargo_bays_available(&self) -> i32 {
        let base_bays = SHIP_TYPES[self.ship_type].cargo_bays;
        let expanded_bays = base_bays + (self.cargo_expansion as i32 * 5);
        expanded_bays - self.total_cargo()
    }
    
    pub fn max_fuel(&self) -> i32 {
        let base_capacity = SHIP_TYPES[self.ship_type].fuel_tanks * 10;
        base_capacity + (self.fuel_expansion as i32 * 50)
    }
}

/// Ship type constants from original Global.c
pub const SHIP_TYPES: &[ShipType] = &[
    // Name, Cargo, Wpn, Shld, Gad, Crew, Fuel, MinTech, FuelCost, Price, Bounty, Occur, Hull, Police, Pirates, Traders, RepairTech, BountyRep
    ShipType {
        name: "Flea",
        cargo_bays: 10,
        weapon_slots: 0,
        shield_slots: 0,
        gadget_slots: 0,
        crew_quarters: 1,
        fuel_tanks: 2,
        min_tech_level: 4,
        cost_of_fuel: 1,
        price: 2000,
        bounty: 5,
        occurrence: 2,
        hull_strength: 25,
        police: 0,
        pirates: 1,
        traders: 1,
        min_tech_level_repair: 1,
        rep_for_bounty: 1,
    },
    ShipType {
        name: "Gnat",
        cargo_bays: 15,
        weapon_slots: 1,
        shield_slots: 0,
        gadget_slots: 1,
        crew_quarters: 1,
        fuel_tanks: 1,
        min_tech_level: 5,
        cost_of_fuel: 2,
        price: 10000,
        bounty: 50,
        occurrence: 28,
        hull_strength: 100,
        police: 0,
        pirates: 5,
        traders: 5,
        min_tech_level_repair: 1,
        rep_for_bounty: 2,
    },
    ShipType {
        name: "Firefly",
        cargo_bays: 20,
        weapon_slots: 1,
        shield_slots: 1,
        gadget_slots: 1,
        crew_quarters: 1,
        fuel_tanks: 1,
        min_tech_level: 5,
        cost_of_fuel: 3,
        price: 25000,
        bounty: 75,
        occurrence: 20,
        hull_strength: 100,
        police: 1,
        pirates: 5,
        traders: 5,
        min_tech_level_repair: 1,
        rep_for_bounty: 3,
    },
    ShipType {
        name: "Mosquito",
        cargo_bays: 15,
        weapon_slots: 2,
        shield_slots: 1,
        gadget_slots: 1,
        crew_quarters: 1,
        fuel_tanks: 1,
        min_tech_level: 5,
        cost_of_fuel: 5,
        price: 30000,
        bounty: 100,
        occurrence: 20,
        hull_strength: 100,
        police: 1,
        pirates: 5,
        traders: 5,
        min_tech_level_repair: 1,
        rep_for_bounty: 3,
    },
    ShipType {
        name: "Bumblebee",
        cargo_bays: 25,
        weapon_slots: 1,
        shield_slots: 2,
        gadget_slots: 2,
        crew_quarters: 2,
        fuel_tanks: 1,
        min_tech_level: 5,
        cost_of_fuel: 7,
        price: 60000,
        bounty: 125,
        occurrence: 15,
        hull_strength: 100,
        police: 2,
        pirates: 5,
        traders: 5,
        min_tech_level_repair: 1,
        rep_for_bounty: 4,
    },
    ShipType {
        name: "Beetle",
        cargo_bays: 50,
        weapon_slots: 0,
        shield_slots: 1,
        gadget_slots: 1,
        crew_quarters: 3,
        fuel_tanks: 1,
        min_tech_level: 5,
        cost_of_fuel: 10,
        price: 80000,
        bounty: 50,
        occurrence: 3,
        hull_strength: 50,
        police: 3,
        pirates: 5,
        traders: 5,
        min_tech_level_repair: 1,
        rep_for_bounty: 4,
    },
    ShipType {
        name: "Hornet",
        cargo_bays: 20,
        weapon_slots: 3,
        shield_slots: 2,
        gadget_slots: 1,
        crew_quarters: 2,
        fuel_tanks: 1,
        min_tech_level: 6,
        cost_of_fuel: 15,
        price: 100000,
        bounty: 200,
        occurrence: 6,
        hull_strength: 150,
        police: 3,
        pirates: 6,
        traders: 5,
        min_tech_level_repair: 1,
        rep_for_bounty: 5,
    },
    ShipType {
        name: "Grasshopper",
        cargo_bays: 30,
        weapon_slots: 2,
        shield_slots: 2,
        gadget_slots: 3,
        crew_quarters: 3,
        fuel_tanks: 1,
        min_tech_level: 6,
        cost_of_fuel: 15,
        price: 150000,
        bounty: 300,
        occurrence: 2,
        hull_strength: 150,
        police: 4,
        pirates: 6,
        traders: 5,
        min_tech_level_repair: 1,
        rep_for_bounty: 5,
    },
    ShipType {
        name: "Termite",
        cargo_bays: 60,
        weapon_slots: 1,
        shield_slots: 3,
        gadget_slots: 2,
        crew_quarters: 3,
        fuel_tanks: 1,
        min_tech_level: 7,
        cost_of_fuel: 20,
        price: 225000,
        bounty: 300,
        occurrence: 2,
        hull_strength: 200,
        police: 5,
        pirates: 6,
        traders: 6,
        min_tech_level_repair: 1,
        rep_for_bounty: 5,
    },
    ShipType {
        name: "Wasp",
        cargo_bays: 35,
        weapon_slots: 3,
        shield_slots: 2,
        gadget_slots: 2,
        crew_quarters: 3,
        fuel_tanks: 1,
        min_tech_level: 7,
        cost_of_fuel: 20,
        price: 300000,
        bounty: 500,
        occurrence: 2,
        hull_strength: 200,
        police: 5,
        pirates: 6,
        traders: 4,
        min_tech_level_repair: 5,
        rep_for_bounty: 4,
    },
];
