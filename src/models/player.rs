#![allow(dead_code)]

use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Position {
    GK,
    DEF,
    MID,
    FWD
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub age: u8,
    pub goals_scored: u32,
    pub matches_played: u32,
    pub is_starting: bool
}

impl Player {
    pub fn new(name: String, position: Position, skill: u8, age: u8) -> Self {
        Self {
            name,
            position,
            skill,
            age,
            goals_scored: 0,
            matches_played: 0,
            is_starting: false
        }
    }
}