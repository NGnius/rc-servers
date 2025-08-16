use super::account_json::UserData;

impl UserData {
    pub(super) async fn generate_fake_players_data(&self, _guid: i64) -> Vec<crate::data::player_data::PlayerData> {
        vec![
            crate::data::player_data::PlayerData {
                name: "FakeUser".to_owned(),
                display_name: "Server".to_owned(),
                mastery: 1,
                tier: 1,
                robot_name: "Very bad but very good".to_owned(),
                robot_map: crate::persist::VALID_ROBOT.into(),
                group: None,
                team: 2,
                has_premium: true,
                robot_uuid: "1234_1234".to_owned(),
                cpu: 42,
                avatar_id: Some(0),
                weapon_order: vec![0,0,0],
                colour_map: crate::persist::VALID_COLOUR.into(),
                is_ai: false,
                spawn_effect: "Spawn".into(),
                death_effect: "Explosion".into(),
                player_rank: 1,
                weapon_rank: Default::default(),
            }
        ]
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
