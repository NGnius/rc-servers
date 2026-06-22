use super::account_json::UserData;

fn db_to_impl(client_emu: &oj_rc_database::schema::multiplayer_game_player::ClientType) -> Option<crate::persist::config::ClientEmulator> {
    match client_emu {
        oj_rc_database::schema::multiplayer_game_player::ClientType::ServerExperimental => Some(crate::persist::config::ClientEmulator::Experiment),
        oj_rc_database::schema::multiplayer_game_player::ClientType::ClientAI => Some(crate::persist::config::ClientEmulator::ClientAI),
        oj_rc_database::schema::multiplayer_game_player::ClientType::Client => None,
    }
}

impl UserData {
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn generate_fake_players_data(
        &self,
        guid_str: &str,
        factory: &dyn oj_rc_factory::VehicleFactoryAdapter,
        cpu_counter: &crate::cubes::CpuListParser,
        weapon_lister: &crate::cubes::WeaponListParser,
        chooser: &dyn super::TeamChooser,
        fake_players: &[crate::persist::config::FakePlayer],
        total_offset: usize,
    ) -> Result<Vec<(crate::data::player_data::PlayerData, crate::persist::config::ClientEmulator)>, polariton_server::operations::SimpleOpError> {
        let mut fakes = Vec::with_capacity(fake_players.len());
        for (i, fake) in fake_players.iter().enumerate() {
            let vehicle = self.resolve_vehicle(&fake.vehicle, factory, weapon_lister, cpu_counter).await?;
            let fake_lobby_desc = super::PlayerLobbyDescriptor {
                user_id: -1,
                team: -1,
                public_id: fake.vehicle.username.clone(),
                display_name: fake.vehicle.username.clone(),
                group: None,
            };
            let out = (
                crate::data::player_data::PlayerData {
                    name: fake.vehicle.username.clone(),
                    display_name: fake.vehicle.username.clone(),
                    mastery: vehicle.mastery,
                    tier: vehicle.mastery,
                    robot_name: vehicle.robot_name,
                    robot_map: vehicle.robot_map,
                    group: None,
                    team: chooser.choose_team(guid_str, total_offset + i, &fake_lobby_desc),
                    has_premium: true,
                    robot_uuid: vehicle.robot_uuid,
                    cpu: vehicle.cpu,
                    avatar_id: Some(0),
                    weapon_order: vehicle.weapon_order,
                    colour_map: vehicle.colour_map,
                    is_ai: fake.implementation == crate::persist::config::ClientEmulator::ClientAI,
                    spawn_effect: vehicle.spawn_effect,
                    death_effect: vehicle.death_effect,
                    player_rank: 1,
                    weapon_rank: vehicle.weapon_rank,
                    clan_name: None,
                },
                fake.implementation
            );
            fakes.push(out);
        }
        Ok(fakes)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn generate_forced_fake_players_data(
        &self,
        guid_str: &str,
        real_players: &[super::PlayerLobbyDescriptor],
        factory: &dyn oj_rc_factory::VehicleFactoryAdapter,
        cpu_counter: &crate::cubes::CpuListParser,
        weapon_lister: &crate::cubes::WeaponListParser,
        chooser: &dyn super::TeamChooser,
    ) -> Result<Vec<(crate::data::player_data::PlayerData, crate::persist::config::ClientEmulator)>, polariton_server::operations::SimpleOpError> {
        let offset = real_players.len();
        self.generate_fake_players_data(
            guid_str,
            factory,
            cpu_counter,
            weapon_lister,
            chooser,
            &self.fake_players,
            offset,
        ).await
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn generate_filler_players_data(
        &self,
        guid_str: &str,
        factory: &dyn oj_rc_factory::VehicleFactoryAdapter,
        cpu_counter: &crate::cubes::CpuListParser,
        weapon_lister: &crate::cubes::WeaponListParser,
        chooser: &dyn super::TeamChooser,
        count: usize,
        total_offset: usize,
    ) -> Result<Vec<(crate::data::player_data::PlayerData, crate::persist::config::ClientEmulator)>, polariton_server::operations::SimpleOpError> {
        self.generate_fake_players_data(
            guid_str,
            factory,
            cpu_counter,
            weapon_lister,
            chooser,
            &self.filler_players[0..count],
            total_offset
        ).await
    }
}

#[async_trait::async_trait]
impl super::MultiplayerUser for UserData {
    async fn current_game(&self) -> Result<Option<super::GameDescriptor>, super::MultiplayerError> {
        Ok(self.db.game_by_user_id_and_completion(self.account.id, false).await
            .map_err(|e| {
                log::error!("Failed to retrieve ongoing game for user {}: {}", self.account.id, e);
                super::MultiplayerError {
                    code: super::MultiplayerErrorCode::CustomString,
                    message: format!("Failed to retrieve ongoing game: {}", e),
                }
            })?
            .map(|game| super::GameDescriptor {
                guid: crate::persist::user::i64_as_uuid_str(game.guid),
                map: game.map,
                mode: crate::data::game_mode::GameMode::from_db(game.mode),
                visibility: crate::data::game_mode::MapVisibility::from_db(game.visibility),
                auto_heal: game.auto_heal,
                is_ranked: matches!(game.variant, oj_rc_database::schema::multiplayer_game::GameType::Ranked),
                is_custom: matches!(game.variant, oj_rc_database::schema::multiplayer_game::GameType::Custom),
                is_complete: game.is_complete,
                overrides: if game.overrides.is_empty() { None } else {
                    match serde_json::from_str::<super::lobby::CustomGameOverrides>(&game.overrides) {
                        Ok(x) => Some(x.to_user()),
                        Err(e) => {
                            log::warn!("Failed to parse overrides JSON: {}\n{}", e, game.overrides);
                            None
                        }
                    }
                }
            }))
    }

    async fn game_players(&self, guid: &str) -> Result<Vec<super::PlayerDescriptor>, super::MultiplayerError> {
        if let Some(guid) = crate::persist::user::str_to_i64(guid) {
            let players = self.db.players_by_game_guid_and_completion(guid, false).await
                .map_err(|e| {
                    log::error!("Failed to retrieve players for game {} for user {}: {}", guid, self.account.id, e);
                    super::MultiplayerError {
                        code: super::MultiplayerErrorCode::CustomString,
                        message: format!("Failed to retrieve players for game {}: {}", guid, e),
                    }
                })?;
            Ok(players.into_iter()
                .map(|player| super::PlayerDescriptor {
                    user_id: player.user_id,
                    player_id: player.player_id as u8,
                    team: player.team,
                    group: player.group,
                    is_rewards_claimed: player.is_claimed,
                    display_name: player.display_name,
                    public_id: player.public_id,
                    mode: db_to_impl(&player.variant),
                })
                .collect())
        } else {
            Err(super::MultiplayerError {
                code: super::MultiplayerErrorCode::IncorrectGameGuid,
                message: format!("Failed to parse game GUID {}", guid),
            })
        }
    }

    async fn complete_game(&self, guid: &str) -> Result<(), super::MultiplayerError> {
        if let Some(guid) = crate::persist::user::str_to_i64(guid) {
            self.db.update_complete_game_by_game_guid(guid).await
                .map_err(|e| {
                log::error!("Failed to complete ongoing game with user {}: {}", self.account.id, e);
                super::MultiplayerError {
                    code: super::MultiplayerErrorCode::CustomString,
                    message: format!("Failed to complete ongoing game: {}", e),
                }
            })
        } else {
            Err(super::MultiplayerError {
                code: super::MultiplayerErrorCode::IncorrectGameGuid,
                message: format!("Failed to parse game GUID {}", guid),
            })
        }
    }

    async fn game_info(&self, guid: &str) -> Result<Option<super::GameDescriptor>, super::MultiplayerError> {
        if let Some(guid) = crate::persist::user::str_to_i64(guid) {
            let game_opt = self.db.game_by_guid(guid.to_owned()).await
                .map_err(|e| {
                    log::error!("Failed to retrieve game {} with user {}: {}", guid, self.account.id, e);
                    super::MultiplayerError {
                        code: super::MultiplayerErrorCode::CustomString,
                        message: format!("Failed to retrieve game {}: {}", guid, e),
                    }
                })?;
            Ok(game_opt.map(|game| super::GameDescriptor {
                guid: crate::persist::user::i64_as_uuid_str(game.guid),
                map: game.map,
                mode: crate::data::game_mode::GameMode::from_db(game.mode),
                visibility: crate::data::game_mode::MapVisibility::from_db(game.visibility),
                auto_heal: game.auto_heal,
                is_ranked: matches!(game.variant, oj_rc_database::schema::multiplayer_game::GameType::Ranked),
                is_custom: matches!(game.variant, oj_rc_database::schema::multiplayer_game::GameType::Custom),
                is_complete: game.is_complete,
                overrides: if game.overrides.is_empty() { None } else {
                    match serde_json::from_str::<super::lobby::CustomGameOverrides>(&game.overrides) {
                        Ok(x) => Some(x.to_user()),
                        Err(e) => {
                            log::warn!("Failed to parse overrides JSON: {}\n{}", e, game.overrides);
                            None
                        }
                    }
                }
            }))
        } else {
            Err(super::MultiplayerError {
                code: super::MultiplayerErrorCode::IncorrectGameGuid,
                message: format!("Failed to parse game GUID {}", guid),
            })
        }
    }

    async fn update_game_score(&self, guid: &str, score: super::PlayerScore) -> Result<i32, super::MultiplayerError> {
        if let Some(guid) = crate::persist::user::str_to_i64(guid) {
            if let Some(score_id) = score.id {
                let model = oj_rc_database::schema::multiplayer_game_score::ActiveModel {
                    id: oj_rc_database::sea_orm::Set(score_id),
                    player_id: oj_rc_database::sea_orm::NotSet,
                    creation_time: oj_rc_database::sea_orm::NotSet,
                    is_claimed: oj_rc_database::sea_orm::NotSet,
                    kills: oj_rc_database::sea_orm::Set(score.kills as i32),
                    deaths: oj_rc_database::sea_orm::Set(score.deaths as i32),
                    assists: oj_rc_database::sea_orm::Set(score.assists as i32),
                    heal_assists: oj_rc_database::sea_orm::Set(score.heal_assists as i32),
                    healed: oj_rc_database::sea_orm::Set(score.healed as i32),
                    received_healed: oj_rc_database::sea_orm::Set(score.received_healed as i32),
                    damaged: oj_rc_database::sea_orm::Set(score.damaged as i32),
                    received_damaged: oj_rc_database::sea_orm::Set(score.received_damaged as i32),
                    crystals: oj_rc_database::sea_orm::Set(score.crystals as i32),
                    total: oj_rc_database::sea_orm::Set(score.total as i32),
                };
                let persisted_model = self.db.update_score(model).await
                .map_err(|e| {
                    log::error!("Failed to update player score for user {} in game {}: {}", self.account.id, guid, e);
                    super::MultiplayerError {
                        code: super::MultiplayerErrorCode::CustomString,
                        message: format!("Failed to update player score for user {} in game {}: {}", self.account.id, guid, e),
                    }
                })?;
                Ok(persisted_model.id)
            } else {
                let player_opt = self.db.player_by_user_id_and_game_guid(self.account.id, guid).await
                .map_err(|e| {
                    log::error!("Failed to retrieve player for user {} in game {}: {}", self.account.id, guid, e);
                    super::MultiplayerError {
                        code: super::MultiplayerErrorCode::CustomString,
                        message: format!("Failed to retrieve player for user {} in game {}: {}", self.account.id, guid, e),
                    }
                })?;
                if let Some(player) = player_opt {
                    let model = oj_rc_database::schema::multiplayer_game_score::ActiveModel {
                        id: oj_rc_database::sea_orm::NotSet,
                        player_id: oj_rc_database::sea_orm::Set(player.id),
                        creation_time: oj_rc_database::sea_orm::Set(chrono::Utc::now().timestamp()),
                        is_claimed: oj_rc_database::sea_orm::Set(false),
                        kills: oj_rc_database::sea_orm::Set(score.kills as i32),
                        deaths: oj_rc_database::sea_orm::Set(score.deaths as i32),
                        assists: oj_rc_database::sea_orm::Set(score.assists as i32),
                        heal_assists: oj_rc_database::sea_orm::Set(score.heal_assists as i32),
                        healed: oj_rc_database::sea_orm::Set(score.healed as i32),
                        received_healed: oj_rc_database::sea_orm::Set(score.received_healed as i32),
                        damaged: oj_rc_database::sea_orm::Set(score.damaged as i32),
                        received_damaged: oj_rc_database::sea_orm::Set(score.received_damaged as i32),
                        crystals: oj_rc_database::sea_orm::Set(score.crystals as i32),
                        total: oj_rc_database::sea_orm::Set(score.total as i32),
                    };
                    let persisted_model = self.db.insert_score(model).await
                    .map_err(|e| {
                        log::error!("Failed to insert player score for user {} in game {}: {}", self.account.id, guid, e);
                        super::MultiplayerError {
                            code: super::MultiplayerErrorCode::CustomString,
                            message: format!("Failed to insert player score for user {} in game {}: {}", self.account.id, guid, e),
                        }
                    })?;
                    Ok(persisted_model.id)
                } else {
                    Err(super::MultiplayerError {
                        code: super::MultiplayerErrorCode::IncorrectGameGuid,
                        message: format!("Failed to find user {} in game {}", self.account.id, guid),
                    })
                }
            }
        } else {
            Err(super::MultiplayerError {
                code: super::MultiplayerErrorCode::IncorrectGameGuid,
                message: format!("Failed to parse game GUID {}", guid),
            })
        }
    }

    async fn save_player_connected_status(&self, guid: &str, is_connected: bool) -> Result<(), super::MultiplayerError> {
        if let Some(guid) = crate::persist::user::str_to_i64(guid) {
            let player_opt = self.db.player_by_user_id_and_game_guid(self.account.id, guid).await
                .map_err(|e| {
                    log::error!("Failed to retrieve player for game {} and user {}: {}", guid, self.account.id, e);
                    super::MultiplayerError {
                        code: super::MultiplayerErrorCode::CustomString,
                        message: format!("Failed to retrieve user's player for game {}: {}", guid, e),
                    }
                })?;
            if let Some(player) = player_opt {
                self.db.player_claim(player.id, is_connected).await
                    .map_err(|e| {
                        log::error!("Failed to claim player for game {} and user {}: {}", guid, self.account.id, e);
                        super::MultiplayerError {
                            code: super::MultiplayerErrorCode::CustomString,
                            message: format!("Failed to claim user's player for game {}: {}", guid, e),
                        }
                    })?;
                Ok(())
            } else {
                log::warn!("Failed to find to-be-claimed player for user {} in game {}", self.account.id, guid);
                Err(super::MultiplayerError {
                    code: super::MultiplayerErrorCode::CustomString,
                    message: "Failed to find user's player to update connected status".to_string(),
                })
            }
        } else {
            Err(super::MultiplayerError {
                code: super::MultiplayerErrorCode::IncorrectGameGuid,
                message: format!("Failed to parse game GUID {}", guid),
            })
        }
    }
}
