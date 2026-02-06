// Main menu UI

use macroquad::prelude::*;

fn draw_starfield() {
    let w = screen_width();
    let h = screen_height();
    let star_count = 120;
    for i in 0..star_count {
        let fx = (i as f32 * 97.0) % w;
        let fy = (i as f32 * 53.0 + (i as f32 * 11.0).sin() * 30.0) % h;
        let brightness = 0.6 + ((i % 7) as f32) * 0.05;
        draw_circle(fx, fy, 1.0 + (i % 3) as f32 * 0.4, Color::new(brightness, brightness, brightness, 1.0));
    }
}

fn draw_moon() {
    let w = screen_width();
    let h = screen_height();
    let moon_x = w * 0.78;
    let moon_y = h * 0.25;
    let moon_r = 60.0;
    draw_circle(moon_x, moon_y, moon_r, Color::from_rgba(210, 210, 220, 255));
    draw_circle(moon_x + 18.0, moon_y - 10.0, 14.0, Color::from_rgba(190, 190, 205, 255));
    draw_circle(moon_x - 15.0, moon_y + 20.0, 10.0, Color::from_rgba(180, 180, 195, 255));
}

fn draw_ship_silhouette() {
    let w = screen_width();
    let h = screen_height();
    let base_x = w * 0.12;
    let base_y = h * 0.72;

    // Hull
    draw_rectangle(base_x, base_y, 160.0, 26.0, Color::from_rgba(30, 40, 60, 255));
    // Nose
    draw_triangle(
        Vec2::new(base_x + 160.0, base_y),
        Vec2::new(base_x + 210.0, base_y + 13.0),
        Vec2::new(base_x + 160.0, base_y + 26.0),
        Color::from_rgba(30, 40, 60, 255),
    );
    // Bridge
    draw_rectangle(base_x + 40.0, base_y - 12.0, 50.0, 12.0, Color::from_rgba(40, 55, 80, 255));
    // Engine glow
    draw_rectangle(base_x - 12.0, base_y + 6.0, 12.0, 14.0, Color::from_rgba(60, 120, 200, 200));
}

pub async fn draw_main_menu() {
    // Background gradient (manual)
    let top = Color::from_rgba(6, 8, 18, 255);
    let bottom = Color::from_rgba(12, 18, 36, 255);
    let h = screen_height();
    let steps = 40;
    for i in 0..steps {
        let t = i as f32 / (steps - 1) as f32;
        let r = top.r + (bottom.r - top.r) * t;
        let g = top.g + (bottom.g - top.g) * t;
        let b = top.b + (bottom.b - top.b) * t;
        let y = h * (i as f32 / steps as f32);
        draw_rectangle(0.0, y, screen_width(), h / steps as f32 + 1.0, Color::new(r, g, b, 1.0));
    }

    draw_starfield();
    draw_moon();
    draw_ship_silhouette();
    
    // Title
    let title = "SPACE TRADER";
    let title_size = 48.0;
    let title_width = measure_text(title, None, title_size as u16, 1.0).width;
    draw_text(
        title,
        screen_width() / 2.0 - title_width / 2.0,
        120.0,
        title_size,
        WHITE,
    );

    // Subtitle
    draw_text(
        "A Rust Port of the Classic Palm OS Game",
        screen_width() / 2.0 - 210.0,
        155.0,
        18.0,
        LIGHTGRAY,
    );
    
    // Menu panel
    let panel_w = 320.0;
    let panel_h = 170.0;
    let panel_x = screen_width() / 2.0 - panel_w / 2.0;
    let panel_y = screen_height() / 2.0 - panel_h / 2.0 + 20.0;
    draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::from_rgba(10, 20, 35, 200));
    draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 2.0, Color::from_rgba(80, 140, 220, 200));

    // Menu options
    draw_text(
        "N - New Game",
        panel_x + 40.0,
        panel_y + 50.0,
        24.0,
        WHITE,
    );
    
    draw_text(
        "L - Load Game",
        panel_x + 40.0,
        panel_y + 90.0,
        24.0,
        WHITE,
    );
    
    draw_text(
        "Q - Quit",
        panel_x + 40.0,
        panel_y + 130.0,
        24.0,
        WHITE,
    );
    
    // Footer
    draw_text(
        "Based on the classic Palm OS game by Pieter Spronck",
        screen_width() / 2.0 - 220.0,
        screen_height() - 50.0,
        16.0,
        GRAY,
    );
    
    draw_text(
        "Inspired by Elite",
        screen_width() / 2.0 - 70.0,
        screen_height() - 25.0,
        16.0,
        GRAY,
    );
}
