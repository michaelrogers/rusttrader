// In-game UI screens

use crate::game::pricing::get_buy_price;
use crate::game::repair::{
    calculate_full_repair_cost, calculate_repair_cost_per_point, can_repair, get_max_hull,
};
use crate::game::trading::max_buyable;
use crate::game::upgrades::get_available_upgrades;
use crate::types::{GameState, TradeGood};
use macroquad::prelude::*;

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
        let text_w = measure_text(label, None, 14, 1.0).width;
        draw_text(label, x + (width - text_w) / 2.0, tab_y + 20.0, 14.0, WHITE);
        x += width + 10.0;
    }
}

fn draw_text_with_limits(text: &str, x: f32, mut y: f32, font_size: f32, color: Color, max_width: f32) {
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
