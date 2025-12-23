#![allow(dead_code)]

use super::AppState;
use crate::models::{Game, Player};
use macroquad::prelude::*;
use crate::constants::{SEGA_LIGHT_BLUE, SEGA_RED, SEGA_WHITE, SEGA_YELLOW};
use crate::ui::{draw_sega_box, draw_sega_button, draw_sega_text, draw_sega_line};

pub fn draw_squad_view(game: &mut Game, font: Option<&Font>, mouse_pos: (f32, f32), page: &mut usize) -> AppState {
    draw_sega_box(50.0, 50.0, 700.0, 500.0, Some("TEAM SQUAD"), font);
    
    let hy = 100.0;
    draw_sega_text("ST?", 60.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("POS", 100.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("NAME", 160.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("SKL", 450.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("MP", 520.0, hy, font, 20, SEGA_YELLOW);
    draw_sega_text("GS", 600.0, hy, font, 20, SEGA_YELLOW);

    draw_sega_line(50.0, 115.0, 750.0, 115.0);
    draw_sega_line(145.0, 80.0, 145.0, 480.0);

    let starter_count = game.teams[game.player_team_index].players.iter().filter(|p| p.is_starting).count();
    let count_color = if starter_count == 11 { SEGA_YELLOW } else { SEGA_RED };
    draw_sega_text(&format!("STARTERS: {} / 11", starter_count), 550.0, 80.0, font, 20, count_color);

    let start_index = *page * 12;
    let mut clicked_player_idx: Option<usize> = None;

    {
        let players = &game.teams[game.player_team_index].players;

        for (i, p) in players.iter().skip(start_index).take(12).enumerate() {
            let absolute_idx = start_index + i;

            let y = 140.0 + (i as f32 * 30.0);

            let row_hover = mouse_pos.0 > 50.0 && mouse_pos.0 < 750.0 && mouse_pos.1 > y - 20.0 && mouse_pos.1 < y + 5.0;

            if row_hover {
                draw_rectangle(52.0, y - 18.0, 696.0, 28.0, SEGA_LIGHT_BLUE);

                if is_mouse_button_pressed(MouseButton::Left) {
                    clicked_player_idx = Some(absolute_idx);
                }
            }

            let mut text_color = SEGA_WHITE;
            if p.is_starting { text_color = SEGA_YELLOW };

            let status_icon = if p.is_starting { "*" } else { "" };

            draw_sega_text(status_icon, 60.0, y, font, 20, text_color);
            draw_sega_text(&p.position.to_string(), 100.0, y, font, 20, text_color);
            draw_sega_text(&p.name, 160.0, y, font, 20, text_color);
            draw_sega_text(&p.skill.to_string(), 450.0, y, font, 20, text_color);
            draw_sega_text(&p.matches_played.to_string(), 520.0, y, font, 20, text_color);
            draw_sega_text(&p.goals_scored.to_string(), 600.0, y, font, 20, text_color);
        }
    }

    if let Some(idx) = clicked_player_idx {
        let player = &mut game.teams[game.player_team_index].players[idx];
        player.is_starting = !player.is_starting;
    }

    let total_players = game.teams[game.player_team_index].players.len();

    if draw_sega_button(250.0, 500.0, 300.0, 40.0, "BACK (SPACE)", font, mouse_pos) ||
    is_key_pressed(KeyCode::Space) {
        return AppState::Dashboard;
    }

    if *page > 0 && draw_sega_button(60.0, 500.0, 150.0, 40.0, "< PREV", font, mouse_pos) ||
    is_key_pressed(KeyCode::Left) {
        *page -= 1;
    }

    if (*page + 1) * 12 < total_players && draw_sega_button(590.0, 500.0, 150.0, 40.0, "NEXT >", font, mouse_pos) ||
    is_key_pressed(KeyCode::Right) {
        *page += 1;
    }

    AppState::SquadView
}