/// Asset loading and management for Space Trader
/// 
/// This module provides functions to load game assets (ship sprites, icons, UI elements)
/// from the assets/ directory.

use macroquad::prelude::*;
use std::collections::HashMap;

pub struct GameAssets {
    pub ships: HashMap<String, Texture2D>,
    pub icons: HashMap<String, Texture2D>,
    pub ui: HashMap<String, Texture2D>,
}

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
                texture.set_filter(FilterMode::Nearest);
                ships.insert(ship.to_string(), texture);
            }
            
            // Damaged
            if let Ok(texture) = load_texture(&format!("assets/ships/{}_damaged.png", ship)).await {
                texture.set_filter(FilterMode::Nearest);
                ships.insert(format!("{}_damaged", ship), texture);
            }
            
            // Shielded variants (not all ships have shields)
            if *ship != "flea" && *ship != "gnat" && *ship != "monster" {
                if let Ok(texture) = load_texture(&format!("assets/ships/{}_shielded.png", ship)).await {
                    texture.set_filter(FilterMode::Nearest);
                    ships.insert(format!("{}_shielded", ship), texture);
                }
                
                if let Ok(texture) = load_texture(&format!("assets/ships/{}_shielded_damaged.png", ship)).await {
                    texture.set_filter(FilterMode::Nearest);
                    ships.insert(format!("{}_shielded_damaged", ship), texture);
                }
            }
        }

        // Load encounter icons
        let icon_names = vec!["pirate", "police", "trader", "alien", "special"];
        for icon in &icon_names {
            if let Ok(texture) = load_texture(&format!("assets/icons/{}.png", icon)).await {
                texture.set_filter(FilterMode::Nearest);
                icons.insert(icon.to_string(), texture);
            }
        }

        // Load UI elements
        let ui_names = vec![
            "system", "current_system", "visited_system",
            "wormhole", "small_wormhole", "attack",
            "about", "attack2", "current_visited_system",
            "destroyed", "retire", "spacetrader", "utopia",
            "system_short_range", "visited_short_range_system"
        ];
        for element in &ui_names {
            if let Ok(texture) = load_texture(&format!("assets/ui/{}.png", element)).await {
                texture.set_filter(FilterMode::Nearest);
                ui.insert(element.to_string(), texture);
            }
        }

        Ok(GameAssets { ships, icons, ui })
    }

    /// Get a ship texture by name and state.
    /// Normalizes the ship name to lowercase for consistent lookup.
    pub fn get_ship(&self, ship_type: &str, damaged: bool, shielded: bool) -> Option<&Texture2D> {
        let name = ship_type.to_lowercase();
        let key = match (damaged, shielded) {
            (false, false) => name,
            (true, false) => format!("{}_damaged", name),
            (false, true) => format!("{}_shielded", name),
            (true, true) => format!("{}_shielded_damaged", name),
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

/// Helper function to draw an encounter icon with scale
pub fn draw_icon(assets: &GameAssets, icon_type: &str, x: f32, y: f32, scale: f32) {
    if let Some(texture) = assets.get_icon(icon_type) {
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

/// Helper function to draw a system marker with scale and color tint
pub fn draw_system_marker(assets: &GameAssets, marker_type: &str, x: f32, y: f32, size: f32, color: Color) {
    if let Some(texture) = assets.get_ui(marker_type) {
        let half = size / 2.0;
        draw_texture_ex(
            texture,
            x - half,
            y - half,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
}

/// Helper function to draw a UI image centered at a position with scale
pub fn draw_ui_image(assets: &GameAssets, name: &str, x: f32, y: f32, scale: f32) {
    if let Some(texture) = assets.get_ui(name) {
        let w = texture.width() * scale;
        let h = texture.height() * scale;
        draw_texture_ex(
            texture,
            x - w / 2.0,
            y - h / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );
    }
}
