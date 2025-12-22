#![allow(dead_code)]

use super::AppState;
use crate::models::Game;
use macroquad::prelude::*;
use crate::constants::{SEGA_YELLOW, SEGA_WHITE};
use crate::ui::{draw_sega_box, draw_sega_button, draw_sega_text};

pub fn draw_end_of_season(game: &mut Game, font: Option<&Font>, mouse_pos: (f32, f32)) -> AppState {
    draw_sega_box(150.0, 150.0, 500.0, 300.0, Some("END OF SEASON"), font);

    let winner = &game.get_standings()[0];
    draw_sega_text("CHAMPION:", 280.0, 220.0, font, 24, SEGA_WHITE);
    draw_sega_text(&winner.name, 280.0, 260.0, font, 30, SEGA_YELLOW);
    if draw_sega_button(250.0, 350.0, 300.0, 50.0, "EXIT TO MAIN MENU (ESC)", font, mouse_pos) ||
    is_key_pressed(KeyCode::Escape) {
        *game = Game::new();
        return AppState::MainMenu;
    }

    AppState::EndOfSeason
}