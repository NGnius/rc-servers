use polariton::{operation::Typed, serdes::TypePrefix};

pub struct GameModeConfig {
    pub respawn_heal_duration: f32,
    pub respawn_full_heal_duration: f32,
    pub kill_limit: i32,
    pub game_time_minutes: i32,
}

impl GameModeConfig {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("respawnHealDuration".into()), Typed::Float(self.respawn_heal_duration)),
            (Typed::Str("respawnFullHealDuration".into()), Typed::Float(self.respawn_full_heal_duration)),
            (Typed::Str("killLimit".into()), Typed::Int(self.kill_limit)),
            (Typed::Str("gameTimeMinutes".into()), Typed::Int(self.game_time_minutes)),
        ].into())
    }
}

pub struct GameModeConfigs {
    pub battle_arena: GameModeConfig,
    pub elimination: GameModeConfig,
    pub the_pit: GameModeConfig,
    pub team_deathmatch: GameModeConfig,
}

impl GameModeConfigs {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::Dict(polariton::operation::Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::HashMap,
            items: vec![
                (Typed::Str("BattleArena".into()), self.battle_arena.as_transmissible()),
                (Typed::Str("Elimination".into()), self.elimination.as_transmissible()),
                (Typed::Str("ThePit".into()), self.the_pit.as_transmissible()),
                (Typed::Str("TeamDeathmatch".into()), self.team_deathmatch.as_transmissible()),
            ],
        })
    }
}

pub enum GameMap {
    Mars1,
    Mars2,
    Mars3,
    Neptune1,
    Neptune2,
    Neptune3,
    Earth1,
    Earth2,
}

impl GameMap {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mars1 => "RC_Planet_Mars_01_CTF", // og flat mars
            Self::Mars2 => "RC_Planet_Mars_02_BA", // the one with the bridge in the middle
            Self::Mars3 => "RC_Planet_Mars_03_BA", // tharsis rift without the rift
            Self::Neptune1 => "RC_Planet_Neptune_01_CTF", // og flat GJ1214b gliese lake without the lake
            Self::Neptune2 => "RC_Planet_Neptune_02_BA", // the one with the cave
            Self::Neptune3 => "RC_Planet_Neptune_03_BA", // spitzer dam
            Self::Earth1 => "RC_Planet_Earth_01_BA", // birmingham power station
            Self::Earth2 => "RC_Planet_Earth_02_BA", // vanguard
        }
    }

    #[inline]
    pub fn from_persist(map: crate::persist::config::GameMap) -> Self {
        match map {
            crate::persist::config::GameMap::Mars1 => Self::Mars1,
            crate::persist::config::GameMap::Mars2 => Self::Mars2,
            crate::persist::config::GameMap::Mars3 => Self::Mars3,
            crate::persist::config::GameMap::Neptune1 => Self::Neptune1,
            crate::persist::config::GameMap::Neptune2 => Self::Neptune2,
            crate::persist::config::GameMap::Neptune3 => Self::Neptune3,
            crate::persist::config::GameMap::Earth1 => Self::Earth1,
            crate::persist::config::GameMap::Earth2 => Self::Earth2,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
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

    #[inline]
    pub fn from_persist(mode: crate::persist::config::GameType) -> Self {
        match mode {
            crate::persist::config::GameType::BattleArena => Self::BattleArena,
            crate::persist::config::GameType::SuddenDeath => Self::SuddenDeath,
            crate::persist::config::GameType::Pit => Self::Pit,
            crate::persist::config::GameType::TestMode => Self::TestMode,
            crate::persist::config::GameType::SinglePlayer => Self::SinglePlayer,
            crate::persist::config::GameType::TeamDeathmatch => Self::TeamDeathmatch,
            crate::persist::config::GameType::Campaign => Self::Campaign,
        }
    }

    #[inline]
    pub(crate) fn from_db(mode: oj_rc_database::schema::multiplayer_game::GameMode) -> Self {
        match mode {
            oj_rc_database::schema::multiplayer_game::GameMode::BattleArena => Self::BattleArena,
            oj_rc_database::schema::multiplayer_game::GameMode::SuddenDeath => Self::SuddenDeath,
            oj_rc_database::schema::multiplayer_game::GameMode::Pit => Self::Pit,
            oj_rc_database::schema::multiplayer_game::GameMode::TestMode => Self::TestMode,
            oj_rc_database::schema::multiplayer_game::GameMode::SinglePlayer => Self::SinglePlayer,
            oj_rc_database::schema::multiplayer_game::GameMode::TeamDeathmatch => Self::TeamDeathmatch,
            oj_rc_database::schema::multiplayer_game::GameMode::Campaign => Self::Campaign,
        }
    }

    #[inline]
    pub(crate) fn to_db(&self) -> oj_rc_database::schema::multiplayer_game::GameMode {
        match self {
            Self::BattleArena => oj_rc_database::schema::multiplayer_game::GameMode::BattleArena,
            Self::SuddenDeath => oj_rc_database::schema::multiplayer_game::GameMode::SuddenDeath,
            Self::Pit => oj_rc_database::schema::multiplayer_game::GameMode::Pit,
            Self::TestMode => oj_rc_database::schema::multiplayer_game::GameMode::TestMode,
            Self::SinglePlayer => oj_rc_database::schema::multiplayer_game::GameMode::SinglePlayer,
            Self::TeamDeathmatch => oj_rc_database::schema::multiplayer_game::GameMode::TeamDeathmatch,
            Self::Campaign => oj_rc_database::schema::multiplayer_game::GameMode::Campaign,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum MapVisibility {
    Good = 0,
    Poor = 1,
    Bad = 2, // VeryPoor
}

impl MapVisibility {
    #[inline]
    pub fn from_persist(mode: crate::persist::config::GameVisibility) -> Self {
        match mode {
            crate::persist::config::GameVisibility::Good => Self::Good,
            crate::persist::config::GameVisibility::Poor => Self::Poor,
            crate::persist::config::GameVisibility::Bad => Self::Bad,
        }
    }

    #[inline]
    pub(crate) fn from_db(mode: oj_rc_database::schema::multiplayer_game::MapVisibility) -> Self {
        match mode {
            oj_rc_database::schema::multiplayer_game::MapVisibility::Good => Self::Good,
            oj_rc_database::schema::multiplayer_game::MapVisibility::Poor => Self::Poor,
            oj_rc_database::schema::multiplayer_game::MapVisibility::Bad => Self::Bad,
        }
    }

    #[inline]
    pub(crate) fn to_db(&self) -> oj_rc_database::schema::multiplayer_game::MapVisibility {
        match self {
            Self::Good => oj_rc_database::schema::multiplayer_game::MapVisibility::Good,
            Self::Poor => oj_rc_database::schema::multiplayer_game::MapVisibility::Poor,
            Self::Bad => oj_rc_database::schema::multiplayer_game::MapVisibility::Bad,
        }
    }
}
