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
        let starters: Vec<&Player> = self.players.iter().filter(|p| p.is_starting).collect();

        if starters.is_empty() { return 0; }

        let count = starters.len();
        let sum: u32 = starters.iter().map(|p| p.skill as u32).sum();

        sum / count as u32
    }
}