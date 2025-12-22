#[derive(Clone)]
pub struct Match {
    pub home_team_idx: usize,
    pub away_team_idx: usize,
    pub played: bool,
    pub home_score: u32,
    pub away_score: u32
}