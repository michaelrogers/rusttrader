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
use types::trade::TRADE_ITEMS;
use assets::{GameAssets, draw_ship};
use ui::draw_main_menu;
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
    GalacticChart,
}

fn draw_navigation_tabs(active_buy: bool, active_sell: bool, active_shipyard: bool, active_warp: bool, y: f32) {
    let tab_h = 28.0;
    let tab_y = y;
    draw_rectangle(0.0, tab_y, screen_width(), tab_h, Color::from_rgba(15, 20, 40, 255));

    let tabs = [
        ("Buy", active_buy, 90.0),
        ("Sell", active_sell, 90.0),
        ("Ship Yard", active_shipyard, 130.0),
        ("Warp", active_warp, 90.0),
    ];

    let mut x = 20.0;
    for (label, active, width) in tabs {
        let bg = if active { Color::from_rgba(80, 120, 200, 255) } else { Color::from_rgba(40, 60, 100, 255) };
        draw_rectangle(x, tab_y + 3.0, width, tab_h - 6.0, bg);
        draw_rectangle_lines(x, tab_y + 3.0, width, tab_h - 6.0, 1.0, Color::from_rgba(130, 170, 220, 255));
        let text_w = measure_text(label, None, 14, 1.0).width;
        draw_text(label, x + (width - text_w) / 2.0, tab_y + 20.0, 14.0, WHITE);
        x += width + 10.0;
    }
}

fn short_range_chart_transform(
    game_state: &GameState,
    pan: Vec2,
    zoom: f32,
    panel_x: f32,
    panel_y: f32,
    panel_w: f32,
    panel_h: f32,
) -> (Vec2, f32, f32) {
    let center = vec2(panel_x + panel_w / 2.0 + pan.x, panel_y + panel_h / 2.0 + pan.y);
    let radius = (panel_w.min(panel_h) * 0.5) - 6.0;
    let max_range = game_state.ship.max_fuel().max(1) as f32;
    let scale = (radius / max_range) * zoom;
    (center, radius, scale)
}

fn short_range_chart_hit_test(
    game_state: &GameState,
    pan: Vec2,
    zoom: f32,
    panel_x: f32,
    panel_y: f32,
    panel_w: f32,
    panel_h: f32,
    mouse: Vec2,
) -> Option<usize> {
    let (center, _radius, scale) = short_range_chart_transform(game_state, pan, zoom, panel_x, panel_y, panel_w, panel_h);
    let current = &game_state.solar_systems[game_state.current_system_id];

    let mut best: Option<(usize, f32)> = None;
    for (idx, system) in game_state.solar_systems.iter().enumerate() {
        let dx = (system.x - current.x) as f32;
        let dy = (system.y - current.y) as f32;
        let px = center.x + dx * scale;
        let py = center.y + dy * scale;
        let in_panel = px >= panel_x + 6.0 && px <= panel_x + panel_w - 6.0 && py >= panel_y + 6.0 && py <= panel_y + panel_h - 6.0;
        if !in_panel {
            continue;
        }
        let dist = mouse.distance(vec2(px, py));
        if dist <= 8.0 {
            if let Some((_, best_dist)) = best {
                if dist < best_dist {
                    best = Some((idx, dist));
                }
            } else {
                best = Some((idx, dist));
            }
        }
    }
    best.map(|(idx, _)| idx)
}

fn galactic_chart_transform(chart_x: f32, chart_y: f32, chart_w: f32, chart_h: f32, pan: Vec2, zoom: f32) -> (Vec2, f32) {
    let scale = (chart_w.min(chart_h) - 20.0) / 150.0 * zoom;
    let origin = vec2(chart_x + 10.0 + pan.x, chart_y + 10.0 + pan.y);
    (origin, scale)
}

fn galactic_chart_hit_test(
    game_state: &GameState,
    chart_x: f32,
    chart_y: f32,
    chart_w: f32,
    chart_h: f32,
    pan: Vec2,
    zoom: f32,
    mouse: Vec2,
) -> Option<usize> {
    let (origin, scale) = galactic_chart_transform(chart_x, chart_y, chart_w, chart_h, pan, zoom);
    let mut best: Option<(usize, f32)> = None;
    for (idx, system) in game_state.solar_systems.iter().enumerate() {
        let px = origin.x + system.x as f32 * scale;
        let py = origin.y + system.y as f32 * scale;
        let in_panel = px >= chart_x + 6.0 && px <= chart_x + chart_w - 6.0 && py >= chart_y + 6.0 && py <= chart_y + chart_h - 6.0;
        if !in_panel {
            continue;
        }
        let dist = mouse.distance(vec2(px, py));
        if dist <= 8.0 {
            if let Some((_, best_dist)) = best {
                if dist < best_dist {
                    best = Some((idx, dist));
                }
            } else {
                best = Some((idx, dist));
            }
        }
    }
    best.map(|(idx, _)| idx)
}

fn draw_short_range_chart(
    game_state: &GameState,
    waypoint_system: Option<usize>,
    selected_system: Option<usize>,
    pan: Vec2,
    zoom: f32,
    panel_x: f32,
    panel_y: f32,
    panel_w: f32,
    panel_h: f32,
) {
    // Panel background
    draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::from_rgba(12, 18, 34, 230));
    draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 1.0, Color::from_rgba(80, 100, 140, 200));

    // Clip to chart bounds
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
    camera.viewport = Some((panel_x as i32, panel_y as i32, panel_w as i32, panel_h as i32));
    set_camera(&camera);

    let (center, radius, scale) = short_range_chart_transform(game_state, pan, zoom, panel_x, panel_y, panel_w, panel_h);
    let center_x = center.x;
    let center_y = center.y;

    // Range circle (capacity)
    draw_circle_lines(center_x, center_y, radius, 2.0, Color::from_rgba(200, 200, 220, 200));

    let current = &game_state.solar_systems[game_state.current_system_id];
    let max_range = game_state.ship.max_fuel().max(1) as f32;
    let current_range = game_state.ship.fuel.max(0) as f32;
    let fuel_radius = radius * (current_range / max_range).min(1.0);
    draw_circle_lines(center_x, center_y, fuel_radius, 2.0, Color::from_rgba(120, 190, 120, 200));

    // Plot systems
    for (idx, system) in game_state.solar_systems.iter().enumerate() {
        let dx = (system.x - current.x) as f32;
        let dy = (system.y - current.y) as f32;
        let dist = (dx * dx + dy * dy).sqrt();
        let px = center_x + dx * scale;
        let py = center_y + dy * scale;

        let in_panel = px >= panel_x + 6.0 && px <= panel_x + panel_w - 6.0 && py >= panel_y + 6.0 && py <= panel_y + panel_h - 6.0;
        if !in_panel {
            continue;
        }

        let is_current = system.name == current.name;
        let is_waypoint = waypoint_system.map(|id| id == idx).unwrap_or(false);
        let is_selected = selected_system.map(|id| id == idx).unwrap_or(false);
        let color = if is_current {
            Color::from_rgba(70, 140, 255, 255)
        } else if is_waypoint {
            Color::from_rgba(255, 180, 80, 255)
        } else if dist <= current_range {
            Color::from_rgba(80, 200, 90, 255)
        } else {
            Color::from_rgba(110, 110, 130, 200)
        };

        draw_circle(px, py, if is_current { 5.0 } else { 4.0 }, color);
        if is_selected {
            draw_circle_lines(px, py, 7.0, 2.0, Color::from_rgba(255, 230, 150, 220));
        }

        // Draw labels for all visible systems
        let name = &system.name;
        let name_w = measure_text(name, None, 12, 1.0).width;
        let label_color = if dist <= current_range { WHITE } else { Color::from_rgba(190, 190, 200, 200) };
        draw_text(name, px - name_w / 2.0, py - 8.0, 12.0, label_color);
    }

    // Current system marker
    draw_line(center_x - 6.0, center_y, center_x + 6.0, center_y, 2.0, Color::from_rgba(70, 140, 255, 255));
    draw_line(center_x, center_y - 6.0, center_x, center_y + 6.0, 2.0, Color::from_rgba(70, 140, 255, 255));

    // Wayfinder line
    if let Some(waypoint_id) = waypoint_system {
        let target = &game_state.solar_systems[waypoint_id];
        let dx = (target.x - current.x) as f32;
        let dy = (target.y - current.y) as f32;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 0.0 {
            let clamped = dist.min(current_range.max(1.0));
            let vx = dx / dist * clamped * scale;
            let vy = dy / dist * clamped * scale;
            draw_line(center_x, center_y, center_x + vx, center_y + vy, 2.0, Color::from_rgba(255, 180, 80, 255));
            draw_circle(center_x + vx, center_y + vy, 5.0, Color::from_rgba(255, 180, 80, 255));
            draw_text(
                &format!("{:.1} parsecs to {}", dist, target.name),
                panel_x + 12.0,
                panel_y + panel_h - 36.0,
                12.0,
                WHITE,
            );
        }
    }

    // Title and legend
    draw_text("Short Range Chart", panel_x + 12.0, panel_y + 20.0, 14.0, SKYBLUE);
    draw_text("Range:", panel_x + 12.0, panel_y + panel_h - 18.0, 12.0, LIGHTGRAY);
    draw_text(&format!("{} parsecs", max_range as i32), panel_x + 70.0, panel_y + panel_h - 18.0, 12.0, WHITE);
    draw_text("● Reachable", panel_x + panel_w - 120.0, panel_y + panel_h - 18.0, 12.0, Color::from_rgba(80, 200, 90, 255));

    // Restore camera
    set_default_camera();
}

fn draw_galactic_chart(
    game_state: &GameState,
    waypoint_system: Option<usize>,
    selected_system: Option<usize>,
    pan: Vec2,
    zoom: f32,
    search_query: &str,
    search_active: bool,
) {
    clear_background(Color::from_rgba(10, 10, 30, 255));

    // Header
    draw_rectangle(0.0, 0.0, screen_width(), 45.0, Color::from_rgba(20, 30, 60, 255));
    draw_text("Galactic Chart", 20.0, 28.0, 24.0, WHITE);
    draw_navigation_tabs(false, false, false, true, 45.0);

    let chart_x = 20.0;
    let chart_y = 80.0;
    let chart_w = screen_width() - 40.0;
    let chart_h = screen_height() - 140.0;
    draw_rectangle(chart_x, chart_y, chart_w, chart_h, Color::from_rgba(12, 18, 34, 230));
    draw_rectangle_lines(chart_x, chart_y, chart_w, chart_h, 1.0, Color::from_rgba(80, 100, 140, 200));

    let (origin, scale) = galactic_chart_transform(chart_x, chart_y, chart_w, chart_h, pan, zoom);
    let origin_x = origin.x;
    let origin_y = origin.y;

    let current = &game_state.solar_systems[game_state.current_system_id];
    let range_r = game_state.ship.max_fuel() as f32 * scale;
    let current_px = origin_x + current.x as f32 * scale;
    let current_py = origin_y + current.y as f32 * scale;
    draw_circle_lines(current_px, current_py, range_r, 2.0, Color::from_rgba(200, 200, 220, 160));

    for (idx, system) in game_state.solar_systems.iter().enumerate() {
        let px = origin_x + system.x as f32 * scale;
        let py = origin_y + system.y as f32 * scale;

        let mut color = Color::from_rgba(80, 200, 90, 255);
        if system.name == current.name {
            color = Color::from_rgba(70, 140, 255, 255);
        } else if Some(idx) == waypoint_system {
            color = Color::from_rgba(255, 180, 80, 255);
        } else if Some(idx) == selected_system {
            color = Color::from_rgba(255, 230, 150, 255);
        }

        draw_rectangle(px - 3.0, py - 3.0, 6.0, 6.0, color);
    }

    // Labels for current/waypoint
    draw_text(&current.name, current_px + 6.0, current_py - 6.0, 12.0, WHITE);
    if let Some(waypoint_id) = waypoint_system {
        let target = &game_state.solar_systems[waypoint_id];
        let tx = origin_x + target.x as f32 * scale;
        let ty = origin_y + target.y as f32 * scale;
        draw_text(&target.name, tx + 6.0, ty - 6.0, 12.0, Color::from_rgba(255, 210, 140, 255));
    }

    // Selected system info panel
    let info_y = screen_height() - 75.0;
    let info_system_id = selected_system.unwrap_or(game_state.current_system_id);
    let info_system = &game_state.solar_systems[info_system_id];
    let dx = (info_system.x - current.x) as f32;
    let dy = (info_system.y - current.y) as f32;
    let dist = (dx * dx + dy * dy).sqrt();

    let tech_names = ["Pre-Agri", "Agri", "Medieval", "Renaissance", "Early Ind", "Industrial", "Post-Ind", "Hi-Tech"];
    let tech_name = tech_names.get(info_system.tech_level as usize).unwrap_or(&"Unknown");

    let politics_names = ["Anarchy", "Capitalist", "Communist", "Confederacy", "Corporate", "Cybernetic",
        "Democracy", "Dictatorship", "Fascist", "Feudal", "Military", "Monarchy",
        "Pacifist", "Socialist", "Satori", "Technocracy", "Theocracy"];
    let politics_name = politics_names.get(info_system.politics as usize).unwrap_or(&"Unknown");

    let size_names = ["Tiny", "Small", "Medium", "Large", "Huge"];
    let size_name = size_names.get(info_system.size as usize).unwrap_or(&"Unknown");

    draw_text(&format!("{}", info_system.name), 20.0, info_y, 18.0, WHITE);
    draw_text(&format!("{:.1} parsecs", dist), 220.0, info_y, 18.0, WHITE);
    draw_text(&format!("{} {} {}", size_name, tech_name, politics_name), 20.0, info_y + 22.0, 14.0, LIGHTGRAY);

    // Footer search/status
    let footer_y = screen_height() - 40.0;
    draw_text("Arrows/WASD: Pan | +/-: Zoom | Click: Select | F: Find | Enter: Set Waypoint | Esc/Q: Back", 20.0, footer_y, 14.0, LIGHTGRAY);
    if search_active {
        draw_text(&format!("Find: {}", search_query), 20.0, footer_y - 22.0, 16.0, WHITE);

        if !search_query.is_empty() {
            let query = search_query.to_lowercase();
            let mut matches: Vec<(usize, &str, bool)> = game_state
                .solar_systems
                .iter()
                .enumerate()
                .filter_map(|(idx, s)| {
                    let name = s.name.to_lowercase();
                    if name.contains(&query) {
                        Some((idx, s.name.as_str(), name.starts_with(&query)))
                    } else {
                        None
                    }
                })
                .collect();

            matches.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.1.cmp(&b.1)));

            draw_text("Suggestions:", 20.0, footer_y - 44.0, 14.0, LIGHTGRAY);
            for (i, (_, name, _)) in matches.into_iter().take(5).enumerate() {
                let color = if i == 0 { Color::from_rgba(255, 210, 140, 255) } else { LIGHTGRAY };
                draw_text(name, 120.0 + (i as f32 * 110.0), footer_y - 44.0, 14.0, color);
            }
        }
    }
}

fn draw_warp_screen(
    game_state: &GameState,
    selected: usize,
    message: &str,
    waypoint_system: Option<usize>,
    selected_chart_system: Option<usize>,
    short_pan: Vec2,
    short_zoom: f32,
) {
    clear_background(Color::from_rgba(10, 10, 30, 255));

    // Header
    draw_rectangle(0.0, 0.0, screen_width(), 45.0, Color::from_rgba(20, 30, 60, 255));

    // Title
    draw_text("Warp - Select Destination", 20.0, 28.0, 24.0, GOLD);
    draw_navigation_tabs(false, false, false, true, 45.0);

    // Current location
    draw_text(
        &format!("Current: {} | Fuel: {}", game_state.current_system_name(), game_state.ship.fuel),
        20.0,
        85.0,
        18.0,
        WHITE,
    );

    // Get systems in range
    let systems = systems_in_range(game_state);

    // Layout
    let chart_x = 20.0;
    let chart_y = 120.0;
    let chart_w = screen_width() * 0.45;
    let chart_h = screen_height() * 0.55;
    draw_short_range_chart(
        game_state,
        waypoint_system,
        selected_chart_system,
        short_pan,
        short_zoom,
        chart_x,
        chart_y,
        chart_w,
        chart_h,
    );

    if systems.is_empty() {
        draw_text("No systems in fuel range!", chart_x + chart_w + 30.0, chart_y + 40.0, 18.0, RED);
        draw_text("Return to station to refuel", chart_x + chart_w + 30.0, chart_y + 65.0, 14.0, YELLOW);
    } else {
        let list_x = chart_x + chart_w + 20.0;
        let y_start = chart_y;
        draw_text("System", list_x, y_start, 16.0, LIGHTGRAY);
        draw_text("Distance", list_x + 160.0, y_start, 16.0, LIGHTGRAY);
        draw_text("Fuel", list_x + 260.0, y_start, 16.0, LIGHTGRAY);

        for (i, &(system_id, distance)) in systems.iter().enumerate() {
            let y = y_start + 30.0 + (i as f32 * 24.0);
            let system = &game_state.solar_systems[system_id];
            let fuel_cost = distance.ceil() as i32;

            let color = if i == selected { YELLOW } else { WHITE };
            if i == selected {
                draw_rectangle(list_x - 5.0, y - 16.0, screen_width() - list_x - 20.0, 22.0, Color::from_rgba(50, 50, 100, 128));
            }

            let fuel_color = if fuel_cost <= game_state.ship.fuel { GREEN } else { RED };

            draw_text(&system.name, list_x, y, 14.0, color);
            draw_text(&format!("{:.1} ly", distance), list_x + 160.0, y, 14.0, color);
            draw_text(&format!("{}", fuel_cost), list_x + 260.0, y, 14.0, fuel_color);

            if system.visited {
                draw_text("✓", list_x + 300.0, y, 14.0, SKYBLUE);
            }
        }

        let info_y = chart_y + chart_h - 80.0;
        let selected_id = selected_chart_system.or_else(|| systems.get(selected).map(|(id, _)| *id));
        if let Some(sel_id) = selected_id {
            let system = &game_state.solar_systems[sel_id];
            let dx = (system.x - game_state.current_system().x) as f32;
            let dy = (system.y - game_state.current_system().y) as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            let fuel_cost = dist.ceil() as i32;
            let in_range = fuel_cost <= game_state.ship.fuel;

            let tech_names = ["Pre-Agri", "Agri", "Medieval", "Renaissance", "Early Ind", "Industrial", "Post-Ind", "Hi-Tech"];
            let tech_name = tech_names.get(system.tech_level as usize).unwrap_or(&"Unknown");

            let politics_names = ["Anarchy", "Capitalist", "Communist", "Confederacy", "Corporate", "Cybernetic",
                "Democracy", "Dictatorship", "Fascist", "Feudal", "Military", "Monarchy",
                "Pacifist", "Socialist", "Satori", "Technocracy", "Theocracy"];
            let politics_name = politics_names.get(system.politics as usize).unwrap_or(&"Unknown");

            draw_text(&format!("Selected: {}", system.name), list_x, info_y, 14.0, WHITE);
            draw_text(&format!("{} | {}", tech_name, politics_name), list_x, info_y + 18.0, 12.0, LIGHTGRAY);
            draw_text(&format!("Distance: {:.1} ly", dist), list_x, info_y + 36.0, 12.0, LIGHTGRAY);
            draw_text(&format!("Fuel: {} ({})", fuel_cost, if in_range { "In Range" } else { "Out of Range" }), list_x, info_y + 54.0, 12.0, if in_range { GREEN } else { RED });
        }
    }

    let inst_y = screen_height() - 100.0;
    draw_text("Controls:", 20.0, inst_y, 18.0, LIGHTGRAY);
    draw_text("↑↓ - Select  |  ENTER/W - Warp  |  G - Galactic Chart  |  ESC/Q - Cancel", 20.0, inst_y + 25.0, 14.0, LIGHTGRAY);
    draw_text("I/J/K/L - Pan Chart  |  +/- or Z/X - Zoom  |  Click: Select", 20.0, inst_y + 45.0, 14.0, LIGHTGRAY);

    if !message.is_empty() {
        let msg_width = measure_text(message, None, 20, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(msg_x - 10.0, screen_height() / 2.0 - 30.0, msg_width + 20.0, 50.0, Color::from_rgba(0, 0, 0, 200));
        draw_text(message, msg_x, screen_height() / 2.0, 20.0, if message.contains("Successfully") { GREEN } else { RED });
    }
}

fn draw_system_info_screen(game_state: &GameState, show_newspaper: bool, newspaper_unlocked: bool, message: &str) {
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

    let visited_label = if current_system.visited { "Yes" } else { "No" };
    let special_event_label = match current_system.special_event {
        0 => "Drought",
        1 => "Crop Failure",
        2 => "War",
        3 => "Boredom",
        4 => "Plague",
        5 => "Labor Shortage",
        6 => "Drug Demand",
        _ => "None",
    };

    // Layout
    let content_top = 55.0;
    let content_bottom = screen_height() - 90.0;
    let content_h = content_bottom - content_top;
    draw_rectangle(10.0, content_top, screen_width() - 20.0, content_h, Color::from_rgba(18, 25, 45, 255));
    draw_rectangle_lines(10.0, content_top, screen_width() - 20.0, content_h, 1.0, Color::from_rgba(80, 80, 120, 255));

    let left_x = 30.0;
    let right_x = screen_width() / 2.0 + 10.0;
    let panel_w = screen_width() / 2.0 - 40.0;
    let panel_h = content_h - 20.0;
    let panel_y = content_top + 10.0;

    draw_rectangle(left_x - 10.0, panel_y, panel_w, panel_h, Color::from_rgba(12, 18, 34, 255));
    draw_rectangle(right_x - 10.0, panel_y, panel_w, panel_h, Color::from_rgba(12, 18, 34, 255));
    draw_text("Overview", left_x, panel_y + 24.0, 18.0, SKYBLUE);
    draw_text("Market Snapshot", right_x, panel_y + 24.0, 18.0, SKYBLUE);
    
    // Overview panel
    let line_height = 28.0;
    let y_start = panel_y + 60.0;
    draw_text("Name:", left_x, y_start, 16.0, LIGHTGRAY);
    draw_text(&current_system.name, left_x + 160.0, y_start, 16.0, WHITE);

    draw_text("Size:", left_x, y_start + line_height, 16.0, LIGHTGRAY);
    draw_text(size_name, left_x + 160.0, y_start + line_height, 16.0, WHITE);

    draw_text("Tech Level:", left_x, y_start + line_height * 2.0, 16.0, LIGHTGRAY);
    draw_text(tech_name, left_x + 160.0, y_start + line_height * 2.0, 16.0, WHITE);

    draw_text("Government:", left_x, y_start + line_height * 3.0, 16.0, LIGHTGRAY);
    draw_text(politics_name, left_x + 160.0, y_start + line_height * 3.0, 16.0, WHITE);

    draw_text("Resources:", left_x, y_start + line_height * 4.0, 16.0, LIGHTGRAY);
    draw_text(resource_name, left_x + 160.0, y_start + line_height * 4.0, 16.0, WHITE);

    draw_text("Coordinates:", left_x, y_start + line_height * 5.0, 16.0, LIGHTGRAY);
    draw_text(
        &format!("{}, {}", current_system.x, current_system.y),
        left_x + 160.0,
        y_start + line_height * 5.0,
        16.0,
        WHITE,
    );

    draw_text("Visited:", left_x, y_start + line_height * 6.0, 16.0, LIGHTGRAY);
    draw_text(visited_label, left_x + 160.0, y_start + line_height * 6.0, 16.0, WHITE);

    draw_text("Special Event:", left_x, y_start + line_height * 7.0, 16.0, LIGHTGRAY);
    draw_text(special_event_label, left_x + 160.0, y_start + line_height * 7.0, 16.0, WHITE);

    // Market snapshot panel
    let market_y = panel_y + 60.0;
    draw_text("Top Price Moves", right_x, market_y, 16.0, LIGHTGRAY);

    let mut modifiers: Vec<(usize, i32)> = (0..10)
        .map(|good_idx| {
            let price = current_system.price_increase[good_idx];
            let base = TRADE_ITEMS[good_idx].base_price;
            (good_idx, price - base)
        })
        .collect();
    modifiers.sort_by(|a, b| b.1.abs().cmp(&a.1.abs()));
    for (idx, (good_idx, delta)) in modifiers.into_iter().take(3).enumerate() {
        let label = TradeGood::from_index(good_idx).name().to_string();
        let sign = if delta >= 0 { "+" } else { "" };
        let color = if delta >= 0 { GREEN } else { RED };
        draw_text(
            &format!("{}: {}{}", label, sign, delta),
            right_x,
            market_y + 28.0 + (idx as f32 * 24.0),
            16.0,
            color,
        );
    }

    draw_text("Low Stock", right_x, market_y + 120.0, 16.0, LIGHTGRAY);
    let mut stocks: Vec<(usize, i32)> = current_system.qty.iter().copied().enumerate().collect();
    stocks.sort_by(|a, b| a.1.cmp(&b.1));
    for (idx, (good_idx, qty)) in stocks.into_iter().take(3).enumerate() {
        let label = TradeGood::from_index(good_idx).name().to_string();
        draw_text(
            &format!("{}: {}", label, qty),
            right_x,
            market_y + 148.0 + (idx as f32 * 24.0),
            16.0,
            WHITE,
        );
    }

    draw_text("Local News", right_x, market_y + 220.0, 16.0, LIGHTGRAY);
    if current_system.news.is_empty() {
        draw_text("No notable news.", right_x, market_y + 248.0, 14.0, WHITE);
    } else if newspaper_unlocked {
        for (idx, line) in current_system.news.iter().take(3).enumerate() {
            draw_text_with_limits(
                line,
                right_x,
                market_y + 248.0 + (idx as f32 * 22.0),
                14.0,
                WHITE,
                panel_w - 20.0,
            );
        }
    } else {
        let preview = &current_system.news[0];
        draw_text_with_limits(
            preview,
            right_x,
            market_y + 248.0,
            14.0,
            WHITE,
            panel_w - 20.0,
        );
        draw_text("Buy newspaper for full report.", right_x, market_y + 270.0, 12.0, GRAY);
    }
    
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
        draw_text("Local newspaper costs 1 credit.", dialog_x + 20.0, dialog_y + 50.0, 14.0, BLACK);
        draw_text("Unlock full market report.", dialog_x + 20.0, dialog_y + 70.0, 14.0, BLACK);
        
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

    draw_navigation_tabs(false, false, true, false, 50.0);
    
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
        let y_start = 110.0;
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
    
    // Header
    draw_rectangle(0.0, 0.0, screen_width(), 45.0, Color::from_rgba(20, 30, 60, 255));
    
    // Title
    draw_text(
        &format!("Trading - {}", game_state.current_system_name()),
        20.0,
        28.0,
        24.0,
        GOLD,
    );

    draw_navigation_tabs(true, true, false, false, 45.0);
    
    // Player info
    draw_text(
        &format!("Credits: {} cr", game_state.credits),
        20.0,
        85.0,
        18.0,
        WHITE,
    );
    
    draw_text(
        &format!("Cargo: {}/{}", game_state.ship.total_cargo(), game_state.ship.cargo_bays_available() + game_state.ship.total_cargo()),
        20.0,
        110.0,
        18.0,
        WHITE,
    );
    
    // Column headers
    let y_start = 145.0;
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
    
    // Selected good history panel
    let panel_x = 520.0;
    let panel_w = screen_width() - panel_x - 20.0;
    if panel_w > 120.0 {
        let panel_y = y_start - 10.0;
        let panel_h = 220.0;
        draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::from_rgba(20, 28, 50, 220));
        draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 1.0, Color::from_rgba(80, 100, 150, 255));

        let good = TradeGood::from_index(selected);
        draw_text("Selected Good", panel_x + 10.0, panel_y + 24.0, 16.0, SKYBLUE);
        draw_text(good.name(), panel_x + 10.0, panel_y + 48.0, 16.0, WHITE);

        let history = &game_state.solar_systems[system_id].price_history[selected];
        draw_text("Price history", panel_x + 10.0, panel_y + 78.0, 14.0, LIGHTGRAY);

        if history.is_empty() {
            draw_text("No history yet.", panel_x + 10.0, panel_y + 102.0, 14.0, WHITE);
        } else {
            let history_text = history
                .iter()
                .map(|price| format!("{}", price))
                .collect::<Vec<String>>()
                .join(" → ");
            draw_text_with_limits(
                &history_text,
                panel_x + 10.0,
                panel_y + 102.0,
                14.0,
                WHITE,
                panel_w - 20.0,
            );

            if history.len() >= 2 {
                let last = history[history.len() - 1];
                let prev = history[history.len() - 2];
                let trend = if last > prev { "↑ Rising" } else if last < prev { "↓ Falling" } else { "→ Flat" };
                draw_text(
                    &format!("Trend: {}", trend),
                    panel_x + 10.0,
                    panel_y + 150.0,
                    14.0,
                    GOLD,
                );
            }
        }
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
        if current_screen == GameScreen::Encounter {
            if let Some(ref encounter) = current_encounter {
                draw_encounter_screen(encounter, &encounter_message);
            }
        } else if current_screen == GameScreen::Trading {
            draw_trading_screen(&game_state, selected_good, &trade_message);
        } else if current_screen == GameScreen::Warp {
            draw_warp_screen(
                &game_state,
                selected_system,
                &trade_message,
                waypoint_system,
                selected_chart_system,
                short_range_pan,
                short_range_zoom,
            );
        } else if current_screen == GameScreen::GalacticChart {
            draw_galactic_chart(
                &game_state,
                waypoint_system,
                selected_chart_system,
                galactic_pan,
                galactic_zoom,
                &search_query,
                search_active,
            );
        } else if current_screen == GameScreen::Shipyard {
            draw_shipyard_screen(&game_state, selected_upgrade, &trade_message);
        } else if current_screen == GameScreen::Repair {
            draw_repair_screen(&game_state, &trade_message);
        } else if current_screen == GameScreen::ShipShop {
            draw_ship_shop_screen(&game_state, selected_upgrade, &trade_message);
        } else {
            // Main game screen
            let w = screen_width();
            let h = screen_height();

            // Background gradient
            let top = Color::from_rgba(8, 10, 20, 255);
            let bottom = Color::from_rgba(14, 20, 40, 255);
            let steps = 30;
            for i in 0..steps {
                let t = i as f32 / (steps - 1) as f32;
                let r = top.r + (bottom.r - top.r) * t;
                let g = top.g + (bottom.g - top.g) * t;
                let b = top.b + (bottom.b - top.b) * t;
                let y = h * (i as f32 / steps as f32);
                draw_rectangle(0.0, y, w, h / steps as f32 + 1.0, Color::new(r, g, b, 1.0));
            }

            // Starfield
            for i in 0..90 {
                let fx = (i as f32 * 91.0) % w;
                let fy = (i as f32 * 57.0 + (i as f32 * 9.0).sin() * 20.0) % (h - 120.0);
                let brightness = 0.5 + ((i % 8) as f32) * 0.05;
                draw_circle(fx, fy, 1.0 + (i % 3) as f32 * 0.4, Color::new(brightness, brightness, brightness, 1.0));
            }

            // Planet graphic
            draw_circle(w * 0.78, h * 0.28, 70.0, Color::from_rgba(80, 110, 180, 255));
            draw_circle(w * 0.80, h * 0.26, 55.0, Color::from_rgba(60, 90, 150, 255));

            // Header bar
            draw_rectangle(0.0, 0.0, w, 46.0, Color::from_rgba(18, 28, 55, 230));

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
                20.0,
                30.0,
                22.0,
                WHITE,
            );
            draw_text(
                &format!("Day {} | Credits: {} cr", game_state.days, game_state.credits),
                w - 300.0,
                30.0,
                16.0,
                GOLD,
            );

            // Panels
            let panel_top = 70.0;
            let panel_h = 300.0;
            let panel_w = (w - 60.0) / 3.0;
            let panel_gap = 10.0;

            let p1_x = 20.0;
            let p2_x = p1_x + panel_w + panel_gap;
            let p3_x = p2_x + panel_w + panel_gap;

            for x in [p1_x, p2_x, p3_x] {
                draw_rectangle(x, panel_top, panel_w, panel_h, Color::from_rgba(12, 18, 34, 230));
                draw_rectangle_lines(x, panel_top, panel_w, panel_h, 1.0, Color::from_rgba(80, 100, 140, 200));
            }

            // System overview panel
            draw_text("System Overview", p1_x + 12.0, panel_top + 28.0, 16.0, SKYBLUE);
            let line = 24.0;
            let y0 = panel_top + 60.0;
            draw_text(&format!("Name: {}", current_system.name), p1_x + 12.0, y0, 14.0, WHITE);
            draw_text(&format!("Size: {}", size_name), p1_x + 12.0, y0 + line, 14.0, WHITE);
            draw_text(&format!("Tech: {}", tech_name), p1_x + 12.0, y0 + line * 2.0, 14.0, WHITE);
            draw_text(&format!("Gov: {}", politics_name), p1_x + 12.0, y0 + line * 3.0, 14.0, WHITE);
            draw_text(&format!("Resources: {}", resource_name), p1_x + 12.0, y0 + line * 4.0, 14.0, WHITE);
            draw_text(
                &format!("Coords: {}, {}", current_system.x, current_system.y),
                p1_x + 12.0,
                y0 + line * 5.0,
                14.0,
                LIGHTGRAY,
            );
            draw_text(
                &format!("Visited: {}", if current_system.visited { "Yes" } else { "No" }),
                p1_x + 12.0,
                y0 + line * 6.0,
                14.0,
                LIGHTGRAY,
            );

            // Ship status panel
            draw_text("Ship Status", p2_x + 12.0, panel_top + 28.0, 16.0, SKYBLUE);
            let total_cargo = game_state.ship.total_cargo();
            let max_cargo = game_state.ship.cargo_bays_available() + total_cargo;
            let max_hull = get_max_hull(&game_state);
            draw_text(&format!("Ship: {}", game_state.ship.name), p2_x + 12.0, y0, 14.0, WHITE);
            draw_text(&format!("Hull: {}/{}", game_state.ship.hull, max_hull), p2_x + 12.0, y0 + line, 14.0, if game_state.ship.hull > (max_hull / 3) { GREEN } else { RED });
            draw_text(&format!("Fuel: {}/{}", game_state.ship.fuel, game_state.ship.max_fuel()), p2_x + 12.0, y0 + line * 2.0, 14.0, WHITE);
            draw_text(&format!("Cargo: {}/{}", total_cargo, max_cargo), p2_x + 12.0, y0 + line * 3.0, 14.0, YELLOW);
            draw_text(&format!("Weapons: {}", game_state.ship.weapon_rating), p2_x + 12.0, y0 + line * 4.0, 14.0, LIGHTGRAY);
            draw_text(&format!("Shield: {}", if game_state.ship.shield_installed { "Yes" } else { "No" }), p2_x + 12.0, y0 + line * 5.0, 14.0, LIGHTGRAY);

            let fuel_cost = get_fuel_cost(&game_state);
            let max_buyable_fuel = max_fuel_buyable(&game_state);
            draw_text(&format!("Fuel: {} cr/unit", fuel_cost), p2_x + 12.0, y0 + line * 6.0, 14.0, LIGHTGRAY);
            draw_text(&format!("Can buy: {} units", max_buyable_fuel), p2_x + 12.0, y0 + line * 7.0, 14.0, LIGHTGRAY);

            if let Some(ref assets) = assets {
                draw_ship(assets, game_state.ship.name.as_str(), p2_x + panel_w - 80.0, panel_top + 170.0, false, false, 0.8);
            } else {
                let ship_x = p2_x + panel_w - 70.0;
                let ship_y = panel_top + 190.0;
                draw_triangle(
                    vec2(ship_x, ship_y - 18.0),
                    vec2(ship_x - 12.0, ship_y + 18.0),
                    vec2(ship_x + 12.0, ship_y + 18.0),
                    GRAY,
                );
            }

            // Next steps panel
            draw_text("Next Steps", p3_x + 12.0, panel_top + 28.0, 16.0, SKYBLUE);
            draw_text("T - Trade (Buy/Sell)", p3_x + 12.0, y0, 14.0, WHITE);
            draw_text("W - Warp to another system", p3_x + 12.0, y0 + line, 14.0, WHITE);
            draw_text("I - System Info", p3_x + 12.0, y0 + line * 2.0, 14.0, WHITE);
            draw_text("U - Ship Yard (Upgrades)", p3_x + 12.0, y0 + line * 3.0, 14.0, WHITE);
            draw_text("R - Repair Dock", p3_x + 12.0, y0 + line * 4.0, 14.0, WHITE);
            draw_text("H - Ship Shop", p3_x + 12.0, y0 + line * 5.0, 14.0, WHITE);
            draw_text("F - Refuel", p3_x + 12.0, y0 + line * 6.0, 14.0, WHITE);
            draw_text("S - Save | Q - Quit", p3_x + 12.0, y0 + line * 7.0, 14.0, LIGHTGRAY);

            // Footer hint
            draw_text(
                "Tip: Use Trade to compare prices or Warp to explore new systems.",
                20.0,
                h - 60.0,
                14.0,
                LIGHTGRAY,
            );

            if !has_assets {
                draw_text(
                    "Note: Run 'python tools/generate_placeholder_assets.py' to generate assets",
                    20.0,
                    h - 35.0,
                    14.0,
                    YELLOW,
                );
            }

            // Show message if any
            if !trade_message.is_empty() {
                let msg_width = measure_text(&trade_message, None, 20, 1.0).width;
                let msg_x = (w - msg_width) / 2.0;
                draw_rectangle(msg_x - 12.0, h / 2.0 - 24.0, msg_width + 24.0, 36.0, Color::from_rgba(0, 0, 0, 200));
                draw_text(&trade_message, msg_x, h / 2.0, 20.0, GREEN);
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
            // Pan/zoom controls
            let pan_speed = 6.0;
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
            if is_key_pressed(KeyCode::Minus) {
                galactic_zoom = (galactic_zoom - 0.1).max(0.6);
            }
            if is_key_pressed(KeyCode::Equal) {
                galactic_zoom = (galactic_zoom + 0.1).min(2.5);
            }

            // Click select system
            let chart_x = 20.0;
            let chart_y = 80.0;
            let chart_w = screen_width() - 40.0;
            let chart_h = screen_height() - 140.0;
            let mouse = vec2(mouse_position().0, mouse_position().1);
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
