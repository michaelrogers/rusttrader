// Space Trader - Rust Port
// Based on the original Space Trader for Palm OS by Pieter Spronck
// Licensed under GPL-2.0-or-later

mod types;
mod game;
mod ui;
mod save;
mod assets;

use macroquad::prelude::*;
use types::{GameState, TradeGood};
use assets::{GameAssets, draw_ship};
use game::trading::{buy_cargo, sell_cargo, max_buyable, buy_fuel, get_fuel_cost, max_fuel_buyable};
use game::pricing::{get_buy_price, determine_prices};
use game::travel::{warp_to_system, systems_in_range};
use game::encounter::{check_for_encounter, resolve_encounter, Encounter, EncounterChoice};
use game::upgrades::{get_available_upgrades, purchase_upgrade};
use game::repair::{calculate_repair_cost_per_point, get_max_hull, calculate_full_repair_cost, repair_ship, repair_full, can_repair};
use game::ships::{get_purchasable_ships, purchase_ship, get_current_ship_info};

#[derive(PartialEq)]
enum GameScreen {
    Main,
    Trading,
    Warp,
    SystemInfo,
    Encounter,
    Shipyard,
    Repair,
    ShipShop,
}

fn draw_warp_screen(game_state: &GameState, selected: usize, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));
    
    // Title
    draw_text(
        "Warp - Select Destination",
        20.0,
        30.0,
        24.0,
        GOLD,
    );
    
    // Current location
    draw_text(
        &format!("Current: {} | Fuel: {}", game_state.current_system_name(), game_state.ship.fuel),
        20.0,
        60.0,
        18.0,
        WHITE,
    );
    
    // Get systems in range
    let systems = systems_in_range(game_state);
    
    if systems.is_empty() {
        draw_text(
            "No systems in fuel range!",
            screen_width() / 2.0 - 120.0,
            screen_height() / 2.0,
            20.0,
            RED,
        );
        draw_text(
            "Return to station to refuel (Coming soon)",
            screen_width() / 2.0 - 160.0,
            screen_height() / 2.0 + 30.0,
            16.0,
            YELLOW,
        );
    } else {
        // Column headers
        let y_start = 100.0;
        draw_text("System", 20.0, y_start, 16.0, LIGHTGRAY);
        draw_text("Distance", 220.0, y_start, 16.0, LIGHTGRAY);
        draw_text("Fuel Cost", 340.0, y_start, 16.0, LIGHTGRAY);
        
        // Draw systems list
        for (i, &(system_id, distance)) in systems.iter().enumerate() {
            let y = y_start + 35.0 + (i as f32 * 25.0);
            let system = &game_state.solar_systems[system_id];
            let fuel_cost = distance.ceil() as i32;
            
            // Highlight selected
            let color = if i == selected { YELLOW } else { WHITE };
            if i == selected {
                draw_rectangle(15.0, y - 18.0, screen_width() - 30.0, 23.0, Color::from_rgba(50, 50, 100, 128));
            }
            
            // Color code by fuel availability
            let fuel_color = if fuel_cost <= game_state.ship.fuel { GREEN } else { RED };
            
            draw_text(&system.name, 20.0, y, 16.0, color);
            draw_text(&format!("{:.1} ly", distance), 220.0, y, 16.0, color);
            draw_text(&format!("{}", fuel_cost), 340.0, y, 16.0, fuel_color);
            
            // Show if visited
            if system.visited {
                draw_text("✓", 420.0, y, 16.0, SKYBLUE);
            }
        }
    }
    
    // Instructions
    let inst_y = screen_height() - 100.0;
    draw_text("Controls:", 20.0, inst_y, 18.0, LIGHTGRAY);
    draw_text("↑↓ - Select  |  ENTER/W - Warp  |  ESC/Q - Cancel", 
        20.0, inst_y + 25.0, 14.0, LIGHTGRAY);
    
    // Show message if any
    if !message.is_empty() {
        let msg_width = measure_text(message, None, 20, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(msg_x - 10.0, screen_height() / 2.0 - 30.0, msg_width + 20.0, 50.0, 
            Color::from_rgba(0, 0, 0, 200));
        draw_text(message, msg_x, screen_height() / 2.0, 20.0, if message.contains("Successfully") { GREEN } else { RED });
    }
}

fn draw_system_info_screen(game_state: &GameState, show_newspaper: bool, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));
    
    // Header
    draw_rectangle(0.0, 0.0, screen_width(), 40.0, Color::from_rgba(80, 0, 160, 255));
    draw_text("System Info", 20.0, 20.0, 24.0, WHITE);
    draw_text("I", screen_width() - 40.0, 20.0, 20.0, WHITE);
    
    let current_system = &game_state.solar_systems[game_state.current_system_id];
    let tech_names = ["Pre-agricultural", "Agricultural", "Medieval", "Renaissance", "Early Industrial", 
                      "Industrial", "Post-Industrial", "Hi-Tech"];
    let tech_idx = current_system.tech_level as usize;
    let tech_name = if tech_idx < tech_names.len() { tech_names[tech_idx] } else { "Unknown" };
    
    let politics_names = ["Anarchy", "Capitalist", "Communist", "Confederacy", "Corporate", "Cybernetic",
                         "Democracy", "Dictatorship", "Fascist", "Feudal", "Military", "Monarchy",
                         "Pacifist", "Socialist", "Satori", "Technocracy", "Theocracy"];
    let pol_idx = current_system.politics as usize;
    let politics_name = if pol_idx < politics_names.len() { politics_names[pol_idx] } else { "Unknown" };
    
    let size_names = ["Tiny", "Small", "Medium", "Large", "Huge"];
    let size_idx = current_system.size as usize;
    let size_name = if size_idx < size_names.len() { size_names[size_idx] } else { "Unknown" };
    
    // System info display
    let left = 40.0;
    let y_start = 70.0;
    let line_height = 35.0;
    
    draw_text("Name:", left, y_start, 18.0, LIGHTGRAY);
    draw_text(&current_system.name, left + 150.0, y_start, 18.0, WHITE);
    
    draw_text("Size:", left, y_start + line_height, 18.0, LIGHTGRAY);
    draw_text(size_name, left + 150.0, y_start + line_height, 18.0, WHITE);
    
    draw_text("Tech Level:", left, y_start + line_height * 2.0, 18.0, LIGHTGRAY);
    draw_text(tech_name, left + 150.0, y_start + line_height * 2.0, 18.0, WHITE);
    
    draw_text("Government:", left, y_start + line_height * 3.0, 18.0, LIGHTGRAY);
    draw_text(politics_name, left + 150.0, y_start + line_height * 3.0, 18.0, WHITE);
    
    // Newspaper dialog
    if show_newspaper {
        let dialog_width = 500.0;
        let dialog_height = 180.0;
        let dialog_x = (screen_width() - dialog_width) / 2.0;
        let dialog_y = screen_height() / 2.0 - 50.0;
        
        draw_rectangle(dialog_x, dialog_y, dialog_width, dialog_height, 
            Color::from_rgba(80, 0, 160, 255));
        draw_rectangle(dialog_x + 2.0, dialog_y + 2.0, dialog_width - 4.0, dialog_height - 4.0,
            Color::from_rgba(200, 200, 255, 255));
        
        draw_text("Buy Newspaper?", dialog_x + 20.0, dialog_y + 20.0, 18.0, BLACK);
        draw_text(&format!("Local newspaper costs 1 credit."), dialog_x + 20.0, dialog_y + 50.0, 14.0, BLACK);
        draw_text("Do you wish to buy a copy?", dialog_x + 20.0, dialog_y + 70.0, 14.0, BLACK);
        
        // Buttons
        draw_rectangle(dialog_x + 50.0, dialog_y + 110.0, 120.0, 40.0, WHITE);
        draw_text("Buy (B)", dialog_x + 70.0, dialog_y + 125.0, 14.0, BLACK);
        
        draw_rectangle(dialog_x + 250.0, dialog_y + 110.0, 120.0, 40.0, WHITE);
        draw_text("Cancel (C)", dialog_x + 265.0, dialog_y + 125.0, 14.0, BLACK);
    }
    
    // Controls
    let inst_y = screen_height() - 60.0;
    draw_text("Controls:", 20.0, inst_y, 14.0, LIGHTGRAY);
    if !show_newspaper {
        draw_text("N - Buy Newspaper  |  ESC/Q - Back", 20.0, inst_y + 20.0, 12.0, LIGHTGRAY);
    } else {
        draw_text("B - Buy  |  C - Cancel", 20.0, inst_y + 20.0, 12.0, LIGHTGRAY);
    }
    
    if !message.is_empty() {
        draw_text(message, 20.0, screen_height() - 30.0, 14.0, GREEN);
    }
}

fn draw_encounter_screen(encounter: &Encounter, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));
    
    // Header
    draw_rectangle(0.0, 0.0, screen_width(), 50.0, Color::from_rgba(80, 0, 160, 255));
    draw_text("Encounter", 20.0, 25.0, 28.0, WHITE);
    draw_text("!", screen_width() - 40.0, 25.0, 28.0, YELLOW);
    
    // Background color for content area
    draw_rectangle(10.0, 60.0, screen_width() - 20.0, screen_height() - 130.0,
        Color::from_rgba(240, 240, 250, 255));
    
    // Draw ship sprites (using simple geometric shapes for now)
    let ship_y = 140.0;
    let left_ship_x = 100.0;
    let right_ship_x = screen_width() - 200.0;
    
    // Left ship (player's ship - blue)
    draw_triangle(
        vec2(left_ship_x, ship_y - 25.0),
        vec2(left_ship_x - 20.0, ship_y + 25.0),
        vec2(left_ship_x + 20.0, ship_y + 25.0),
        BLUE,
    );
    
    // Right ship (enemy/encounter ship)
    let (r, g, b) = encounter.get_color_rgb();
    let encounter_color = Color::from_rgba(r, g, b, 255);
    
    // Draw a more complex shape for variety
    draw_circle(right_ship_x, ship_y, 20.0, encounter_color);
    draw_rectangle(right_ship_x - 15.0, ship_y - 10.0, 30.0, 20.0, encounter_color);
    
    // Sun in corner
    draw_circle(screen_width() - 80.0, 100.0, 15.0, YELLOW);
    
    // Description text
    let text_x = 40.0;
    let text_y = 280.0;
    let max_width = screen_width() - 80.0;
    
    draw_text_with_limits(&encounter.description, text_x, text_y, 20.0, BLACK, max_width);
    
    // Action buttons
    let button_y = screen_height() - 100.0;
    let button_width = 140.0;
    let button_height = 50.0;
    let button_spacing = 200.0;
    
    let attack_x = screen_width() / 2.0 - button_spacing / 2.0 - button_width / 2.0;
    let ignore_x = screen_width() / 2.0 + button_spacing / 2.0 - button_width / 2.0;
    
    // Attack button
    draw_rectangle(attack_x, button_y, button_width, button_height, WHITE);
    draw_rectangle_lines(attack_x, button_y, button_width, button_height, 3.0, BLACK);
    draw_text("Attack (A)", attack_x + 20.0, button_y + 30.0, 18.0, BLACK);
    
    // Ignore button
    draw_rectangle(ignore_x, button_y, button_width, button_height, WHITE);
    draw_rectangle_lines(ignore_x, button_y, button_width, button_height, 3.0, BLACK);
    draw_text("Ignore (I)", ignore_x + 20.0, button_y + 30.0, 18.0, BLACK);
    
    // Message
    if !message.is_empty() {
        draw_text(message, 20.0, screen_height() - 25.0, 14.0, GREEN);
    }
}

fn draw_text_with_limits(text: &str, x: f32, mut y: f32, font_size: f32, color: Color, max_width: f32) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();
    
    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        
        let test_width = measure_text(&test_line, None, font_size as u16, 1.0).width;
        
        if test_width > max_width {
            if !current_line.is_empty() {
                draw_text(&current_line, x, y, font_size, color);
                y += font_size + 5.0;
            }
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
    }
    
    if !current_line.is_empty() {
        draw_text(&current_line, x, y, font_size, color);
    }
}

fn draw_ship_shop_screen(game_state: &GameState, selected: usize, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));
    
    // Header
    draw_rectangle(0.0, 0.0, screen_width(), 50.0, Color::from_rgba(0, 160, 80, 255));
    draw_text("Ship Shop", 20.0, 25.0, 28.0, WHITE);
    draw_text(&format!("Credits: {}", game_state.credits), screen_width() - 200.0, 25.0, 18.0, GOLD);
    
    let _current_ship = get_current_ship_info(game_state);
    let purchasable = get_purchasable_ships(game_state.solar_systems[game_state.current_system_id].tech_level as i32);
    
    if purchasable.is_empty() {
        draw_text(
            "No ships available at this tech level",
            screen_width() / 2.0 - 180.0,
            screen_height() / 2.0,
            20.0,
            LIGHTGRAY,
        );
    } else {
        // Column headers
        let y_start = 80.0;
        let name_col = 40.0;
        let desc_col = 150.0;
        let stats_col = 450.0;
        let cost_col = screen_width() - 180.0;
        
        draw_text("Ship", name_col, y_start, 16.0, LIGHTGRAY);
        draw_text("Description", desc_col, y_start, 12.0, LIGHTGRAY);
        draw_text("Stats", stats_col, y_start, 12.0, LIGHTGRAY);
        draw_text("Cost", cost_col, y_start, 16.0, LIGHTGRAY);
        
        // Draw ships list
        for (i, ship) in purchasable.iter().enumerate() {
            let y = y_start + 40.0 + (i as f32 * 70.0);
            
            // Highlight selected
            if i == selected {
                draw_rectangle(15.0, y - 20.0, screen_width() - 30.0, 65.0, Color::from_rgba(50, 100, 50, 128));
            }
            
            // Current ship indicator
            let is_current = ship.ship_type_id == game_state.ship.ship_type;
            let color = if is_current { YELLOW } else { WHITE };
            
            let ship_label = if is_current {
                format!("{} (CURRENT)", ship.name)
            } else {
                ship.name.to_string()
            };
            
            draw_text(&ship_label, name_col, y, 16.0, color);
            draw_text_with_limits(ship.description, desc_col, y, 11.0, LIGHTGRAY, 280.0);
            
            let stats = format!(
                "Cargo: {} | Weapon: {} | Shield: {} | Hull: {}",
                ship.cargo_bays, ship.weapon_slots, ship.shield_slots, ship.hull_strength
            );
            draw_text(&stats, stats_col, y, 10.0, SKYBLUE);
            
            if is_current {
                draw_text("OWNED", cost_col, y, 14.0, YELLOW);
            } else {
                let cost = ship.upgrade_cost_from_current(game_state);
                let cost_color = if game_state.credits >= cost { GREEN } else { RED };
                draw_text(&format!("{} cr", cost), cost_col, y, 16.0, cost_color);
            }
        }
    }
    
    // Controls
    let inst_y = screen_height() - 100.0;
    draw_text("Controls:", 20.0, inst_y, 18.0, LIGHTGRAY);
    draw_text(
        "↑↓ - Select  |  ENTER/B - Buy  |  ESC/Q - Back",
        20.0,
        inst_y + 25.0,
        14.0,
        LIGHTGRAY,
    );
    
    // Show message if any
    if !message.is_empty() {
        let msg_width = measure_text(message, None, 18, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(msg_x - 10.0, screen_height() / 2.0 + 50.0, msg_width + 20.0, 50.0,
            Color::from_rgba(0, 0, 0, 200));
        let msg_color = if message.contains("Purchased") { GREEN } else { RED };
        draw_text(message, msg_x, screen_height() / 2.0 + 75.0, 18.0, msg_color);
    }
}

fn draw_repair_screen(game_state: &GameState, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));
    
    // Header
    draw_rectangle(0.0, 0.0, screen_width(), 50.0, Color::from_rgba(80, 0, 160, 255));
    draw_text("Repair Dock", 20.0, 25.0, 28.0, WHITE);
    draw_text(&format!("Credits: {}", game_state.credits), screen_width() - 200.0, 25.0, 18.0, GOLD);
    
    if !can_repair(game_state) {
        draw_text(
            "No repair facilities available at this tech level",
            screen_width() / 2.0 - 240.0,
            screen_height() / 2.0,
            20.0,
            RED,
        );
    } else {
        let max_hull = get_max_hull(game_state);
        let damage_taken = max_hull - game_state.ship.hull;
        let cost_per_point = calculate_repair_cost_per_point(game_state);
        let full_repair_cost = calculate_full_repair_cost(game_state);
        
        // Hull Status
        let left = 40.0;
        let y_start = 90.0;
        
        draw_text("Hull Status:", left, y_start, 18.0, LIGHTGRAY);
        
        let hull_color = if game_state.ship.hull > 15 { GREEN } else { RED };
        draw_text(
            &format!("{} / {} HP", game_state.ship.hull, max_hull),
            left,
            y_start + 30.0,
            18.0,
            hull_color,
        );
        
        // Damage bar
        let bar_width = 300.0;
        let bar_height = 20.0;
        let bar_x = left;
        let bar_y = y_start + 60.0;
        
        draw_rectangle(bar_x, bar_y, bar_width, bar_height, Color::from_rgba(50, 50, 50, 255));
        let repair_percentage = game_state.ship.hull as f32 / max_hull as f32;
        draw_rectangle(bar_x, bar_y, bar_width * repair_percentage, bar_height, GREEN);
        draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, WHITE);
        
        // Repair Options
        let option_y = y_start + 120.0;
        draw_text("Repair Options:", left, option_y, 18.0, LIGHTGRAY);
        
        // Repair 10 hp
        let repair_small = 10.min(damage_taken);
        let cost_small = calculate_repair_cost_per_point(game_state) * repair_small;
        let affordable_small = game_state.credits >= cost_small && damage_taken > 0;
        let color_small = if affordable_small { GREEN } else { RED };
        let cost_small_str = if cost_small > 0 { format!("{} cr", cost_small) } else { "FREE".to_string() };
        
        draw_text(
            &format!("1 - Repair {} HP ({})", repair_small, cost_small_str),
            left,
            option_y + 35.0,
            14.0,
            color_small,
        );
        
        // Repair 50 hp
        let repair_medium = 50.min(damage_taken);
        let cost_medium = calculate_repair_cost_per_point(game_state) * repair_medium;
        let affordable_medium = game_state.credits >= cost_medium && damage_taken > 0;
        let color_medium = if affordable_medium { GREEN } else { RED };
        let cost_medium_str = if cost_medium > 0 { format!("{} cr", cost_medium) } else { "FREE".to_string() };
        
        draw_text(
            &format!("2 - Repair {} HP ({})", repair_medium, cost_medium_str),
            left,
            option_y + 55.0,
            14.0,
            color_medium,
        );
        
        // Repair Full
        let affordable_full = game_state.credits >= full_repair_cost && damage_taken > 0;
        let color_full = if affordable_full { GREEN } else { RED };
        let full_repair_str = if full_repair_cost > 0 { format!("{} cr", full_repair_cost) } else { "FREE".to_string() };
        
        draw_text(
            &format!("3 - Repair All Damage ({})", full_repair_str),
            left,
            option_y + 75.0,
            14.0,
            color_full,
        );
        
        // Cost per point info
        draw_text(
            &format!("Cost per HP: {} cr", cost_per_point),
            left,
            option_y + 110.0,
            12.0,
            LIGHTGRAY,
        );
        
        if damage_taken == 0 {
            draw_text(
                "Ship is fully repaired!",
                left,
                option_y + 140.0,
                14.0,
                SKYBLUE,
            );
        }
    }
    
    // Controls
    let inst_y = screen_height() - 100.0;
    draw_text("Controls:", 20.0, inst_y, 18.0, LIGHTGRAY);
    draw_text(
        "1 - Repair 10 HP  |  2 - Repair 50 HP  |  3 - Repair All  |  ESC/Q - Back",
        20.0,
        inst_y + 25.0,
        14.0,
        LIGHTGRAY,
    );
    
    // Show message if any
    if !message.is_empty() {
        let msg_width = measure_text(message, None, 18, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(msg_x - 10.0, screen_height() / 2.0 + 50.0, msg_width + 20.0, 50.0,
            Color::from_rgba(0, 0, 0, 200));
        let msg_color = if message.contains("Repaired") { GREEN } else { RED };
        draw_text(message, msg_x, screen_height() / 2.0 + 75.0, 18.0, msg_color);
    }
}

fn draw_shipyard_screen(game_state: &GameState, selected: usize, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));
    
    // Header
    draw_rectangle(0.0, 0.0, screen_width(), 50.0, Color::from_rgba(80, 0, 160, 255));
    draw_text("Shipyard", 20.0, 25.0, 28.0, WHITE);
    draw_text(&format!("Credits: {}", game_state.credits), screen_width() - 200.0, 25.0, 18.0, GOLD);
    
    let upgrades = get_available_upgrades(game_state);
    
    if upgrades.is_empty() {
        draw_text(
            "No upgrades available at this tech level",
            screen_width() / 2.0 - 180.0,
            screen_height() / 2.0,
            20.0,
            LIGHTGRAY,
        );
    } else {
        // Column headers
        let y_start = 80.0;
        let name_col = 40.0;
        let desc_col = 250.0;
        let cost_col = screen_width() - 150.0;
        
        draw_text("Upgrade", name_col, y_start, 16.0, LIGHTGRAY);
        draw_text("Description", desc_col, y_start, 14.0, LIGHTGRAY);
        draw_text("Cost", cost_col, y_start, 16.0, LIGHTGRAY);
        
        // Draw upgrades list
        for (i, (upgrade, cost)) in upgrades.iter().enumerate() {
            let y = y_start + 40.0 + (i as f32 * 60.0);
            
            // Highlight selected
            if i == selected {
                draw_rectangle(15.0, y - 20.0, screen_width() - 30.0, 55.0, Color::from_rgba(50, 50, 100, 128));
            }
            
            let color = if i == selected { YELLOW } else { WHITE };
            
            draw_text(upgrade.name(), name_col, y, 16.0, color);
            draw_text_with_limits(upgrade.description(), desc_col, y, 12.0, LIGHTGRAY, 350.0);
            
            let cost_color = if game_state.credits >= *cost { GREEN } else { RED };
            draw_text(&format!("{} cr", cost), cost_col, y, 16.0, cost_color);
        }
    }
    
    // Controls
    let inst_y = screen_height() - 100.0;
    draw_text("Controls:", 20.0, inst_y, 18.0, LIGHTGRAY);
    draw_text(
        "↑↓ - Select  |  ENTER/P - Purchase  |  ESC/Q - Back",
        20.0,
        inst_y + 25.0,
        14.0,
        LIGHTGRAY,
    );
    
    // Show message if any
    if !message.is_empty() {
        let msg_width = measure_text(message, None, 18, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(msg_x - 10.0, screen_height() / 2.0 + 50.0, msg_width + 20.0, 50.0,
            Color::from_rgba(0, 0, 0, 200));
        let msg_color = if message.contains("Installed") || message.contains("upgraded") { GREEN } else { RED };
        draw_text(message, msg_x, screen_height() / 2.0 + 75.0, 18.0, msg_color);
    }
}

fn draw_trading_screen(game_state: &GameState, selected: usize, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));
    
    // Title
    draw_text(
        &format!("Trading - {}", game_state.current_system_name()),
        20.0,
        30.0,
        24.0,
        GOLD,
    );
    
    // Player info
    draw_text(
        &format!("Credits: {} cr", game_state.credits),
        20.0,
        60.0,
        18.0,
        WHITE,
    );
    
    draw_text(
        &format!("Cargo: {}/{}", game_state.ship.total_cargo(), game_state.ship.cargo_bays_available() + game_state.ship.total_cargo()),
        20.0,
        85.0,
        18.0,
        WHITE,
    );
    
    // Column headers
    let y_start = 120.0;
    draw_text("Good", 20.0, y_start, 16.0, LIGHTGRAY);
    draw_text("Price", 180.0, y_start, 16.0, LIGHTGRAY);
    draw_text("Avail", 260.0, y_start, 16.0, LIGHTGRAY);
    draw_text("Cargo", 340.0, y_start, 16.0, LIGHTGRAY);
    draw_text("Max", 420.0, y_start, 16.0, LIGHTGRAY);
    
    // Draw goods list
    let system_id = game_state.current_system_id;
    for i in 0..10 {
        let y = y_start + 35.0 + (i as f32 * 25.0);
        let good = TradeGood::from_index(i);
        let price = get_buy_price(game_state, good);
        let available = game_state.solar_systems[system_id].qty[i];
        let in_hold = game_state.ship.cargo[i];
        let max = max_buyable(game_state, good);
        
        // Highlight selected
        let color = if i == selected { YELLOW } else { WHITE };
        if i == selected {
            draw_rectangle(15.0, y - 18.0, screen_width() - 30.0, 23.0, Color::from_rgba(50, 50, 100, 128));
        }
        
        draw_text(good.name(), 20.0, y, 16.0, color);
        draw_text(&format!("{} cr", price), 180.0, y, 16.0, color);
        draw_text(&format!("{}", available), 260.0, y, 16.0, color);
        draw_text(&format!("{}", in_hold), 340.0, y, 16.0, color);
        draw_text(&format!("{}", max), 420.0, y, 16.0, color);
    }
    
    // Instructions
    let inst_y = screen_height() - 100.0;
    draw_text("Controls:", 20.0, inst_y, 18.0, LIGHTGRAY);
    draw_text("↑↓ - Select  |  B - Buy 1  |  5 - Buy 5  |  S - Sell 1  |  A - Sell All", 
        20.0, inst_y + 25.0, 14.0, LIGHTGRAY);
    draw_text("ESC/Q - Exit Trading", 20.0, inst_y + 50.0, 14.0, LIGHTGRAY);
    
    // Show message if any
    if !message.is_empty() {
        let msg_width = measure_text(message, None, 20, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(msg_x - 10.0, screen_height() / 2.0 - 30.0, msg_width + 20.0, 50.0, 
            Color::from_rgba(0, 0, 0, 200));
        draw_text(message, msg_x, screen_height() / 2.0, 20.0, GREEN);
    }
}

#[macroquad::main("Space Trader")]
async fn main() {
    let mut game_state = GameState::new();
    
    // Try to load assets (non-blocking, will use fallback if not available)
    let assets_result = GameAssets::load().await;
    let has_assets = assets_result.is_ok();
    let assets = assets_result.ok();
    
    println!("Space Trader - Rust Edition");
    println!("============================");
    println!();
    println!("Based on the classic Palm OS game by Pieter Spronck");
    println!("Inspired by Elite");
    println!();
    
    // Main game loop
    loop {
        clear_background(BLACK);
        
        // Draw title
        draw_text(
            "SPACE TRADER",
            screen_width() / 2.0 - 150.0,
            50.0,
            40.0,
            WHITE,
        );
        
        draw_text(
            "Press N for New Game, L to Load, Q to Quit",
            screen_width() / 2.0 - 200.0,
            screen_height() / 2.0,
            20.0,
            LIGHTGRAY,
        );
        
        // Handle input
        if is_key_pressed(KeyCode::N) {
            game_state.start_new_game();
            // Generate prices for starting system
            let current_id = game_state.current_system_id;
            determine_prices(&mut game_state, current_id);
            break;
        }
        
        if is_key_pressed(KeyCode::L) {
            if game_state.load_game() {
                break;
            } else {
                draw_text(
                    "No saved game found!",
                    screen_width() / 2.0 - 100.0,
                    screen_height() / 2.0 + 50.0,
                    20.0,
                    RED,
                );
            }
        }
        
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            return;
        }
        
        next_frame().await;
    }
    
    // Game loop
    println!("Game starting...");
    println!("Commander: {}", game_state.commander_name);
    println!("Current System: {}", game_state.current_system_name());
    println!("Credits: {}", game_state.credits);
    if has_assets {
        println!("✓ Assets loaded successfully");
    } else {
        println!("⚠ Assets not loaded - using geometric placeholders");
    }
    
    let mut current_screen = GameScreen::Main;
    let mut selected_good: usize = 0;
    let mut selected_system: usize = 0;
    let mut selected_upgrade: usize = 0;
    let mut trade_message = String::new();
    let mut message_timer = 0.0;
    let mut current_encounter: Option<Encounter> = None;
    let mut encounter_message = String::new();
    
    loop {
        clear_background(Color::from_rgba(10, 10, 30, 255));
        
        // Draw screen based on current state
        if current_screen == GameScreen::Encounter {
            if let Some(ref encounter) = current_encounter {
                draw_encounter_screen(encounter, &encounter_message);
            }
        } else if current_screen == GameScreen::Trading {
            draw_trading_screen(&game_state, selected_good, &trade_message);
        } else if current_screen == GameScreen::Warp {
            draw_warp_screen(&game_state, selected_system, &trade_message);
        } else if current_screen == GameScreen::Shipyard {
            draw_shipyard_screen(&game_state, selected_upgrade, &trade_message);
        } else if current_screen == GameScreen::Repair {
            draw_repair_screen(&game_state, &trade_message);
        } else if current_screen == GameScreen::ShipShop {
            draw_ship_shop_screen(&game_state, selected_upgrade, &trade_message);
        } else {
            // Main game screen
            let left_col = 20.0;
            let right_col = screen_width() / 2.0 + 20.0;
            
            // LEFT COLUMN - Commander & System Info
            draw_text(
                &format!("Commander: {}", game_state.commander_name),
                left_col,
                30.0,
                20.0,
                WHITE,
            );
            
            draw_text(
                &format!("Credits: {} cr", game_state.credits),
                left_col,
                60.0,
                18.0,
                GOLD,
            );
            
            // System info
            let current_system = &game_state.solar_systems[game_state.current_system_id];
            let tech_names = ["Pre-Agri", "Agri", "Medieval", "Renaissance", "Early Ind", "Industrial", "Post-Ind", "Hi-Tech"];
            let tech_idx = current_system.tech_level as usize;
            let tech_name = if tech_idx < tech_names.len() { tech_names[tech_idx] } else { "Unknown" };
            
            draw_text(
                &format!("System: {}", game_state.current_system_name()),
                left_col,
                90.0,
                18.0,
                SKYBLUE,
            );
            
            draw_text(
                &format!("Tech Level: {}", tech_name),
                left_col,
                115.0,
                16.0,
                LIGHTGRAY,
            );
            
            draw_text(
                &format!("Days: {}", game_state.days),
                left_col,
                140.0,
                16.0,
                LIGHTGRAY,
            );
            
            // Cargo info
            let total_cargo = game_state.ship.total_cargo();
            let max_cargo = game_state.ship.cargo_bays_available() + total_cargo;
            draw_text(
                &format!("Cargo: {}/{}", total_cargo, max_cargo),
                left_col,
                170.0,
                18.0,
                YELLOW,
            );
            
            // Cargo list - show what we're carrying
            if total_cargo > 0 {
                draw_text("Carrying:", left_col, 195.0, 14.0, LIGHTGRAY);
                let mut cargo_line = 215.0;
                for i in 0..10 {
                    let amount = game_state.ship.cargo[i];
                    if amount > 0 {
                        let good = TradeGood::from_index(i);
                        draw_text(&format!("  {} {}", amount, good.name()), left_col, cargo_line, 12.0, WHITE);
                        cargo_line += 18.0;
                        if cargo_line > 350.0 { break; }
                    }
                }
            }
            
            // RIGHT COLUMN - Ship & Fuel Info
            draw_text(
                &format!("Ship: Flea"),
                right_col,
                30.0,
                18.0,
                WHITE,
            );
            
            draw_text(
                &format!("Hull: {}", game_state.ship.hull),
                right_col,
                60.0,
                16.0,
                if game_state.ship.hull > 15 { GREEN } else { RED },
            );
            
            draw_text(
                &format!("Fuel: {}/{}", game_state.ship.fuel, game_state.ship.max_fuel()),
                right_col,
                90.0,
                18.0,
                GREEN,
            );
            
            // Show fuel cost and how much can be bought
            let fuel_cost = get_fuel_cost(&game_state);
            let max_buyable_fuel = max_fuel_buyable(&game_state);
            draw_text(
                &format!("Fuel Cost: {} cr/unit", fuel_cost),
                right_col,
                120.0,
                14.0,
                LIGHTGRAY,
            );
            
            draw_text(
                &format!("Can buy: {} units", max_buyable_fuel),
                right_col,
                140.0,
                14.0,
                LIGHTGRAY,
            );
            
            // Draw ship sprite if assets are loaded
            if let Some(ref assets) = assets {
                draw_ship(assets, "flea", right_col - 10.0, 170.0, false, false, 1.0);
            } else {
                // Fallback: draw simple geometric ship
                let ship_x = right_col + 10.0;
                let ship_y = 200.0;
                draw_triangle(
                    vec2(ship_x, ship_y - 20.0),
                    vec2(ship_x - 15.0, ship_y + 20.0),
                    vec2(ship_x + 15.0, ship_y + 20.0),
                    GRAY,
                );
            }
            
            // Instructions
            draw_text(
                "Controls:",
                20.0,
                screen_height() - 120.0,
                18.0,
                LIGHTGRAY,
            );
            
            draw_text(
                "T - Trade, W - Warp, I - Info, U - Upgrade, R - Repair, H - Ships, F - Refuel, S - Save, Q - Quit",
                20.0,
                screen_height() - 90.0,
                16.0,
                LIGHTGRAY,
            );
            
            if !has_assets {
                draw_text(
                    "Note: Run 'python tools/generate_placeholder_assets.py' to generate assets",
                    20.0,
                    screen_height() - 50.0,
                    14.0,
                    YELLOW,
                );
            }
            
            // Show message if any
            if !trade_message.is_empty() {
                draw_text(
                    &trade_message,
                    screen_width() / 2.0 - 100.0,
                    screen_height() / 2.0,
                    20.0,
                    GREEN,
                );
            }
        }
        
        // Handle input based on screen
        if current_screen == GameScreen::Encounter {
            if let Some(ref encounter) = current_encounter.clone() {
                // Attack
                if is_key_pressed(KeyCode::A) {
                    let result = resolve_encounter(&mut game_state, encounter, EncounterChoice::Attack);
                    encounter_message = result;
                    message_timer = 3.0;
                    current_encounter = None;
                    current_screen = GameScreen::Main;
                }
                
                // Ignore
                if is_key_pressed(KeyCode::I) {
                    let result = resolve_encounter(&mut game_state, encounter, EncounterChoice::Ignore);
                    encounter_message = result;
                    trade_message = encounter_message.clone();
                    message_timer = 3.0;
                    current_encounter = None;
                    current_screen = GameScreen::Main;
                }
                
                // Back/Cancel
                if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
                    current_encounter = None;
                    current_screen = GameScreen::Main;
                }
            }
        } else if current_screen == GameScreen::Trading {
            // Navigation
            if is_key_pressed(KeyCode::Up) && selected_good > 0 {
                selected_good -= 1;
            }
            if is_key_pressed(KeyCode::Down) && selected_good < 9 {
                selected_good += 1;
            }
            
            // Buy 1
            if is_key_pressed(KeyCode::B) {
                let good = TradeGood::from_index(selected_good);
                let max = max_buyable(&game_state, good);
                if max > 0 {
                    match buy_cargo(&mut game_state, good, 1) {
                        Ok(_) => {
                            trade_message = format!("Bought 1 {} for {} cr", 
                                good.name(), get_buy_price(&game_state, good));
                            message_timer = 2.0;
                        }
                        Err(e) => {
                            trade_message = e;
                            message_timer = 2.0;
                        }
                    }
                } else {
                    trade_message = "Cannot buy".to_string();
                    message_timer = 2.0;
                }
            }
            
            // Buy 5
            if is_key_pressed(KeyCode::Key5) {
                let good = TradeGood::from_index(selected_good);
                let max = max_buyable(&game_state, good);
                if max > 0 {
                    let amount = 5.min(max);
                    match buy_cargo(&mut game_state, good, amount) {
                        Ok(_) => {
                            let price = get_buy_price(&game_state, good);
                            trade_message = format!("Bought {} {} for {} cr", 
                                amount, good.name(), price * amount);
                            message_timer = 2.0;
                        }
                        Err(e) => {
                            trade_message = e;
                            message_timer = 2.0;
                        }
                    }
                }
            }
            
            // Sell 1
            if is_key_pressed(KeyCode::S) {
                let good = TradeGood::from_index(selected_good);
                let have = game_state.ship.cargo[selected_good];
                if have > 0 {
                    match sell_cargo(&mut game_state, good, 1) {
                        Ok(_) => {
                            trade_message = format!("Sold 1 {} for {} cr", 
                                good.name(), get_buy_price(&game_state, good));
                            message_timer = 2.0;
                        }
                        Err(e) => {
                            trade_message = e;
                            message_timer = 2.0;
                        }
                    }
                } else {
                    trade_message = "No cargo to sell".to_string();
                    message_timer = 2.0;
                }
            }
            
            // Sell All
            if is_key_pressed(KeyCode::A) {
                let good = TradeGood::from_index(selected_good);
                let have = game_state.ship.cargo[selected_good];
                if have > 0 {
                    match sell_cargo(&mut game_state, good, have) {
                        Ok(_) => {
                            let price = get_buy_price(&game_state, good);
                            trade_message = format!("Sold {} {} for {} cr", 
                                have, good.name(), price * have);
                            message_timer = 2.0;
                        }
                        Err(e) => {
                            trade_message = e;
                            message_timer = 2.0;
                        }
                    }
                }
            }
            
            // Exit trading
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
                current_screen = GameScreen::Main;
                trade_message.clear();
            }
        } else if current_screen == GameScreen::Warp {
            let systems = systems_in_range(&game_state);
            
            if !systems.is_empty() {
                // Navigation
                if is_key_pressed(KeyCode::Up) && selected_system > 0 {
                    selected_system -= 1;
                }
                if is_key_pressed(KeyCode::Down) && selected_system < systems.len() - 1 {
                    selected_system += 1;
                }
                
                // Warp to selected system
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::W) {
                    if selected_system < systems.len() {
                        let (target_id, _distance) = systems[selected_system];
                        match warp_to_system(&mut game_state, target_id) {
                            Ok(_) => {
                                // Regenerate prices for new system
                                let current_id = game_state.current_system_id;
                                determine_prices(&mut game_state, current_id);
                                
                                trade_message = format!("Successfully warped to {}!", 
                                    game_state.current_system_name());
                                message_timer = 2.0;
                                selected_system = 0;
                                
                                // Check for random encounters after warp
                                if let Some(encounter) = check_for_encounter(&game_state) {
                                    current_encounter = Some(encounter);
                                    current_screen = GameScreen::Encounter;
                                    encounter_message.clear();
                                } else {
                                    current_screen = GameScreen::Main;
                                }
                            }
                            Err(e) => {
                                trade_message = e;
                                message_timer = 2.0;
                            }
                        }
                    }
                }
            }
            
            // Exit warp screen
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
                current_screen = GameScreen::Main;
                trade_message.clear();
                selected_system = 0;
            }
        } else if current_screen == GameScreen::SystemInfo {
            static mut SHOW_NEWSPAPER: bool = false;
            
            // View newspaper
            if is_key_pressed(KeyCode::N) && unsafe { !SHOW_NEWSPAPER } {
                unsafe { SHOW_NEWSPAPER = true; }
            }
            
            // Buy newspaper
            if is_key_pressed(KeyCode::B) && unsafe { SHOW_NEWSPAPER } {
                if game_state.credits >= 1 {
                    game_state.credits -= 1;
                    trade_message = "Bought newspaper!".to_string();
                    unsafe { SHOW_NEWSPAPER = false; }
                    message_timer = 2.0;
                } else {
                    trade_message = "Not enough credits!".to_string();
                    message_timer = 2.0;
                }
            }
            
            // Cancel newspaper dialog
            if is_key_pressed(KeyCode::C) && unsafe { SHOW_NEWSPAPER } {
                unsafe { SHOW_NEWSPAPER = false; }
            }
            
            // Exit system info
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
                current_screen = GameScreen::Main;
                trade_message.clear();
                unsafe { SHOW_NEWSPAPER = false; }
            }
            
            draw_system_info_screen(&game_state, unsafe { SHOW_NEWSPAPER }, &trade_message);
        } else if current_screen == GameScreen::Shipyard {
            let upgrades = get_available_upgrades(&game_state);
            
            if !upgrades.is_empty() {
                // Navigation
                if is_key_pressed(KeyCode::Up) && selected_upgrade > 0 {
                    selected_upgrade -= 1;
                }
                if is_key_pressed(KeyCode::Down) && selected_upgrade < upgrades.len() - 1 {
                    selected_upgrade += 1;
                }
                
                // Purchase upgrade
                if (is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::P)) && selected_upgrade < upgrades.len() {
                    let (upgrade, _cost) = upgrades[selected_upgrade];
                    match purchase_upgrade(&mut game_state, upgrade) {
                        Ok(msg) => {
                            trade_message = msg;
                            message_timer = 2.0;
                        }
                        Err(e) => {
                            trade_message = e;
                            message_timer = 2.0;
                        }
                    }
                }
            }
            
            // Exit shipyard
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
                current_screen = GameScreen::Main;
                trade_message.clear();
                selected_upgrade = 0;
            }
        } else if current_screen == GameScreen::Repair {
            if can_repair(&game_state) {
                // Repair 10 HP
                if is_key_pressed(KeyCode::Key1) {
                    match repair_ship(&mut game_state, 10) {
                        Ok(msg) => {
                            trade_message = msg;
                            message_timer = 2.0;
                        }
                        Err(e) => {
                            trade_message = e;
                            message_timer = 2.0;
                        }
                    }
                }
                
                // Repair 50 HP
                if is_key_pressed(KeyCode::Key2) {
                    match repair_ship(&mut game_state, 50) {
                        Ok(msg) => {
                            trade_message = msg;
                            message_timer = 2.0;
                        }
                        Err(e) => {
                            trade_message = e;
                            message_timer = 2.0;
                        }
                    }
                }
                
                // Repair All
                if is_key_pressed(KeyCode::Key3) {
                    match repair_full(&mut game_state) {
                        Ok(msg) => {
                            trade_message = msg;
                            message_timer = 2.0;
                        }
                        Err(e) => {
                            trade_message = e;
                            message_timer = 2.0;
                        }
                    }
                }
            }
            
            // Exit repair screen
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
                current_screen = GameScreen::Main;
                trade_message.clear();
            }
        } else if current_screen == GameScreen::ShipShop {
            let purchasable = get_purchasable_ships(game_state.solar_systems[game_state.current_system_id].tech_level as i32);
            
            if !purchasable.is_empty() {
                // Navigation
                if is_key_pressed(KeyCode::Up) && selected_upgrade > 0 {
                    selected_upgrade -= 1;
                }
                if is_key_pressed(KeyCode::Down) && selected_upgrade < purchasable.len() - 1 {
                    selected_upgrade += 1;
                }
                
                // Purchase ship
                if (is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::B)) && selected_upgrade < purchasable.len() {
                    let ship = &purchasable[selected_upgrade];
                    if ship.ship_type_id != game_state.ship.ship_type {
                        match purchase_ship(&mut game_state, ship.ship_type_id) {
                            Ok(msg) => {
                                trade_message = msg;
                                message_timer = 2.0;
                            }
                            Err(e) => {
                                trade_message = e;
                                message_timer = 2.0;
                            }
                        }
                    } else {
                        trade_message = "You already own this ship!".to_string();
                        message_timer = 2.0;
                    }
                }
            }
            
            // Exit ship shop
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
                current_screen = GameScreen::Main;
                trade_message.clear();
                selected_upgrade = 0;
            }
        } else {
            // Main screen input
            if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
                break;
            }
            
            if is_key_pressed(KeyCode::T) {
                current_screen = GameScreen::Trading;
                selected_good = 0;
            }
            
            if is_key_pressed(KeyCode::W) {
                current_screen = GameScreen::Warp;
                selected_system = 0;
            }
            
            if is_key_pressed(KeyCode::I) {
                current_screen = GameScreen::SystemInfo;
            }
            
            if is_key_pressed(KeyCode::U) {
                current_screen = GameScreen::Shipyard;
                selected_upgrade = 0;
            }
            
            if is_key_pressed(KeyCode::R) {
                current_screen = GameScreen::Repair;
            }
            
            if is_key_pressed(KeyCode::H) {
                current_screen = GameScreen::ShipShop;
                selected_upgrade = 0;
            }
            
            // Handle fuel purchasing
            if is_key_pressed(KeyCode::F) {
                let max_buyable_fuel = max_fuel_buyable(&game_state);
                if max_buyable_fuel > 0 {
                    match buy_fuel(&mut game_state, max_buyable_fuel) {
                        Ok(_) => {
                            let fuel_cost = get_fuel_cost(&game_state);
                            trade_message = format!("Bought {} fuel for {} cr", 
                                max_buyable_fuel, fuel_cost * max_buyable_fuel);
                            message_timer = 2.0;
                        }
                        Err(e) => {
                            trade_message = e;
                            message_timer = 2.0;
                        }
                    }
                } else if game_state.ship.fuel >= game_state.ship.max_fuel() {
                    trade_message = "Fuel tanks already full!".to_string();
                    message_timer = 2.0;
                } else {
                    trade_message = "Not enough credits to buy fuel".to_string();
                    message_timer = 2.0;
                }
            }
            
            if is_key_pressed(KeyCode::S) {
                if game_state.save_game() {
                    trade_message = "Game saved!".to_string();
                    message_timer = 2.0;
                }
            }
        }
        
        // Update message timer
        if message_timer > 0.0 {
            message_timer -= get_frame_time();
            if message_timer <= 0.0 {
                trade_message.clear();
            }
        }
        
        next_frame().await;
    }
    
    println!("Thanks for playing!");
}
