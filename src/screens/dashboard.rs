#![allow(dead_code)]

use super::AppState;
use crate::models::Game;
use macroquad::prelude::*;
use crate::constants::{SEGA_YELLOW, SEGA_WHITE, SEGA_GRAY, SEGA_RED};
use crate::ui::{draw_sega_box, draw_sega_button, draw_sega_line, draw_sega_text};

pub fn draw_dashboard(game: &mut Game, font: Option<&Font>, mouse_pos: (f32, f32), squad_page: &mut usize, show_news: &mut bool) -> AppState {
    draw_sega_box(20.0, 20.0, 760.0, 80.0, None, font);
    let team_name = &game.teams[game.player_team_index].name;
    draw_sega_text(&format!("TEAM: {}", team_name), 40.0, 50.0, font, 24, SEGA_YELLOW);
    draw_sega_text(&format!("WEEK: {} / {}", game.week, game.total_weeks), 40.0, 80.0, font, 24, SEGA_YELLOW);

    draw_sega_box(20.0, 120.0, 300.0, 400.0, Some("MENU"), font);

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

    if !*show_news {
        let starter_count = game.teams[game.player_team_index].players.iter().filter(|p| p.is_starting).count();
        let is_squad_valid = starter_count == 11;

        if draw_sega_button(40.0, 160.0, 260.0, 50.0, "SQUAD (A)", font, mouse_pos) ||
        is_key_pressed(KeyCode::A) {
            *squad_page = 0;
            return AppState::SquadView;
        }

        if draw_sega_button(40.0, 220.0, 260.0, 50.0, "TOP SCORERS (W)", font, mouse_pos) ||
        is_key_pressed(KeyCode::W) {
            return AppState::TopScorers;
        }

        if is_squad_valid {
            if draw_sega_button(40.0, 280.0, 260.0, 50.0, "NEXT MATCH (S)", font, mouse_pos) ||
            is_key_pressed(KeyCode::S) {
                if game.week <= game.total_weeks {
                    game.simulate_week();
                    return AppState::MatchSimulation;
                } else {
                    return AppState::EndOfSeason;
                }
            }
        } else {
            draw_sega_box(40.0, 280.0, 260.0, 50.0, None, font);
            draw_sega_text("INVALID SQUAD!", 60.0, 315.0, font, 24, SEGA_RED);
        }

        if draw_sega_button(40.0, 340.0, 260.0, 50.0, "STANDINGS (D)", font, mouse_pos) ||
        is_key_pressed(KeyCode::D) {
            return AppState::LeagueTable;
        }

        if draw_sega_button(500.0, 540.0, 260.0, 50.0, "NEWS / HELP (N)", font, mouse_pos) ||
        is_key_pressed(KeyCode::N) {
            *show_news = true;
        }

        if draw_sega_button(40.0, 440.0, 260.0, 50.0, "EXIT (ESC)", font, mouse_pos) ||
        is_key_down(KeyCode::Escape) {
            return AppState::MainMenu;
        }
    }

    if *show_news {
        draw_rectangle(0.0, 0.0, 800.0, 600.0, Color::new(0.0, 0.0, 0.0, 0.8));

        let mw = 500.0;
        let mh = 400.0;
        let mx = (800.0 - mw) / 2.0;
        let my = (600.0 - mh) / 2.0;

        draw_sega_box(mx, my, mw, mh, Some("RELEASE NOTES v0.3.0"), font);

        let start_text_y = my + 60.0;
        draw_sega_text("- SQUAD MANAGEMENT:", mx + 20.0, start_text_y, font, 20, SEGA_YELLOW);
        draw_sega_text(" You can pick your starting XI.", mx + 20.0, start_text_y + 30.0, font, 20, SEGA_WHITE);

        draw_sega_text(" - GOAL SCORERS:", mx + 20.0, start_text_y + 70.0, font, 20, SEGA_YELLOW);
        draw_sega_text(" Track who scores the most goals.", mx + 20.0, start_text_y + 100.0, font, 20, SEGA_WHITE);

        draw_sega_text(" - NEW UI:", mx + 20.0, start_text_y + 140.0, font, 20, SEGA_YELLOW);
        draw_sega_text(" Interactive buttons & mouse support.", mx + 20.0, start_text_y + 170.0, font, 20, SEGA_WHITE);

        draw_sega_line(mx + 20.0, start_text_y + 210.0, mx + mw - 20.0, start_text_y + 210.0);
        draw_sega_text("Good luck, Manager!", mx + 130.0, start_text_y + 240.0, font, 20, SEGA_GRAY);

        if draw_sega_button(mx + 100.0, my + mh - 70.0, 300.0, 50.0, "CLOSE (X)", font, mouse_pos) ||
        is_key_pressed(KeyCode::X) {
            *show_news = false;
        }
    }

    AppState::Dashboard
}