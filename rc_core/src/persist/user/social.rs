use super::account_json::UserData;

impl UserData {
    async fn apply_friend_state_to(&self, public_id: String, state: oj_rc_database::schema::friend::FriendStatus) -> Result<bool, polariton_server::operations::SimpleOpError> {
        let target_user = self.db.user_by_public_id(public_id.clone()).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} to statify friend by user {}: {}", public_id, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user to statify friend: {}", e),
                )
            })?;
        if let Some(target_user) = target_user {
            self.db.update_friends_state(target_user.id, self.account.id, state).await
                .map_err(|e| {
                    log::error!("Failed to update friend state of user {} by user {}: {}", target_user.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to update friend state: {}", e),
                    )
                })?;
            Ok(true)
        } else {
            log::debug!("Cannot statify non-existent user {} friend request", public_id);
            Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::UserDoesNotExist as i16,
                format!("Failed to find user {} to statify friend", public_id),
            ))
        }
    }
}

#[async_trait::async_trait]
impl super::SocialUser for UserData {
    async fn accept_friend(&self, username: String) -> Result<bool, polariton_server::operations::SimpleOpError> {
        self.apply_friend_state_to(username, oj_rc_database::schema::friend::FriendStatus::Accepted).await
    }

    async fn decline_friend(&self, username: String) -> Result<bool, polariton_server::operations::SimpleOpError> {
        self.apply_friend_state_to(username, oj_rc_database::schema::friend::FriendStatus::Declined).await
    }

    async fn cancel_friend(&self, username: String) -> Result<bool, polariton_server::operations::SimpleOpError> {
        self.apply_friend_state_to(username, oj_rc_database::schema::friend::FriendStatus::Cancelled).await
    }

    async fn remove_friend(&self, username: String) -> Result<bool, polariton_server::operations::SimpleOpError> {
        self.apply_friend_state_to(username, oj_rc_database::schema::friend::FriendStatus::Removed).await
    }

    async fn list_friends(&self) -> Result<Vec<super::FriendData>, polariton_server::operations::SimpleOpError> {
        let friends = self.db.friends_by_user_id(self.account.id, oj_rc_database::schema::friend::FINAL_STATUSES).await
            .map_err(|e| {
                log::error!("Failed to retrieve friends for user {} : {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve friends: {}", e),
                )
            })?;
        let friend_ids = friends.iter().map(|(_, user)| user.id);
        let friend_avatars = self.db.user_auxs_by_user_ids_and_descriptor(friend_ids, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await
            .map_err(|e| {
                log::error!("Failed to retrieve friend avatars for user {} : {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve friend avatars: {}", e),
                )
            })?;
        let friend_avatar_map: std::collections::HashMap<i32, u32> = friend_avatars.iter()
            .filter_map(|avatar| avatar.data.parse().ok().map(|avatar_id| (avatar.user_id, avatar_id)))
            .collect();
        Ok(friends.into_iter()
            .map(|(friend, user)| super::FriendData {
                public_id: user.public_id,
                display_name: user.display_name,
                clan_name: None, // TODO clan
                state: super::FriendInviteStatus::from_db(friend.state),
                avatar_id: friend_avatar_map.get(&user.id).copied().unwrap_or(u32::MAX)
            })
            .collect()
        )
    }

    async fn list_social_info(&self, public_ids: &[String]) -> Result<Vec<super::SocialInfo>, polariton_server::operations::SimpleOpError> {
        let users = self.db.users_by_public_id(public_ids.iter()).await
            .map_err(|e| {
                log::error!("Failed to retrieve friend avatars for user {} : {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve friend avatars: {}", e),
                )
            })?;
        let user_ids = users.iter().map(|user| user.id);
        let user_avatars = self.db.user_auxs_by_user_ids_and_descriptor(user_ids, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await
            .map_err(|e| {
                log::error!("Failed to retrieve friend avatars for user {} : {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve friend avatars: {}", e),
                )
            })?;
        let avatar_map: std::collections::HashMap<i32, u32> = user_avatars.iter()
            .filter_map(|avatar| avatar.data.parse().ok().map(|avatar_id| (avatar.user_id, avatar_id)))
            .collect();
        Ok(users.iter()
            .map(|user| super::SocialInfo {
                public_id: user.public_id.clone(),
                display_name: user.display_name.clone(),
                avatar_id: avatar_map.get(&user.id).and_then(|&avatar_id| if avatar_id == u32::MAX { None } else { Some(avatar_id as i32) }),
            })
            .collect()
        )
    }

    async fn has_unclaimed_match_rewards(&self) -> Result<bool, polariton_server::operations::SimpleOpError> {
        let count = self.db.count_score_by_user_id_and_claimed(self.account.id, false).await
            .map_err(|e| {
                log::error!("Failed to count unclaimed matches for user {} : {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to count unclaimed matches: {}", e),
                )
            })?;
        Ok(count > 0)
    }

    async fn get_unclaimed_match_rewards(&self) -> Result<super::MatchRewards, polariton_server::operations::SimpleOpError> {
        let unclaimed_score_opt = self.db.score_by_user_id_and_claimed_oldest(self.account.id, false).await
            .map_err(|e| {
                log::error!("Failed to retrieve unclaimed player score for user {} : {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve unclaimed player score: {}", e),
                )
            })?;

        if let Some(unclaimed_score) = unclaimed_score_opt {
            // TODO calculate these values
            return Ok(super::MatchRewards {
                season_experience: unclaimed_score.total,
                experience_award_base: unclaimed_score.total,
                experience_award_premium: unclaimed_score.total, // FIXME actually figure out if player has premium
                experience_award_party: 0,
                experience_award_tier: 0,
                robits_total: unclaimed_score.total,
                average_experience: unclaimed_score.total,
                clan_experience: unclaimed_score.total,
                robits_earned: unclaimed_score.total,
                premium_robits_earned: unclaimed_score.total,
            });
        }
        log::warn!("No unclaimed match rewards found for user {}", self.account.id);
        Err(polariton_server::operations::SimpleOpError::with_message(
            crate::data::error_codes::SocialErrorCode::UnexpectedError as i16,
            "No remaining unclaimed match rewards for current user".to_owned(),
        ))
    }

    async fn claim_match_rewards(&self) -> Result<bool, polariton_server::operations::SimpleOpError> {
        let unclaimed_score_opt = self.db.score_by_user_id_and_claimed_oldest(self.account.id, false).await
            .map_err(|e| {
                log::error!("Failed to retrieve to-be-claimed player score for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve unclaimed player score: {}", e),
                )
            })?;
        if let Some(unclaimed_score) = unclaimed_score_opt {
            self.db.score_claim(unclaimed_score.id).await
                .map_err(|e| {
                    log::error!("Failed to claim player score {} for user {} : {}", unclaimed_score.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to claim player score: {}", e),
                    )
                })?;
            // TODO calculate currency rewards
            let reward = unclaimed_score.total as u64;
            self.currency_op(super::CurrencyType::Free, super::CurrencyOp::Add(reward)).await
                .map_err(|e| {
                    log::error!("Failed to save currency reward for user {}: {}", self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to save currency reward: {}", e),
                    )
                })?;
            let experience = unclaimed_score.total as u64 * 4;
            self.currency_op(super::CurrencyType::Experience, super::CurrencyOp::Add(experience)).await
                .map_err(|e| {
                    log::error!("Failed to save experience reward for user {}: {}", self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to save experience reward: {}", e),
                    )
                })?;
            Ok(self.db.count_score_by_user_id_and_claimed(self.account.id, false).await.is_ok_and(|x| x > 0))
        } else {
            Ok(false)
        }
    }
}

#[async_trait::async_trait]
impl <C> super::SocialUserC<C> for UserData {
    async fn invite_friend(&self, username: String) -> Result<super::FriendInviteReturn<C>, polariton_server::operations::SimpleOpError> {
        let target_user = self.db.user_by_some_social_id(username.clone()).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} by user {}: {}", username, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user: {}", e),
                )
            })?;
        if let Some(target_user) = target_user {
            let now = chrono::Utc::now();
            self.db.insert_friends([
                // inviter -> invitee
                oj_rc_database::schema::friend::ActiveModel {
                    id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                    creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now.timestamp()),
                    friend_source: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
                    friend_target: oj_rc_database::sea_orm::ActiveValue::Set(target_user.id),
                    state: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::friend::FriendStatus::InviteSent),
                },
                // invitee -> inviter (reciprocal)
                oj_rc_database::schema::friend::ActiveModel {
                    id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                    creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now.timestamp()),
                    friend_source: oj_rc_database::sea_orm::ActiveValue::Set(target_user.id),
                    friend_target: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
                    state: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::friend::FriendStatus::InvitePending),
                },
            ]).await.map_err(|e| {
                log::error!("Failed to retrieve user {} to send friend request by user {}: {}", username, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user to send friend request: {}", e),
                )
            })?;
            let my_user_avatar_aux = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await.map_err(|e| {
                log::error!("Failed to retrieve avatar for user {} (invite_friend): {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Could not retrieve avatar: {}", e),
                )
            })?
            .ok_or_else(|| {
                log::error!("Failed to find avatar for user {}", self.account.id);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::UnexpectedError as i16,
                    format!("No avatar for user {}", self.account.id),
                )
            })?;
            let target_user_avatar_aux = self.db.user_aux_by_user_id_and_descriptor(target_user.id, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await.map_err(|e| {
                log::error!("Failed to retrieve avatar for user {} (invite_friend): {}", target_user.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Could not retrieve avatar: {}", e),
                )
            })?
            .ok_or_else(|| {
                log::error!("Failed to find avatar for user {} by user {}", target_user.id, self.account.id);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::UnexpectedError as i16,
                    format!("No avatar for user {}", target_user.id),
                )
            })?;
            let my_avatar_id: Result<u32, _> = my_user_avatar_aux.data.parse();
            let target_avatar_id: Result<i32, _> = target_user_avatar_aux.data.parse();
            Ok(super::FriendInviteReturn {
                target_public_id: target_user.public_id,
                target_display_name: target_user.display_name,
                my_clan_name: None, // TODO clan
                target_clan_name: None, // TODO clan
                my_avatar_id: my_avatar_id.unwrap_or(0),
                target_player: polariton::operation::Typed::HashMap(vec![
                    (polariton::operation::Typed::Str("useCustomAvatar".into()), polariton::operation::Typed::Bool(target_avatar_id.is_err())),
                    (polariton::operation::Typed::Str("avatarId".into()), polariton::operation::Typed::Int(target_avatar_id.unwrap_or_default())),
                ].into()),
            })
        } else {
            log::debug!("Cannot invite non-existent user {}", username);
            Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::UserDoesNotExist as i16,
                format!("Failed to find user {}", username),
            ))
        }
    }
}
