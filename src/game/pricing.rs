// Trade goods pricing system
// Ported from the original DeterminePrices() and RecalculateBuyPrices() functions

use crate::types::{GameState, TradeGood};
use crate::types::trade::TRADE_ITEMS;
use rand::Rng;

pub fn determine_prices(game_state: &mut GameState, system_id: usize) {
    let mut rng = rand::thread_rng();
    let system = &game_state.solar_systems[system_id];
    let tech_level = system.tech_level as i32;
    
    for good in 0..10 {
        let trade_item = &TRADE_ITEMS[good];
        
        // Base price calculation
        let mut price = trade_item.base_price;
        
        // Adjust for tech level
        price += (tech_level - trade_item.min_tech_prod) * trade_item.inc_price_per_level;
        
        // Add random variance
        if trade_item.variance > 0 {
            let variance = rng.gen_range(0..trade_item.variance);
            price += variance;
        }
        
        // Ensure price is within bounds
        price = price.max(trade_item.min_price).min(trade_item.max_price);
        
        // Store price
        game_state.solar_systems[system_id].price_increase[good] = price;
    }
}

pub fn get_buy_price(game_state: &GameState, good: TradeGood) -> i32 {
    let system_id = game_state.current_system_id;
    game_state.solar_systems[system_id].price_increase[good as usize]
}

pub fn get_sell_price(game_state: &GameState, good: TradeGood) -> i32 {
    // Sell price is typically the same as buy price in this version
    get_buy_price(game_state, good)
}
