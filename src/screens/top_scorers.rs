#![allow(dead_code)]

use super::AppState;
use crate::models::Game;
use macroquad::prelude::*;
use crate::constants::{SEGA_YELLOW, SEGA_WHITE, SEGA_GRAY};
use crate::ui::{draw_sega_box, draw_sega_button, draw_sega_text, draw_sega_line};

pub fn draw_top_scorers(game: &Game, font: Option<&Font>, mouse_pos: (f32, f32)) -> AppState {
    draw_sega_box(50.0, 50.0, 700.0, 500.0, Some("TOP SCORERS"), font);

    let hy = 100.0;
    draw_sega_text("#", 60.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("PLAYER", 120.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("TEAM", 380.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("GOALS", 600.0, hy, font, 20, SEGA_YELLOW);

    draw_sega_line(50.0, 115.0, 750.0, 115.0);
    draw_sega_line(100.0, 80.0, 100.0, 450.0);

    let scorers = game.get_top_scorers();
    if scorers.is_empty() {
        draw_sega_text("NO GOALS SCORED YET...", 280.0, 250.0, font, 24, SEGA_GRAY);
    } else {
        for (i, (player_name, team_name, goals)) in scorers.iter().enumerate() {
            let y = 140.0 + (i as f32 * 30.0);
            let color = if *team_name == game.teams[game.player_team_index].name { SEGA_YELLOW } else { SEGA_WHITE };

            draw_sega_text(&format!("#{}", i + 1), 60.0, y, font, 20, color);
            draw_sega_text(player_name, 120.0, y, font, 20, color);
            draw_sega_text(team_name, 380.0, y, font, 20, color);
            draw_sega_text(&goals.to_string(), 600.0, y, font, 20, color);
        }
    }

    if draw_sega_button(250.0, 500.0, 300.0, 40.0, "BACK (SPACE)", font, mouse_pos) ||
    is_key_pressed(KeyCode::Space) {
        return AppState::Dashboard;
    }

    AppState::TopScorers
}