use constants::*;
use models::Game;
use macroquad::prelude::*;
use ui::{draw_sega_box, draw_sega_text, draw_sega_button};

mod ui;
mod models;
mod constants;

enum AppState {
    MainMenu,
    Dashboard,
    LeagueTable,
    MatchSimulation,
    SeasonEnd
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
                draw_sega_box(100.0, 100.0, 600.0, 400.0, Some("SEGA FOOTBALL MANAGER - v0.1.1 (BETA)"), font_ref);

                if draw_sega_button(250.0, 280.0, 300.0, 50.0, "PLAY", font_ref) || is_key_pressed(KeyCode::Enter) {
                    state = AppState::Dashboard;
                }
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

                if draw_sega_button(40.0, 180.0, 260.0, 50.0, "NEXT MATCH", font_ref) || is_key_pressed(KeyCode::A) {
                    if game.week <= game.total_weeks {
                        game.simulate_week();
                        state = AppState::MatchSimulation;
                    } else {
                        state = AppState::SeasonEnd;
                    }
                }

                if draw_sega_button(40.0, 250.0, 260.0, 50.0, "STANDINGS", font_ref) || is_key_pressed(KeyCode::S) {
                    state = AppState::LeagueTable;
                }

                if draw_sega_button(40.0, 440.0, 260.0, 50.0, "EXIT", font_ref) || is_key_pressed(KeyCode::Escape) {
                    state = AppState::MainMenu;
                }

                let res_box_x = 340.0;
                let res_box_y = 120.0;
                draw_sega_box(res_box_x, res_box_y, 440.0, 400.0, Some("LAST WEEK"), font_ref);

                if game.last_week_results.is_empty() {
                    draw_sega_text("NO RESULTS...", res_box_x + 120.0, res_box_y + 60.0, font_ref, 20, GRAY);
                } else {
                    for (i, res) in game.last_week_results.iter().enumerate() {
                        draw_sega_text(res, res_box_x + 20.0, res_box_y + 60.0 + (i as f32 * 30.0), font_ref, 20, SEGA_WHITE);
                    }
                }
            }
            AppState::MatchSimulation => {
                draw_sega_box(150.0, 150.0, 500.0, 300.0, Some("MATCHDAY"), font_ref);

                draw_sega_text("ROUND IS PLAYED!", 220.0, 250.0, font_ref, 30, SEGA_WHITE);
                if draw_sega_button(250.0, 350.0, 300.0, 50.0, "NEXT (SPACE)", font_ref) || is_key_pressed(KeyCode::Space) {
                    if game.week > game.total_weeks {
                        state = AppState::SeasonEnd
                    } else {
                        state = AppState::Dashboard;
                    }
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

                if draw_sega_button(250.0, 500.0, 300.0, 40.0, "BACK (SPACE)", font_ref) || is_key_pressed(KeyCode::Space) {
                    state = AppState::Dashboard;
                }
            }
            AppState::SeasonEnd => {
                draw_sega_box(150.0, 150.0, 500.0, 300.0, Some("END OF SEASON"), font_ref);

                let winner = &game.get_standings()[0];
                draw_sega_text("CHAMPIONS: ", 280.0, 220.0, font_ref, 24, SEGA_WHITE);
                draw_sega_text(&winner.name, 280.0, 260.0, font_ref, 30, SEGA_YELLOW);

                if draw_sega_button(250.0, 350.0, 300.0, 50.0, "BACK TO MAIN MENU (ESC)", font_ref) || is_key_pressed(KeyCode::Escape) {
                    state = AppState::MainMenu;
                    game = Game::new();
                }
            }
        }

        next_frame().await
    }
}