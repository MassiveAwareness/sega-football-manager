#![allow(dead_code)]

use super::AppState;
use crate::models::Game;
use macroquad::prelude::*;
use crate::constants::{SEGA_YELLOW, SEGA_WHITE};
use crate::ui::{draw_sega_box, draw_sega_button, draw_sega_text, draw_sega_line};

pub fn draw_squad_view(game: &Game, font: Option<&Font>, mouse_pos: (f32, f32), page: &mut usize) -> AppState {
    draw_sega_box(50.0, 50.0, 700.0, 500.0, Some("TEAM SQUAD"), font);
    
    let hy = 100.0;
    draw_sega_text("POS", 70.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("NAME", 140.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("SKL", 450.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("MP", 520.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("GS", 600.0, hy, font, 20, SEGA_YELLOW);

    draw_sega_line(50.0, 115.0, 750.0, 115.0);
    draw_sega_line(125.0, 80.0, 125.0, 480.0);

    let players = &game.teams[game.player_team_index].players;
    let start = *page * 12;
    for (i, p) in players.iter().skip(start).take(12).enumerate() {
        let y = 140.0 + (i as f32 * 30.0);
        let skill_color = if p.skill > 80 { SEGA_YELLOW } else { SEGA_WHITE };
        let goal_color = if p.goals_scored > 0 { SEGA_YELLOW } else { SEGA_WHITE };

        draw_sega_text(&p.position.to_string(), 70.0, y, font, 20, SEGA_WHITE);
        draw_sega_text(&p.name, 140.0, y, font, 20, SEGA_WHITE);
        draw_sega_text(&p.skill.to_string(), 450.0, y, font, 20, skill_color);
        draw_sega_text(&p.matches_played.to_string(), 520.0, y, font, 20, SEGA_WHITE);
        draw_sega_text(&p.goals_scored.to_string(), 600.0, y, font, 20, goal_color);
    }

    if draw_sega_button(250.0, 500.0, 300.0, 40.0, "BACK (SPACE)", font, mouse_pos) ||
    is_key_pressed(KeyCode::Space) {
        return AppState::Dashboard;
    }

    if *page > 0 && draw_sega_button(60.0, 500.0, 150.0, 40.0, "< PREV", font, mouse_pos) ||
    is_key_pressed(KeyCode::Left) {
        *page -= 1;
    }

    if (*page + 1) * 12 < players.len() && draw_sega_button(590.0, 500.0, 150.0, 40.0, "NEXT >", font, mouse_pos) ||
    is_key_pressed(KeyCode::Right) {
        *page += 1;
    }

    AppState::SquadView
}