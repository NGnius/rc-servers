use super::account_json::UserData;

pub enum TeamChooser {
    /// Alternating between team 0 and team 1
    Alternating,
    /// All players will be put on the specified team
    AllOn(u8),
    /// Each player will be put on their own team (like in Pit mode)
    OnePer,
}

impl TeamChooser {
    pub fn team(&self, index: usize) -> i32 {
        match self {
            Self::Alternating => (index % 2) as i32,
            Self::AllOn(team) => *team as i32,
            Self::OnePer => index as i32,
        }
    }
}

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

    async fn team_chooser(&self, game: &super::GameDescriptor) -> TeamChooser {
        match game.mode {
            crate::data::game_mode::GameMode::Pit => TeamChooser::OnePer,
            _ => TeamChooser::Alternating,
        }
    }

    async fn start_game(
        &self,
        game: super::GameDescriptor,
        players: Vec<super::PlayerLobbyDescriptor>,
        factory: &dyn oj_rc_factory::VehicleFactoryAdapter,
        cpu_counter: &crate::cubes::CpuListParser,
        weapon_lister: &crate::cubes::WeaponListParser,
        chooser: &TeamChooser,
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

        let fake_players = self.generate_fake_players_data(guid, &players, factory, cpu_counter, weapon_lister, chooser).await?;

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
        };
        let game_dbo = self.db.insert_game(game_dbo).await.map_err(|e| {
            log::error!("Failed to create game {} through user_id {}: {}", game.guid, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::LobbyReasonCode::UnexpectedError as i16,
                format!("Failed to create game {}: {}", game.guid, e),
            )
        })?;

        let players_len = players.len();

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
                    group: oj_rc_database::sea_orm::ActiveValue::Set(player.group),
                    is_claimed: oj_rc_database::sea_orm::ActiveValue::Set(false),
                    public_id: oj_rc_database::sea_orm::ActiveValue::Set(player.public_id),
                    display_name: oj_rc_database::sea_orm::ActiveValue::Set(player.display_name),
                    variant: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::multiplayer_game_player::ClientType::Client),
                }
            })
            .chain(fake_players.iter()
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
            .collect();
        self.db.insert_players(players).await.map_err(|e| {
            log::error!("Failed to create game players for {} through user_id {}: {}", game.guid, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::LobbyReasonCode::UnexpectedError as i16,
                format!("Failed to create game players for {}: {}", game.guid, e),
            )
        })?;

        Ok(super::FakePlayers { players: fake_players })
    }
}
