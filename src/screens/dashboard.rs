#![allow(dead_code)]

use super::AppState;
use crate::models::Game;
use macroquad::prelude::*;
use crate::constants::{SEGA_YELLOW, SEGA_WHITE, SEGA_GRAY};
use crate::ui::{draw_sega_box, draw_sega_button, draw_sega_text};

pub fn draw_dashboard(game: &mut Game, font: Option<&Font>, mouse_pos: (f32, f32), squad_page: &mut usize) -> AppState {
    draw_sega_box(20.0, 20.0, 760.0, 80.0, None, font);
    let team_name = &game.teams[game.player_team_index].name;
    draw_sega_text(&format!("TEAM: {}", team_name), 40.0, 50.0, font, 24, SEGA_YELLOW);
    draw_sega_text(&format!("WEEK: {} / {}", game.week, game.total_weeks), 40.0, 80.0, font, 24, SEGA_YELLOW);

    draw_sega_box(20.0, 120.0, 300.0, 400.0, Some("MENU"), font);

    if draw_sega_button(40.0, 160.0, 260.0, 50.0, "SQUAD (A)", font, mouse_pos) ||
    is_key_pressed(KeyCode::A) {
        *squad_page = 0;
        return AppState::SquadView;
    }

    if draw_sega_button(40.0, 220.0, 260.0, 50.0, "TOP SCORERS (W)", font, mouse_pos) ||
    is_key_pressed(KeyCode::W) {
        return AppState::TopScorers;
    }

    if draw_sega_button(40.0, 280.0, 260.0, 50.0, "NEXT MATCH (S)", font, mouse_pos) ||
    is_key_pressed(KeyCode::S) {
        if game.week <= game.total_weeks {
            game.simulate_week();
            return AppState::MatchSimulation;
        } else {
            return AppState::EndOfSeason;
        }
    }

    if draw_sega_button(40.0, 340.0, 260.0, 50.0, "STANDINGS (D)", font, mouse_pos) ||
    is_key_pressed(KeyCode::D) {
        return AppState::LeagueTable;
    }

    if draw_sega_button(40.0, 440.0, 260.0, 50.0, "EXIT (ESC)", font, mouse_pos) ||
    is_key_down(KeyCode::Escape) {
        return AppState::MainMenu;
    }

    let rx = 340.0;
    let ry = 120.0;
    draw_sega_box(rx, ry, 440.0, 400.0, Some("LAST WEEK RESULTS"), font);
    if game.last_week_results.is_empty() {
        draw_sega_text("NO RESULTS...", rx + 120.0, ry + 60.0, font, 20, SEGA_GRAY);
    } else {
        for (i, res) in game.last_week_results.iter().enumerate() {
            draw_sega_text(res, rx + 120.0, ry + 60.0 + (i as f32 * 30.0), font, 20, SEGA_WHITE);
        }
    }

    AppState::Dashboard
}