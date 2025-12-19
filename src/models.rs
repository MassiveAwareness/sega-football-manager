#![allow(dead_code)]
use rand::Rng;

#[derive(Clone)]
pub struct Team {
    pub name: String,
    pub strength: u32,
    pub points: u32,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    pub goals_for: u32,
    pub goals_against: u32
}

impl Team {
    pub fn new(name: &str, strength: u32) -> Self {
        Self {
            name: name.to_string(),
            strength,
            points: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            goals_for: 0,
            goals_against: 0
        }
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
        let teams = vec![
            Team::new("DVTK", 85),
            Team::new("Spartacus FC", 78),
            Team::new("ETO FC", 80),
            Team::new("Debrecen", 76),
            Team::new("Videoton", 74),
            Team::new("MTK Budapest", 72),
            Team::new("Vasas FC", 70),
            Team::new("Zalaegerszeg", 68)
        ];

        // Menetrend generálása
        let (schedule, total_weeks) = Self::generate_schedule(teams.len());
        let mut rng = rand::thread_rng();
        let team_list = teams.clone();

        Self {
            teams,
            player_team_index: rng.gen_range(0..(team_list.len() - 1)),
            week: 1,
            total_weeks: total_weeks as u32,
            schedule,
            last_week_results: Vec::new()
        }
    }

    // Round-Robin algoritmus (Circle Method)
    fn generate_schedule(num_teams: usize) -> (Vec<Vec<Match>>, usize) {
        let mut schedule = Vec::new();
        let mut indices: Vec<usize> = (0..num_teams).collect();
        let rounds = num_teams - 1; // Egy kör hossza (összes - 1, mivel a csapatok önmaguk ellen nem játszanak)

        // 1. Egy kör
        for _ in 0..rounds {
            let mut week_matches = Vec::new();
            let mid = num_teams  / 2;

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

            // Forgatás (A 0. index fix, a többi forog)
            let last = indices.pop().unwrap();
            indices.insert(1, last);
        }

        // 2. Második kör
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

        // 3. Mindkettőt még egyszer megcsináljuk (14 -> 28 meccs)
        for _ in 0..rounds {
            let mut week_matches = Vec::new();
            let mid = num_teams  / 2;

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

        let total_weeks = schedule.len();
        (schedule, total_weeks)
    }

    pub fn simulate_week(&mut self) {
        if self.week > self.total_weeks {
            return; // Vége a szezonnak
        }

        let week_idx = (self.week - 1) as usize;
        self.last_week_results.clear();

        // Mivel a schedule-ben lévő meccseket módosítjuk, és közben a csapatokat is,
        // trükkös a borrow checker miatt. Ezért kimenti az adatokat, számol, majd visszaírja.
        let mut results_to_process: Vec<(usize, usize, u32, u32)> = Vec::new();

        // Eredmények kiszámolása
        for match_obj in &mut self.schedule[week_idx] {
            let home_str = self.teams[match_obj.home_team_idx].strength + 5;
            let away_str = self.teams[match_obj.away_team_idx].strength;

            let home_goals = Self::calculate_goals(home_str, away_str);
            let away_goals = Self::calculate_goals(away_str, home_str);

            match_obj.home_score = home_goals;
            match_obj.away_score = away_goals;
            match_obj.played = true;

            results_to_process.push((match_obj.home_team_idx, match_obj.away_team_idx, home_goals, away_goals));

            self.last_week_results.push(format!(
                "{} {} - {} {}",
                self.teams[match_obj.home_team_idx].name, home_goals, away_goals, self.teams[match_obj.away_team_idx].name
            ));
        }

        // Csapat statisztikák frissítése
        for (h_idx, a_idx, h_g, a_g) in results_to_process {
            self.update_team_stats(h_idx, h_g, a_g);
            self.update_team_stats(a_idx, a_g, h_g);
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

            if rng.gen_range(0..100) < chance_threshold {
                goals += 1;
            }
        }

        goals
    }

    fn update_team_stats(&mut self, team_idx: usize, gf: u32, ga: u32) {
        let team = &mut self.teams[team_idx];
        team.goals_for += gf;
        team.goals_against += ga;

        if gf > ga {
            team.wins += 1;
            team.points += 3;
        } else if gf == ga {
            team.draws += 1;
            team.points += 1;
        } else {
            team.losses += 1;
        }
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