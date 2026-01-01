pub struct GameResult {
    pub mode: super::game_mode::GameMode,
    pub is_custom: bool,
    pub winners: Vec<PlayerAward>,
    pub losers: Vec<PlayerAward>,
}

impl GameResult {
    pub fn parse(r: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let mut bytes_buf = [0u8; 2];
        r.read_exact(&mut bytes_buf)?;
        let mode = super::game_mode::GameMode::from_u8(bytes_buf[0])
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid game mode {}", bytes_buf[0])))?;
        let is_custom = bytes_buf[1] != 0;
        let mut winners_count_buf = [0u8; 4];
        r.read_exact(&mut winners_count_buf)?;
        let winners_count = i32::from_le_bytes(winners_count_buf);
        let mut winners = Vec::with_capacity(winners_count as _);
        for _ in 0..winners_count {
            let award = PlayerAward::parse(r)?;
            winners.push(award);
        }
        let mut losers_count_buf = [0u8; 4];
        r.read_exact(&mut losers_count_buf)?;
        let losers_count = i32::from_le_bytes(losers_count_buf);
        let mut losers = Vec::with_capacity(losers_count as _);
        for _ in 0..losers_count {
            let award = PlayerAward::parse(r)?;
            losers.push(award);
        }
        Ok(Self {
            mode,
            is_custom,
            winners,
            losers,
        })
    }
}

pub struct PlayerAward {
    pub player_name: String,
    pub mastery: u8,
    pub is_alive: bool,
    pub is_disconnected: bool,
    pub party_members: u8,
    pub score: i32,
    pub score_position: u8,
    pub score_position_in_team: u8,
    pub awards: std::collections::HashMap<PlayerAwardId, i32>,
    pub weapons: Vec<WeaponUsage>,
    pub players_died_before: u8,
}

impl PlayerAward {
    pub fn parse(r: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let player_name = super::read_str_for_binwriter(r)?;
        let mut bytes_buf = [0u8; 4];
        r.read_exact(&mut bytes_buf)?;
        let mastery = bytes_buf[0];
        let is_alive = bytes_buf[1] != 0;
        let is_disconnected = bytes_buf[2] != 0;
        let party_members = bytes_buf[3];
        let mut score_buf = [0u8; 4];
        r.read_exact(&mut score_buf)?;
        let score = i32::from_le_bytes(score_buf);
        let mut bytes_buf = [0u8; 3];
        r.read_exact(&mut bytes_buf)?;
        let score_position = bytes_buf[0];
        let score_position_in_team = bytes_buf[1];
        let awards_count = bytes_buf[2];
        let mut awards = std::collections::HashMap::with_capacity(awards_count as _);
        for _ in 0..awards_count {
            let mut bytes_buf = [0u8; 1];
            r.read_exact(&mut bytes_buf)?;
            let key = PlayerAwardId::from_u8(bytes_buf[0])
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid player award {}", bytes_buf[0])))?;
            let mut value_buf = [0u8; 4];
            r.read_exact(&mut value_buf)?;
            let value = i32::from_le_bytes(value_buf);
            awards.insert(key, value);
        }
        let mut bytes_buf = [0u8; 1];
        r.read_exact(&mut bytes_buf)?;
        let weapons_count = bytes_buf[0];
        let mut weapons = Vec::with_capacity(weapons_count as _);
        for _ in 0..weapons_count {
            let weapon = WeaponUsage::parse(r)?;
            weapons.push(weapon);
        }
        let mut bytes_buf = [0u8; 1];
        r.read_exact(&mut bytes_buf)?;
        let players_died_before = bytes_buf[0];
        Ok(Self {
            player_name,
            mastery,
            is_alive,
            is_disconnected,
            party_members,
            score,
            score_position,
            score_position_in_team,
            awards,
            weapons,
            players_died_before,
        })
    }
}

#[repr(u8)]
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum PlayerAwardId {
    DestroyedCubes = 1,
    DestroyedCubesInProtection = 2,
    DestroyedCubesDefendingBase = 3,
    Kill = 4,
    KillAssist = 5,
    HealCubes = 6,
    HealAssist = 7,
    // why are there gaps?
    Score = 11,
    CompletionBonus = 17,
}

impl PlayerAwardId {
    pub fn from_u8(b: u8) -> Option<Self> {
        match b {
            1 => Some(Self::DestroyedCubes),
            2 => Some(Self::DestroyedCubesInProtection),
            3 => Some(Self::DestroyedCubesDefendingBase),
            4 => Some(Self::Kill),
            5 => Some(Self::KillAssist),
            6 => Some(Self::HealCubes),
            7 => Some(Self::HealAssist),
            11 => Some(Self::Score),
            17 => Some(Self::CompletionBonus),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

pub struct WeaponUsage {
    pub category: super::weapon_list::ItemCategory,
    pub size: super::cube_list::ItemTier,
    pub usage_ratio: f32,
}

impl WeaponUsage {
    pub fn parse(r: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let mut category_buf = [0u8; 4];
        r.read_exact(&mut category_buf)?;
        let category_num = i32::from_le_bytes(category_buf);
        let category = super::weapon_list::ItemCategory::from_smaller(category_num)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid item category {}", category_num)))?;
        let mut size_buf = [0u8; 4];
        r.read_exact(&mut size_buf)?;
        let size_num = i32::from_le_bytes(size_buf);
        let size = super::cube_list::ItemTier::from_u32(size_num as _)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid item size/tier {}", size_num)))?;
        let mut ratio_buf = [0u8; 4];
        r.read_exact(&mut ratio_buf)?;
        let usage_ratio = f32::from_le_bytes(ratio_buf);
        Ok(Self {
            category,
            size,
            usage_ratio,
        })
    }
}
