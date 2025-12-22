#![allow(dead_code)]
use crate::constants::*;
use macroquad::prelude::*;

pub fn draw_sega_box(x: f32, y: f32, w: f32, h: f32, title: Option<&str>, font: Option<&Font>) {
    draw_rectangle(x + 4.0, y + 4.0, w, h, BLACK);
    draw_rectangle(x, y, w, h, SEGA_BLUE);
    draw_rectangle_lines(x, y, w, h, 2.0, SEGA_WHITE);

    if let Some(t) = title {
        // Címsor háttér sáv
        let header_height = 32.0;
        draw_rectangle(x + 3.0, y + 3.0, w - 6.0, header_height, SEGA_LIGHT_BLUE);

        // Szöveg mérése a középre igazításhoz
        let font_size = 24;
        let text_dims = measure_text(t, font, font_size, 1.0);

        // Középre igazítás képlete: box_x + (box_width - text_width) / 2
        let text_x = x + (w - text_dims.width) / 2.0;
        // Y pozíció: box_y + header_center + (font_height / 2)
        let text_y = y + 24.0;

        draw_text_ex(t, text_x, text_y, TextParams {
            font,
            font_size,
            color: SEGA_YELLOW,
            ..Default::default()
        });
    }
}

// Segédfüggvény stövegíráshoz, hogy ne kelljen mindig begépelni a TextParams-t
pub fn draw_sega_text(text: &str, x: f32, y: f32, font: Option<&Font>, size: u16, color: Color) {
    draw_text_ex(text, x, y, TextParams {
        font,
        font_size: size,
        color,
        ..Default::default()
    });
}

// Interaktív gombok
pub fn draw_sega_button(x: f32, y: f32, w: f32, h: f32, text: &str, font: Option<&Font>, mouse_pos: (f32, f32)) -> bool {
    let mx = mouse_pos.0;
    let my = mouse_pos.1;

    let is_hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;

    let bg_color = if is_hovered { SEGA_LIGHT_BLUE } else { SEGA_BLUE };
    let text_color = if is_hovered { SEGA_YELLOW } else { SEGA_WHITE };

    draw_rectangle(x + 4.0, y+ 4.0, w, h, BLACK);
    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, 2.0, SEGA_WHITE);

    let font_size = 24;
    let text_dims = measure_text(text, font, font_size, 1.0);
    let text_x = x + (w - text_dims.width) / 2.0;
    let text_y = y + (h / 2.0) + (text_dims.height / 3.0);

    draw_text_ex(text, text_x, text_y, TextParams {
        font,
        font_size,
        color: text_color,
        ..Default::default()
    });

    if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
        return true;
    }

    false
}

pub fn draw_sega_line(x1: f32, y1: f32, x2: f32, y2: f32) {
    let thickness = 5.0;

    if (x1 - x2).abs() < 0.1 {
        let h = (y2 - y1).abs();
        let start_y = y1.min(y2);
        draw_rectangle(x1, start_y, thickness, h, SEGA_WHITE);
    } else if (y1- y2).abs() < 0.1 {
        let w = (x2 - x1).abs();
        let start_x = x1.min(x2);
        draw_rectangle(start_x, y1, w, thickness, SEGA_WHITE);
    } else {
        draw_line(x1, y1, x2, y2, thickness, SEGA_WHITE);
    }
}