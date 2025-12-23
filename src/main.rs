use constants::*;
use models::Game;
use screens::AppState;
use macroquad::prelude::*;

mod ui;
mod models;
mod screens;
mod constants;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sega Football Manager - v0.2.1 (BETA)".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: true,
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
    let mut is_fullscreen = true;
    let mut squad_page = 0;

    let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let virtual_camera = Camera2D {
        render_target: Some(render_target.clone()),
        zoom: vec2(1.0 / (VIRTUAL_WIDTH / 2.0), 1.0 / (VIRTUAL_HEIGHT / 2.0)),
        target: vec2(VIRTUAL_WIDTH / 2.0, VIRTUAL_HEIGHT / 2.0),
        ..Default::default()
    };

    loop {
        if is_key_pressed(KeyCode::F11) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
        }

        let screen_w = screen_width();
        let screen_h = screen_height();

        let scale = (screen_w / VIRTUAL_WIDTH).min(screen_h / VIRTUAL_HEIGHT);
        let new_w = VIRTUAL_WIDTH * scale;
        let new_h = VIRTUAL_HEIGHT * scale;

        let offset_x = (screen_w - new_w) / 2.0;
        let offset_y = (screen_h - new_h) / 2.0;

        let (raw_mx, raw_my) = mouse_position();
        let virtual_mx = (raw_mx - offset_x) / scale;
        let virtual_my = (raw_my - offset_y) / scale;
        let virtual_mouse_pos = (virtual_mx, virtual_my);

        set_camera(&virtual_camera);
        clear_background(BACKGROUND_COLOR);

        state = match state {
            AppState::MainMenu => screens::main_menu::draw_main_menu(font_ref, virtual_mouse_pos),
            AppState::Dashboard => screens::dashboard::draw_dashboard(&mut game, font_ref, virtual_mouse_pos, &mut squad_page),
            AppState::SquadView => screens::squad_view::draw_squad_view(&mut game, font_ref, virtual_mouse_pos, &mut squad_page),
            AppState::LeagueTable => screens::league_table::draw_league_table(&game, font_ref, virtual_mouse_pos),
            AppState::TopScorers => screens::top_scorers::draw_top_scorers(&game, font_ref, virtual_mouse_pos),
            AppState::MatchSimulation => screens::match_simulation::draw_match_simulation(&game, font_ref, virtual_mouse_pos),
            AppState::EndOfSeason => screens::end_of_season::draw_end_of_season(&mut game, font_ref, virtual_mouse_pos)
        };

        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &render_target.texture,
            offset_x,
            offset_y,
            SEGA_WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(new_w, new_h)),
                ..Default::default()
            }
        );

        next_frame().await
    }
}