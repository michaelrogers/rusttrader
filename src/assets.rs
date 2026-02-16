/// Asset loading and management for Space Trader
/// 
/// This module provides functions to load game assets (ship sprites, icons, UI elements)
/// from the assets/ directory.

use macroquad::prelude::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct GameAssets {
    pub ships: HashMap<String, Texture2D>,
    pub icons: HashMap<String, Texture2D>,
    pub ui: HashMap<String, Texture2D>,
}

#[allow(dead_code)]
impl GameAssets {
    /// Load all game assets from the assets directory
    pub async fn load() -> Result<Self, String> {
        let mut ships = HashMap::new();
        let mut icons = HashMap::new();
        let mut ui = HashMap::new();

        // Load ship sprites
        let ship_names = vec![
            "flea", "gnat", "firefly", "mosquito", "bumblebee",
            "beetle", "hornet", "grasshopper", "termite", "wasp",
            "monster", "dragonfly", "mantis", "scarab", "bottle"
        ];

        for ship in &ship_names {
            // Normal
            if let Ok(texture) = load_texture(&format!("assets/ships/{}.png", ship)).await {
                ships.insert(ship.to_string(), texture);
            }
            
            // Damaged
            if let Ok(texture) = load_texture(&format!("assets/ships/{}_damaged.png", ship)).await {
                ships.insert(format!("{}_damaged", ship), texture);
            }
            
            // Shielded variants (not all ships have shields)
            if *ship != "flea" && *ship != "gnat" && *ship != "monster" {
                if let Ok(texture) = load_texture(&format!("assets/ships/{}_shielded.png", ship)).await {
                    ships.insert(format!("{}_shielded", ship), texture);
                }
                
                if let Ok(texture) = load_texture(&format!("assets/ships/{}_shielded_damaged.png", ship)).await {
                    ships.insert(format!("{}_shielded_damaged", ship), texture);
                }
            }
        }

        // Load encounter icons
        let icon_names = vec!["pirate", "police", "trader", "alien", "special"];
        for icon in &icon_names {
            if let Ok(texture) = load_texture(&format!("assets/icons/{}.png", icon)).await {
                icons.insert(icon.to_string(), texture);
            }
        }

        // Load UI elements
        let ui_names = vec![
            "system", "current_system", "visited_system",
            "wormhole", "small_wormhole", "attack"
        ];
        for element in &ui_names {
            if let Ok(texture) = load_texture(&format!("assets/ui/{}.png", element)).await {
                ui.insert(element.to_string(), texture);
            }
        }

        Ok(GameAssets { ships, icons, ui })
    }

    /// Get a ship texture by name and state
    pub fn get_ship(&self, ship_type: &str, damaged: bool, shielded: bool) -> Option<&Texture2D> {
        let key = match (damaged, shielded) {
            (false, false) => ship_type.to_string(),
            (true, false) => format!("{}_damaged", ship_type),
            (false, true) => format!("{}_shielded", ship_type),
            (true, true) => format!("{}_shielded_damaged", ship_type),
        };
        self.ships.get(&key)
    }

    /// Get an encounter icon
    pub fn get_icon(&self, icon_type: &str) -> Option<&Texture2D> {
        self.icons.get(icon_type)
    }

    /// Get a UI element
    pub fn get_ui(&self, element: &str) -> Option<&Texture2D> {
        self.ui.get(element)
    }
}

/// Helper function to draw a ship sprite
pub fn draw_ship(
    assets: &GameAssets,
    ship_type: &str,
    x: f32,
    y: f32,
    damaged: bool,
    shielded: bool,
    scale: f32,
) {
    if let Some(texture) = assets.get_ship(ship_type, damaged, shielded) {
        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(texture.width() * scale, texture.height() * scale)),
                ..Default::default()
            },
        );
    }
}

/// Helper function to draw an encounter icon
#[allow(dead_code)]
pub fn draw_icon(assets: &GameAssets, icon_type: &str, x: f32, y: f32) {
    if let Some(texture) = assets.get_icon(icon_type) {
        draw_texture(texture, x, y, WHITE);
    }
}

/// Helper function to draw a system marker
#[allow(dead_code)]
pub fn draw_system_marker(assets: &GameAssets, marker_type: &str, x: f32, y: f32) {
    if let Some(texture) = assets.get_ui(marker_type) {
        draw_texture(texture, x, y, WHITE);
    }
}
