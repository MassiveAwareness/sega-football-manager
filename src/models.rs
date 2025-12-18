use rand::Rng;

#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct Game {
    pub teams: Vec<Team>,
    pub player_team_index: usize,
    pub week: u32,
    pub last_week_results: Vec<String>
}

#[allow(dead_code)]
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

        Self {
            teams,
            player_team_index: 0,
            week: 1,
            last_week_results: Vec::new()
        }
    }

    pub fn simulate_week(&mut self) {
        let mut rng = rand::thread_rng();
        self.last_week_results.clear();

        let mut indices: Vec<usize> = (0..self.teams.len()).collect();
        // Egyszerű keverés
        for _ in 0..10 {
            let i = rng.gen_range(0..indices.len());
            let j = rng.gen_range(0..indices.len());
            indices.swap(i, j);
        }

        for i in (0..indices.len()).step_by(2) {
            let home_idx = indices[i];
            let away_idx = indices[i + 1];

            // Erősség alapú gólok
            let home_str = self.teams[home_idx].strength + 5;
            let away_str = self.teams[away_idx].strength;

            let home_goals = self.calculate_goals(home_str, away_str);
            let away_goals = self.calculate_goals(away_str, home_str);

            self.update_team_stats(home_idx, home_goals, away_goals);
            self.update_team_stats(away_idx, away_goals, home_goals);

            self.last_week_results.push(format!(
                "{} {} - {} {}",
                self.teams[home_idx].name, home_goals, away_goals, self.teams[away_idx].name
            ));
        }

        self.week += 1;
    }

    fn calculate_goals(&self, attack_str: u32, defense_str: u32) -> u32 {
        let mut rng = rand::thread_rng();
        let mut goals = 0;

        // 6 helyzet van egy meccsen átlagosan
        for _ in 0..6 {
            // A gól esélye függ a támadó erősségétől és egy picit a védekezőtől
            // Base chance: Strength / 4
            // Módosító: (Attacker - Defender) / 2

            let base_chance = attack_str as i32 / 4;
            let diff_modifier = (attack_str as i32 - defense_str as i32) / 2;
            let chance_threshold = (base_chance + diff_modifier).clamp(5, 95); // Min 5%, Max 95%

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