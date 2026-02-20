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
use ui::{
    draw_encounter_screen, draw_galactic_chart, draw_main_menu, draw_panel, draw_repair_screen,
    draw_shipyard_screen, draw_system_info_screen, draw_text_with_limits, draw_trading_screen,
    draw_warp_screen, galactic_chart_hit_test, short_range_chart_hit_test, theme,
};
use game::trading::{buy_cargo, sell_cargo, max_buyable, buy_fuel, get_fuel_cost, max_fuel_buyable};
use game::pricing::{get_buy_price, determine_prices};
use game::travel::{warp_to_system, systems_in_range};
use game::encounter::{check_for_encounter, resolve_encounter, Encounter, EncounterChoice};
use game::upgrades::{get_available_upgrades, purchase_upgrade};
use game::repair::{can_repair, get_max_hull, repair_full, repair_ship};
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
    GalacticChart,
}

fn draw_ship_shop_screen(game_state: &GameState, selected: usize, message: &str) {
    let t = theme();
    clear_background(Color::from_rgba(10, 10, 30, 255));
    
    // Header
    draw_rectangle(0.0, 0.0, screen_width(), t.header_height, Color::from_rgba(0, 160, 80, 255));
    draw_text("Ship Shop", t.margin, t.header_height * 0.6, t.font_title, WHITE);
    draw_text(&format!("Credits: {}", game_state.credits), screen_width() - t.margin * 10.0, t.header_height * 0.6, t.font_medium, GOLD);
    
    let _current_ship = get_current_ship_info(game_state);
    let purchasable = get_purchasable_ships(game_state.solar_systems[game_state.current_system_id].tech_level as i32);
    
    if purchasable.is_empty() {
        draw_text(
            "No ships available at this tech level",
            screen_width() / 2.0 - 180.0 * t.scale,
            screen_height() / 2.0,
            t.font_large,
            LIGHTGRAY,
        );
    } else {
        // Column headers
        let y_start = t.header_height + t.margin;
        let list_w = screen_width() - t.margin * 2.0;
        let name_col = t.margin * 2.0;
        let desc_col = t.margin + list_w * 0.12;
        let stats_col = t.margin + list_w * 0.45;
        let cost_col = t.margin + list_w * 0.82;
        
        draw_text("Ship", name_col, y_start, t.font_medium, LIGHTGRAY);
        draw_text("Description", desc_col, y_start, t.font_small, LIGHTGRAY);
        draw_text("Stats", stats_col, y_start, t.font_small, LIGHTGRAY);
        draw_text("Cost", cost_col, y_start, t.font_medium, LIGHTGRAY);
        
        // Draw ships list
        let row_h = t.row_height * 2.5;
        for (i, ship) in purchasable.iter().enumerate() {
            let y = y_start + t.line_height * 1.5 + (i as f32 * row_h);
            
            // Highlight selected
            if i == selected {
                draw_rectangle(t.padding, y - t.padding * 2.0, screen_width() - t.padding * 2.0, row_h - t.padding, Color::from_rgba(50, 100, 50, 128));
            }
            
            // Current ship indicator
            let is_current = ship.ship_type_id == game_state.ship.ship_type;
            let color = if is_current { YELLOW } else { WHITE };
            
            let ship_label = if is_current {
                format!("{} (CURRENT)", ship.name)
            } else {
                ship.name.to_string()
            };
            
            draw_text(&ship_label, name_col, y, t.font_medium, color);
            draw_text_with_limits(ship.description, desc_col, y, t.font_small, LIGHTGRAY, list_w * 0.30);
            
            let stats = format!(
                "Cargo: {} | Weapon: {} | Shield: {} | Hull: {}",
                ship.cargo_bays, ship.weapon_slots, ship.shield_slots, ship.hull_strength
            );
            draw_text(&stats, stats_col, y, t.font_small, SKYBLUE);
            
            if is_current {
                draw_text("OWNED", cost_col, y, t.font_medium, YELLOW);
            } else {
                let cost = ship.upgrade_cost_from_current(game_state);
                let cost_color = if game_state.credits >= cost { GREEN } else { RED };
                draw_text(&format!("{} cr", cost), cost_col, y, t.font_medium, cost_color);
            }
        }
    }
    
    // Controls
    let inst_y = screen_height() - t.header_height * 2.5;
    draw_text("Controls:", t.margin, inst_y, t.font_medium, LIGHTGRAY);
    draw_text(
        "↑↓ - Select  |  ENTER/B - Buy  |  ESC/Q - Back",
        t.margin,
        inst_y + t.line_height_small,
        t.font_small,
        LIGHTGRAY,
    );
    
    // Show message if any
    if !message.is_empty() {
        let msg_width = measure_text(message, None, t.font_large as u16, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(msg_x - t.padding, screen_height() / 2.0 + t.margin * 2.5, msg_width + t.padding * 2.0, t.line_height * 2.0,
            Color::from_rgba(0, 0, 0, 200));
        let msg_color = if message.contains("Purchased") { GREEN } else { RED };
        draw_text(message, msg_x, screen_height() / 2.0 + t.margin * 3.8, t.font_large, msg_color);
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
    let mut menu_message = String::new();
    let mut menu_timer = 0.0f32;
    loop {
        draw_main_menu().await;

        if menu_timer > 0.0 {
            menu_timer -= get_frame_time();
            let msg_width = measure_text(&menu_message, None, 20, 1.0).width;
            let msg_x = (screen_width() - msg_width) / 2.0;
            draw_rectangle(msg_x - 12.0, screen_height() / 2.0 + 70.0 - 26.0, msg_width + 24.0, 36.0, Color::from_rgba(0, 0, 0, 200));
            draw_text(&menu_message, msg_x, screen_height() / 2.0 + 70.0, 20.0, RED);
        }
        
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
                menu_message = "No saved game found!".to_string();
                menu_timer = 2.0;
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
    let mut waypoint_system: Option<usize> = None;
    let mut selected_chart_system: Option<usize> = None;
    let mut short_range_pan = vec2(0.0, 0.0);
    let mut short_range_zoom = 1.0f32;
    let mut galactic_pan = vec2(0.0, 0.0);
    let mut galactic_zoom = 1.0f32;
    let mut search_query = String::new();
    let mut search_active = false;
    let mut show_newspaper_prompt = false;
    let mut newspaper_unlocked = false;
    
    loop {
        clear_background(Color::from_rgba(10, 10, 30, 255));
        
        // Draw screen based on current state
        match current_screen {
            GameScreen::Encounter => {
                if let Some(ref encounter) = current_encounter {
                    draw_encounter_screen(encounter, &encounter_message);
                }
            }
            GameScreen::Trading => {
                draw_trading_screen(&game_state, selected_good, &trade_message);
            }
            GameScreen::Warp => {
                draw_warp_screen(
                    &game_state,
                    selected_system,
                    &trade_message,
                    waypoint_system,
                    selected_chart_system,
                    short_range_pan,
                    short_range_zoom,
                );
            }
            GameScreen::GalacticChart => {
                draw_galactic_chart(
                    &game_state,
                    waypoint_system,
                    selected_chart_system,
                    galactic_pan,
                    galactic_zoom,
                    &search_query,
                    search_active,
                );
            }
            GameScreen::Shipyard => {
                draw_shipyard_screen(&game_state, selected_upgrade, &trade_message);
            }
            GameScreen::Repair => {
                draw_repair_screen(&game_state, &trade_message);
            }
            GameScreen::ShipShop => {
                draw_ship_shop_screen(&game_state, selected_upgrade, &trade_message);
            }
            GameScreen::SystemInfo => {
                draw_system_info_screen(
                    &game_state,
                    show_newspaper_prompt,
                    newspaper_unlocked,
                    &trade_message,
                );
            }
            GameScreen::Main => {
                // Main game screen
            let t = theme();
            let w = screen_width();
            let h = screen_height();

            // Background gradient
            let top = Color::from_rgba(8, 10, 20, 255);
            let bottom = Color::from_rgba(14, 20, 40, 255);
            let steps = 30;
            for i in 0..steps {
                let ti = i as f32 / (steps - 1) as f32;
                let r = top.r + (bottom.r - top.r) * ti;
                let g = top.g + (bottom.g - top.g) * ti;
                let b = top.b + (bottom.b - top.b) * ti;
                let y = h * (i as f32 / steps as f32);
                draw_rectangle(0.0, y, w, h / steps as f32 + 1.0, Color::new(r, g, b, 1.0));
            }

            // Starfield
            let star_scale = t.scale.max(0.8);
            for i in 0..90 {
                let fx = (i as f32 * 91.0) % w;
                let fy = (i as f32 * 57.0 + (i as f32 * 9.0).sin() * 20.0) % (h - 120.0);
                let brightness = 0.5 + ((i % 8) as f32) * 0.05;
                draw_circle(fx, fy, (1.0 + (i % 3) as f32 * 0.4) * star_scale, Color::new(brightness, brightness, brightness, 1.0));
            }

            // Planet graphic
            let planet_size = (70.0 * t.scale).clamp(50.0, 110.0);
            draw_circle(w * 0.78, h * 0.28, planet_size, Color::from_rgba(80, 110, 180, 255));
            draw_circle(w * 0.80, h * 0.26, planet_size * 0.78, Color::from_rgba(60, 90, 150, 255));

            // Header bar
            draw_rectangle(0.0, 0.0, w, t.header_height, Color::from_rgba(18, 28, 55, 230));

            let current_system = &game_state.solar_systems[game_state.current_system_id];
            let tech_names = ["Pre-Agri", "Agri", "Medieval", "Renaissance", "Early Ind", "Industrial", "Post-Ind", "Hi-Tech"];
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

            let resource_names = [
                "None",
                "Mineral Rich",
                "Mineral Poor",
                "Desert",
                "Lots of Water",
                "Rich Soil",
                "Poor Soil",
                "Rich Fauna",
                "Lifeless",
                "Weird Mushrooms",
                "Lots of Herbs",
                "Artistic",
                "Warlike",
            ];
            let res_idx = current_system.special_resource as usize;
            let resource_name = if res_idx < resource_names.len() { resource_names[res_idx] } else { "Unknown" };

            // Header text
            draw_text(
                &format!("Arrived at {}", game_state.current_system_name()),
                t.margin,
                t.header_height * 0.65,
                t.font_title,
                WHITE,
            );
            draw_text(
                &format!("Day {} | Credits: {} cr", game_state.days, game_state.credits),
                w - t.margin * 15.0,
                t.header_height * 0.65,
                t.font_medium,
                GOLD,
            );

            // Panels
            let panel_top = t.header_height + t.margin;
            let panel_h = (h - panel_top - t.header_height * 2.0).max(200.0);
            let panel_w = (w - t.margin * 2.0 - t.padding * 2.0) / 3.0;
            let panel_gap = t.padding;

            let p1_x = t.margin;
            let p2_x = p1_x + panel_w + panel_gap;
            let p3_x = p2_x + panel_w + panel_gap;

            for x in [p1_x, p2_x, p3_x] {
                draw_panel(x, panel_top, panel_w, panel_h);
            }

            // System overview panel
            let text_inset = t.padding * 1.2;
            let line = t.line_height;
            let y0 = panel_top + t.line_height * 2.5;
            draw_text("System Overview", p1_x + text_inset, panel_top + t.line_height, t.font_medium, SKYBLUE);
            draw_text(&format!("Name: {}", current_system.name), p1_x + text_inset, y0, t.font_medium, WHITE);
            draw_text(&format!("Size: {}", size_name), p1_x + text_inset, y0 + line, t.font_medium, WHITE);
            draw_text(&format!("Tech: {}", tech_name), p1_x + text_inset, y0 + line * 2.0, t.font_medium, WHITE);
            draw_text(&format!("Gov: {}", politics_name), p1_x + text_inset, y0 + line * 3.0, t.font_medium, WHITE);
            draw_text(&format!("Resources: {}", resource_name), p1_x + text_inset, y0 + line * 4.0, t.font_medium, WHITE);
            draw_text(
                &format!("Coords: {}, {}", current_system.x, current_system.y),
                p1_x + text_inset,
                y0 + line * 5.0,
                t.font_medium,
                LIGHTGRAY,
            );
            draw_text(
                &format!("Visited: {}", if current_system.visited { "Yes" } else { "No" }),
                p1_x + text_inset,
                y0 + line * 6.0,
                t.font_medium,
                LIGHTGRAY,
            );

            // Ship status panel
            draw_text("Ship Status", p2_x + text_inset, panel_top + t.line_height, t.font_medium, SKYBLUE);
            let total_cargo = game_state.ship.total_cargo();
            let max_cargo = game_state.ship.cargo_bays_available() + total_cargo;
            let max_hull = get_max_hull(&game_state);
            draw_text(&format!("Ship: {}", game_state.ship.name), p2_x + text_inset, y0, t.font_medium, WHITE);
            draw_text(&format!("Hull: {}/{}", game_state.ship.hull, max_hull), p2_x + text_inset, y0 + line, t.font_medium, if game_state.ship.hull > (max_hull / 3) { GREEN } else { RED });
            draw_text(&format!("Fuel: {}/{}", game_state.ship.fuel, game_state.ship.max_fuel()), p2_x + text_inset, y0 + line * 2.0, t.font_medium, WHITE);
            draw_text(&format!("Cargo: {}/{}", total_cargo, max_cargo), p2_x + text_inset, y0 + line * 3.0, t.font_medium, YELLOW);
            draw_text(&format!("Weapons: {}", game_state.ship.weapon_rating), p2_x + text_inset, y0 + line * 4.0, t.font_medium, LIGHTGRAY);
            draw_text(&format!("Shield: {}", if game_state.ship.shield_installed { "Yes" } else { "No" }), p2_x + text_inset, y0 + line * 5.0, t.font_medium, LIGHTGRAY);

            let fuel_cost = get_fuel_cost(&game_state);
            let max_buyable_fuel = max_fuel_buyable(&game_state);
            draw_text(&format!("Fuel: {} cr/unit", fuel_cost), p2_x + text_inset, y0 + line * 6.0, t.font_medium, LIGHTGRAY);
            draw_text(&format!("Can buy: {} units", max_buyable_fuel), p2_x + text_inset, y0 + line * 7.0, t.font_medium, LIGHTGRAY);

            if let Some(ref assets) = assets {
                let ship_scale = (0.8 * t.scale).clamp(0.5, 1.5);
                draw_ship(assets, game_state.ship.name.as_str(), p2_x + panel_w - t.margin * 4.0, panel_top + panel_h * 0.5, false, false, ship_scale);
            } else {
                let ship_x = p2_x + panel_w - t.margin * 3.5;
                let ship_y = panel_top + panel_h * 0.55;
                let ship_size = (18.0 * t.scale).clamp(12.0, 30.0);
                draw_triangle(
                    vec2(ship_x, ship_y - ship_size),
                    vec2(ship_x - ship_size * 0.67, ship_y + ship_size),
                    vec2(ship_x + ship_size * 0.67, ship_y + ship_size),
                    GRAY,
                );
            }

            // Next steps panel
            draw_text("Next Steps", p3_x + text_inset, panel_top + t.line_height, t.font_medium, SKYBLUE);
            draw_text("T - Trade (Buy/Sell)", p3_x + text_inset, y0, t.font_medium, WHITE);
            draw_text("W - Warp to another system", p3_x + text_inset, y0 + line, t.font_medium, WHITE);
            draw_text("I - System Info", p3_x + text_inset, y0 + line * 2.0, t.font_medium, WHITE);
            draw_text("U - Ship Yard (Upgrades)", p3_x + text_inset, y0 + line * 3.0, t.font_medium, WHITE);
            draw_text("R - Repair Dock", p3_x + text_inset, y0 + line * 4.0, t.font_medium, WHITE);
            draw_text("H - Ship Shop", p3_x + text_inset, y0 + line * 5.0, t.font_medium, WHITE);
            draw_text("F - Refuel", p3_x + text_inset, y0 + line * 6.0, t.font_medium, WHITE);
            draw_text("S - Save | Q - Quit", p3_x + text_inset, y0 + line * 7.0, t.font_medium, LIGHTGRAY);

            // Footer hint
            draw_text(
                "Tip: Use Trade to compare prices or Warp to explore new systems.",
                t.margin,
                h - t.margin * 3.0,
                t.font_medium,
                LIGHTGRAY,
            );

            if !has_assets {
                draw_text(
                    "Note: Run 'python tools/generate_placeholder_assets.py' to generate assets",
                    t.margin,
                    h - t.margin * 1.75,
                    t.font_medium,
                    YELLOW,
                );
            }

            // Show message if any
                if !trade_message.is_empty() {
                let msg_width = measure_text(&trade_message, None, t.font_large as u16, 1.0).width;
                let msg_x = (w - msg_width) / 2.0;
                draw_rectangle(msg_x - t.padding, h / 2.0 - t.line_height, msg_width + t.padding * 2.0, t.line_height * 1.5, Color::from_rgba(0, 0, 0, 200));
                draw_text(&trade_message, msg_x, h / 2.0, t.font_large, GREEN);
            }
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
            let chart_x = 20.0;
            let chart_y = 120.0;
            let chart_w = screen_width() * 0.45;
            let chart_h = screen_height() * 0.55;
            let mouse = vec2(mouse_position().0, mouse_position().1);

            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some(hit_id) = short_range_chart_hit_test(
                    &game_state,
                    short_range_pan,
                    short_range_zoom,
                    chart_x,
                    chart_y,
                    chart_w,
                    chart_h,
                    mouse,
                ) {
                    selected_chart_system = Some(hit_id);
                    if let Some(pos) = systems.iter().position(|(id, _)| *id == hit_id) {
                        selected_system = pos;
                    }
                }
            }

            // Short range chart pan/zoom
            let pan_speed = 6.0;
            if is_key_down(KeyCode::I) {
                short_range_pan.y += pan_speed;
            }
            if is_key_down(KeyCode::K) {
                short_range_pan.y -= pan_speed;
            }
            if is_key_down(KeyCode::J) {
                short_range_pan.x += pan_speed;
            }
            if is_key_down(KeyCode::L) {
                short_range_pan.x -= pan_speed;
            }
            if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::Z) {
                short_range_zoom = (short_range_zoom - 0.1).max(0.6);
            }
            if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::X) {
                short_range_zoom = (short_range_zoom + 0.1).min(2.0);
            }
            
            if !systems.is_empty() {
                // Navigation
                if is_key_pressed(KeyCode::Up) && selected_system > 0 {
                    selected_system -= 1;
                    selected_chart_system = Some(systems[selected_system].0);
                }
                if is_key_pressed(KeyCode::Down) && selected_system < systems.len() - 1 {
                    selected_system += 1;
                    selected_chart_system = Some(systems[selected_system].0);
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

                                newspaper_unlocked = false;
                                show_newspaper_prompt = false;
                                
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

            // Open galactic chart
            if is_key_pressed(KeyCode::G) {
                current_screen = GameScreen::GalacticChart;
            }
        } else if current_screen == GameScreen::GalacticChart {
            // Chart bounds for input handling
            let chart_x = 20.0;
            let chart_y = 80.0;
            let chart_w = screen_width() - 40.0;
            let chart_h = screen_height() - 140.0;
            let mouse = vec2(mouse_position().0, mouse_position().1);
            
            // Pan/zoom controls - scale pan speed with zoom for consistent feel
            let pan_speed = 6.0 / galactic_zoom;
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                galactic_pan.x += pan_speed;
            }
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                galactic_pan.x -= pan_speed;
            }
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                galactic_pan.y += pan_speed;
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                galactic_pan.y -= pan_speed;
            }
            
            // Keyboard zoom
            if is_key_pressed(KeyCode::Minus) {
                galactic_zoom = (galactic_zoom - 0.1).max(0.5);
            }
            if is_key_pressed(KeyCode::Equal) {
                galactic_zoom = (galactic_zoom + 0.1).min(3.0);
            }
            
            // Mouse wheel zoom (zoom toward center)
            let (_, wheel_y) = mouse_wheel();
            if wheel_y != 0.0 {
                let zoom_delta = wheel_y * 0.1;
                galactic_zoom = (galactic_zoom + zoom_delta).clamp(0.5, 3.0);
            }
            
            // Mouse drag pan (right mouse button)
            if is_mouse_button_down(MouseButton::Right) {
                let delta = mouse_delta_position();
                galactic_pan.x += delta.x / galactic_zoom;
                galactic_pan.y += delta.y / galactic_zoom;
            }
            
            // Reset view (R key or Home)
            if is_key_pressed(KeyCode::R) || is_key_pressed(KeyCode::Home) {
                galactic_pan = vec2(0.0, 0.0);
                galactic_zoom = 1.0;
            }

            // Click select system
            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some(hit_id) = galactic_chart_hit_test(
                    &game_state,
                    chart_x,
                    chart_y,
                    chart_w,
                    chart_h,
                    galactic_pan,
                    galactic_zoom,
                    mouse,
                ) {
                    selected_chart_system = Some(hit_id);
                }
            }

            // Toggle search
            if is_key_pressed(KeyCode::F) {
                search_active = true;
                search_query.clear();
            }

            if search_active {
                while let Some(ch) = get_char_pressed() {
                    if ch.is_ascii_alphanumeric() || ch == ' ' || ch == '-' {
                        search_query.push(ch);
                    }
                }
                if is_key_pressed(KeyCode::Backspace) {
                    search_query.pop();
                }
                if is_key_pressed(KeyCode::Enter) {
                    let query = search_query.to_lowercase();
                    if !query.is_empty() {
                        let mut matches: Vec<(usize, bool)> = game_state
                            .solar_systems
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, s)| {
                                let name = s.name.to_lowercase();
                                if name.contains(&query) {
                                    Some((idx, name.starts_with(&query)))
                                } else {
                                    None
                                }
                            })
                            .collect();

                        matches.sort_by(|a, b| b.1.cmp(&a.1));
                        if let Some((idx, _)) = matches.into_iter().next() {
                            waypoint_system = Some(idx);
                            selected_chart_system = Some(idx);
                        }
                    }
                    search_active = false;
                }
                if is_key_pressed(KeyCode::Escape) {
                    search_active = false;
                }
            }

            // Exit chart
            if is_key_pressed(KeyCode::Q) {
                current_screen = GameScreen::Warp;
            }
        } else if current_screen == GameScreen::SystemInfo {
            // View newspaper
            if is_key_pressed(KeyCode::N) && !show_newspaper_prompt {
                show_newspaper_prompt = true;
            }

            // Buy newspaper
            if is_key_pressed(KeyCode::B) && show_newspaper_prompt {
                if game_state.credits >= 1 {
                    game_state.credits -= 1;
                    newspaper_unlocked = true;
                    trade_message = "Bought newspaper!".to_string();
                    show_newspaper_prompt = false;
                    message_timer = 2.0;
                } else {
                    trade_message = "Not enough credits!".to_string();
                    message_timer = 2.0;
                }
            }

            // Cancel newspaper dialog
            if is_key_pressed(KeyCode::C) && show_newspaper_prompt {
                show_newspaper_prompt = false;
            }

            // Exit system info
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
                current_screen = GameScreen::Main;
                trade_message.clear();
                show_newspaper_prompt = false;
            }

            draw_system_info_screen(&game_state, show_newspaper_prompt, newspaper_unlocked, &trade_message);
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
