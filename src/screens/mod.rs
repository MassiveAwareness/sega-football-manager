#![allow(unused_imports)]

use macroquad::prelude::*;

pub mod main_menu;
pub mod dashboard;
pub mod squad_view;
pub mod top_scorers;
pub mod league_table;
pub mod end_of_season;
pub mod match_simulation;

#[derive(Clone, Copy, PartialEq)]
pub enum AppState {
    MainMenu,
    Dashboard,
    SquadView,
    LeagueTable,
    TopScorers,
    MatchSimulation,
    EndOfSeason
}

pub use main_menu::draw_main_menu;
pub use dashboard::draw_dashboard;
pub use squad_view::draw_squad_view;
pub use league_table::draw_league_table;
pub use top_scorers::draw_top_scorers;
pub use match_simulation::draw_match_simulation;
pub use end_of_season::draw_end_of_season;