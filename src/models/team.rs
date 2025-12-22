use super::player::Player;

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