// Main menu UI

use macroquad::prelude::*;

pub async fn draw_main_menu() {
    clear_background(BLACK);
    
    // Title
    draw_text(
        "SPACE TRADER",
        screen_width() / 2.0 - 150.0,
        100.0,
        40.0,
        WHITE,
    );
    
    // Menu options
    draw_text(
        "N - New Game",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 - 40.0,
        20.0,
        LIGHTGRAY,
    );
    
    draw_text(
        "L - Load Game",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0,
        20.0,
        LIGHTGRAY,
    );
    
    draw_text(
        "Q - Quit",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 + 40.0,
        20.0,
        LIGHTGRAY,
    );
    
    // Credits
    draw_text(
        "Based on the classic Palm OS game by Pieter Spronck",
        screen_width() / 2.0 - 200.0,
        screen_height() - 60.0,
        16.0,
        GRAY,
    );
    
    draw_text(
        "Inspired by Elite",
        screen_width() / 2.0 - 60.0,
        screen_height() - 30.0,
        16.0,
        GRAY,
    );
}
