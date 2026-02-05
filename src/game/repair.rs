// Ship repair system

use crate::types::GameState;

/// Calculate the cost to repair 1 hull point
pub fn calculate_repair_cost_per_point(game_state: &GameState) -> i32 {
    let system = &game_state.solar_systems[game_state.current_system_id];
    
    // Base cost: 50 credits per hull point
    // Increases with tech level (can repair better at higher tech)
    let base_cost = 50;
    let tech_multiplier = (system.tech_level as i32 + 1) as f32 / 8.0;
    
    ((base_cost as f32) * tech_multiplier) as i32
}

/// Get the maximum hull integrity for the ship
pub fn get_max_hull(game_state: &GameState) -> i32 {
    let base_hull = 25; // Flea base hull
    let reinforcement_bonus = game_state.ship.hull_reinforcement as i32 * 10;
    base_hull + reinforcement_bonus
}

/// Calculate total cost to fully repair ship
pub fn calculate_full_repair_cost(game_state: &GameState) -> i32 {
    let max_hull = get_max_hull(game_state);
    let damage_taken = max_hull - game_state.ship.hull;
    
    if damage_taken <= 0 {
        0
    } else {
        let cost_per_point = calculate_repair_cost_per_point(game_state);
        damage_taken * cost_per_point
    }
}

/// Calculate cost to repair N hull points
pub fn calculate_repair_cost(game_state: &GameState, amount: i32) -> i32 {
    let cost_per_point = calculate_repair_cost_per_point(game_state);
    amount * cost_per_point
}

/// Repair the ship by specified amount
pub fn repair_ship(game_state: &mut GameState, amount: i32) -> Result<String, String> {
    let max_hull = get_max_hull(game_state);
    
    // Check if ship needs repair
    if game_state.ship.hull >= max_hull {
        return Err("Ship is already fully repaired!".to_string());
    }
    
    // Check if amount is valid
    if amount <= 0 {
        return Err("Invalid repair amount".to_string());
    }
    
    // Calculate actual repair amount (don't exceed max hull)
    let actual_repair = (game_state.ship.hull + amount).min(max_hull) - game_state.ship.hull;
    let cost = calculate_repair_cost(game_state, actual_repair);
    
    // Check credits
    if game_state.credits < cost {
        return Err(format!(
            "Not enough credits. Need {} but only have {}",
            cost, game_state.credits
        ));
    }
    
    // Apply repair
    game_state.credits -= cost;
    game_state.ship.hull += actual_repair;
    
    Ok(format!("Repaired {} hull points for {} credits", actual_repair, cost))
}

/// Repair all damage on the ship
pub fn repair_full(game_state: &mut GameState) -> Result<String, String> {
    let max_hull = get_max_hull(game_state);
    let damage_taken = max_hull - game_state.ship.hull;
    
    if damage_taken <= 0 {
        return Err("Ship is already fully repaired!".to_string());
    }
    
    repair_ship(game_state, damage_taken)
}

/// Check if repairs are available at current system
pub fn can_repair(game_state: &GameState) -> bool {
    // Most systems have repair facilities except pre-agricultural
    let system = &game_state.solar_systems[game_state.current_system_id];
    system.tech_level as i32 >= 1
}
