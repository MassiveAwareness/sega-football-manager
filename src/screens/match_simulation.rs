#![allow(dead_code)]

use super::AppState;
use crate::models::Game;
use macroquad::prelude::*;
use crate::constants::SEGA_WHITE;
use crate::ui::{draw_sega_box, draw_sega_button, draw_sega_text};

pub fn draw_match_simulation(game: &Game, font: Option<&Font>, mouse_pos: (f32, f32)) -> AppState {
    draw_sega_box(150.0, 150.0, 500.0, 300.0, Some("MATCHDAY"), font);

    draw_sega_text("ROUND IS PLAYED...", 240.0, 250.0, font, 30, SEGA_WHITE);
    if draw_sega_button(250.0, 350.0, 300.0, 50.0, "NEXT (SPACE)", font, mouse_pos) ||
    is_key_pressed(KeyCode::Space) {
        if game.week > game.total_weeks {
            return AppState::EndOfSeason;
        } else {
            return AppState::Dashboard;
        }
    }

    AppState::MatchSimulation
}