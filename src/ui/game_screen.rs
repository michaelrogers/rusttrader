// In-game UI screens

use crate::game::encounter::Encounter;
use crate::game::pricing::get_buy_price;
use crate::game::repair::{
    calculate_full_repair_cost, calculate_repair_cost_per_point, can_repair, get_max_hull,
};
use crate::game::trading::max_buyable;
use crate::game::travel::systems_in_range;
use crate::game::upgrades::get_available_upgrades;
use crate::types::trade::TRADE_ITEMS;
use crate::types::{GameState, TradeGood};
use macroquad::prelude::*;

// Base resolution for UI scaling - designed at 800x600, scales proportionally
const BASE_WIDTH: f32 = 800.0;
const BASE_HEIGHT: f32 = 600.0;

/// Calculate UI scale factor based on current screen size
/// Returns a multiplier where 1.0 = base resolution (800x600)
pub fn ui_scale() -> f32 {
    (screen_width() / BASE_WIDTH).min(screen_height() / BASE_HEIGHT)
}

/// UI theme with all computed dimensions for current screen size
#[derive(Clone, Copy)]
pub struct UiTheme {
    pub scale: f32,
    // Font sizes
    pub font_small: f32,
    pub font_medium: f32,
    pub font_large: f32,
    pub font_title: f32,
    pub font_header: f32,
    // Spacing
    pub margin: f32,
    pub padding: f32,
    pub line_height: f32,
    pub line_height_small: f32,
    // Tab bar
    pub tab_height: f32,
    pub header_height: f32,
    pub header_height_large: f32,
    // System markers
    pub system_marker_size: f32,
    pub system_marker_size_small: f32,
    pub hit_radius: f32,
    // Buttons
    pub button_width: f32,
    pub button_height: f32,
    pub row_height: f32,
    pub row_height_large: f32,
}

impl UiTheme {
    pub fn new() -> Self {
        let scale = ui_scale();
        UiTheme {
            scale,
            // Font sizes with minimum clamps for readability
            font_small: (10.0 * scale).max(9.0),
            font_medium: (14.0 * scale).max(11.0),
            font_large: (18.0 * scale).max(14.0),
            font_title: (24.0 * scale).max(18.0),
            font_header: (28.0 * scale).max(20.0),
            // Spacing
            margin: (20.0 * scale).max(10.0),
            padding: (10.0 * scale).max(6.0),
            line_height: (24.0 * scale).max(18.0),
            line_height_small: (20.0 * scale).max(16.0),
            // Tab bar
            tab_height: (28.0 * scale).max(22.0),
            header_height: (45.0 * scale).max(35.0),
            header_height_large: (50.0 * scale).max(40.0),
            // System markers
            system_marker_size: (14.0 * scale).clamp(8.0, 28.0),
            system_marker_size_small: (10.0 * scale).clamp(6.0, 20.0),
            hit_radius: (10.0 * scale).max(8.0),
            // Buttons
            button_width: (140.0 * scale).max(100.0),
            button_height: (50.0 * scale).max(36.0),
            row_height: (25.0 * scale).clamp(20.0, 40.0),
            row_height_large: (60.0 * scale).clamp(40.0, 90.0),
        }
    }
}

/// Get current UI theme (computed fresh each frame)
pub fn theme() -> UiTheme {
    UiTheme::new()
}

pub fn draw_panel(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::from_rgba(12, 18, 34, 230));
    draw_rectangle_lines(x, y, w, h, 1.0, Color::from_rgba(80, 100, 140, 200));
}

fn draw_navigation_tabs(active_buy: bool, active_sell: bool, active_shipyard: bool, active_warp: bool, y: f32) {
    let t = theme();
    let tab_h = t.tab_height;
    let tab_y = y;
    draw_rectangle(0.0, tab_y, screen_width(), tab_h, Color::from_rgba(15, 20, 40, 255));

    let scale = t.scale;
    let tabs = [
        ("Buy", active_buy, 90.0 * scale),
        ("Sell", active_sell, 90.0 * scale),
        ("Ship Yard", active_shipyard, 130.0 * scale),
        ("Warp", active_warp, 90.0 * scale),
    ];

    let mut x = t.margin;
    for (label, active, width) in tabs {
        let bg = if active {
            Color::from_rgba(80, 120, 200, 255)
        } else {
            Color::from_rgba(40, 60, 100, 255)
        };
        draw_rectangle(x, tab_y + 3.0, width, tab_h - 6.0, bg);
        draw_rectangle_lines(
            x,
            tab_y + 3.0,
            width,
            tab_h - 6.0,
            1.0,
            Color::from_rgba(130, 170, 220, 255),
        );
        let text_w = measure_text(label, None, t.font_medium as u16, 1.0).width;
        draw_text(label, x + (width - text_w) / 2.0, tab_y + tab_h * 0.7, t.font_medium, WHITE);
        x += width + t.padding;
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

pub fn short_range_chart_hit_test(
    game_state: &GameState,
    pan: Vec2,
    zoom: f32,
    panel_x: f32,
    panel_y: f32,
    panel_w: f32,
    panel_h: f32,
    mouse: Vec2,
) -> Option<usize> {
    let (center, _radius, scale) =
        short_range_chart_transform(game_state, pan, zoom, panel_x, panel_y, panel_w, panel_h);
    let current = &game_state.solar_systems[game_state.current_system_id];

    let mut best: Option<(usize, f32)> = None;
    for (idx, system) in game_state.solar_systems.iter().enumerate() {
        let dx = (system.x - current.x) as f32;
        let dy = (system.y - current.y) as f32;
        let px = center.x + dx * scale;
        let py = center.y + dy * scale;
        let in_panel = px >= panel_x + 6.0
            && px <= panel_x + panel_w - 6.0
            && py >= panel_y + 6.0
            && py <= panel_y + panel_h - 6.0;
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

fn galactic_chart_transform(
    chart_x: f32,
    chart_y: f32,
    chart_w: f32,
    chart_h: f32,
    pan: Vec2,
    zoom: f32,
) -> (Vec2, f32) {
    // Galaxy coordinates are 0-150 in both dimensions
    let galaxy_size = 150.0;
    
    // Calculate scale to fit galaxy in available space (maintaining aspect ratio)
    let scale = (chart_w.min(chart_h) - 20.0) / galaxy_size * zoom;
    
    // Center the galaxy in the chart area
    let galaxy_rendered_size = galaxy_size * scale;
    let center_x = chart_x + (chart_w - galaxy_rendered_size) / 2.0;
    let center_y = chart_y + (chart_h - galaxy_rendered_size) / 2.0;
    
    // Apply pan (scaled by zoom for consistent feel)
    let origin = vec2(center_x + pan.x * zoom, center_y + pan.y * zoom);
    (origin, scale)
}

pub fn galactic_chart_hit_test(
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
        let in_panel = px >= chart_x + 6.0
            && px <= chart_x + chart_w - 6.0
            && py >= chart_y + 6.0
            && py <= chart_y + chart_h - 6.0;
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
    let t = theme();
    draw_panel(panel_x, panel_y, panel_w, panel_h);

    // Use full-screen camera with viewport for coordinate consistency
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
    camera.viewport = Some((panel_x as i32, panel_y as i32, panel_w as i32, panel_h as i32));
    set_camera(&camera);

    let (center, radius, scale) =
        short_range_chart_transform(game_state, pan, zoom, panel_x, panel_y, panel_w, panel_h);
    let center_x = center.x;
    let center_y = center.y;

    // Helper: check if a point with radius is visible in panel
    let drawable_circle = |x: f32, y: f32, r: f32| -> bool {
        x + r >= panel_x + 6.0
            && x - r <= panel_x + panel_w - 6.0
            && y + r >= panel_y + 6.0
            && y - r <= panel_y + panel_h - 6.0
    };

    // Draw range circles if they fit
    if drawable_circle(center_x, center_y, radius) {
        draw_circle_lines(
            center_x,
            center_y,
            radius,
            2.0,
            Color::from_rgba(200, 200, 220, 200),
        );
    }

    let current = &game_state.solar_systems[game_state.current_system_id];
    let max_range = game_state.ship.max_fuel().max(1) as f32;
    let current_range = game_state.ship.fuel.max(0) as f32;
    let fuel_radius = radius * (current_range / max_range).min(1.0);
    
    if drawable_circle(center_x, center_y, fuel_radius) {
        draw_circle_lines(
            center_x,
            center_y,
            fuel_radius,
            2.0,
            Color::from_rgba(120, 190, 120, 200),
        );
    }

    // Collect labels to render after switching camera
    let mut labels: Vec<(String, f32, f32, Color)> = Vec::new();

    for (idx, system) in game_state.solar_systems.iter().enumerate() {
        let dx = (system.x - current.x) as f32;
        let dy = (system.y - current.y) as f32;
        let dist = (dx * dx + dy * dy).sqrt();
        let px = center_x + dx * scale;
        let py = center_y + dy * scale;

        let in_panel = px >= panel_x + 6.0
            && px <= panel_x + panel_w - 6.0
            && py >= panel_y + 6.0
            && py <= panel_y + panel_h - 6.0;
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

        let marker_r = if is_current { t.system_marker_size * 0.4 } else { t.system_marker_size * 0.3 };
        draw_circle(px, py, marker_r, color);
        if is_selected {
            draw_circle_lines(px, py, marker_r + 2.0 * t.scale, 2.0, Color::from_rgba(255, 230, 150, 220));
        }

        let label_color = if dist <= current_range {
            WHITE
        } else {
            Color::from_rgba(190, 190, 200, 200)
        };
        labels.push((system.name.clone(), px, py, label_color));
    }

    // Draw center crosshair if it's in view
    if center_x >= panel_x + 6.0 && center_x <= panel_x + panel_w - 6.0
        && center_y >= panel_y + 6.0 && center_y <= panel_y + panel_h - 6.0
    {
        draw_line(
            center_x - 6.0,
            center_y,
            center_x + 6.0,
            center_y,
            2.0,
            Color::from_rgba(70, 140, 255, 255),
        );
        draw_line(
            center_x,
            center_y - 6.0,
            center_x,
            center_y + 6.0,
            2.0,
            Color::from_rgba(70, 140, 255, 255),
        );
    }

    let mut waypoint_info: Option<(String, f32)> = None;
    if let Some(waypoint_id) = waypoint_system {
        let target = &game_state.solar_systems[waypoint_id];
        let dx = (target.x - current.x) as f32;
        let dy = (target.y - current.y) as f32;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 0.0 {
            let clamped = dist.min(current_range.max(1.0));
            let vx = dx / dist * clamped * scale;
            let vy = dy / dist * clamped * scale;
            let end_x = center_x + vx;
            let end_y = center_y + vy;
            
            // Only draw waypoint line if endpoints are visible
            if (center_x >= panel_x + 6.0 && center_x <= panel_x + panel_w - 6.0
                && center_y >= panel_y + 6.0 && center_y <= panel_y + panel_h - 6.0)
                && (end_x >= panel_x + 6.0 && end_x <= panel_x + panel_w - 6.0
                && end_y >= panel_y + 6.0 && end_y <= panel_y + panel_h - 6.0)
            {
                draw_line(
                    center_x,
                    center_y,
                    end_x,
                    end_y,
                    2.0,
                    Color::from_rgba(255, 180, 80, 255),
                );
                draw_circle(
                    end_x,
                    end_y,
                    5.0,
                    Color::from_rgba(255, 180, 80, 255),
                );
            }
            waypoint_info = Some((target.name.clone(), dist));
        }
    }

    // Switch back to default camera for UI text rendering (screen space, full resolution)
    set_default_camera();

    // Render system labels in screen space to avoid clipping and maintain legibility
    let label_size = t.font_small;
    let marker_offset = t.system_marker_size * 0.6;
    for (name, px, py, color) in labels {
        let name_w = measure_text(&name, None, label_size as u16, 1.0).width;
        let label_x = px - name_w / 2.0;
        let label_y = py - marker_offset;
        
        // Check bounds in screen space
        let text_in_bounds = label_x >= panel_x + t.padding
            && label_x + name_w <= panel_x + panel_w - t.padding
            && label_y >= panel_y + t.padding
            && label_y + label_size <= panel_y + panel_h - t.padding;
            
        if text_in_bounds {
            draw_text(&name, label_x, label_y, label_size, color);
        }
    }

    // Render waypoint info
    if let Some((target_name, dist)) = waypoint_info {
        draw_text(
            &format!("{:.1} parsecs to {}", dist, target_name),
            panel_x + t.padding,
            panel_y + panel_h - t.line_height_small * 1.8,
            t.font_small,
            WHITE,
        );
    }

    draw_text("Short Range Chart", panel_x + t.padding, panel_y + t.line_height_small, t.font_medium, SKYBLUE);
    draw_text(
        "Range:",
        panel_x + t.padding,
        panel_y + panel_h - t.padding,
        t.font_small,
        LIGHTGRAY,
    );
    let range_label_w = measure_text("Range: ", None, t.font_small as u16, 1.0).width;
    draw_text(
        &format!("{} parsecs", max_range as i32),
        panel_x + t.padding + range_label_w,
        panel_y + panel_h - t.padding,
        t.font_small,
        WHITE,
    );
    let reachable_w = measure_text("● Reachable", None, t.font_small as u16, 1.0).width;
    draw_text(
        "● Reachable",
        panel_x + panel_w - reachable_w - t.padding,
        panel_y + panel_h - t.padding,
        t.font_small,
        Color::from_rgba(80, 200, 90, 255),
    );
}

pub fn draw_galactic_chart(
    game_state: &GameState,
    waypoint_system: Option<usize>,
    selected_system: Option<usize>,
    pan: Vec2,
    zoom: f32,
    search_query: &str,
    search_active: bool,
) {
    let t = theme();
    clear_background(Color::from_rgba(10, 10, 30, 255));

    draw_rectangle(0.0, 0.0, screen_width(), t.header_height, Color::from_rgba(20, 30, 60, 255));
    draw_text("Galactic Chart", t.margin, t.header_height * 0.62, t.font_title, WHITE);
    draw_navigation_tabs(false, false, false, true, t.header_height);

    let chart_x = t.margin;
    let chart_y = t.header_height + t.tab_height + t.padding;
    let chart_w = screen_width() - t.margin * 2.0;
    let chart_h = screen_height() - chart_y - t.header_height * 2.0;
    draw_panel(chart_x, chart_y, chart_w, chart_h);

    // Set up viewport for chart rendering
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
    camera.viewport = Some((chart_x as i32, chart_y as i32, chart_w as i32, chart_h as i32));
    set_camera(&camera);

    let (origin, scale) = galactic_chart_transform(chart_x, chart_y, chart_w, chart_h, pan, zoom);
    let origin_x = origin.x;
    let origin_y = origin.y;

    let current = &game_state.solar_systems[game_state.current_system_id];
    let range_r = game_state.ship.max_fuel() as f32 * scale;
    let current_px = origin_x + current.x as f32 * scale;
    let current_py = origin_y + current.y as f32 * scale;
    
    // Only draw range circle if it's at least partially visible
    if current_px + range_r >= chart_x && current_px - range_r <= chart_x + chart_w
        && current_py + range_r >= chart_y && current_py - range_r <= chart_y + chart_h
    {
        draw_circle_lines(
            current_px,
            current_py,
            range_r,
            2.0,
            Color::from_rgba(200, 200, 220, 160),
        );
    }

    // Collect labels to render after switching camera
    let mut labels: Vec<(String, f32, f32, Color)> = Vec::new();
    let marker_size = t.system_marker_size_small;
    let half_marker = marker_size / 2.0;

    for (idx, system) in game_state.solar_systems.iter().enumerate() {
        let px = origin_x + system.x as f32 * scale;
        let py = origin_y + system.y as f32 * scale;

        // Only draw if visible in panel
        if px < chart_x - half_marker || px > chart_x + chart_w + half_marker
            || py < chart_y - half_marker || py > chart_y + chart_h + half_marker
        {
            continue;
        }

        let mut color = Color::from_rgba(80, 200, 90, 255);
        if system.name == current.name {
            color = Color::from_rgba(70, 140, 255, 255);
        } else if Some(idx) == waypoint_system {
            color = Color::from_rgba(255, 180, 80, 255);
        } else if Some(idx) == selected_system {
            color = Color::from_rgba(255, 230, 150, 255);
        }

        draw_rectangle(px - half_marker, py - half_marker, marker_size, marker_size, color);
        labels.push((system.name.clone(), px, py, color));
    }

    // Switch back to default camera for UI text rendering (screen space, full resolution)
    set_default_camera();

    // Zoom-aware font sizing for labels with scaling
    let label_font_size = (t.font_small * zoom).clamp(t.font_small * 0.8, t.font_medium);
    
    // Only show most labels when zoomed in (reduces clutter)
    let show_all_labels = zoom >= 1.2;

    // Render system labels in screen space to avoid clipping
    for (name, px, py, color) in labels {
        let is_important = color.r > 0.5 || color.g > 0.8; // Current, waypoint, or selected
        
        // Skip non-important labels when zoomed out
        if !show_all_labels && !is_important {
            continue;
        }
        
        let text_w = measure_text(&name, None, label_font_size as u16, 1.0).width;
        let text_x = px + half_marker;
        let text_y = py - half_marker;
        
        // Check bounds in screen space
        let text_in_bounds = text_x >= chart_x + t.padding
            && text_x + text_w <= chart_x + chart_w - t.padding
            && text_y >= chart_y + t.padding
            && text_y + label_font_size <= chart_y + chart_h - t.padding;
            
        if text_in_bounds {
            draw_text(&name, text_x, text_y, label_font_size, WHITE);
        }
    }

    // Render current and waypoint labels (always visible, important systems)
    let current_text_x = current_px + half_marker;
    let current_text_y = current_py - half_marker;
    if current_text_x >= chart_x + t.padding
        && current_text_x + measure_text(&current.name, None, label_font_size as u16, 1.0).width <= chart_x + chart_w - t.padding
        && current_text_y >= chart_y + t.padding
        && current_text_y + label_font_size <= chart_y + chart_h - t.padding
    {
        draw_text(&current.name, current_text_x, current_text_y, label_font_size, Color::from_rgba(100, 180, 255, 255));
    }

    if let Some(waypoint_id) = waypoint_system {
        let target = &game_state.solar_systems[waypoint_id];
        let tx = origin_x + target.x as f32 * scale;
        let ty = origin_y + target.y as f32 * scale;
        let waypoint_text_x = tx + half_marker;
        let waypoint_text_y = ty - half_marker;
        
        if waypoint_text_x >= chart_x + t.padding
            && waypoint_text_x + measure_text(&target.name, None, label_font_size as u16, 1.0).width <= chart_x + chart_w - t.padding
            && waypoint_text_y >= chart_y + t.padding
            && waypoint_text_y + label_font_size <= chart_y + chart_h - t.padding
        {
            draw_text(
                &target.name,
                waypoint_text_x,
                waypoint_text_y,
                label_font_size,
                Color::from_rgba(255, 210, 140, 255),
            );
        }
    }

    let info_y = screen_height() - 75.0;
    let info_system_id = selected_system.unwrap_or(game_state.current_system_id);
    let info_system = &game_state.solar_systems[info_system_id];
    let dx = (info_system.x - current.x) as f32;
    let dy = (info_system.y - current.y) as f32;
    let dist = (dx * dx + dy * dy).sqrt();

    let tech_names = [
        "Pre-Agri",
        "Agri",
        "Medieval",
        "Renaissance",
        "Early Ind",
        "Industrial",
        "Post-Ind",
        "Hi-Tech",
    ];
    let tech_name = tech_names
        .get(info_system.tech_level as usize)
        .unwrap_or(&"Unknown");

    let politics_names = [
        "Anarchy",
        "Capitalist",
        "Communist",
        "Confederacy",
        "Corporate",
        "Cybernetic",
        "Democracy",
        "Dictatorship",
        "Fascist",
        "Feudal",
        "Military",
        "Monarchy",
        "Pacifist",
        "Socialist",
        "Satori",
        "Technocracy",
        "Theocracy",
    ];
    let politics_name = politics_names
        .get(info_system.politics as usize)
        .unwrap_or(&"Unknown");

    let size_names = ["Tiny", "Small", "Medium", "Large", "Huge"];
    let size_name = size_names.get(info_system.size as usize).unwrap_or(&"Unknown");

    draw_text(&format!("{}", info_system.name), t.margin, info_y, t.font_large, WHITE);
    let name_w = measure_text(&info_system.name, None, t.font_large as u16, 1.0).width;
    draw_text(&format!("{:.1} parsecs", dist), t.margin + name_w + t.margin, info_y, t.font_large, WHITE);
    draw_text(
        &format!("{} {} {}", size_name, tech_name, politics_name),
        t.margin,
        info_y + t.line_height_small,
        t.font_medium,
        LIGHTGRAY,
    );

    let footer_y = screen_height() - t.margin * 2.0;
    
    // Zoom indicator in top-right of chart
    let zoom_text = format!("Zoom: {:.1}x", zoom);
    let zoom_text_w = measure_text(&zoom_text, None, t.font_small as u16, 1.0).width;
    draw_text(&zoom_text, chart_x + chart_w - zoom_text_w - t.padding, chart_y + t.line_height_small, t.font_small, LIGHTGRAY);
    
    // Pan indicator if not centered
    if pan.x.abs() > 0.5 || pan.y.abs() > 0.5 {
        let reset_text = "[R to reset view]";
        let reset_w = measure_text(reset_text, None, t.font_small as u16, 1.0).width;
        draw_text(reset_text, chart_x + chart_w - zoom_text_w - reset_w - t.margin, chart_y + t.line_height_small, t.font_small, 
            Color::from_rgba(150, 150, 170, 200));
    }
    
    draw_text(
        "Pan: Arrows/WASD/Drag | Zoom: +/-/Wheel | R: Reset | Click: Select | F: Find | Esc: Back",
        t.margin,
        footer_y,
        t.font_medium,
        LIGHTGRAY,
    );
    if search_active {
        draw_text(
            &format!("Find: {}", search_query),
            t.margin,
            footer_y - t.line_height,
            t.font_medium + 2.0,
            WHITE,
        );

        if !search_query.is_empty() {
            let query = search_query.to_lowercase();
            let mut matches: Vec<(usize, &str, bool)> = game_state
                .solar_systems
                .iter()
                .enumerate()
                .filter_map(|(idx, system)| {
                    let name = system.name.to_lowercase();
                    if name.contains(&query) {
                        Some((idx, system.name.as_str(), name.starts_with(&query)))
                    } else {
                        None
                    }
                })
                .collect();

            matches.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.1.cmp(&b.1)));

            draw_text("Suggestions:", t.margin, footer_y - t.line_height * 2.0, t.font_medium, LIGHTGRAY);
            let mut suggestion_x = t.margin + measure_text("Suggestions: ", None, t.font_medium as u16, 1.0).width;
            for (index, (_, name, _)) in matches.into_iter().take(5).enumerate() {
                let color = if index == 0 {
                    Color::from_rgba(255, 210, 140, 255)
                } else {
                    LIGHTGRAY
                };
                draw_text(name, suggestion_x, footer_y - t.line_height * 2.0, t.font_medium, color);
                suggestion_x += measure_text(name, None, t.font_medium as u16, 1.0).width + t.margin;
            }
        }
    }
}

pub fn draw_warp_screen(
    game_state: &GameState,
    selected: usize,
    message: &str,
    waypoint_system: Option<usize>,
    selected_chart_system: Option<usize>,
    short_pan: Vec2,
    short_zoom: f32,
) {
    let t = theme();
    clear_background(Color::from_rgba(10, 10, 30, 255));

    draw_rectangle(0.0, 0.0, screen_width(), t.header_height, Color::from_rgba(20, 30, 60, 255));
    draw_text("Warp - Select Destination", t.margin, t.header_height * 0.62, t.font_title, GOLD);
    draw_navigation_tabs(false, false, false, true, t.header_height);

    draw_text(
        &format!(
            "Current: {} | Fuel: {}",
            game_state.current_system_name(),
            game_state.ship.fuel
        ),
        t.margin,
        t.header_height + t.tab_height + t.line_height,
        t.font_large,
        WHITE,
    );

    let systems = systems_in_range(game_state);

    let chart_x = t.margin;
    let chart_y = t.header_height + t.tab_height + t.line_height * 2.0;
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
        draw_text(
            "No systems in fuel range!",
            chart_x + chart_w + t.margin,
            chart_y + t.line_height * 2.0,
            t.font_large,
            RED,
        );
        draw_text(
            "Return to station to refuel",
            chart_x + chart_w + t.margin,
            chart_y + t.line_height * 3.5,
            t.font_medium,
            YELLOW,
        );
    } else {
        let list_x = chart_x + chart_w + t.margin;
        let list_w = screen_width() - list_x - t.margin;
        let col_system = list_x;
        let col_dist = list_x + list_w * 0.45;
        let col_fuel = list_x + list_w * 0.7;
        let col_visit = list_x + list_w * 0.85;
        let y_start = chart_y;
        draw_text("System", col_system, y_start, t.font_medium + 2.0, LIGHTGRAY);
        draw_text("Distance", col_dist, y_start, t.font_medium + 2.0, LIGHTGRAY);
        draw_text("Fuel", col_fuel, y_start, t.font_medium + 2.0, LIGHTGRAY);

        for (index, &(system_id, distance)) in systems.iter().enumerate() {
            let y = y_start + t.line_height * 1.5 + (index as f32 * t.line_height);
            let system = &game_state.solar_systems[system_id];
            let fuel_cost = distance.ceil() as i32;

            let color = if index == selected { YELLOW } else { WHITE };
            if index == selected {
                draw_rectangle(
                    list_x - t.padding * 0.5,
                    y - t.line_height * 0.65,
                    list_w,
                    t.line_height * 0.9,
                    Color::from_rgba(50, 50, 100, 128),
                );
            }

            let fuel_color = if fuel_cost <= game_state.ship.fuel {
                GREEN
            } else {
                RED
            };

            draw_text(&system.name, col_system, y, t.font_medium, color);
            draw_text(&format!("{:.1} ly", distance), col_dist, y, t.font_medium, color);
            draw_text(&format!("{}", fuel_cost), col_fuel, y, t.font_medium, fuel_color);

            if system.visited {
                draw_text("✓", col_visit, y, t.font_medium, SKYBLUE);
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

            let tech_names = [
                "Pre-Agri",
                "Agri",
                "Medieval",
                "Renaissance",
                "Early Ind",
                "Industrial",
                "Post-Ind",
                "Hi-Tech",
            ];
            let tech_name = tech_names.get(system.tech_level as usize).unwrap_or(&"Unknown");

            let politics_names = [
                "Anarchy",
                "Capitalist",
                "Communist",
                "Confederacy",
                "Corporate",
                "Cybernetic",
                "Democracy",
                "Dictatorship",
                "Fascist",
                "Feudal",
                "Military",
                "Monarchy",
                "Pacifist",
                "Socialist",
                "Satori",
                "Technocracy",
                "Theocracy",
            ];
            let politics_name = politics_names.get(system.politics as usize).unwrap_or(&"Unknown");

            draw_text(&format!("Selected: {}", system.name), list_x, info_y, 14.0, WHITE);
            draw_text(
                &format!("{} | {}", tech_name, politics_name),
                list_x,
                info_y + 18.0,
                12.0,
                LIGHTGRAY,
            );
            draw_text(
                &format!("Distance: {:.1} ly", dist),
                list_x,
                info_y + 36.0,
                12.0,
                LIGHTGRAY,
            );
            draw_text(
                &format!(
                    "Fuel: {} ({})",
                    fuel_cost,
                    if in_range { "In Range" } else { "Out of Range" }
                ),
                list_x,
                info_y + 54.0,
                12.0,
                if in_range { GREEN } else { RED },
            );
        }
    }

    let inst_y = screen_height() - 100.0;
    draw_text("Controls:", 20.0, inst_y, 18.0, LIGHTGRAY);
    draw_text(
        "↑↓ - Select  |  ENTER/W - Warp  |  G - Galactic Chart  |  ESC/Q - Cancel",
        20.0,
        inst_y + 25.0,
        14.0,
        LIGHTGRAY,
    );
    draw_text(
        "I/J/K/L - Pan Chart  |  +/- or Z/X - Zoom  |  Click: Select",
        20.0,
        inst_y + 45.0,
        14.0,
        LIGHTGRAY,
    );

    if !message.is_empty() {
        let msg_width = measure_text(message, None, 20, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(
            msg_x - 10.0,
            screen_height() / 2.0 - 30.0,
            msg_width + 20.0,
            50.0,
            Color::from_rgba(0, 0, 0, 200),
        );
        draw_text(
            message,
            msg_x,
            screen_height() / 2.0,
            20.0,
            if message.contains("Successfully") {
                GREEN
            } else {
                RED
            },
        );
    }
}

pub fn draw_text_with_limits(text: &str, x: f32, mut y: f32, font_size: f32, color: Color, max_width: f32) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();
    let line_height = font_size * 1.2;

    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        let text_metrics = measure_text(&test_line, None, font_size as u16, 1.0);

        if text_metrics.width <= max_width {
            current_line = test_line;
        } else {
            if !current_line.is_empty() {
                draw_text(&current_line, x, y, font_size, color);
                y += line_height;
            }
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        draw_text(&current_line, x, y, font_size, color);
    }
}

pub fn draw_repair_screen(game_state: &GameState, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));

    draw_rectangle(0.0, 0.0, screen_width(), 50.0, Color::from_rgba(80, 0, 160, 255));
    draw_text("Repair Dock", 20.0, 25.0, 28.0, WHITE);
    draw_text(
        &format!("Credits: {}", game_state.credits),
        screen_width() - 200.0,
        25.0,
        18.0,
        GOLD,
    );

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

        let bar_width = 300.0;
        let bar_height = 20.0;
        let bar_x = left;
        let bar_y = y_start + 60.0;

        draw_rectangle(
            bar_x,
            bar_y,
            bar_width,
            bar_height,
            Color::from_rgba(50, 50, 50, 255),
        );
        let repair_percentage = game_state.ship.hull as f32 / max_hull as f32;
        draw_rectangle(bar_x, bar_y, bar_width * repair_percentage, bar_height, GREEN);
        draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, WHITE);

        let option_y = y_start + 120.0;
        draw_text("Repair Options:", left, option_y, 18.0, LIGHTGRAY);

        let repair_small = 10.min(damage_taken);
        let cost_small = calculate_repair_cost_per_point(game_state) * repair_small;
        let affordable_small = game_state.credits >= cost_small && damage_taken > 0;
        let color_small = if affordable_small { GREEN } else { RED };
        let cost_small_str = if cost_small > 0 {
            format!("{} cr", cost_small)
        } else {
            "FREE".to_string()
        };

        draw_text(
            &format!("1 - Repair {} HP ({})", repair_small, cost_small_str),
            left,
            option_y + 35.0,
            14.0,
            color_small,
        );

        let repair_medium = 50.min(damage_taken);
        let cost_medium = calculate_repair_cost_per_point(game_state) * repair_medium;
        let affordable_medium = game_state.credits >= cost_medium && damage_taken > 0;
        let color_medium = if affordable_medium { GREEN } else { RED };
        let cost_medium_str = if cost_medium > 0 {
            format!("{} cr", cost_medium)
        } else {
            "FREE".to_string()
        };

        draw_text(
            &format!("2 - Repair {} HP ({})", repair_medium, cost_medium_str),
            left,
            option_y + 55.0,
            14.0,
            color_medium,
        );

        let affordable_full = game_state.credits >= full_repair_cost && damage_taken > 0;
        let color_full = if affordable_full { GREEN } else { RED };
        let full_repair_str = if full_repair_cost > 0 {
            format!("{} cr", full_repair_cost)
        } else {
            "FREE".to_string()
        };

        draw_text(
            &format!("3 - Repair All Damage ({})", full_repair_str),
            left,
            option_y + 75.0,
            14.0,
            color_full,
        );

        draw_text(
            &format!("Cost per HP: {} cr", cost_per_point),
            left,
            option_y + 110.0,
            12.0,
            LIGHTGRAY,
        );

        if damage_taken == 0 {
            draw_text("Ship is fully repaired!", left, option_y + 140.0, 14.0, SKYBLUE);
        }
    }

    let inst_y = screen_height() - 100.0;
    draw_text("Controls:", 20.0, inst_y, 18.0, LIGHTGRAY);
    draw_text(
        "1 - Repair 10 HP  |  2 - Repair 50 HP  |  3 - Repair All  |  ESC/Q - Back",
        20.0,
        inst_y + 25.0,
        14.0,
        LIGHTGRAY,
    );

    if !message.is_empty() {
        let msg_width = measure_text(message, None, 18, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(
            msg_x - 10.0,
            screen_height() / 2.0 + 50.0,
            msg_width + 20.0,
            50.0,
            Color::from_rgba(0, 0, 0, 200),
        );
        let msg_color = if message.contains("Repaired") { GREEN } else { RED };
        draw_text(message, msg_x, screen_height() / 2.0 + 75.0, 18.0, msg_color);
    }
}

pub fn draw_shipyard_screen(game_state: &GameState, selected: usize, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));

    draw_rectangle(0.0, 0.0, screen_width(), 50.0, Color::from_rgba(80, 0, 160, 255));
    draw_text("Shipyard", 20.0, 25.0, 28.0, WHITE);
    draw_text(
        &format!("Credits: {}", game_state.credits),
        screen_width() - 200.0,
        25.0,
        18.0,
        GOLD,
    );

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
        let y_start = 110.0;
        let name_col = 40.0;
        let desc_col = 250.0;
        let cost_col = screen_width() - 150.0;

        draw_text("Upgrade", name_col, y_start, 16.0, LIGHTGRAY);
        draw_text("Description", desc_col, y_start, 14.0, LIGHTGRAY);
        draw_text("Cost", cost_col, y_start, 16.0, LIGHTGRAY);

        for (i, (upgrade, cost)) in upgrades.iter().enumerate() {
            let y = y_start + 40.0 + (i as f32 * 60.0);

            if i == selected {
                draw_rectangle(
                    15.0,
                    y - 20.0,
                    screen_width() - 30.0,
                    55.0,
                    Color::from_rgba(50, 50, 100, 128),
                );
            }

            let color = if i == selected { YELLOW } else { WHITE };

            draw_text(upgrade.name(), name_col, y, 16.0, color);
            draw_text_with_limits(upgrade.description(), desc_col, y, 12.0, LIGHTGRAY, 350.0);

            let cost_color = if game_state.credits >= *cost { GREEN } else { RED };
            draw_text(&format!("{} cr", cost), cost_col, y, 16.0, cost_color);
        }
    }

    let inst_y = screen_height() - 100.0;
    draw_text("Controls:", 20.0, inst_y, 18.0, LIGHTGRAY);
    draw_text(
        "↑↓ - Select  |  ENTER/P - Purchase  |  ESC/Q - Back",
        20.0,
        inst_y + 25.0,
        14.0,
        LIGHTGRAY,
    );

    if !message.is_empty() {
        let msg_width = measure_text(message, None, 18, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(
            msg_x - 10.0,
            screen_height() / 2.0 + 50.0,
            msg_width + 20.0,
            50.0,
            Color::from_rgba(0, 0, 0, 200),
        );
        let msg_color = if message.contains("Installed") || message.contains("upgraded") {
            GREEN
        } else {
            RED
        };
        draw_text(message, msg_x, screen_height() / 2.0 + 75.0, 18.0, msg_color);
    }
}

pub fn draw_trading_screen(game_state: &GameState, selected: usize, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));

    draw_rectangle(0.0, 0.0, screen_width(), 45.0, Color::from_rgba(20, 30, 60, 255));

    draw_text(
        &format!("Trading - {}", game_state.current_system_name()),
        20.0,
        28.0,
        24.0,
        GOLD,
    );

    draw_navigation_tabs(true, true, false, false, 45.0);

    draw_text(
        &format!("Credits: {} cr", game_state.credits),
        20.0,
        85.0,
        18.0,
        WHITE,
    );

    draw_text(
        &format!(
            "Cargo: {}/{}",
            game_state.ship.total_cargo(),
            game_state.ship.cargo_bays_available() + game_state.ship.total_cargo()
        ),
        20.0,
        110.0,
        18.0,
        WHITE,
    );

    let y_start = 145.0;
    draw_text("Good", 20.0, y_start, 16.0, LIGHTGRAY);
    draw_text("Price", 180.0, y_start, 16.0, LIGHTGRAY);
    draw_text("Avail", 260.0, y_start, 16.0, LIGHTGRAY);
    draw_text("Cargo", 340.0, y_start, 16.0, LIGHTGRAY);
    draw_text("Max", 420.0, y_start, 16.0, LIGHTGRAY);

    let system_id = game_state.current_system_id;
    for i in 0..10 {
        let y = y_start + 35.0 + (i as f32 * 25.0);
        let good = TradeGood::from_index(i);
        let price = get_buy_price(game_state, good);
        let available = game_state.solar_systems[system_id].qty[i];
        let in_hold = game_state.ship.cargo[i];
        let max = max_buyable(game_state, good);

        let color = if i == selected { YELLOW } else { WHITE };
        if i == selected {
            draw_rectangle(
                15.0,
                y - 18.0,
                screen_width() - 30.0,
                23.0,
                Color::from_rgba(50, 50, 100, 128),
            );
        }

        draw_text(good.name(), 20.0, y, 16.0, color);
        draw_text(&format!("{} cr", price), 180.0, y, 16.0, color);
        draw_text(&format!("{}", available), 260.0, y, 16.0, color);
        draw_text(&format!("{}", in_hold), 340.0, y, 16.0, color);
        draw_text(&format!("{}", max), 420.0, y, 16.0, color);
    }

    let panel_x = 520.0;
    let panel_w = screen_width() - panel_x - 20.0;
    if panel_w > 120.0 {
        let panel_y = y_start - 10.0;
        let panel_h = 220.0;
        draw_rectangle(
            panel_x,
            panel_y,
            panel_w,
            panel_h,
            Color::from_rgba(20, 28, 50, 220),
        );
        draw_rectangle_lines(
            panel_x,
            panel_y,
            panel_w,
            panel_h,
            1.0,
            Color::from_rgba(80, 100, 150, 255),
        );

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
                let trend = if last > prev {
                    "↑ Rising"
                } else if last < prev {
                    "↓ Falling"
                } else {
                    "→ Flat"
                };
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

    let inst_y = screen_height() - 100.0;
    draw_text("Controls:", 20.0, inst_y, 18.0, LIGHTGRAY);
    draw_text(
        "↑↓ - Select  |  B - Buy 1  |  5 - Buy 5  |  S - Sell 1  |  A - Sell All",
        20.0,
        inst_y + 25.0,
        14.0,
        LIGHTGRAY,
    );
    draw_text("ESC/Q - Exit Trading", 20.0, inst_y + 50.0, 14.0, LIGHTGRAY);

    if !message.is_empty() {
        let msg_width = measure_text(message, None, 20, 1.0).width;
        let msg_x = (screen_width() - msg_width) / 2.0;
        draw_rectangle(
            msg_x - 10.0,
            screen_height() / 2.0 - 30.0,
            msg_width + 20.0,
            50.0,
            Color::from_rgba(0, 0, 0, 200),
        );
        draw_text(message, msg_x, screen_height() / 2.0, 20.0, GREEN);
    }
}

pub fn draw_system_info_screen(
    game_state: &GameState,
    show_newspaper: bool,
    newspaper_unlocked: bool,
    message: &str,
) {
    clear_background(Color::from_rgba(10, 10, 30, 255));

    draw_rectangle(0.0, 0.0, screen_width(), 40.0, Color::from_rgba(80, 0, 160, 255));
    draw_text("System Info", 20.0, 20.0, 24.0, WHITE);
    draw_text("I", screen_width() - 40.0, 20.0, 20.0, WHITE);

    let current_system = &game_state.solar_systems[game_state.current_system_id];
    let tech_names = [
        "Pre-agricultural",
        "Agricultural",
        "Medieval",
        "Renaissance",
        "Early Industrial",
        "Industrial",
        "Post-Industrial",
        "Hi-Tech",
    ];
    let tech_idx = current_system.tech_level as usize;
    let tech_name = if tech_idx < tech_names.len() {
        tech_names[tech_idx]
    } else {
        "Unknown"
    };

    let politics_names = [
        "Anarchy",
        "Capitalist",
        "Communist",
        "Confederacy",
        "Corporate",
        "Cybernetic",
        "Democracy",
        "Dictatorship",
        "Fascist",
        "Feudal",
        "Military",
        "Monarchy",
        "Pacifist",
        "Socialist",
        "Satori",
        "Technocracy",
        "Theocracy",
    ];
    let pol_idx = current_system.politics as usize;
    let politics_name = if pol_idx < politics_names.len() {
        politics_names[pol_idx]
    } else {
        "Unknown"
    };

    let size_names = ["Tiny", "Small", "Medium", "Large", "Huge"];
    let size_idx = current_system.size as usize;
    let size_name = if size_idx < size_names.len() {
        size_names[size_idx]
    } else {
        "Unknown"
    };

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
    let resource_name = if res_idx < resource_names.len() {
        resource_names[res_idx]
    } else {
        "Unknown"
    };

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

    let content_top = 55.0;
    let content_bottom = screen_height() - 90.0;
    let content_h = content_bottom - content_top;
    draw_rectangle(
        10.0,
        content_top,
        screen_width() - 20.0,
        content_h,
        Color::from_rgba(18, 25, 45, 255),
    );
    draw_rectangle_lines(
        10.0,
        content_top,
        screen_width() - 20.0,
        content_h,
        1.0,
        Color::from_rgba(80, 80, 120, 255),
    );

    let left_x = 30.0;
    let right_x = screen_width() / 2.0 + 10.0;
    let panel_w = screen_width() / 2.0 - 40.0;
    let panel_h = content_h - 20.0;
    let panel_y = content_top + 10.0;

    draw_rectangle(
        left_x - 10.0,
        panel_y,
        panel_w,
        panel_h,
        Color::from_rgba(12, 18, 34, 255),
    );
    draw_rectangle(
        right_x - 10.0,
        panel_y,
        panel_w,
        panel_h,
        Color::from_rgba(12, 18, 34, 255),
    );
    draw_text("Overview", left_x, panel_y + 24.0, 18.0, SKYBLUE);
    draw_text("Market Snapshot", right_x, panel_y + 24.0, 18.0, SKYBLUE);

    let line_height = 28.0;
    let y_start = panel_y + 60.0;
    draw_text("Name:", left_x, y_start, 16.0, LIGHTGRAY);
    draw_text(&current_system.name, left_x + 160.0, y_start, 16.0, WHITE);

    draw_text("Size:", left_x, y_start + line_height, 16.0, LIGHTGRAY);
    draw_text(size_name, left_x + 160.0, y_start + line_height, 16.0, WHITE);

    draw_text("Tech Level:", left_x, y_start + line_height * 2.0, 16.0, LIGHTGRAY);
    draw_text(
        tech_name,
        left_x + 160.0,
        y_start + line_height * 2.0,
        16.0,
        WHITE,
    );

    draw_text("Government:", left_x, y_start + line_height * 3.0, 16.0, LIGHTGRAY);
    draw_text(
        politics_name,
        left_x + 160.0,
        y_start + line_height * 3.0,
        16.0,
        WHITE,
    );

    draw_text("Resources:", left_x, y_start + line_height * 4.0, 16.0, LIGHTGRAY);
    draw_text(
        resource_name,
        left_x + 160.0,
        y_start + line_height * 4.0,
        16.0,
        WHITE,
    );

    draw_text("Coordinates:", left_x, y_start + line_height * 5.0, 16.0, LIGHTGRAY);
    draw_text(
        &format!("{}, {}", current_system.x, current_system.y),
        left_x + 160.0,
        y_start + line_height * 5.0,
        16.0,
        WHITE,
    );

    draw_text("Visited:", left_x, y_start + line_height * 6.0, 16.0, LIGHTGRAY);
    draw_text(
        visited_label,
        left_x + 160.0,
        y_start + line_height * 6.0,
        16.0,
        WHITE,
    );

    draw_text("Special Event:", left_x, y_start + line_height * 7.0, 16.0, LIGHTGRAY);
    draw_text(
        special_event_label,
        left_x + 160.0,
        y_start + line_height * 7.0,
        16.0,
        WHITE,
    );

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
        draw_text_with_limits(preview, right_x, market_y + 248.0, 14.0, WHITE, panel_w - 20.0);
        draw_text(
            "Buy newspaper for full report.",
            right_x,
            market_y + 270.0,
            12.0,
            GRAY,
        );
    }

    if show_newspaper {
        let dialog_width = 500.0;
        let dialog_height = 180.0;
        let dialog_x = (screen_width() - dialog_width) / 2.0;
        let dialog_y = screen_height() / 2.0 - 50.0;

        draw_rectangle(
            dialog_x,
            dialog_y,
            dialog_width,
            dialog_height,
            Color::from_rgba(80, 0, 160, 255),
        );
        draw_rectangle(
            dialog_x + 2.0,
            dialog_y + 2.0,
            dialog_width - 4.0,
            dialog_height - 4.0,
            Color::from_rgba(200, 200, 255, 255),
        );

        draw_text("Buy Newspaper?", dialog_x + 20.0, dialog_y + 20.0, 18.0, BLACK);
        draw_text(
            "Local newspaper costs 1 credit.",
            dialog_x + 20.0,
            dialog_y + 50.0,
            14.0,
            BLACK,
        );
        draw_text(
            "Unlock full market report.",
            dialog_x + 20.0,
            dialog_y + 70.0,
            14.0,
            BLACK,
        );

        draw_rectangle(dialog_x + 50.0, dialog_y + 110.0, 120.0, 40.0, WHITE);
        draw_text("Buy (B)", dialog_x + 70.0, dialog_y + 125.0, 14.0, BLACK);

        draw_rectangle(dialog_x + 250.0, dialog_y + 110.0, 120.0, 40.0, WHITE);
        draw_text("Cancel (C)", dialog_x + 265.0, dialog_y + 125.0, 14.0, BLACK);
    }

    let inst_y = screen_height() - 60.0;
    draw_text("Controls:", 20.0, inst_y, 14.0, LIGHTGRAY);
    if !show_newspaper {
        draw_text(
            "N - Buy Newspaper  |  ESC/Q - Back",
            20.0,
            inst_y + 20.0,
            12.0,
            LIGHTGRAY,
        );
    } else {
        draw_text("B - Buy  |  C - Cancel", 20.0, inst_y + 20.0, 12.0, LIGHTGRAY);
    }

    if !message.is_empty() {
        draw_text(message, 20.0, screen_height() - 30.0, 14.0, GREEN);
    }
}

pub fn draw_encounter_screen(encounter: &Encounter, message: &str) {
    clear_background(Color::from_rgba(10, 10, 30, 255));

    draw_rectangle(0.0, 0.0, screen_width(), 50.0, Color::from_rgba(80, 0, 160, 255));
    draw_text("Encounter", 20.0, 25.0, 28.0, WHITE);
    draw_text("!", screen_width() - 40.0, 25.0, 28.0, YELLOW);

    draw_rectangle(
        10.0,
        60.0,
        screen_width() - 20.0,
        screen_height() - 130.0,
        Color::from_rgba(240, 240, 250, 255),
    );

    let ship_y = 140.0;
    let left_ship_x = 100.0;
    let right_ship_x = screen_width() - 200.0;

    draw_triangle(
        vec2(left_ship_x, ship_y - 25.0),
        vec2(left_ship_x - 20.0, ship_y + 25.0),
        vec2(left_ship_x + 20.0, ship_y + 25.0),
        BLUE,
    );

    let (red, green, blue) = encounter.get_color_rgb();
    let encounter_color = Color::from_rgba(red, green, blue, 255);
    draw_circle(right_ship_x, ship_y, 20.0, encounter_color);
    draw_rectangle(right_ship_x - 15.0, ship_y - 10.0, 30.0, 20.0, encounter_color);

    draw_circle(screen_width() - 80.0, 100.0, 15.0, YELLOW);

    let text_x = 40.0;
    let text_y = 280.0;
    let max_width = screen_width() - 80.0;
    draw_text_with_limits(
        &encounter.description,
        text_x,
        text_y,
        20.0,
        BLACK,
        max_width,
    );

    let button_y = screen_height() - 100.0;
    let button_width = 140.0;
    let button_height = 50.0;
    let button_spacing = 200.0;

    let attack_x = screen_width() / 2.0 - button_spacing / 2.0 - button_width / 2.0;
    let ignore_x = screen_width() / 2.0 + button_spacing / 2.0 - button_width / 2.0;

    draw_rectangle(attack_x, button_y, button_width, button_height, WHITE);
    draw_rectangle_lines(attack_x, button_y, button_width, button_height, 3.0, BLACK);
    draw_text("Attack (A)", attack_x + 20.0, button_y + 30.0, 18.0, BLACK);

    draw_rectangle(ignore_x, button_y, button_width, button_height, WHITE);
    draw_rectangle_lines(ignore_x, button_y, button_width, button_height, 3.0, BLACK);
    draw_text("Ignore (I)", ignore_x + 20.0, button_y + 30.0, 18.0, BLACK);

    if !message.is_empty() {
        draw_text(message, 20.0, screen_height() - 25.0, 14.0, GREEN);
    }
}
