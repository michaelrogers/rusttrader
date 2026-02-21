// Space encounter system (pirates, police, traders)

use crate::types::{GameState, TradeGood};

#[derive(Clone, Debug, PartialEq)]
pub enum EncounterType {
    Trader,
    Pirate,
    Police,
    SpaceMonster,
}

#[derive(Clone, Debug)]
pub struct Encounter {
    pub encounter_type: EncounterType,
    pub description: String,
    pub ship_name: String,
    pub ship_type: usize,
    pub distance_clicks: i32,
    #[allow(dead_code)]
    pub system_name: String,
}

impl Encounter {
    pub fn new(
        encounter_type: EncounterType,
        description: String,
        ship_name: String,
        ship_type: usize,
        distance_clicks: i32,
        system_name: String,
    ) -> Self {
        Encounter {
            encounter_type,
            description,
            ship_name,
            ship_type,
            distance_clicks,
            system_name,
        }
    }
    
    pub fn icon_name(&self) -> &'static str {
        match self.encounter_type {
            EncounterType::Trader => "trader",
            EncounterType::Pirate => "pirate",
            EncounterType::Police => "police",
            EncounterType::SpaceMonster => "alien",
        }
    }

    pub fn get_color_rgb(&self) -> (u8, u8, u8) {
        match self.encounter_type {
            EncounterType::Trader => (0, 100, 200),
            EncounterType::Pirate => (200, 0, 0),
            EncounterType::Police => (0, 150, 0),
            EncounterType::SpaceMonster => (150, 0, 150),
        }
    }

    pub fn ship_type_name(&self) -> &'static str {
        use crate::types::ship::SHIP_TYPES;
        // Extra ships beyond SHIP_TYPES array (indices 10-14)
        const EXTRA_SHIP_NAMES: &[&str] = &["Monster", "Dragonfly", "Mantis", "Scarab", "Bottle"];
        if self.ship_type < SHIP_TYPES.len() {
            SHIP_TYPES[self.ship_type].name
        } else if self.ship_type - SHIP_TYPES.len() < EXTRA_SHIP_NAMES.len() {
            EXTRA_SHIP_NAMES[self.ship_type - SHIP_TYPES.len()]
        } else {
            "Flea"
        }
    }
}

/// Map encounter type to an appropriate ship type index
fn encounter_ship_type(encounter_type: &EncounterType, seed: u64) -> usize {
    match encounter_type {
        // Traders use purchasable ships (indices 0-9)
        EncounterType::Trader => (seed % 10) as usize,
        // Pirates bias toward heavier ships (indices 4-9)
        EncounterType::Pirate => 4 + (seed % 6) as usize,
        // Police use mid-range ships (indices 2-7)
        EncounterType::Police => 2 + (seed % 6) as usize,
        // Space monster is always index 10
        EncounterType::SpaceMonster => 10,
    }
}

pub fn check_for_encounter(game_state: &GameState) -> Option<Encounter> {
    // Original Space Trader encounter system:
    // - Uses GetRandom(44 - (2 * Difficulty)) for encounter test
    // - On Easy: GetRandom(42), Hard: GetRandom(40), etc.
    // - Compared against politics strength values (typically 0-7)
    // - This gives roughly 15-20% base encounter rate
    // - Doubled rate (50%+) for more frequent encounters as requested
    
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};
    
    let mut hasher = RandomState::new().build_hasher();
    game_state.current_system_id.hash(&mut hasher);
    let day_hash = (game_state.days as u64).wrapping_mul(73856093);
    let seed = hasher.finish() ^ day_hash;
    
    // Encounter probability: ~50% chance (increased from 20%)
    // Original was roughly 15-20%, we're making it much more frequent
    let encounter_roll = (seed % 100) as i32;
    
    if encounter_roll < 50 {
        let encounter_type_roll = (seed / 100) % 4;
        let encounter_type = match encounter_type_roll {
            0 => EncounterType::Trader,
            1 => EncounterType::Pirate,
            2 => EncounterType::Police,
            _ => EncounterType::SpaceMonster,
        };
        
        let distance = ((seed / 500) % 30) as i32 + 10;
        let system = &game_state.solar_systems[game_state.current_system_id];
        
        let (description, ship_name) = generate_encounter_description(&encounter_type, &system.name);
        let ship_type_idx = encounter_ship_type(&encounter_type, seed);
        
        Some(Encounter::new(
            encounter_type,
            description,
            ship_name,
            ship_type_idx,
            distance,
            system.name.clone(),
        ))
    } else {
        None
    }
}

fn generate_encounter_description(
    encounter_type: &EncounterType,
    system_name: &str,
) -> (String, String) {
    let trader_ships = vec![
        "bumblebee", "merchant", "freighter", "hauler", "wheeler",
    ];
    let pirate_ships = vec![
        "corsair", "marauder", "raider", "scourge", "dreadnought",
    ];
    let police_ships = vec![
        "enforcer", "guardian", "sentinel", "patrol", "defender",
    ];
    let monster_ships = vec![
        "creature", "beast", "leviathan", "wyrm", "anomaly",
    ];
    
    match encounter_type {
        EncounterType::Trader => {
            let ship = trader_ships[system_name.len() % trader_ships.len()];
            (
                format!(
                    "At some distance from {}, you encounter a trader {}.\n\nIt ignores you.",
                    system_name, ship
                ),
                ship.to_string(),
            )
        }
        EncounterType::Pirate => {
            let ship = pirate_ships[system_name.len() % pirate_ships.len()];
            (
                format!(
                    "A pirate {} appears, weapons charged!\n\nThey demand your cargo or your life!",
                    ship
                ),
                ship.to_string(),
            )
        }
        EncounterType::Police => {
            let ship = police_ships[system_name.len() % police_ships.len()];
            (
                format!(
                    "A {} vessel approaches.\n\nThey scan your cargo manifest and ask your business here.",
                    ship
                ),
                ship.to_string(),
            )
        }
        EncounterType::SpaceMonster => {
            let ship = monster_ships[system_name.len() % monster_ships.len()];
            (
                format!(
                    "A massive space {} emerges from the cosmic void!\n\nIt appears to view your ship as either prey or threat.",
                    ship
                ),
                ship.to_string(),
            )
        }
    }
}

pub fn resolve_encounter(
    game_state: &mut GameState,
    encounter: &Encounter,
    player_choice: EncounterChoice,
) -> String {
    let seed = encounter_seed(game_state, encounter.distance_clicks);
    match (&encounter.encounter_type, player_choice) {
        (EncounterType::Trader, EncounterChoice::Ignore) => {
            let tip = 50 + (seed % 100) as i32;
            game_state.credits += tip;
            game_state.police_record_score = (game_state.police_record_score - 2).max(0);
            format!("The trader shares a market tip. You gain {} credits.", tip)
        }
        (EncounterType::Trader, EncounterChoice::Attack) => {
            game_state.police_record_score += 15;
            game_state.reputation_score = (game_state.reputation_score - 5).min(100);
            let loot = 300 + (seed % 200) as i32;
            game_state.credits += loot;
            if let Some(cargo_msg) = capture_cargo(game_state, seed) {
                format!("You attack the {} and take {} credits. {}", encounter.ship_name, loot, cargo_msg)
            } else {
                format!("You attack the {} and take {} credits!", encounter.ship_name, loot)
            }
        }
        
        (EncounterType::Pirate, EncounterChoice::Attack) => {
            let player_power = game_state.ship.weapon_rating as i32 + if game_state.ship.shield_installed { 2 } else { 0 };
            let enemy_power = 3 + (seed % 4) as i32;
            let damage = (22 - (player_power * 2) + enemy_power).max(5);
            apply_damage(&mut game_state.ship.hull, damage);
            let bounty = 200 + (player_power * 40);
            game_state.credits += bounty;
            game_state.reputation_score += 2;
            format!("You drive off the pirates. Hull -{}; bounty +{} cr.", damage, bounty)
        }
        (EncounterType::Pirate, EncounterChoice::Ignore) => {
            if let Some((good_idx, amount)) = steal_cargo(game_state, seed) {
                let good = TradeGood::from_index(good_idx);
                format!("Pirates raid your hold and steal {} {}.", amount, good.name())
            } else {
                let loss = (game_state.credits / 5).min(3000).max(50);
                game_state.credits = (game_state.credits - loss).max(0);
                format!("You pay {} credits in tribute to escape.", loss)
            }
        }
        
        (EncounterType::Police, EncounterChoice::Attack) => {
            game_state.police_record_score += 60;
            let damage = 12 + (seed % 10) as i32;
            apply_damage(&mut game_state.ship.hull, damage);
            format!("You fire on the police! Hull -{}. Wanted level increased.", damage)
        }
        (EncounterType::Police, EncounterChoice::Ignore) => {
            let illegal = illegal_cargo_total(game_state);
            if illegal > 0 {
                let fine = (illegal * 150).min(6000).max(200);
                confiscate_illegal(game_state);
                game_state.credits = (game_state.credits - fine).max(0);
                game_state.police_record_score += 10;
                format!("Police confiscate illegal goods and fine you {} credits.", fine)
            } else if game_state.police_record_score > 50 {
                let fine = (game_state.police_record_score * 50).min(5000);
                game_state.credits = (game_state.credits - fine).max(0);
                game_state.police_record_score -= 25;
                format!("You pay a fine of {} credits.", fine)
            } else {
                "They scan your cargo and wave you through.".to_string()
            }
        }
        
        (EncounterType::SpaceMonster, EncounterChoice::Attack) => {
            let player_power = game_state.ship.weapon_rating as i32 + if game_state.ship.shield_installed { 2 } else { 0 };
            let damage = (35 - player_power * 2).max(10) + (seed % 10) as i32;
            apply_damage(&mut game_state.ship.hull, damage);
            let salvage = 400 + (seed % 200) as i32;
            game_state.credits += salvage;
            format!("Brutal combat! Hull -{}; salvage +{} cr.", damage, salvage)
        }
        (EncounterType::SpaceMonster, EncounterChoice::Ignore) => {
            if game_state.ship.fuel > 25 {
                game_state.ship.fuel -= 25;
                "You accelerate away at full burn!".to_string()
            } else {
                let damage = 18 + (seed % 12) as i32;
                apply_damage(&mut game_state.ship.hull, damage);
                format!("You manage to escape but take {} damage!", damage)
            }
        }
    }
}

fn encounter_seed(game_state: &GameState, salt: i32) -> u32 {
    let base = (game_state.days * 31 + game_state.current_system_id as i32 * 97 + salt) as u32;
    base ^ (game_state.ship.hull as u32 * 17)
}

fn apply_damage(hull: &mut i32, damage: i32) {
    *hull = (*hull - damage).max(0);
}

fn illegal_cargo_total(game_state: &GameState) -> i32 {
    let firearms = game_state.ship.cargo[TradeGood::Firearms as usize];
    let narcotics = game_state.ship.cargo[TradeGood::Narcotics as usize];
    firearms + narcotics
}

fn confiscate_illegal(game_state: &mut GameState) {
    game_state.ship.cargo[TradeGood::Firearms as usize] = 0;
    game_state.ship.cargo[TradeGood::Narcotics as usize] = 0;
}

fn steal_cargo(game_state: &mut GameState, seed: u32) -> Option<(usize, i32)> {
    let mut max_idx = None;
    let mut max_amt = 0;
    for i in 0..game_state.ship.cargo.len() {
        let amt = game_state.ship.cargo[i];
        if amt > max_amt {
            max_amt = amt;
            max_idx = Some(i);
        }
    }

    max_idx.map(|idx| {
        let amt = game_state.ship.cargo[idx];
        let steal = (1 + (seed % 3) as i32).min(amt.max(1)).max(1);
        game_state.ship.cargo[idx] -= steal;
        (idx, steal)
    })
}

fn capture_cargo(game_state: &mut GameState, seed: u32) -> Option<String> {
    if game_state.ship.cargo_bays_available() <= 0 {
        return None;
    }
    let idx = (seed % 10) as usize;
    let amount = (1 + (seed % 3) as i32).min(game_state.ship.cargo_bays_available());
    if amount <= 0 {
        return None;
    }
    game_state.ship.cargo[idx] += amount;
    let good = TradeGood::from_index(idx);
    Some(format!("You seize {} {}.", amount, good.name()))
}

#[derive(Clone, Copy, PartialEq)]
pub enum EncounterChoice {
    Attack,
    Ignore,
}
