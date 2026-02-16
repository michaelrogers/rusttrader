#![allow(dead_code)]
// Save/load system

use crate::types::GameState;
use std::fs;
use std::path::Path;

const SAVE_FILE: &str = "spacetrader.sav";

pub fn save_game(game_state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(game_state)?;
    fs::write(SAVE_FILE, json)?;
    Ok(())
}

pub fn load_game() -> Result<GameState, Box<dyn std::error::Error>> {
    if !Path::new(SAVE_FILE).exists() {
        return Err("Save file not found".into());
    }
    
    let json = fs::read_to_string(SAVE_FILE)?;
    let game_state = serde_json::from_str(&json)?;
    Ok(game_state)
}

pub fn save_exists() -> bool {
    Path::new(SAVE_FILE).exists()
}
