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
