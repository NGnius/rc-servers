use super::account_json::UserData;

#[inline]
fn award_to_active_value(opt: Option<&i32>) -> oj_rc_database::sea_orm::ActiveValue<i32> {
    match opt {
        Some(t) => oj_rc_database::sea_orm::ActiveValue::Set(*t),
        None => oj_rc_database::sea_orm::ActiveValue::Set(0),
    }
}

#[async_trait::async_trait]
impl super::SingleplayerUser for UserData {
    async fn save_game_result(&self, guid: &str, result: crate::data::game_result::GameResult) -> Result<(), polariton_server::operations::SimpleOpError> {
        let now = chrono::Utc::now().timestamp();
        // TODO make this a single transaction
        let game = self.db.insert_game(oj_rc_database::schema::multiplayer_game::ActiveModel {
            id: oj_rc_database::sea_orm::ActiveValue::NotSet,
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
            guid: oj_rc_database::sea_orm::ActiveValue::Set(0),
            map: oj_rc_database::sea_orm::ActiveValue::Set(format!("singleplayer|guid:{}", guid)), // invalid by design
            mode: oj_rc_database::sea_orm::ActiveValue::Set(result.mode.to_db()),
            visibility: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::multiplayer_game::MapVisibility::Good),
            auto_heal: oj_rc_database::sea_orm::ActiveValue::Set(false),
            variant: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::multiplayer_game::GameType::Standard),
            is_complete: oj_rc_database::sea_orm::ActiveValue::Set(true),
        }).await
        .map_err(|e| {
            log::error!("Failed to create singleplayer game {} for user {}: {}", guid, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SingleplayerErrorCode::DatabaseError as i16,
                format!("Failed to create singleplayer game {}: {}", guid, e),
            )
        })?;
        let iter = result.winners.iter().map(|p| (0, p)).chain(result.losers.iter().map(|p| (1, p)));
        for (player_team, player_result) in iter {
            if player_result.player_name != self.account.display_name { continue; }
            let player = self.db.insert_player(oj_rc_database::schema::multiplayer_game_player::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                user_id: oj_rc_database::sea_orm::ActiveValue::Set(Some(self.account.id)),
                game_id: oj_rc_database::sea_orm::ActiveValue::Set(game.id),
                creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                player_id: oj_rc_database::sea_orm::ActiveValue::Set(0),
                team: oj_rc_database::sea_orm::ActiveValue::Set(player_team),
                group: oj_rc_database::sea_orm::ActiveValue::Set(None),
                is_claimed: oj_rc_database::sea_orm::ActiveValue::Set(true),
                public_id: oj_rc_database::sea_orm::ActiveValue::Set("".to_owned()), // no point in adding this info
                display_name: oj_rc_database::sea_orm::ActiveValue::Set("".to_owned()),
                variant: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::multiplayer_game_player::ClientType::Client),
            }).await
            .map_err(|e| {
                log::error!("Failed to create singleplayer game {} player for user {}: {}", guid, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SingleplayerErrorCode::DatabaseError as i16,
                    format!("Failed to create singleplayer game {} player: {}", guid, e),
                )
            })?;
            self.db.insert_score(oj_rc_database::schema::multiplayer_game_score::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                player_id: oj_rc_database::sea_orm::ActiveValue::Set(player.id),
                creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                is_claimed: oj_rc_database::sea_orm::ActiveValue::Set(false),
                kills: award_to_active_value(player_result.awards.get(&crate::data::game_result::PlayerAwardId::Kill)),
                deaths: oj_rc_database::sea_orm::ActiveValue::Set(0),
                assists: award_to_active_value(player_result.awards.get(&crate::data::game_result::PlayerAwardId::KillAssist)),
                heal_assists: award_to_active_value(player_result.awards.get(&crate::data::game_result::PlayerAwardId::HealAssist)),
                healed: award_to_active_value(player_result.awards.get(&crate::data::game_result::PlayerAwardId::HealCubes)),
                received_healed: oj_rc_database::sea_orm::ActiveValue::Set(0),
                damaged: award_to_active_value(player_result.awards.get(&crate::data::game_result::PlayerAwardId::DestroyedCubes)),
                received_damaged: oj_rc_database::sea_orm::ActiveValue::Set(0),
                crystals: oj_rc_database::sea_orm::ActiveValue::Set(0),
                total: award_to_active_value(player_result.awards.get(&crate::data::game_result::PlayerAwardId::Score)),
            }).await
            .map_err(|e| {
                log::error!("Failed to create singleplayer game {} score for user {}: {}", guid, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SingleplayerErrorCode::DatabaseError as i16,
                    format!("Failed to create singleplayer game {} score: {}", guid, e),
                )
            })?;
            break;
        }

        Ok(())
    }
}
