#![allow(dead_code)]

#[repr(u8)]
pub enum GameMode {
    BattleArena = 0,
    SuddenDeath = 1,
    Pit = 2,
    TestMode = 3,
    SinglePlayer = 4,
    TeamDeathmatch = 5,
    Campaign = 6,
}

impl GameMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            GameMode::BattleArena => "BattleArena",
            GameMode::SuddenDeath => "SuddenDeath",
            GameMode::Pit => "Pit",
            GameMode::TestMode => "TestMode",
            GameMode::SinglePlayer => "SinglePlayerTDM",
            GameMode::TeamDeathmatch => "TeamDeathmatch",
            GameMode::Campaign => "Campaign",
        }
    }
}

#[repr(u8)]
pub enum MapVisibility {
    Good = 0,
    Poor = 1,
    Bad = 2, // VeryPoor
}
