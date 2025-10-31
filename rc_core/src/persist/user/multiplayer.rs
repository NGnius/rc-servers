use super::account_json::UserData;

fn db_to_impl(client_emu: &oj_rc_database::schema::multiplayer_game_player::ClientType) -> Option<crate::persist::config::ClientEmulator> {
    match client_emu {
        oj_rc_database::schema::multiplayer_game_player::ClientType::ServerExperimental => Some(crate::persist::config::ClientEmulator::Experiment),
        oj_rc_database::schema::multiplayer_game_player::ClientType::ClientAI => Some(crate::persist::config::ClientEmulator::ClientAI),
        oj_rc_database::schema::multiplayer_game_player::ClientType::Client => None,
    }
}

impl UserData {
    pub(super) async fn generate_fake_players_data(
        &self,
        _guid: i64,
        real_players: &Vec<super::PlayerLobbyDescriptor>,
        factory: &dyn oj_rc_factory::VehicleFactoryAdapter,
        cpu_counter: &crate::cubes::CpuListParser,
        weapon_lister: &crate::cubes::WeaponListParser,
        chooser: &super::TeamChooser,
    ) -> Result<Vec<(crate::data::player_data::PlayerData, crate::persist::config::ClientEmulator)>, polariton_server::operations::SimpleOpError> {
        let mut fakes = Vec::with_capacity(self.fake_players.len());
        let mut fake_i = real_players.len();
        for fake in self.fake_players.iter() {
            let vehicle = self.resolve_vehicle(&fake.vehicle, factory, weapon_lister, cpu_counter).await?;
            let out = (
                crate::data::player_data::PlayerData {
                    name: fake.vehicle.username.clone(),
                    display_name: fake.vehicle.username.clone(),
                    mastery: vehicle.mastery,
                    tier: vehicle.mastery,
                    robot_name: vehicle.robot_name,
                    robot_map: vehicle.robot_map,
                    group: None,
                    team: fake.team.map(|t| t as i32)
                        .unwrap_or_else(|| {
                            let assigned_team = chooser.team(fake_i);
                            fake_i += 1;
                            assigned_team
                        }),
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
                },
                fake.implementation
            );
            fakes.push(out);
        }
        Ok(fakes)
    }
}

#[async_trait::async_trait]
impl super::MultiplayerUser for UserData {
    fn user_id(&self) -> i32 {
        self.account.id
    }

    fn user_name(&self) -> &'_ str {
        &self.account.public_id
    }

    fn display_name(&self) -> &'_ str {
        &self.account.display_name
    }

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
            }))
        } else {
            Err(super::MultiplayerError {
                code: super::MultiplayerErrorCode::IncorrectGameGuid,
                message: format!("Failed to parse game GUID {}", guid),
            })
        }
    }
}
