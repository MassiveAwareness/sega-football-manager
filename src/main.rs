use constants::*;
use models::Game;
use macroquad::prelude::*;
use ui::{draw_sega_box, draw_sega_text, draw_sega_button};

mod constants;
mod models;
mod ui;

enum AppState {
    MainMenu,
    Dashboard,
    LeagueTable,
    MatchSimulation,
    EndOfSeason
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Sega Footbal Manager - v0.1.2 (BETA)".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("assets/fonts/font.ttf").await.ok();
    let font_ref = font.as_ref();

    let mut game = Game::new();
    let mut state = AppState::MainMenu;
    let mut is_fullscreen = false;

    // --- VIRTUÁLIS KÉPERNYŐ BEÁLLÍTÁSA ---
    let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let /* mut */ virtual_camera = Camera2D {
        render_target: Some(render_target.clone()),
        zoom: vec2(1.0 / (VIRTUAL_WIDTH / 2.0), 1.0 / (VIRTUAL_HEIGHT / 2.0)),
        target: vec2(VIRTUAL_WIDTH / 2.0, VIRTUAL_HEIGHT / 2.0),
        ..Default::default()
    };

    // virtual_camera.zoom.y = -virtual_camera.zoom.y;

    loop {
        if is_key_pressed(KeyCode::F11) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
        }

        // Méretezés kiszámítás
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Kiszámolja, hányszorosára kell nagyítani a képet
        let scale = (screen_w / VIRTUAL_WIDTH).min(screen_h / VIRTUAL_HEIGHT);

        // A skálázott méret
        let new_w = VIRTUAL_WIDTH * scale;
        let new_h = VIRTUAL_HEIGHT * scale;

        // Az eltolás, hogy középen legyen (black bars)
        let offset_x = (screen_w - new_w) / 2.0;
        let offset_y = (screen_h - new_h) / 2.0;

        // A valódi egérpozícióból kivonja az eltolást és elosztja a skálával
        let (raw_mx, raw_my) = mouse_position();
        let virtual_mx = (raw_mx - offset_x) / scale;
        let virtual_my = (raw_my - offset_y) / scale;
        let virtual_mouse_pos = (virtual_mx, virtual_my);

        // Bekapcsolja a kamerát, innentől minden a `render_target`-re megy
        set_camera(&virtual_camera);

        // Háttér törlése
        clear_background(BACKGROUND_COLOR);

        match state {
            AppState::MainMenu => {
                draw_sega_box(100.0, 100.0, 600.0, 400.0, Some("SEGA FOOTBALL MANAGER - v0.1.2 (BETA)"), font_ref);
                if draw_sega_button(250.0, 280.0, 300.0, 50.0, "PLAY (ENTER)", font_ref, virtual_mouse_pos) ||
                is_key_pressed(KeyCode::Enter) {
                    state = AppState::Dashboard;
                }

                draw_sega_text("(C) Dave Summer, 2025", 300.0, 450.0, font_ref, 16, GRAY);
            }
            AppState::Dashboard => {
                draw_sega_box(20.0, 20.0, 760.0, 80.0, None, font_ref);
                let team_name = &game.teams[game.player_team_index].name;

                draw_sega_text(&format!("TEAM: {}", team_name), 40.0, 50.0, font_ref, 24, SEGA_YELLOW);
                draw_sega_text(&format!("WEEK: {} / {}", game.week, game.total_weeks), 40.0, 80.0, font_ref, 24, SEGA_YELLOW);

                draw_sega_box(20.0, 120.0, 300.0, 400.0, Some("MENU"), font_ref);

                if draw_sega_button(40.0, 180.0, 260.0, 50.0, "NEXT MATCH (A)", font_ref, virtual_mouse_pos) ||
                is_key_pressed(KeyCode::A) {
                    if game.week <= game.total_weeks {
                        game.simulate_week();
                        state = AppState::MatchSimulation;
                    } else {
                        state = AppState::EndOfSeason;
                    }
                }

                if draw_sega_button(40.0, 250.0, 260.0, 50.0, "STANDINGS (S)", font_ref, virtual_mouse_pos) ||
                is_key_pressed(KeyCode::S) {
                    state = AppState::LeagueTable;
                }

                if draw_sega_button(40.0, 440.0, 260.0, 50.0, "EXIT (ESC)", font_ref, virtual_mouse_pos) ||
                is_key_pressed(KeyCode::Escape) {
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
                draw_sega_text("ROUND IS PLAYED...", 240.0, 250.0, font_ref, 30, SEGA_WHITE);

                if draw_sega_button(250.0, 350.0, 300.0, 50.0, "NEXT (D)", font_ref, virtual_mouse_pos) ||
                is_key_pressed(KeyCode::D) {
                    if game.week > game.total_weeks {
                        state = AppState::EndOfSeason;
                    } else {
                        state = AppState::Dashboard;
                    }
                }
            }
            AppState::LeagueTable => {
                draw_sega_box(50.0, 50.0, 700.0, 500.0, Some("STANDINGS"), font_ref);

                let header_y = 110.0;
                draw_sega_text("#", 70.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("TEAM", 130.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("W", 350.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("D", 400.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("L", 450.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("GF", 500.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("GA", 550.0, header_y, font_ref, 20, SEGA_YELLOW);
                draw_sega_text("PTS", 600.0, header_y, font_ref, 20, SEGA_YELLOW);

                draw_line(50.0, 115.0, 750.0, 115.0, 2.0, SEGA_WHITE);
                draw_line(120.0, 80.0, 120.0, 420.0, 2.0, SEGA_WHITE);

                let standings = game.get_standings();
                for (i, team) in standings.iter().enumerate() {
                    let y_pos = 140.0 + (i as f32 * 30.0);
                    let color = if team.name == game.teams[game.player_team_index].name { SEGA_YELLOW } else { SEGA_WHITE };

                    draw_sega_text(&format!("#{}", i + 1), 130.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.name, 200.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.wins.to_string(), 400.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.draws.to_string(), 450.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.losses.to_string(), 500.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.goals_for.to_string(), 550.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.goals_against.to_string(), 600.0, y_pos, font_ref, 20, color);
                    draw_sega_text(&team.points.to_string(), 650.0, y_pos, font_ref, 20, color);
                }

                if draw_sega_button(250.0, 500.0, 300.0, 40.0, "BACK (A)", font_ref, virtual_mouse_pos) ||
                is_key_pressed(KeyCode::A) {
                    state = AppState::Dashboard;
                }
            }
            AppState::EndOfSeason => {
                draw_sega_box(150.0, 150.0, 500.0, 300.0, Some("END OF SEASON"), font_ref);
                let winner = &game.get_standings()[0];
                draw_sega_text("CHAMPION:", 280.0, 220.0, font_ref, 24, SEGA_WHITE);
                draw_sega_text(&winner.name, 280.0, 260.0, font_ref, 30, SEGA_YELLOW);

                if draw_sega_button(250.0, 350.0, 300.0, 50.0, "EXIT TO MAIN MENU (ESC)", font_ref, virtual_mouse_pos) ||
                is_key_pressed(KeyCode::Escape) {
                    state = AppState::MainMenu;
                    game = Game::new();
                }
            }
        }

        // Visszaáll az alapértelmezett kamerára
        set_default_camera();

        // A valódi ablak hátterét feketére állítja
        clear_background(BLACK);

        // Kirajzolja a `render_target` textúráját a számolt pozícióra és méretre
        draw_texture_ex(
            &render_target.texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(new_w, new_h)),
                ..Default::default()
            }
        );

        next_frame().await
    }
}