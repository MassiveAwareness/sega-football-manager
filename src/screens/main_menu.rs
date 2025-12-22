#![allow(dead_code)]

use super::AppState;
use macroquad::prelude::*;
use crate::constants::{SEGA_GRAY};
use crate::ui::{draw_sega_box, draw_sega_button, draw_sega_text};

pub fn draw_main_menu(font: Option<&Font>, mouse_pos: (f32, f32)) -> AppState {
    draw_sega_box(100.0, 100.0, 600.0, 400.0, Some("SEGA FOOTBALL MANAGER - v0.2.1 (BETA)"), font);

    if draw_sega_button(250.0, 280.0, 300.0, 50.0, "PLAY (ENTER)", font, mouse_pos) ||
    is_key_pressed(KeyCode::Enter) {
        return AppState::Dashboard;
    }

    draw_sega_text("(C) Dave Summer, 2025", 300.0, 450.0, font, 16, SEGA_GRAY);
    AppState::MainMenu
}