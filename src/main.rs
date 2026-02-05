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

#[derive(PartialEq)]
enum GameScreen {
    Main,
    Trading,
    Warp,
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
    let mut trade_message = String::new();
    let mut message_timer = 0.0;
    
    loop {
        clear_background(Color::from_rgba(10, 10, 30, 255));
        
        // Draw screen based on current state
        if current_screen == GameScreen::Trading {
            draw_trading_screen(&game_state, selected_good, &trade_message);
        } else if current_screen == GameScreen::Warp {
            draw_warp_screen(&game_state, selected_system, &trade_message);
        } else {
            // Main game screen
            // Draw game UI
            draw_text(
                &format!("Commander: {}", game_state.commander_name),
                20.0,
                30.0,
                20.0,
                WHITE,
            );
            
            draw_text(
                &format!("Credits: {}", game_state.credits),
                20.0,
                60.0,
                20.0,
                GOLD,
            );
            
            draw_text(
                &format!("System: {}", game_state.current_system_name()),
                20.0,
                90.0,
                20.0,
                SKYBLUE,
            );
            
            draw_text(
                &format!("Fuel: {}/{}", game_state.ship.fuel, game_state.ship.max_fuel()),
                20.0,
                120.0,
                20.0,
                GREEN,
            );
            
            // Show fuel cost and how much can be bought
            let fuel_cost = get_fuel_cost(&game_state);
            let max_buyable_fuel = max_fuel_buyable(&game_state);
            draw_text(
                &format!("Fuel Cost: {} cr/unit | Can buy: {}", fuel_cost, max_buyable_fuel),
                20.0,
                150.0,
                16.0,
                LIGHTGRAY,
            );
            
            // Draw ship sprite if assets are loaded
            if let Some(ref assets) = assets {
                draw_ship(assets, "flea", screen_width() / 2.0 - 24.0, 150.0, false, false, 1.0);
                draw_text("Your Ship (Flea)", screen_width() / 2.0 - 60.0, 220.0, 16.0, LIGHTGRAY);
            } else {
                // Fallback: draw simple geometric ship
                let ship_x = screen_width() / 2.0;
                let ship_y = 180.0;
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
                "T - Trade, W - Warp, F - Refuel, S - Save, Q - Quit",
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
        if current_screen == GameScreen::Trading {
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
                                current_screen = GameScreen::Main;
                                selected_system = 0;
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
