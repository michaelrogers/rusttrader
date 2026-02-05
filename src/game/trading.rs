// Trading system logic

use crate::types::{GameState, TradeGood};
use crate::game::pricing::{get_buy_price, get_sell_price};

pub fn buy_cargo(game_state: &mut GameState, good: TradeGood, amount: i32) -> Result<(), String> {
    let price = get_buy_price(game_state, good);
    let total_cost = price * amount;
    
    // Check if player has enough money
    if game_state.credits < total_cost {
        return Err(format!("Not enough credits (need {} cr)", total_cost));
    }
    
    // Check if ship has enough cargo space
    if game_state.ship.cargo_bays_available() < amount {
        return Err("Not enough cargo space".to_string());
    }
    
    // Check system has goods available
    let system_id = game_state.current_system_id;
    let available = game_state.solar_systems[system_id].qty[good as usize];
    if available < amount {
        return Err(format!("Only {} available", available));
    }
    
    // Execute trade
    game_state.credits -= total_cost;
    game_state.ship.cargo[good as usize] += amount;
    game_state.solar_systems[system_id].qty[good as usize] -= amount;
    
    Ok(())
}

pub fn sell_cargo(game_state: &mut GameState, good: TradeGood, amount: i32) -> Result<(), String> {
    // Check if player has enough cargo
    if game_state.ship.cargo[good as usize] < amount {
        return Err("Not enough cargo to sell".to_string());
    }
    
    let price = get_sell_price(game_state, good);
    let total_value = price * amount;
    
    // Execute trade
    game_state.credits += total_value;
    game_state.ship.cargo[good as usize] -= amount;
    
    // Add to system inventory
    let system_id = game_state.current_system_id;
    game_state.solar_systems[system_id].qty[good as usize] += amount;
    
    Ok(())
}

pub fn max_buyable(game_state: &GameState, good: TradeGood) -> i32 {
    let price = get_buy_price(game_state, good);
    if price <= 0 {
        return 0;
    }
    
    let can_afford = game_state.credits / price;
    let cargo_space = game_state.ship.cargo_bays_available();
    let system_id = game_state.current_system_id;
    let available = game_state.solar_systems[system_id].qty[good as usize];
    
    can_afford.min(cargo_space).min(available)
}

/// Get fuel cost per unit at current system
pub fn get_fuel_cost(game_state: &GameState) -> i32 {
    use crate::types::ship::SHIP_TYPES;
    SHIP_TYPES[game_state.ship.ship_type].cost_of_fuel
}

/// Calculate maximum fuel that can be purchased given current credits
pub fn max_fuel_buyable(game_state: &GameState) -> i32 {
    let fuel_cost = get_fuel_cost(game_state);
    if fuel_cost <= 0 {
        return 0;
    }
    
    let can_afford = game_state.credits / fuel_cost;
    let max_fuel = game_state.ship.max_fuel();
    let current_fuel = game_state.ship.fuel;
    let space_available = max_fuel - current_fuel;
    
    can_afford.min(space_available).max(0)
}

/// Purchase fuel at the current system
pub fn buy_fuel(game_state: &mut GameState, amount: i32) -> Result<(), String> {
    use crate::types::ship::SHIP_TYPES;
    
    // Validate amount
    if amount <= 0 {
        return Err("Must buy at least 1 fuel".to_string());
    }
    
    // Check max fuel capacity
    let max_fuel = game_state.ship.max_fuel();
    if game_state.ship.fuel >= max_fuel {
        return Err("Fuel tanks already full".to_string());
    }
    
    // Check how much space is available
    let space_available = max_fuel - game_state.ship.fuel;
    let amount_to_buy = amount.min(space_available);
    
    // Check if player has enough credits
    let fuel_cost = SHIP_TYPES[game_state.ship.ship_type].cost_of_fuel;
    let total_cost = fuel_cost * amount_to_buy;
    
    if game_state.credits < total_cost {
        return Err(format!("Not enough credits (need {} cr, have {})", total_cost, game_state.credits));
    }
    
    // Execute purchase
    game_state.credits -= total_cost;
    game_state.ship.fuel += amount_to_buy;
    
    Ok(())
}
