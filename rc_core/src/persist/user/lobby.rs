use serde::{Serialize, Deserialize};

use super::account_json::UserData;

fn fake_impl_to_db(client_emu: &crate::persist::config::ClientEmulator) -> oj_rc_database::schema::multiplayer_game_player::ClientType {
    match client_emu {
        crate::persist::config::ClientEmulator::Experiment => oj_rc_database::schema::multiplayer_game_player::ClientType::ServerExperimental,
        crate::persist::config::ClientEmulator::ClientAI => oj_rc_database::schema::multiplayer_game_player::ClientType::ClientAI,
    }
}

#[async_trait::async_trait]
impl super::LobbyUser for UserData {
    fn user_id(&self) -> i32 {
        self.account.id
    }

    async fn player_data(&self, cpu_counter: &crate::cubes::CpuListParser) -> Result<crate::data::player_data::PlayerData, polariton_server::operations::SimpleOpError> {
        self.user_player_data(cpu_counter).await.map_err(|e| {
            if let Some(msg) = e.error_msg() {
                polariton_server::operations::SimpleOpError::with_message(crate::data::error_codes::LobbyReasonCode::from_service_error(e.error_code()) as i16, msg.to_owned())
            } else {
                polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::LobbyReasonCode::from_service_error(e.error_code()) as i16)
            }

        })
    }

    async fn team_chooser(&self, game: &super::GameDescriptor) -> super::StandardTeamChooser {
        match game.mode {
            crate::data::game_mode::GameMode::Pit => super::StandardTeamChooser::OnePer,
            _ => super::StandardTeamChooser::alternating(),
        }
    }

    async fn start_game(
        &self,
        game: super::GameDescriptor,
        players: Vec<super::PlayerLobbyDescriptor>,
        factory: &dyn oj_rc_factory::VehicleFactoryAdapter,
        cpu_counter: &crate::cubes::CpuListParser,
        weapon_lister: &crate::cubes::WeaponListParser,
        chooser: &dyn super::TeamChooser,
        missing_players: usize,
    ) -> Result<super::FakePlayers, polariton_server::operations::SimpleOpError> {
        let now = chrono::Utc::now().timestamp();
        let guid = crate::persist::user::str_to_i64(&game.guid)
            .ok_or_else(|| polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::LobbyReasonCode::UnexpectedError as i16, "Invalid GUID".to_owned()
            )
        )?;
        let variant = if game.is_ranked {
            oj_rc_database::schema::multiplayer_game::GameType::Ranked
        } else if game.is_custom {
            oj_rc_database::schema::multiplayer_game::GameType::Custom
        } else {
            oj_rc_database::schema::multiplayer_game::GameType::Standard
        };

        let forced_fake_players = self.generate_forced_fake_players_data(&game.guid, &players, factory, cpu_counter, weapon_lister, chooser).await?;
        let filler_players = self.generate_filler_players_data(&game.guid, factory, cpu_counter, weapon_lister, chooser, missing_players, players.len() + forced_fake_players.len()).await?;

        let game_dbo = oj_rc_database::schema::multiplayer_game::ActiveModel {
            id: oj_rc_database::sea_orm::ActiveValue::NotSet,
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
            guid: oj_rc_database::sea_orm::ActiveValue::Set(guid),
            map: oj_rc_database::sea_orm::ActiveValue::Set(game.map),
            mode: oj_rc_database::sea_orm::ActiveValue::Set(game.mode.to_db()),
            visibility: oj_rc_database::sea_orm::ActiveValue::Set(game.visibility.to_db()),
            auto_heal: oj_rc_database::sea_orm::ActiveValue::Set(game.auto_heal),
            variant: oj_rc_database::sea_orm::ActiveValue::Set(variant),
            is_complete: oj_rc_database::sea_orm::ActiveValue::Set(false),
            overrides: oj_rc_database::sea_orm::ActiveValue::Set("".to_owned()),
        };
        let game_dbo = self.db.insert_game(game_dbo).await.map_err(|e| {
            log::error!("Failed to create game {} through user_id {}: {}", game.guid, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::LobbyReasonCode::UnexpectedError as i16,
                format!("Failed to create game {}: {}", game.guid, e),
            )
        })?;

        let players_len = players.len();
        let forced_fake_players_len = forced_fake_players.len();

        let mut group_map = std::collections::HashMap::new();
        let mut group_num = 1;
        for player in players.iter() {
            if let Some(group) = &player.group {
                if !group_map.contains_key(group) {
                    group_map.insert(group.to_owned(), group_num);
                    group_num += 1;
                }
            }
        }

        let players: Vec<oj_rc_database::schema::multiplayer_game_player::ActiveModel> = players.into_iter()
            .enumerate()
            .map(|(i, player)| {
                oj_rc_database::schema::multiplayer_game_player::ActiveModel {
                    id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                    user_id: oj_rc_database::sea_orm::ActiveValue::Set(Some(player.user_id)),
                    game_id: oj_rc_database::sea_orm::ActiveValue::Set(game_dbo.id),
                    creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                    player_id: oj_rc_database::sea_orm::ActiveValue::Set((i as u8) as _),
                    team: oj_rc_database::sea_orm::ActiveValue::Set(player.team),
                    group: oj_rc_database::sea_orm::ActiveValue::Set(player.group.as_ref().and_then(|x| group_map.get(x)).copied()),
                    is_claimed: oj_rc_database::sea_orm::ActiveValue::Set(false),
                    public_id: oj_rc_database::sea_orm::ActiveValue::Set(player.public_id),
                    display_name: oj_rc_database::sea_orm::ActiveValue::Set(player.display_name),
                    variant: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::multiplayer_game_player::ClientType::Client),
                }
            })
            .chain(forced_fake_players.iter()
                .enumerate()
                .map(|(i, (fake, variant))| {
                    oj_rc_database::schema::multiplayer_game_player::ActiveModel {
                        id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                        user_id: oj_rc_database::sea_orm::ActiveValue::Set(None), // if ClientAI, they will be assigned to a user during game loading
                        game_id: oj_rc_database::sea_orm::ActiveValue::Set(game_dbo.id),
                        creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                        player_id: oj_rc_database::sea_orm::ActiveValue::Set(((i + players_len) as u8) as _),
                        team: oj_rc_database::sea_orm::ActiveValue::Set(fake.team),
                        group: oj_rc_database::sea_orm::ActiveValue::Set(None),
                        is_claimed: oj_rc_database::sea_orm::ActiveValue::Set(true),
                        public_id: oj_rc_database::sea_orm::ActiveValue::Set(fake.name.clone()),
                        display_name: oj_rc_database::sea_orm::ActiveValue::Set(fake.display_name.clone()),
                        variant: oj_rc_database::sea_orm::ActiveValue::Set(fake_impl_to_db(variant)),
                    }
                })
            )
            .chain(filler_players.iter()
                .enumerate()
                .map(|(i, (fake, variant))| {
                    oj_rc_database::schema::multiplayer_game_player::ActiveModel {
                        id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                        user_id: oj_rc_database::sea_orm::ActiveValue::Set(None), // if ClientAI, they will be assigned to a user during game loading
                        game_id: oj_rc_database::sea_orm::ActiveValue::Set(game_dbo.id),
                        creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                        player_id: oj_rc_database::sea_orm::ActiveValue::Set(((i + players_len + forced_fake_players_len) as u8) as _),
                        team: oj_rc_database::sea_orm::ActiveValue::Set(fake.team),
                        group: oj_rc_database::sea_orm::ActiveValue::Set(None),
                        is_claimed: oj_rc_database::sea_orm::ActiveValue::Set(true),
                        public_id: oj_rc_database::sea_orm::ActiveValue::Set(fake.name.clone()),
                        display_name: oj_rc_database::sea_orm::ActiveValue::Set(fake.display_name.clone()),
                        variant: oj_rc_database::sea_orm::ActiveValue::Set(fake_impl_to_db(variant)),
                    }
                })
            )
            .collect();
        self.db.insert_players(players).await.map_err(|e| {
            log::error!("Failed to create game players for {} through user_id {}: {}", game.guid, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::LobbyReasonCode::UnexpectedError as i16,
                format!("Failed to create game players for {}: {}", game.guid, e),
            )
        })?;

        Ok(super::FakePlayers {
            players: forced_fake_players.into_iter().chain(filler_players).collect(),
        })
    }

    async fn start_custom_game(&self, game: super::GameDescriptor, players: Vec<super::PlayerLobbyDescriptor>) -> Result<(), polariton_server::operations::SimpleOpError> {
        let now = chrono::Utc::now().timestamp();
        let guid = crate::persist::user::str_to_i64(&game.guid)
            .ok_or_else(|| polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::LobbyReasonCode::UnexpectedError as i16, "Invalid GUID".to_owned()
            )
        )?;
        let variant = if game.is_ranked {
            oj_rc_database::schema::multiplayer_game::GameType::Ranked
        } else if game.is_custom {
            oj_rc_database::schema::multiplayer_game::GameType::Custom
        } else {
            oj_rc_database::schema::multiplayer_game::GameType::Standard
        };

        let conf_str = if let Some(config) = game.overrides {
            serde_json::to_string_pretty(&CustomGameOverrides::from_user(&config)).unwrap()
        } else {
            "".to_owned()
        };

        let game_dbo = oj_rc_database::schema::multiplayer_game::ActiveModel {
            id: oj_rc_database::sea_orm::ActiveValue::NotSet,
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
            guid: oj_rc_database::sea_orm::ActiveValue::Set(guid),
            map: oj_rc_database::sea_orm::ActiveValue::Set(game.map),
            mode: oj_rc_database::sea_orm::ActiveValue::Set(game.mode.to_db()),
            visibility: oj_rc_database::sea_orm::ActiveValue::Set(game.visibility.to_db()),
            auto_heal: oj_rc_database::sea_orm::ActiveValue::Set(game.auto_heal),
            variant: oj_rc_database::sea_orm::ActiveValue::Set(variant),
            is_complete: oj_rc_database::sea_orm::ActiveValue::Set(false),
            overrides: oj_rc_database::sea_orm::ActiveValue::Set(conf_str),
        };
        let game_dbo = self.db.insert_game(game_dbo).await.map_err(|e| {
            log::error!("Failed to create custom game {} through user_id {}: {}", game.guid, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::LobbyReasonCode::UnexpectedError as i16,
                format!("Failed to create custom game {}: {}", game.guid, e),
            )
        })?;

        let players: Vec<oj_rc_database::schema::multiplayer_game_player::ActiveModel> = players.into_iter()
            .enumerate()
            .map(|(i, player)| {
                oj_rc_database::schema::multiplayer_game_player::ActiveModel {
                    id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                    user_id: oj_rc_database::sea_orm::ActiveValue::Set(Some(player.user_id)),
                    game_id: oj_rc_database::sea_orm::ActiveValue::Set(game_dbo.id),
                    creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                    player_id: oj_rc_database::sea_orm::ActiveValue::Set((i as u8) as _),
                    team: oj_rc_database::sea_orm::ActiveValue::Set(player.team),
                    group: oj_rc_database::sea_orm::ActiveValue::Set(None),
                    is_claimed: oj_rc_database::sea_orm::ActiveValue::Set(false),
                    public_id: oj_rc_database::sea_orm::ActiveValue::Set(player.public_id),
                    display_name: oj_rc_database::sea_orm::ActiveValue::Set(player.display_name),
                    variant: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::multiplayer_game_player::ClientType::Client),
                }
            })
            .collect();
        self.db.insert_players(players).await.map_err(|e| {
            log::error!("Failed to create game players for {} through user_id {}: {}", game.guid, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::LobbyReasonCode::UnexpectedError as i16,
                format!("Failed to create game players for {}: {}", game.guid, e),
            )
        })?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct CustomGameOverrides {
    pub capture_segment_memory: bool,
    pub base_shields_go_down: bool,
    pub damage_mult: i32,
    pub health_mult: i32,
    pub power_mult: i32,
    pub game_time: i32, // minutes
    pub capture_speed: i32, // seconds
    pub points_kill_streak: bool,
    pub points_total_required: i32,
    pub number_of_kills_to_win: i32,
    pub respawn_time: i32,
    pub core_appear_frequency: i32,
    pub core_health_multiplier: i32,
    pub core_destroy_time: i32,
    pub protonium_harvest: i32,
    pub ceiling_multiplier: i32,
    pub min_cpu: i32,
    pub max_cpu: i32,
}

impl CustomGameOverrides {
    pub(super) fn from_user(intercom: &super::GameOverrides) -> Self {
        Self {
            capture_segment_memory: intercom.capture_segment_memory,
            base_shields_go_down: intercom.base_shields_go_down,
            damage_mult: intercom.damage_mult,
            health_mult: intercom.health_mult,
            power_mult: intercom.power_mult,
            game_time: intercom.game_time,
            capture_speed: intercom.capture_speed,
            points_kill_streak: intercom.points_kill_streak,
            points_total_required: intercom.points_total_required,
            number_of_kills_to_win: intercom.number_of_kills_to_win,
            respawn_time: intercom.respawn_time,
            core_appear_frequency: intercom.core_appear_frequency,
            core_health_multiplier: intercom.core_health_multiplier,
            core_destroy_time: intercom.core_destroy_time,
            protonium_harvest: intercom.protonium_harvest,
            ceiling_multiplier: intercom.ceiling_multiplier,
            min_cpu: intercom.min_cpu,
            max_cpu: intercom.max_cpu,
        }
    }

    pub(super) fn to_user(&self) -> super::GameOverrides {
        super::GameOverrides {
            capture_segment_memory: self.capture_segment_memory,
            base_shields_go_down: self.base_shields_go_down,
            damage_mult: self.damage_mult,
            health_mult: self.health_mult,
            power_mult: self.power_mult,
            game_time: self.game_time,
            capture_speed: self.capture_speed,
            points_kill_streak: self.points_kill_streak,
            points_total_required: self.points_total_required,
            number_of_kills_to_win: self.number_of_kills_to_win,
            respawn_time: self.respawn_time,
            core_appear_frequency: self.core_appear_frequency,
            core_health_multiplier: self.core_health_multiplier,
            core_destroy_time: self.core_destroy_time,
            protonium_harvest: self.protonium_harvest,
            ceiling_multiplier: self.ceiling_multiplier,
            min_cpu: self.min_cpu,
            max_cpu: self.max_cpu,
        }
    }
}
