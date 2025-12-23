#![allow(dead_code)]

use rand::Rng;
use super::team::Team;
use super::match_model::Match;
use super::player::{Player, Position};

pub struct Game {
    pub teams: Vec<Team>,
    pub player_team_index: usize,
    pub week: u32,
    pub total_weeks: u32,
    pub schedule: Vec<Vec<Match>>,
    pub last_week_results: Vec<String>
}

impl Game {
    pub fn new() -> Self {
        let team_definitions = vec![
            ("DVTK", 85),
            ("Spartacus FC", 78),
            ("ETO FC", 80),
            ("Debrecen", 76),
            ("Videoton", 74),
            ("MTK Budapest", 72),
            ("Vasas FC", 70),
            ("Zalaegerszeg", 68)
        ];

        let mut teams = Vec::new();
        for (name, base_skill) in &team_definitions {
            let mut team = Team::new(name);
            team.players = Self::generate_squad(*base_skill);
            teams.push(team);
        }

        let (schedule, total_weeks) = Self::generate_schedule(teams.len());
        let player_team_index = rand::thread_rng().gen_range(0..teams.len());

        Self {
            teams,
            player_team_index,
            total_weeks: total_weeks as u32,
            week: 1,
            schedule,
            last_week_results: Vec::new()
        }
    }

    fn generate_squad(base_skill: u8) -> Vec<Player> {
        let mut rng = rand::thread_rng();
        let mut players = Vec::new();
        
        let last_names = vec![
            "Kovacs", "Nagy", "Szabo", "Juhasz", "Toth", "Kiss", "Varga", "Molnar", "Nemeth", "Farkas", "Balogh",
            "Papp", "Takacs", "Meszaros", "Simon", "Racz", "Fekete", "Szilagyi", "Torok", "Vincze"
        ];

        let first_names = vec![
            "Istvan", "Laszlo", "Zoltan", "Janos", "Gabor", "Sandor", "Jozsef", "Attila", "Tamas", "Peter",
            "Balazs", "Andras", "Ferenc", "Zsolt", "Csaba", "Miklos", "Tibor", "Imre", "Robert", "Adam"
        ];

        let positions = vec![
            (Position::GK, 3),
            (Position::DEF, 8),
            (Position::MID, 8),
            (Position::FWD, 7)
        ];

        for (pos, count) in positions {
            for _ in 0..count {
                let ln = last_names[rng.gen_range(0..last_names.len())];
                let fn_ = first_names[rng.gen_range(0..first_names.len())];

                let variation = rng.gen_range(0..20) as i32 - 10;
                let skill = (base_skill as i32 + variation).clamp(1, 100) as u8;
                let age = rng.gen_range(15..45);

                players.push(Player::new(format!("{} {}", fn_, ln), pos, skill, age));
            }
        }

        players.sort_by_key(|p| p.position as u8);

        for i in 0..11 {
            if i < players.len() {
                players[i].is_starting = true;
            }
        }
        
        players
    }

    fn generate_schedule(num_teams: usize) -> (Vec<Vec<Match>>, usize) {
        let mut schedule = Vec::new();
        let mut indices: Vec<usize> = (0..num_teams).collect();
        let rounds = num_teams - 1;

        for _ in 0..2 {
            for _ in 0..rounds {
                let mut week_matches = Vec::new();
                let mid = num_teams / 2;
                for i in 0..mid {
                    week_matches.push(Match {
                        home_team_idx: indices[i],
                        away_team_idx: indices[num_teams - 1 - i],
                        played: false,
                        home_score: 0,
                        away_score: 0
                    });
                }
                schedule.push(week_matches);
                let last = indices.pop().unwrap();
                indices.insert(1, last);
            }

            for i in 0..rounds {
                let mut week_matches = Vec::new();
                for m in &schedule[i] {
                    week_matches.push(Match {
                        home_team_idx: m.away_team_idx,
                        away_team_idx: m.home_team_idx,
                        played: false,
                        home_score: 0,
                        away_score: 0
                    });
                }
                schedule.push(week_matches);
            }
        }

        let total_rounds = schedule.len();
        (schedule, total_rounds)
    }

    pub fn simulate_week(&mut self) {
        if self.week > self.total_weeks { return; }
        let week_idx = (self.week - 1) as usize;
        self.last_week_results.clear();

        let matches_indices: Vec<(usize, usize)> = self.schedule[week_idx].iter()
            .map(|m| (m.home_team_idx, m.away_team_idx)).collect();

        for (i, (home_idx, away_idx)) in matches_indices.into_iter().enumerate() {
            let home_str = self.teams[home_idx].get_strength() + 5;
            let away_str = self.teams[away_idx].get_strength();

            let home_goals = Self::calculate_goals(home_str, away_str);
            let away_goals = Self::calculate_goals(away_str, home_str);

            for p in &mut self.teams[home_idx].players { p.matches_played += 1; }
            for p in &mut self.teams[away_idx].players { p.matches_played += 1; }
            Self::assign_goals(&mut self.teams[home_idx].players, home_goals);
            Self::assign_goals(&mut self.teams[away_idx].players, away_goals);

            self.schedule[week_idx][i].home_score = home_goals;
            self.schedule[week_idx][i].away_score = away_goals;
            self.schedule[week_idx][i].played = true;

            self.update_team_stats(home_idx, home_goals, away_goals);
            self.update_team_stats(away_idx, away_goals, home_goals);

            self.last_week_results.push(
                format!("{} {} : {} {}", self.teams[home_idx].name, home_goals, away_goals, self.teams[away_idx].name)
            );
        }

        self.week += 1;
    }

    fn calculate_goals(attack: u32, defense: u32) -> u32 {
        let mut rng = rand::thread_rng();
        let mut goals = 0;
        for _ in 0..6 {
            let chance = ((attack as i32 / 4) + (attack as i32 - defense as i32) / 2).clamp(5, 95);
            if rng.gen_range(0..100) < chance { goals += 1; }
        }

        goals
    }

    fn assign_goals(players: &mut Vec<Player>, goals: u32) {
        if players.is_empty() || goals == 0 { return; }
        let mut rng = rand::thread_rng();
        for _ in 0..goals {
            loop {
                let idx = rng.gen_range(0..players.len());
                let chance = match players[idx].position {
                    Position::FWD => 100,
                    Position::MID => 60,
                    Position::DEF => 15,
                    Position::GK => 1
                };

                if rng.gen_range(0..100) < chance {
                    players[idx].goals_scored += 1;
                    break;
                }
            }
        }
    }

    fn update_team_stats(&mut self, idx: usize, gf: u32, ga: u32) {
        let t = &mut self.teams[idx];
        t.goals_for += gf;
        t.goals_against += ga;

        if gf > ga {
            t.wins += 1;
            t.points += 3;
        } else if gf == ga {
            t.draws += 1;
            t.points += 1;
        } else {
            t.losses += 1;
        }
    }

    pub fn get_standings(&self) -> Vec<&Team> {
        let mut sorted: Vec<&Team> = self.teams.iter().collect();
        sorted.sort_by(|a, b| b.points.cmp(&a.points)
            .then_with(|| (b.goals_for as i32 - b.goals_against as i32).cmp(&(a.goals_for as i32 - a.goals_against as i32))));

        sorted
    }

    pub fn get_top_scorers(&self) -> Vec<(String, String, u32)> {
        let mut scorers = Vec::new();
        for t in &self.teams {
            for p in &t.players {
                if p.goals_scored > 0 { scorers.push((p.name.clone(), t.name.clone(), p.goals_scored)); }
            }
        }
        scorers.sort_by(|a, b| b.2.cmp(&a.2));
        scorers.into_iter().take(10).collect()
    }
}