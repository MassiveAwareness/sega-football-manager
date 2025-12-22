#![allow(dead_code)]
use rand::Rng;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Position {
    GK,
    DEF,
    MID,
    FWD
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Position::GK => write!(f, "GK"),
            Position::DEF => write!(f, "DEF"),
            Position::MID => write!(f, "MID"),
            Position::FWD => write!(f, "FWD")
        }
    }
}

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub position: Position,
    pub skill: u8,
    pub age: u8
}

impl Player {
    pub fn new(name: String, position: Position, skill: u8, age: u8) -> Self {
        Self { name, position, skill, age }
    }
}

#[derive(Clone)]
pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
    pub points: u32,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    pub goals_for: u32,
    pub goals_against: u32
}

impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            players: Vec::new(),
            points: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            goals_for: 0,
            goals_against: 0
        }
    }

    pub fn get_strength(&self) -> u32 {
        if self.players.is_empty() { return 0; }

        let mut sorted_players = self.players.clone();
        sorted_players.sort_by(|a, b| b.skill.cmp(&a.skill));

        let count = sorted_players.len().min(11);
        let sum: u32 = sorted_players.iter().take(count).map(|p| p.skill as u32).sum();

        sum / count as u32
    }
}

#[derive(Clone)]
pub struct Match {
    pub home_team_idx: usize,
    pub away_team_idx: usize,
    pub played: bool,
    pub home_score: u32,
    pub away_score: u32
}

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

        // Menetrend generálása
        let (schedule, total_weeks) = Self::generate_schedule(teams.len());

        Self {
            teams,
            player_team_index: rand::thread_rng().gen_range(0..team_definitions.len()-1),
            week: 1,
            total_weeks: total_weeks as u32,
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
                let name = format!("{} {}", fn_, ln);

                let variation = rng.gen_range(0..20) as i32 - 10;
                let skill = (base_skill as i32 + variation).clamp(1, 100) as u8;

                let age = rng.gen_range(15..45);

                players.push(Player::new(name, pos, skill, age));
            }
        }

        players.sort_by_key(|p| p.position as u8);

        players
    }

    // Round-Robin algoritmus (Circle Method)
    fn generate_schedule(num_teams: usize) -> (Vec<Vec<Match>>, usize) {
        let mut schedule = Vec::new();
        let mut indices: Vec<usize> = (0..num_teams).collect();
        let rounds = num_teams - 1;

        for _ in 0..2 {
            for _ in 0..rounds {
                let mut week_matches = Vec::new();
                let mid = num_teams / 2;
                for i in 0..mid {
                    let home = indices[i];
                    let away = indices[num_teams - 1 - i];
                    week_matches.push(Match {
                        home_team_idx: home,
                        away_team_idx: away,
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

        let total_weeks = schedule.len();
        (schedule, total_weeks)
    }

    pub fn simulate_week(&mut self) {
        if self.week > self.total_weeks { return; }
        let week_idx = (self.week - 1) as usize;
        self.last_week_results.clear();

        // let mut results_to_process: Vec<(usize, usize, u32, u32)> = Vec::new();
        let matches_indices: Vec<(usize, usize)> = self.schedule[week_idx].iter()
            .map(|m| (m.home_team_idx, m.away_team_idx)).collect();

        for (i, (home_idx, away_idx)) in matches_indices.into_iter().enumerate() {
            let home_str = self.teams[home_idx].get_strength() + 5;
            let away_str = self.teams[away_idx].get_strength();

            let home_goals = Self::calculate_goals(home_str, away_str);
            let away_goals = Self::calculate_goals(away_str, home_str);

            self.schedule[week_idx][i].home_score = home_goals;
            self.schedule[week_idx][i].away_score = away_goals;
            self.schedule[week_idx][i].played = true;

            self.update_team_stats(home_idx, home_goals, away_goals);
            self.update_team_stats(away_idx, away_goals, home_goals);

            self.last_week_results.push(format!(
                "{} {} : {} {}",
                self.teams[home_idx].name, home_goals, away_goals, self.teams[away_idx].name
            ));
        }

        self.week += 1;
    }

    fn calculate_goals(attack_str: u32, defense_str: u32) -> u32 {
        let mut rng = rand::thread_rng();
        let mut goals = 0;
        for _ in 0..6 {
            let base_chance = attack_str as i32 / 4;
            let diff_modifier = (attack_str as i32 - defense_str as i32) / 2;
            let chance_threshold = (base_chance + diff_modifier).clamp(5, 95);
            if rng.gen_range(0..100) < chance_threshold { goals += 1; }
        }

        goals
    }

    fn update_team_stats(&mut self, team_idx: usize, gf: u32, ga: u32) {
        let team = &mut self.teams[team_idx];
        team.goals_for += gf;
        team.goals_against += ga;

        if gf > ga { team.wins += 1; team.points += 3; }
        else if gf == ga {  team.draws += 1; team.points += 1; }
        else { team.losses += 1; }
    }

    pub fn get_standings(&self) -> Vec<&Team> {
        let mut sorted_teams: Vec<&Team> = self.teams.iter().collect();
        sorted_teams.sort_by(|a, b| {
            b.points.cmp(&a.points)
                .then_with(|| (b.goals_for as i32 - b.goals_against as i32).cmp(&(a.goals_for as i32 - a.goals_against as i32)))
        });

        sorted_teams
    }
}