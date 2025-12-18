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