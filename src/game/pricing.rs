// Trade goods pricing system
// Ported from the original DeterminePrices() and RecalculateBuyPrices() functions

use crate::types::{GameState, TradeGood};
use crate::types::solar_system::SpecialResource;
use crate::types::trade::TRADE_ITEMS;
use rand::Rng;

const HISTORY_LEN: usize = 5;

const EVENT_DROUGHT: i32 = 0;
const EVENT_CROP_FAILURE: i32 = 1;
const EVENT_WAR: i32 = 2;
const EVENT_BOREDOM: i32 = 3;
const EVENT_PLAGUE: i32 = 4;
const EVENT_LACK_OF_WORKERS: i32 = 5;
const EVENT_DRUG_DEMAND: i32 = 6;

pub fn determine_prices(game_state: &mut GameState, system_id: usize) {
    let mut rng = rand::thread_rng();
    let system = &mut game_state.solar_systems[system_id];
    let tech_level = system.tech_level as i32;
    let system_name = system.name.clone();
    let special_resource = system.special_resource;

    if rng.gen_range(0..100) < 20 {
        system.special_event = rng.gen_range(0..=6);
    } else {
        system.special_event = -1;
    }
    let special_event = system.special_event;

    let mut event_goods: Vec<&'static str> = Vec::new();
    let mut low_resource_goods: Vec<&'static str> = Vec::new();
    let mut high_resource_goods: Vec<&'static str> = Vec::new();
    
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

        // Resource-based adjustments
        if special_resource as i32 == trade_item.resource_low_price {
            price -= trade_item.base_price / 4;
            low_resource_goods.push(trade_item.name);
        } else if special_resource as i32 == trade_item.resource_high_price {
            price += trade_item.base_price / 4;
            high_resource_goods.push(trade_item.name);
        }

        // Event-based adjustments
        if special_event >= 0 && trade_item.price_increase_event == special_event {
            price += trade_item.base_price / 2;
            event_goods.push(trade_item.name);
        }

        // Apply rounding
        if trade_item.rounding > 1 {
            price = (price / trade_item.rounding) * trade_item.rounding;
        }
        
        // Ensure price is within bounds
        price = price.max(trade_item.min_price).min(trade_item.max_price);
        
        // Store price
        system.price_increase[good] = price;

        // Store price history
        let history = &mut system.price_history[good];
        history.push(price);
        if history.len() > HISTORY_LEN {
            history.remove(0);
        }
    }

    system.news = build_news(
        &system_name,
        special_event,
        special_resource,
        &event_goods,
        &low_resource_goods,
        &high_resource_goods,
    );
}

pub fn get_buy_price(game_state: &GameState, good: TradeGood) -> i32 {
    let system_id = game_state.current_system_id;
    game_state.solar_systems[system_id].price_increase[good as usize]
}

pub fn get_sell_price(game_state: &GameState, good: TradeGood) -> i32 {
    // Sell price is typically the same as buy price in this version
    get_buy_price(game_state, good)
}

fn build_news(
    system_name: &str,
    special_event: i32,
    special_resource: SpecialResource,
    event_goods: &[&'static str],
    low_resource_goods: &[&'static str],
    high_resource_goods: &[&'static str],
) -> Vec<String> {
    let mut news: Vec<String> = Vec::new();

    if special_event >= 0 {
        if let Some(event_name) = event_name(special_event) {
            if !event_goods.is_empty() {
                news.push(format!(
                    "{} on {}: {} prices spike.",
                    event_name,
                    system_name,
                    join_goods(event_goods)
                ));
            } else {
                news.push(format!("{} reported on {}.", event_name, system_name));
            }
        }
    }

    if special_resource != SpecialResource::Nothing {
        let resource_name = resource_name(special_resource);
        if !low_resource_goods.is_empty() {
            news.push(format!(
                "{} boosts supply of {}.",
                resource_name,
                join_goods(low_resource_goods)
            ));
        } else if !high_resource_goods.is_empty() {
            news.push(format!(
                "{} drives demand for {}.",
                resource_name,
                join_goods(high_resource_goods)
            ));
        } else {
            news.push(format!("Local condition: {}.", resource_name));
        }
    }

    if news.is_empty() {
        news.push("Markets calm across the system.".to_string());
    }

    news
}

fn event_name(event_id: i32) -> Option<&'static str> {
    match event_id {
        EVENT_DROUGHT => Some("Drought"),
        EVENT_CROP_FAILURE => Some("Crop Failure"),
        EVENT_WAR => Some("War"),
        EVENT_BOREDOM => Some("Boredom"),
        EVENT_PLAGUE => Some("Plague"),
        EVENT_LACK_OF_WORKERS => Some("Labor Shortage"),
        EVENT_DRUG_DEMAND => Some("Drug Demand"),
        _ => None,
    }
}

fn resource_name(resource: SpecialResource) -> &'static str {
    match resource {
        SpecialResource::Nothing => "None",
        SpecialResource::MineralRich => "Mineral Rich",
        SpecialResource::MineralPoor => "Mineral Poor",
        SpecialResource::Desert => "Desert",
        SpecialResource::LotsOfWater => "Lots of Water",
        SpecialResource::RichSoil => "Rich Soil",
        SpecialResource::PoorSoil => "Poor Soil",
        SpecialResource::RichFauna => "Rich Fauna",
        SpecialResource::Lifeless => "Lifeless",
        SpecialResource::WeirdMushrooms => "Weird Mushrooms",
        SpecialResource::LotsOfHerbs => "Lots of Herbs",
        SpecialResource::Artistic => "Artistic",
        SpecialResource::Warlike => "Warlike",
    }
}

fn join_goods(goods: &[&'static str]) -> String {
    match goods.len() {
        0 => "".to_string(),
        1 => goods[0].to_string(),
        2 => format!("{} and {}", goods[0], goods[1]),
        _ => {
            let mut list = goods[..goods.len() - 1].join(", ");
            list.push_str(&format!(", and {}", goods[goods.len() - 1]));
            list
        }
    }
}
