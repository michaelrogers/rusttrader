// Ship shop and purchasing system

use crate::types::{GameState, Ship};
use crate::types::ship::SHIP_TYPES;

/// Get information about all available ships
pub fn get_available_ships() -> Vec<ShipInfo> {
    vec![
        ShipInfo {
            ship_type_id: 0,
            name: "Flea",
            description: "Cheap starter ship. No frills, but reliable.",
            price: 2000,
            cargo_bays: 10,
            weapon_slots: 0,
            shield_slots: 0,
            fuel_capacity: 20,
            hull_strength: 25,
            min_tech_level: 4,
            special_traits: "Budget friendly",
        },
        ShipInfo {
            ship_type_id: 1,
            name: "Gnat",
            description: "Balanced trader. Good cargo and fuel capacity.",
            price: 10000,
            cargo_bays: 15,
            weapon_slots: 1,
            shield_slots: 0,
            fuel_capacity: 10,
            hull_strength: 100,
            min_tech_level: 5,
            special_traits: "Balanced stats",
        },
        ShipInfo {
            ship_type_id: 2,
            name: "Firefly",
            description: "Combat-ready trader. Weapons and shield.",
            price: 25000,
            cargo_bays: 20,
            weapon_slots: 1,
            shield_slots: 1,
            fuel_capacity: 10,
            hull_strength: 100,
            min_tech_level: 5,
            special_traits: "Balanced combat",
        },
        ShipInfo {
            ship_type_id: 3,
            name: "Mosquito",
            description: "Fast and agile. Good for running cargo.",
            price: 30000,
            cargo_bays: 15,
            weapon_slots: 1,
            shield_slots: 1,
            fuel_capacity: 20,
            hull_strength: 80,
            min_tech_level: 6,
            special_traits: "High speed",
        },
        ShipInfo {
            ship_type_id: 4,
            name: "Bumblebee",
            description: "Large cargo hold. Perfect for traders.",
            price: 35000,
            cargo_bays: 50,
            weapon_slots: 1,
            shield_slots: 1,
            fuel_capacity: 10,
            hull_strength: 150,
            min_tech_level: 6,
            special_traits: "Huge cargo",
        },
        ShipInfo {
            ship_type_id: 5,
            name: "Beetle",
            description: "Heavy armor and shields. Combat specialist.",
            price: 50000,
            cargo_bays: 20,
            weapon_slots: 2,
            shield_slots: 2,
            fuel_capacity: 15,
            hull_strength: 200,
            min_tech_level: 7,
            special_traits: "Combat tank",
        },
    ]
}

#[derive(Clone, Debug)]
pub struct ShipInfo {
    pub ship_type_id: usize,
    pub name: &'static str,
    pub description: &'static str,
    pub price: i32,
    pub cargo_bays: i32,
    pub weapon_slots: i32,
    pub shield_slots: i32,
    pub fuel_capacity: i32,
    pub hull_strength: i32,
    pub min_tech_level: i32,
    pub special_traits: &'static str,
}

impl ShipInfo {
    pub fn upgrade_cost_from_current(&self, game_state: &GameState) -> i32 {
        // Selling current ship gives you credits equal to 50% of original price
        let current_ship_refund = if game_state.ship.ship_type == 0 {
            2000 / 2 // Flea refund
        } else {
            // Look up original price from current ship
            let current_ship = &SHIP_TYPES[game_state.ship.ship_type];
            (current_ship.price / 2).max(100)
        };

        // Net cost is new price minus refund
        (self.price - current_ship_refund).max(0)
    }
}

/// Get current ship information
pub fn get_current_ship_info(game_state: &GameState) -> ShipInfo {
    let available = get_available_ships();
    available
        .into_iter()
        .find(|s| s.ship_type_id == game_state.ship.ship_type)
        .unwrap_or_else(|| {
            // Fallback to Flea if not found
            ShipInfo {
                ship_type_id: 0,
                name: "Flea",
                description: "Cheap starter ship. No frills, but reliable.",
                price: 2000,
                cargo_bays: 10,
                weapon_slots: 0,
                shield_slots: 0,
                fuel_capacity: 20,
                hull_strength: 25,
                min_tech_level: 4,
                special_traits: "Budget friendly",
            }
        })
}

/// Purchase a new ship and transfer cargo/upgrades
pub fn purchase_ship(
    game_state: &mut GameState,
    new_ship_id: usize,
) -> Result<String, String> {
    let available_ships = get_available_ships();

    let new_ship_info = available_ships
        .iter()
        .find(|s| s.ship_type_id == new_ship_id)
        .ok_or("Ship not found".to_string())?;

    // Check if already own this ship
    if game_state.ship.ship_type == new_ship_id {
        return Err("You already own this ship!".to_string());
    }

    // Check tech level requirement
    if new_ship_info.min_tech_level > game_state.solar_systems[game_state.current_system_id].tech_level as i32 {
        return Err(format!(
            "This system's tech level is too low to purchase this ship (needs level {})",
            new_ship_info.min_tech_level
        ));
    }

    // Calculate cost
    let net_cost = new_ship_info.upgrade_cost_from_current(game_state);

    // Check credits
    if game_state.credits < net_cost {
        return Err(format!(
            "Not enough credits. Need {} but only have {}",
            net_cost, game_state.credits
        ));
    }

    // Store current cargo and upgrades
    let old_cargo = game_state.ship.cargo;
    let old_fuel = game_state.ship.fuel;
    let cargo_expansion = game_state.ship.cargo_expansion;
    let fuel_expansion = game_state.ship.fuel_expansion;
    let weapon_rating = game_state.ship.weapon_rating;
    let shield_installed = game_state.ship.shield_installed;
    let hull_reinforcement = game_state.ship.hull_reinforcement;

    // Create new ship
    let mut new_ship = Ship::new_flea();
    new_ship.ship_type = new_ship_id;
    new_ship.name = new_ship_info.name.to_string();

    // Transfer cargo (limit by new cargo capacity)
    let new_cargo_capacity = new_ship_info.cargo_bays + (cargo_expansion as i32 * 5);
    let mut total_cargo = 0;
    for (i, &amount) in old_cargo.iter().enumerate() {
        if total_cargo + amount <= new_cargo_capacity {
            new_ship.cargo[i] = amount;
            total_cargo += amount;
        } else {
            // Only transfer what fits
            let space_left = new_cargo_capacity - total_cargo;
            if space_left > 0 {
                new_ship.cargo[i] = space_left;
            }
            break;
        }
    }

    // Transfer upgrades
    new_ship.cargo_expansion = cargo_expansion;
    new_ship.fuel_expansion = fuel_expansion;
    new_ship.weapon_rating = weapon_rating;
    new_ship.shield_installed = shield_installed;
    new_ship.hull_reinforcement = hull_reinforcement;

    // Transfer fuel (limit by new capacity)
    let new_max_fuel = new_ship_info.fuel_capacity * 10 + (fuel_expansion as i32 * 50);
    new_ship.fuel = old_fuel.min(new_max_fuel);

    // Set hull to max for new ship
    new_ship.hull = new_ship_info.hull_strength + (hull_reinforcement as i32 * 10);

    // Deduct cost
    game_state.credits -= net_cost;

    // Replace ship
    game_state.ship = new_ship;

    Ok(format!(
        "Purchased {} for {} credits!",
        new_ship_info.name, net_cost
    ))
}

/// Get ships available for purchase at a specific tech level
pub fn get_purchasable_ships(tech_level: i32) -> Vec<ShipInfo> {
    get_available_ships()
        .into_iter()
        .filter(|s| s.min_tech_level <= tech_level)
        .collect()
}
