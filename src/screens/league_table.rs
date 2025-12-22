#![allow(dead_code)]

use super::AppState;
use crate::models::Game;
use macroquad::prelude::*;
use crate::constants::{SEGA_YELLOW, SEGA_WHITE};
use crate::ui::{draw_sega_box, draw_sega_button, draw_sega_text, draw_sega_line};

pub fn draw_league_table(game: &Game, font: Option<&Font>, mouse_pos: (f32, f32)) -> AppState {
    draw_sega_box(50.0, 50.0, 700.0, 500.0, Some("STANDINGS"), font);

    let hy = 110.0;
    draw_sega_text("#", 60.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("TEAM", 130.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("W", 350.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("D", 400.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("L", 450.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("GF", 500.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("GA", 550.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("PTS", 600.0, hy, font, 20, SEGA_YELLOW);

    draw_sega_line(50.0, 115.0, 750.0, 115.0);
    draw_sega_line(120.0, 80.0, 120.0, 420.0);

    for (i, t) in game.get_standings().iter().enumerate() {
        let y = 140.0 + (i as f32 * 30.0);
        let color = if t.name == game.teams[game.player_team_index].name { SEGA_YELLOW } else { SEGA_WHITE };

        draw_sega_text(&format!("#{}", i + 1), 60.0, y, font, 20, color);
        draw_sega_text(&t.name, 130.0, y, font, 20, color);
        draw_sega_text(&t.wins.to_string(), 350.0, y, font, 20, color);
        draw_sega_text(&t.draws.to_string(), 400.0, y, font, 20, color);
        draw_sega_text(&t.losses.to_string(), 450.0, y, font, 20, color);
        draw_sega_text(&t.goals_for.to_string(), 500.0, y, font, 20, color);
        draw_sega_text(&t.goals_against.to_string(), 550.0, y, font, 20, color);
        draw_sega_text(&t.points.to_string(), 600.0, y, font, 20, color);
    }

    if draw_sega_button(250.0, 500.0, 300.0, 40.0, "BACK (SPACE)", font, mouse_pos) ||
    is_key_pressed(KeyCode::Space) {
        return AppState::Dashboard;
    }

    AppState::LeagueTable
}