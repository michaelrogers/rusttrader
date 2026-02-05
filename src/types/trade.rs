// Trade system data structures
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeGood {
    Water = 0,
    Furs = 1,
    Food = 2,
    Ore = 3,
    Games = 4,
    Firearms = 5,
    Medicine = 6,
    Machines = 7,
    Narcotics = 8,
    Robots = 9,
}

/// Trade item definition from original TRADEITEM struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeItem {
    pub name: &'static str,
    pub min_tech_prod: i32,     // Min tech level to produce
    pub min_tech_use: i32,      // Min tech level to use
    pub tech_top_prod: i32,     // Tech level for max production
    pub base_price: i32,        // Base price
    pub inc_price_per_level: i32, // Price increase per tech level
    pub variance: i32,          // Max random variance in price
    pub price_increase_event: i32, // Event that increases price
    pub resource_low_price: i32,   // Resource that lowers price
    pub resource_high_price: i32,  // Resource that increases price
    pub min_price: i32,         // Minimum price
    pub max_price: i32,         // Maximum price
    pub rounding: i32,          // Price rounding
}

impl TradeGood {
    pub fn name(&self) -> &'static str {
        TRADE_ITEMS[*self as usize].name
    }
    
    pub fn base_price(&self) -> i32 {
        TRADE_ITEMS[*self as usize].base_price
    }
    
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => TradeGood::Water,
            1 => TradeGood::Furs,
            2 => TradeGood::Food,
            3 => TradeGood::Ore,
            4 => TradeGood::Games,
            5 => TradeGood::Firearms,
            6 => TradeGood::Medicine,
            7 => TradeGood::Machines,
            8 => TradeGood::Narcotics,
            _ => TradeGood::Robots,
        }
    }
    
    pub fn all() -> [TradeGood; 10] {
        [
            TradeGood::Water,
            TradeGood::Furs,
            TradeGood::Food,
            TradeGood::Ore,
            TradeGood::Games,
            TradeGood::Firearms,
            TradeGood::Medicine,
            TradeGood::Machines,
            TradeGood::Narcotics,
            TradeGood::Robots,
        ]
    }
}

/// Trade item constants from original Global.c
pub const TRADE_ITEMS: &[TradeItem] = &[
    // Water
    TradeItem {
        name: "Water",
        min_tech_prod: 0,
        min_tech_use: 0,
        tech_top_prod: 2,
        base_price: 30,
        inc_price_per_level: 3,
        variance: 4,
        price_increase_event: 0, // DROUGHT
        resource_low_price: 4,   // LOTSOFWATER
        resource_high_price: 3,  // DESERT
        min_price: 30,
        max_price: 50,
        rounding: 1,
    },
    // Furs
    TradeItem {
        name: "Furs",
        min_tech_prod: 0,
        min_tech_use: 0,
        tech_top_prod: 0,
        base_price: 250,
        inc_price_per_level: -10,
        variance: 10,
        price_increase_event: -1,
        resource_low_price: 7,  // RICHFAUNA
        resource_high_price: 8, // LIFELESS
        min_price: 160,
        max_price: 340,
        rounding: 5,
    },
    // Food
    TradeItem {
        name: "Food",
        min_tech_prod: 1,
        min_tech_use: 0,
        tech_top_prod: 1,
        base_price: 100,
        inc_price_per_level: 5,
        variance: 5,
        price_increase_event: 1, // CROPFAILURE
        resource_low_price: 5,   // RICHSOIL
        resource_high_price: 6,  // POORSOIL
        min_price: 90,
        max_price: 160,
        rounding: 5,
    },
    // Ore
    TradeItem {
        name: "Ore",
        min_tech_prod: 2,
        min_tech_use: 2,
        tech_top_prod: 3,
        base_price: 350,
        inc_price_per_level: 20,
        variance: 10,
        price_increase_event: 2, // WAR
        resource_low_price: 1,   // MINERALRICH
        resource_high_price: 2,  // MINERALPOOR
        min_price: 350,
        max_price: 420,
        rounding: 10,
    },
    // Games
    TradeItem {
        name: "Games",
        min_tech_prod: 3,
        min_tech_use: 1,
        tech_top_prod: 6,
        base_price: 250,
        inc_price_per_level: -10,
        variance: 5,
        price_increase_event: 3, // BOREDOM
        resource_low_price: 11,  // ARTISTIC
        resource_high_price: -1,
        min_price: 160,
        max_price: 270,
        rounding: 5,
    },
    // Firearms
    TradeItem {
        name: "Firearms",
        min_tech_prod: 3,
        min_tech_use: 1,
        tech_top_prod: 5,
        base_price: 1250,
        inc_price_per_level: -75,
        variance: 100,
        price_increase_event: 2, // WAR
        resource_low_price: 12,  // WARLIKE
        resource_high_price: -1,
        min_price: 600,
        max_price: 1100,
        rounding: 25,
    },
    // Medicine
    TradeItem {
        name: "Medicine",
        min_tech_prod: 4,
        min_tech_use: 1,
        tech_top_prod: 6,
        base_price: 650,
        inc_price_per_level: -20,
        variance: 10,
        price_increase_event: 4, // PLAGUE
        resource_low_price: 10,  // LOTSOFHERBS
        resource_high_price: -1,
        min_price: 400,
        max_price: 700,
        rounding: 25,
    },
    // Machines
    TradeItem {
        name: "Machines",
        min_tech_prod: 4,
        min_tech_use: 3,
        tech_top_prod: 5,
        base_price: 900,
        inc_price_per_level: -30,
        variance: 5,
        price_increase_event: 5, // LACKOFWORKERS
        resource_low_price: -1,
        resource_high_price: -1,
        min_price: 600,
        max_price: 800,
        rounding: 25,
    },
    // Narcotics
    TradeItem {
        name: "Narcotics",
        min_tech_prod: 5,
        min_tech_use: 0,
        tech_top_prod: 5,
        base_price: 3500,
        inc_price_per_level: -125,
        variance: 150,
        price_increase_event: 6, // BOREDOM
        resource_low_price: 9,   // WEIRDMUSHROOMS
        resource_high_price: -1,
        min_price: 2000,
        max_price: 3000,
        rounding: 50,
    },
    // Robots
    TradeItem {
        name: "Robots",
        min_tech_prod: 6,
        min_tech_use: 4,
        tech_top_prod: 7,
        base_price: 5000,
        inc_price_per_level: -150,
        variance: 100,
        price_increase_event: 5, // LACKOFWORKERS
        resource_low_price: -1,
        resource_high_price: -1,
        min_price: 3500,
        max_price: 5000,
        rounding: 100,
    },
];
