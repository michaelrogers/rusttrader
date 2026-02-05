// Constants from the original Space Trader
#![allow(dead_code)]

// Galaxy constants
pub const MAX_SOLAR_SYSTEM: usize = 120;
pub const MAX_WORMHOLE: usize = 6;
pub const CLOSE_DISTANCE: i32 = 13;

// Ship constants
pub const MAX_SHIP_TYPE: usize = 9;
pub const EXTRA_SHIPS: usize = 5; // Ships that can't be bought normally

// Equipment constants
pub const MAX_WEAPON: usize = 3;
pub const MAX_SHIELD: usize = 3;
pub const MAX_GADGET: usize = 3;
pub const MAX_CREW: usize = 3;

pub const MAX_WEAPON_TYPE: usize = 5;
pub const MAX_SHIELD_TYPE: usize = 5;
pub const MAX_GADGET_TYPE: usize = 5;
pub const EXTRA_WEAPONS: usize = 1;
pub const EXTRA_GADGETS: usize = 1;

// Trade constants
pub const MAX_TRADE_ITEM: usize = 10;

// Special events
pub const MAX_SPECIAL_EVENT: usize = 37;
pub const MOON_COST: i32 = 500000;

// Skills
pub const MAX_SKILL: i32 = 10;

// Difficulty
pub const MAX_DIFFICULTY: usize = 5;

// Politics types
pub const MAX_POLITICS: usize = 8;

// Tech levels
pub const MAX_TECH_LEVEL: usize = 8;

// System sizes
pub const MAX_SIZE: usize = 5;

// Special resources
pub const MAX_RESOURCES: usize = 13;

// Reputation
pub const MAX_REPUTATION: usize = 10;
pub const HARMLESS_REP: i32 = 0;
pub const MOSTLY_HARMLESS_REP: i32 = 10;
pub const POOR_REP: i32 = 20;
pub const AVERAGE_SCORE: i32 = 40;
pub const ABOVE_AVERAGE_SCORE: i32 = 80;
pub const COMPETENT_REP: i32 = 150;
pub const DANGEROUS_REP: i32 = 300;
pub const DEADLY_REP: i32 = 600;
pub const ELITE_SCORE: i32 = 1400;

// Police record
pub const MAX_POLICE_RECORD: usize = 10;
pub const PSYCHOPATH_SCORE: i32 = -100;
pub const VILLAIN_SCORE: i32 = -70;
pub const CRIMINAL_SCORE: i32 = -30;
pub const CROOK_SCORE: i32 = -10;
pub const DUBIOUS_SCORE: i32 = -5;
pub const CLEAN_SCORE: i32 = 0;
pub const LAWFUL_SCORE: i32 = 5;
pub const TRUSTED_SCORE: i32 = 10;
pub const HELPER_SCORE: i32 = 25;
pub const HERO_SCORE: i32 = 75;

// Fuel
pub const FUEL_TANK_CAPACITY: i32 = 10; // Each tank holds fuel for 10 parsecs

// Starting money
pub const STARTING_CREDITS: i32 = 1000;
