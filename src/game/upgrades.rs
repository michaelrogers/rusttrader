// Ship upgrade system

use crate::types::{GameState, solar_system::TechLevel};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpgradeType {
    CargoHold,      // +5 cargo bays
    FuelTank,       // +50 fuel capacity
    Weapons,        // +5 armor/combat rating
    ShieldGenerator, // New: reduces damage by 20%
    Hull,           // +10 hull integrity
}

impl UpgradeType {
    pub fn name(&self) -> &str {
        match self {
            UpgradeType::CargoHold => "Cargo Hold Expansion",
            UpgradeType::FuelTank => "Fuel Tank Upgrade",
            UpgradeType::Weapons => "Weapons System",
            UpgradeType::ShieldGenerator => "Shield Generator",
            UpgradeType::Hull => "Hull Reinforcement",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            UpgradeType::CargoHold => "Increases cargo capacity by 5 bays",
            UpgradeType::FuelTank => "Increases fuel tank capacity by 50 units",
            UpgradeType::Weapons => "Improves combat capabilities",
            UpgradeType::ShieldGenerator => "Reduces incoming damage by 20%",
            UpgradeType::Hull => "Increases hull strength by 10 points",
        }
    }

    pub fn base_cost(&self) -> i32 {
        match self {
            UpgradeType::CargoHold => 5000,
            UpgradeType::FuelTank => 4000,
            UpgradeType::Weapons => 10000,
            UpgradeType::ShieldGenerator => 15000,
            UpgradeType::Hull => 6000,
        }
    }

    pub fn min_tech_level(&self) -> TechLevel {
        match self {
            UpgradeType::CargoHold => TechLevel::Agricultural,
            UpgradeType::FuelTank => TechLevel::Medieval,
            UpgradeType::Weapons => TechLevel::Renaissance,
            UpgradeType::ShieldGenerator => TechLevel::HiTech,
            UpgradeType::Hull => TechLevel::Medieval,
        }
    }
}

pub fn get_available_upgrades(game_state: &GameState) -> Vec<(UpgradeType, i32)> {
    let system = &game_state.solar_systems[game_state.current_system_id];
    let tech_level = system.tech_level;

    vec![
        UpgradeType::CargoHold,
        UpgradeType::FuelTank,
        UpgradeType::Weapons,
        UpgradeType::ShieldGenerator,
        UpgradeType::Hull,
    ]
    .into_iter()
    .filter_map(|upgrade| {
        if upgrade.min_tech_level() <= tech_level {
            let cost = calculate_upgrade_cost(upgrade, game_state);
            Some((upgrade, cost))
        } else {
            None
        }
    })
    .collect()
}

pub fn calculate_upgrade_cost(upgrade: UpgradeType, _game_state: &GameState) -> i32 {
    let base_cost = upgrade.base_cost();

    // Adjust cost based on system tech level and politics
    let tech_multiplier = (upgrade.min_tech_level() as i32 as f32 + 1.0) / 2.0;
    ((base_cost as f32) * tech_multiplier) as i32
}

pub fn purchase_upgrade(
    game_state: &mut GameState,
    upgrade: UpgradeType,
) -> Result<String, String> {
    let system = &game_state.solar_systems[game_state.current_system_id];

    // Check if available at this tech level
    if upgrade.min_tech_level() > system.tech_level {
        return Err(format!(
            "This system's tech level is too low for {}",
            upgrade.name()
        ));
    }

    let cost = calculate_upgrade_cost(upgrade, game_state);

    // Check if already purchased
    match upgrade {
        UpgradeType::CargoHold => {
            if game_state.ship.cargo_expansion >= 2 {
                return Err("Already at maximum cargo capacity".to_string());
            }
        }
        UpgradeType::FuelTank => {
            if game_state.ship.fuel_expansion >= 2 {
                return Err("Already at maximum fuel capacity".to_string());
            }
        }
        UpgradeType::Weapons => {
            if game_state.ship.weapon_rating >= 5 {
                return Err("Weapons already at maximum capability".to_string());
            }
        }
        UpgradeType::ShieldGenerator => {
            if game_state.ship.shield_installed {
                return Err("Shield generator already installed".to_string());
            }
        }
        UpgradeType::Hull => {
            if game_state.ship.hull_reinforcement >= 2 {
                return Err("Hull already fully reinforced".to_string());
            }
        }
    }

    // Check credits
    if game_state.credits < cost {
        return Err(format!(
            "Not enough credits. Need {} but only have {}",
            cost, game_state.credits
        ));
    }

    // Apply upgrade
    game_state.credits -= cost;

    match upgrade {
        UpgradeType::CargoHold => {
            game_state.ship.cargo_expansion += 1;
            Ok(format!("Installed cargo hold expansion! (+5 bays)"))
        }
        UpgradeType::FuelTank => {
            game_state.ship.fuel_expansion += 1;
            Ok(format!("Installed fuel tank upgrade! (+50 capacity)"))
        }
        UpgradeType::Weapons => {
            game_state.ship.weapon_rating += 1;
            Ok(format!("Installed weapons system upgrade!"))
        }
        UpgradeType::ShieldGenerator => {
            game_state.ship.shield_installed = true;
            Ok(format!("Installed shield generator!"))
        }
        UpgradeType::Hull => {
            game_state.ship.hull_reinforcement += 1;
            Ok(format!("Reinforced hull! (+10 integrity)"))
        }
    }
}
