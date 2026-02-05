// In-game UI screens

use macroquad::prelude::*;
use crate::types::GameState;

pub async fn draw_game_screen(game_state: &GameState) {
    clear_background(BLACK);
    
    // Header
    draw_text(
        &format!("Commander: {}", game_state.commander_name),
        10.0,
        30.0,
        20.0,
        WHITE,
    );
    
    draw_text(
        &format!("Credits: {}", game_state.credits),
        10.0,
        60.0,
        20.0,
        GOLD,
    );
    
    draw_text(
        &format!("System: {}", game_state.current_system_name()),
        10.0,
        90.0,
        20.0,
        LIGHTGRAY,
    );
    
    draw_text(
        &format!("Day: {}", game_state.days),
        10.0,
        120.0,
        20.0,
        LIGHTGRAY,
    );
    
    // Ship status
    draw_text(
        &format!("Ship: {}", game_state.ship.name),
        screen_width() - 200.0,
        30.0,
        20.0,
        WHITE,
    );
    
    draw_text(
        &format!("Fuel: {}/{}", game_state.ship.fuel, game_state.ship.max_fuel()),
        screen_width() - 200.0,
        60.0,
        20.0,
        if game_state.ship.fuel < 10 { RED } else { GREEN },
    );
    
    draw_text(
        &format!("Hull: {}", game_state.ship.hull),
        screen_width() - 200.0,
        90.0,
        20.0,
        if game_state.ship.hull < 50 { RED } else { GREEN },
    );
    
    // TODO: Add more UI elements (cargo, trading, navigation, etc.)
}
