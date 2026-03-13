use crate::data::custom_games::*;

fn custom_game_key(creator: &str) -> String {
    let now = chrono::Utc::now().timestamp();
    format!("{}_{}_cg", creator, now)
}

pub struct UserInfo {
    pub public_id: String,
    pub is_invited: bool,
    pub team: u8,
    pub state: PlayerSessionStatus,
}

pub struct SessionInfo {
    pub session_id: String,
    pub config: std::collections::HashMap<String, String>,
    pub users: Vec<UserInfo>,
}

pub struct KickInfo {
    pub session_id: String,
    pub was_invited: bool,
}

pub struct CustomGameMesh {
    games: tokio::sync::RwLock<std::collections::HashMap<String, GameHandle>>,
    user_to_game: tokio::sync::RwLock<std::collections::HashMap<String, String>>,
}

struct GameHandle {
    users: Vec<UserHandle>,
    config: GameConfig,
}

struct UserHandle {
    public_id: String,
    is_invited: std::sync::atomic::AtomicBool,
    team: u8,
    status: std::sync::atomic::AtomicU8,
}

impl CustomGameMesh {
    pub fn new() -> Self {
        Self {
            games: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            user_to_game: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub async fn create_game(&self, public_id: &str) -> Result<String, SessionCreateResponseCode> {
        let mut user_lock = self.user_to_game.write().await;
        if user_lock.contains_key(public_id) {
            log::debug!("Rejected custom game session create; user {} is already in a session", public_id);
            return Err(SessionCreateResponseCode::AlreadyInSession);
        }
        let game_id = custom_game_key(public_id);
        let owner_handle = UserHandle {
            public_id: public_id.to_owned(),
            is_invited: std::sync::atomic::AtomicBool::new(false),
            team: 0,
            status: std::sync::atomic::AtomicU8::new(PlayerSessionStatus::Ready.to_u8()),
        };
        let game_handle = GameHandle {
            users: vec![owner_handle],
            config: GameConfig::default(),
        };
        let mut games_lock = self.games.write().await;
        user_lock.insert(public_id.to_owned(), game_id.clone());
        games_lock.insert(game_id.clone(), game_handle);
        log::debug!("Custom game session {} created", game_id);
        Ok(game_id)
    }

    pub async fn leave_game(&self, public_id: &str) -> (SessionLeaveResponseCode, Option<SessionInfo>) {
        if let Some(game_id) = { self.user_to_game.write().await.remove(public_id) } {
            let mut games_lock = self.games.write().await;
            if let Some(game) = games_lock.get_mut(&game_id) {
                let session = if game.users.len() == 1 || game.users.iter().all(|u| u.public_id == public_id || u.is_invited.load(std::sync::atomic::Ordering::Relaxed)) {
                    log::debug!("User {} has disbanded custom game {}", public_id, game_id);
                    if game.users.len() != 1 {
                        let mut user_lock = self.user_to_game.write().await;
                        for user in game.users.iter() {
                            user_lock.remove(&user.public_id);
                        }
                        log::debug!("Removed {} invited stragglers from custom game {}", game.users.len() - 1, game_id);
                    }
                    let session = Self::session_from_game(&game_id, game);
                    games_lock.remove(&game_id);
                    session
                } else {
                    log::debug!("User {} has left custom game {}", public_id, game_id);
                    game.users.retain(|user| user.public_id != public_id);
                    Self::session_from_game(&game_id, game)
                };
                return (SessionLeaveResponseCode::Success, Some(session));
            }
        }
        (SessionLeaveResponseCode::NotInSession, None)
    }

    pub async fn kick_from_game(&self, kicker: &str, kickee: &str) -> (KickResponseCode, Option<(KickInfo, SessionInfo)>) {
        if let Some(game_id) = { self.user_to_game.write().await.remove(kickee) } {
            let mut games_lock = self.games.write().await;
            if let Some(game) = games_lock.get_mut(&game_id) {
                let leader = game.users.first().unwrap();
                if leader.public_id != kicker {
                    return (KickResponseCode::UserIsNotSessionLeader, None);
                }
                let target = game.users.iter().find(|mem| mem.public_id == kickee).unwrap();
                let kick_info = KickInfo {
                    session_id: game_id.clone(),
                    was_invited: target.is_invited.load(std::sync::atomic::Ordering::Relaxed),
                };
                game.users.retain(|user| user.public_id != kickee);
                let session = Self::session_from_game(&game_id, game);
                return (KickResponseCode::UserRemovedFromSession, Some((kick_info, session)));
            }
        }
        (KickResponseCode::KickTargetIsNotInsession, None)
    }

    pub async fn get_user_game(&self, public_id: &str) -> Option<SessionInfo> {
        if let Some(game_id) = self.user_to_game.read().await.get(public_id) {
            if let Some(game) = self.games.read().await.get(game_id) {
                return Some(SessionInfo {
                    session_id: game_id.to_owned(),
                    config: game.config.as_map(),
                    users: game.users.iter()
                        .map(|user| UserInfo {
                            public_id: user.public_id.clone(),
                            is_invited: user.is_invited.load(std::sync::atomic::Ordering::Relaxed),
                            team: user.team,
                            state: PlayerSessionStatus::from_u8(user.status.load(std::sync::atomic::Ordering::Relaxed)).unwrap(),
                        })
                        .collect()
                });
            }
        }
        None
    }

    pub async fn invite_user(&self, inviter: &str, invitee: &str, is_team_a: bool) -> (InviteToCustomGameResponseCode, Option<SessionInfo>) {
        if let Some(game_id) = { self.user_to_game.read().await.get(inviter).cloned() } {
            if let Some(game) = self.games.write().await.get_mut(&game_id) {
                if game.users.iter().find(|x| x.public_id == invitee).is_some() {
                    let session = Self::session_from_game(&game_id, game);
                    (InviteToCustomGameResponseCode::InviteeHasAlreadyBeenInvited, Some(session))
                } else {
                    let invitee_handle = UserHandle {
                        public_id: invitee.to_owned(),
                        is_invited: std::sync::atomic::AtomicBool::new(true),
                        team: if is_team_a { 0 } else { 1 },
                        status: std::sync::atomic::AtomicU8::new(PlayerSessionStatus::Unknown.to_u8())
                    };
                    game.users.push(invitee_handle);
                    let session = Self::session_from_game(&game_id, game);
                    self.user_to_game.write().await.insert(invitee.to_owned(), game_id);
                    log::debug!("User {} invited to custom game", invitee);
                    (InviteToCustomGameResponseCode::UserInvited, Some(session))
                }
            } else {
                (InviteToCustomGameResponseCode::UserIsNotInSession, None)
            }
        } else {
            (InviteToCustomGameResponseCode::UserIsNotInSession, None)
        }
    }

    pub async fn update_invite_user(&self, invitee: &str, is_accept: bool) -> (InviteReplyCustomGameResponseCode, Option<SessionInfo>) {
        if let Some(game_id) = { self.user_to_game.read().await.get(invitee).cloned() } {
            if is_accept {
                // fully join custom game
                if let Some(game) = self.games.read().await.get(&game_id) {
                    let invitee_handle_opt = game.users.iter().find(|user| user.public_id == invitee);
                    if let Some(invitee_handle) = invitee_handle_opt {
                        invitee_handle.is_invited.store(false, std::sync::atomic::Ordering::Relaxed);
                        invitee_handle.status.store(PlayerSessionStatus::Ready.to_u8(), std::sync::atomic::Ordering::Relaxed);
                        let session = Self::session_from_game(&game_id, game);
                        (InviteReplyCustomGameResponseCode::Success, Some(session))
                    } else {
                        (InviteReplyCustomGameResponseCode::UserIsNoLongerInvited, None)
                    }
                } else {
                    (InviteReplyCustomGameResponseCode::UserIsNoLongerInvited, None)
                }
            } else {
                // leave custom game
                if let Some(game) = self.games.write().await.get_mut(&game_id) {
                    game.users.retain(|user| user.public_id != invitee);
                    self.user_to_game.write().await.remove(invitee);
                    let session = Self::session_from_game(&game_id, game);
                    (InviteReplyCustomGameResponseCode::Success, Some(session))
                } else {
                    (InviteReplyCustomGameResponseCode::UserIsNoLongerInvited, None)
                }
            }

        } else {
            (InviteReplyCustomGameResponseCode::UserIsNotInSession, None)
        }
    }

    pub async fn update_user_status(&self, public_id: &str, status: PlayerSessionStatus) -> Option<SessionInfo> {
        if let Some(game_id) = self.user_to_game.read().await.get(public_id) {
            if let Some(game) = self.games.read().await.get(game_id) {
                let target = game.users.iter()
                    .find(|mem| mem.public_id == public_id)
                    .unwrap();
                target.status.store(status.to_u8(), std::sync::atomic::Ordering::Relaxed);
                let session = Self::session_from_game(&game_id, game);
                return Some(session);
            }
        }
        None
    }

    pub async fn set_config_field(&self, public_id: &str, field: &str, value: &str) -> (AdjustCustomGameConfigResponseCode, Option<SessionInfo>) {
        if let Some(game_id) = self.user_to_game.read().await.get(public_id) {
            if let Some(game) = self.games.write().await.get_mut(game_id) {
                if game.users[0].public_id != public_id {
                    log::debug!("Update custom game session {} config rejected (not leader)", game_id);
                    return (AdjustCustomGameConfigResponseCode::AdjustmentRejected, None);
                }
                if let Ok(_) = game.config.set_field(field, value) {
                    log::debug!("Update custom game session {} config {} to {}", game_id, field, value);
                    let session = Self::session_from_game(&game_id, game);
                    return (AdjustCustomGameConfigResponseCode::Success, Some(session));
                }
            }
            return (AdjustCustomGameConfigResponseCode::AdjustmentRejected, None);
        } else {
            (AdjustCustomGameConfigResponseCode::NotInSession, None)
        }
    }

    fn session_from_game(game_id: &str, game: &GameHandle) -> SessionInfo {
        SessionInfo {
            session_id: game_id.to_owned(),
            config: game.config.as_map(),
            users: game.users.iter()
                .map(|user| UserInfo {
                    public_id: user.public_id.clone(),
                    is_invited: user.is_invited.load(std::sync::atomic::Ordering::Relaxed),
                    team: user.team,
                    state: PlayerSessionStatus::from_u8(user.status.load(std::sync::atomic::Ordering::Relaxed))
                        .unwrap_or(PlayerSessionStatus::Unknown),
                })
                .collect()
        }
    }
}

// multipliers are percents (as in, 100 is no change; 200 is 2x original, 10 is 0.1x)

struct GameConfig {
    game_mode: oj_rc_core::data::game_mode::GameMode,
    map: String, // TODO maybe this should be validated
    map_visibility: oj_rc_core::data::game_mode::MapVisibility,
    health_regen: bool,
    capture_segment_memory: bool,
    base_shields_go_down: bool,
    damage_mult: i32,
    health_mult: i32,
    power_mult: i32,
    game_time: i32, // minutes?
    capture_speed: i32, // this is two things in one (seconds?)
    points_kill_streak: bool,
    points_total_required: i32,
    number_of_kills_to_win: i32,
    respawn_time: i32, // this is three things in one
    core_appear_frequency: i32,
    core_health_multiplier: i32,
    core_destroy_time: i32,
    protonium_harvest: i32,
    ceiling_multiplier: i32,
    min_cpu: i32,
    max_cpu: i32,
}

impl core::default::Default for GameConfig {
    fn default() -> Self {
        Self {
            game_mode: oj_rc_core::data::game_mode::GameMode::BattleArena,
            map: oj_rc_core::data::game_mode::GameMap::Earth2.as_str().to_owned(),
            map_visibility: oj_rc_core::data::game_mode::MapVisibility::Good,
            health_regen: true,
            capture_segment_memory: true,
            base_shields_go_down: true,
            damage_mult: 100,
            health_mult: 100,
            power_mult: 100,
            game_time: 10,
            capture_speed: 120,
            points_kill_streak: false,
            points_total_required: 1,
            number_of_kills_to_win: 1,
            respawn_time: 1,
            core_appear_frequency: 1,
            core_health_multiplier: 100,
            core_destroy_time: 10,
            protonium_harvest: 10,
            ceiling_multiplier: 100,
            min_cpu: 200,
            max_cpu: 100_000,
        }
    }
}

enum ConfigSetError {
    ValueParseError,
    InvalidField,
}

impl GameConfig {
    fn as_map(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::with_capacity(10);
        map.insert("GameMode".to_owned(), self.game_mode.as_str().to_owned());
        map.insert("MapChoice".to_owned(), self.map.clone());
        map.insert("MapVisibility".to_owned(), self.map_visibility.as_str().to_owned());
        map.insert("HealthRegen".to_owned(), if self.health_regen { "True".to_owned() } else { "False".to_owned() });
        map.insert("CaptureSegmentMemory".to_owned(), if self.capture_segment_memory { "True".to_owned() } else { "False".to_owned() });
        map.insert("BaseShieldsGoDown".to_owned(), if self.base_shields_go_down { "True".to_owned() } else { "False".to_owned() });
        map.insert("DamageMultiplier".to_owned(), self.damage_mult.to_string());
        map.insert("HealthMultiplier".to_owned(), self.health_mult.to_string());
        map.insert("PowerMultiplier".to_owned(), self.power_mult.to_string());
        map.insert("GameTime".to_owned(), self.game_time.to_string());
        map.insert("CaptureSpeedElimination".to_owned(), self.capture_speed.to_string());
        map.insert("PointsKillStreakOnOff".to_owned(), if self.points_kill_streak { "True".to_owned() } else { "False".to_owned() });
        map.insert("PointsTotalRequired".to_owned(), self.points_total_required.to_string());
        map.insert("NumberOfKillsToWin".to_owned(), self.number_of_kills_to_win.to_string());
        map.insert("RespawnTimeBA".to_owned(), self.respawn_time.to_string());
        map.insert("RespawnTimeTDM".to_owned(), self.respawn_time.to_string());
        map.insert("RespawnTimePit".to_owned(), self.respawn_time.to_string());
        map.insert("CoreAppearFrequency".to_owned(), self.core_appear_frequency.to_string());
        map.insert("CoreHealthMultiplier".to_owned(), self.core_health_multiplier.to_string());
        map.insert("CoreDestroyTimeValue".to_owned(), self.core_destroy_time.to_string());
        map.insert("CaptureSpeedBA".to_owned(), self.capture_speed.to_string());
        map.insert("ProtoniumHarvestBA".to_owned(), self.protonium_harvest.to_string());
        map.insert("CeilingMultiplier".to_owned(), self.ceiling_multiplier.to_string());
        map.insert("MinCPU".to_owned(), self.min_cpu.to_string());
        map.insert("MaxCPU".to_owned(), self.max_cpu.to_string());
        map
    }

    fn set_field(&mut self, field: &str, value: &str) -> Result<(), ConfigSetError> {
        match field {
            "GameMode" => {
                let val = match value {
                    "TeamDeathmatch" => Ok(oj_rc_core::data::game_mode::GameMode::TeamDeathmatch),
                    "BattleArena" => Ok(oj_rc_core::data::game_mode::GameMode::BattleArena),
                    "Pit" => Ok(oj_rc_core::data::game_mode::GameMode::Pit),
                    "SuddenDeath" => Ok(oj_rc_core::data::game_mode::GameMode::SuddenDeath),
                    idk_val => {
                        log::warn!("Unrecognized game mode {}", idk_val);
                        Err(ConfigSetError::ValueParseError)
                    }
                }?;
                self.game_mode = val;
                Ok(())
            },
            "MapChoice" => {
                self.map = value.to_owned();
                Ok(())
            },
            "MapVisibility" => {
                let val = match value {
                    "VeryPoor" => Ok(oj_rc_core::data::game_mode::MapVisibility::Bad),
                    "Poor" => Ok(oj_rc_core::data::game_mode::MapVisibility::Poor),
                    "Good" => Ok(oj_rc_core::data::game_mode::MapVisibility::Good),
                    idk_val => {
                        log::warn!("Unrecognized map visibility {}", idk_val);
                        Err(ConfigSetError::ValueParseError)
                    }
                }?;
                self.map_visibility = val;
                Ok(())
            },
            "HealthRegen" => {
                match value.to_lowercase().parse() { // parsing only accepts "true" or "false", C# uses "True" or "False"
                    Err(e) => {
                        log::warn!("Failed to parse value {} as bool for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.health_regen = val;
                        Ok(())
                    }
                }
            },
            "CaptureSegmentMemory" => {
                match value.to_lowercase().parse() { // parsing only accepts "true" or "false", C# uses "True" or "False"
                    Err(e) => {
                        log::warn!("Failed to parse value {} as bool for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.capture_segment_memory = val;
                        Ok(())
                    }
                }
            },
            "BaseShieldsGoDown" => {
                match value.to_lowercase().parse() { // parsing only accepts "true" or "false", C# uses "True" or "False"
                    Err(e) => {
                        log::warn!("Failed to parse value {} as bool for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.base_shields_go_down = val;
                        Ok(())
                    }
                }
            },
            "DamageMultiplier" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.damage_mult = val;
                        Ok(())
                    }
                }
            },
            "HealthMultiplier" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.health_mult = val;
                        Ok(())
                    }
                }
            },
            "PowerMultiplier" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.power_mult = val;
                        Ok(())
                    }
                }
            },
            "GameTime" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.game_time = val;
                        Ok(())
                    }
                }
            },
            "CaptureSpeedElimination" | "CaptureSpeedBA" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.capture_speed = val;
                        Ok(())
                    }
                }
            },
            "PointsKillStreakOnOff" => {
                match value.to_lowercase().parse() { // parsing only accepts "true" or "false", C# uses "True" or "False"
                    Err(e) => {
                        log::warn!("Failed to parse value {} as bool for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.points_kill_streak = val;
                        Ok(())
                    }
                }
            },
            "PointsTotalRequired" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.points_total_required = val;
                        Ok(())
                    }
                }
            },
            "NumberOfKillsToWin" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.number_of_kills_to_win = val;
                        Ok(())
                    }
                }
            },
            "RespawnTimeBA" | "RespawnTimeTDM" | "RespawnTimePit" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.respawn_time = val;
                        Ok(())
                    }
                }
            },
            "CoreAppearFrequency" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.core_appear_frequency = val;
                        Ok(())
                    }
                }
            },
            "CoreHealthMultiplier" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.core_health_multiplier = val;
                        Ok(())
                    }
                }
            },
            "CoreDestroyTimeValue" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.core_destroy_time = val;
                        Ok(())
                    }
                }
            },
            "ProtoniumHarvestBA" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.protonium_harvest = val;
                        Ok(())
                    }
                }
            },
            "CeilingMultiplier" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.ceiling_multiplier = val;
                        Ok(())
                    }
                }
            },
            "MinCPU" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.min_cpu = val;
                        Ok(())
                    }
                }
            },
            "MaxCPU" => {
                match value.parse() {
                    Err(e) => {
                        log::warn!("Failed to parse value {} as i32 for field {} : {}", value, field, e);
                        Err(ConfigSetError::ValueParseError)
                    },
                    Ok(val) => {
                        self.max_cpu = val;
                        Ok(())
                    }
                }
            },
            _ => {
                log::warn!("Unrecognized custom game config field {} (val: {})", field, value);
                Err(ConfigSetError::InvalidField)
            }
        }
    }
}
