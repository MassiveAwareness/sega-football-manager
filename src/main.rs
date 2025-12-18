use constants::*;
use models::Game;
use macroquad::prelude::*;
use ui::{draw_sega_box, draw_sega_text};

mod ui;
mod models;
mod constants;

enum AppState {
    MainMenu,
    Dashboard,
    LeagueTable,
    MatchSimulation
}

#[macroquad::main("Sega Football Manager")]
async fn main() {
    let font = load_ttf_font("assets/fonts/font.ttf").await.ok();
    let font_ref = font.as_ref();

    let mut game = Game::new();
    let mut state = AppState::MainMenu;

    loop {
        clear_background(BACKGROUND_COLOR);

        match state {
            AppState::MainMenu => {
                draw_sega_box(100.0, 100.0, 600.0, 400.0, Some("SEGA FOOTBALL MANAGER - v0.0.2"), font_ref);

                draw_sega_text("Press Enter to start...", 220.0, 300.0, font_ref, 30, SEGA_WHITE);
                draw_sega_text("(C) 2025 MassiveAwareness", 280.0, 450.0, font_ref, 16, GRAY);

                if is_key_pressed(KeyCode::Enter) {
                    state = AppState::Dashboard;
                }
            }
            AppState::Dashboard => {
                // Felső infósáv
                draw_sega_box(20.0, 20.0, 760.0, 80.0, None, font_ref);
                let team_name = &game.teams[game.player_team_index].name;

                draw_sega_text(&format!("TEAM: {}", team_name), 40.0, 50.0, font_ref, 24, SEGA_YELLOW);
                draw_sega_text(&format!("WEEK {}", game.week), 40.0, 80.0, font_ref, 24, SEGA_WHITE);

                // Bal oldali menü
                draw_sega_box(20.0, 120.0, 300.0, 400.0, Some("MENU"), font_ref);
                let options = vec!["[A] NEXT MATCH", "[S] STANDINGS", "[ESC] EXIT"];
                for (i, opt) in options.iter().enumerate() {
                    draw_sega_text(opt, 40.0, 180.0 + (i as f32 * 40.0), font_ref, 24, SEGA_WHITE);
                }

                // Jobb oldali moenü
                let res_box_x = 340.0;
                let res_box_y = 120.0;
                draw_sega_box(res_box_x, res_box_y, 440.0, 400.0, Some("LAST WEEK"), font_ref);
                if game.last_week_results.is_empty() {
                    draw_sega_text("NO AVAILABLE RESULTS!", res_box_x + 120.0, res_box_y + 60.0, font_ref, 20, GRAY);
                } else {
                    for (i, res) in game.last_week_results.iter().enumerate() {
                        draw_sega_text(res, res_box_x + 20.0, res_box_y + 60.0 + (i as f32 * 30.0), font_ref, 20, SEGA_WHITE);
                    }
                }

                if is_key_pressed(KeyCode::A) {
                    game.simulate_week();
                    state = AppState::MatchSimulation;
                }

                if is_key_pressed(KeyCode::S) {
                    state = AppState::LeagueTable;
                }

                if is_key_pressed(KeyCode::D) {
                    state = AppState::MainMenu;
                }
            }
            AppState::MatchSimulation => {
                draw_sega_box(150.0, 150.0, 500.0, 300.0, Some("MATCH DAY"), font_ref);

                draw_sega_text("ROUND IS PLAYED!", 220.0, 250.0, font_ref, 30, SEGA_WHITE);
                draw_sega_text("PRESS [SPACE] TO CONTINUE...", 190.0, 350.0, font_ref, 20, SEGA_YELLOW);

                if is_key_pressed(KeyCode::Space) {
                    state = AppState::Dashboard;
                }
            }
            AppState::LeagueTable => {
                draw_sega_box(50.0, 50.0, 700.0, 500.0, Some("STANDINGS"), font_ref);

                let header_y = 100.0;
                draw_sega_text("#", 70.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("TEAM", 130.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("W", 400.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("D", 450.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("L", 500.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("PTS", 600.0, header_y, font_ref, 20, SEGA_YELLOW);

                let standings = game.get_standings();
                for (i, team) in standings.iter().enumerate() {
                    let y_pos = 140.0 + (i as f32 * 30.0);
                    let color = if team.name == game.teams[game.player_team_index].name { SEGA_YELLOW } else { SEGA_WHITE };

                    draw_sega_text(&format!("#{}", i + 1), 70.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.name, 130.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.wins.to_string(), 400.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.draws.to_string(), 450.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.losses.to_string(), 500.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.points.to_string(), 600.0, y_pos, font_ref, 20, color);
                }

                draw_sega_text("[SPACE] BACK", 500.0, 530.0, font_ref, 20, SEGA_YELLOW);

                if is_key_pressed(KeyCode::Space) {
                    state = AppState::Dashboard;
                }
            }
        }

        next_frame().await
    }
}