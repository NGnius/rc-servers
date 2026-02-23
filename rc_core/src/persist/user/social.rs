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

    async fn clan_members_of_clan(&self, clan_id: i32) -> Result<Vec<super::ClanMember>, polariton_server::operations::SimpleOpError> {
        let members = self.db.clan_members_by_clan_id(clan_id).await
            .map_err(|e| {
                    log::error!("Failed to retrieve members of clan {} by user {}: {}", clan_id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve members of clan: {}", e),
                    )
                })?;
        let avatar_infos = self.db.user_auxs_by_user_ids_and_descriptor(members.iter().map(|(m, _)| m.user_id), oj_rc_database::schema::user_aux::Descriptor::AvatarId).await
            .map_err(|e| {
                log::error!("Failed to retrieve clan {} member avatars by user {}: {}", clan_id, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve clan member avatars: {}", e),
                )
            })?;
        let avatar_infos: std::collections::HashMap<i32, Option<i32>> = avatar_infos.into_iter()
            .map(|aux| (aux.user_id, aux.data.parse().ok()))
            .collect();
        Ok(members.into_iter()
            .map(|(member, user)| super::ClanMember {
                public_id: user.public_id,
                display_name: user.display_name,
                is_confirmed: matches!(member.status, oj_rc_database::schema::clan_member::ClanMemberStatus::Confirmed),
                avatar_id: avatar_infos.get(&user.id).copied().flatten(),
                rank: super::ClanMemberRank::db_to_core(&member.rank),
                season_xp: 0, // TODO clan seasons
            })
            .collect()
        )
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

    async fn my_clan_info(&self, include_members: bool) -> Result<Option<(super::ClanData, Vec<super::ClanMember>)>, polariton_server::operations::SimpleOpError> {
        let clan_opt = self.db.clan_by_user_id(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve clan for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve clan: {}", e),
                )
            })?;
        if let Some((clan, _member)) = clan_opt {
            if include_members {
                let members = self.clan_members_of_clan(clan.id).await?;
                Ok(Some((
                    super::ClanData {
                        name: clan.name,
                        description: clan.description,
                        ty: super::ClanType::db_to_core(&clan.variant),
                        size: members.iter().filter(|x| x.is_confirmed).count() as _,
                    },
                    members,
                )))
            } else {
                Ok(Some((
                    super::ClanData {
                        name: clan.name,
                        description: clan.description,
                        ty: super::ClanType::db_to_core(&clan.variant),
                        size: 0,
                    },
                    Vec::default(),
                )))
            }
        } else {
            Ok(None)
        }
    }

    async fn clan_info(&self, clan_name: &str) -> Result<Option<(super::ClanData, Vec<super::ClanMember>)>, polariton_server::operations::SimpleOpError> {
        let clan_opt = self.db.clan_by_name(clan_name.to_owned()).await
            .map_err(|e| {
                log::error!("Failed to retrieve clan {} for user {}: {}", clan_name, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve clan {}: {}", clan_name, e),
                )
            })?;
        if let Some(clan) = clan_opt {
            let members = self.clan_members_of_clan(clan.id).await?;
            Ok(Some((
                super::ClanData {
                    name: clan.name,
                    description: clan.description,
                    ty: super::ClanType::db_to_core(&clan.variant),
                    size: members.iter().filter(|x| x.is_confirmed).count() as _,
                },
                members,
            )))
        } else {
            Ok(None)
        }
    }

    async fn search_clan(&self, search: super::ClanSearchQuery)-> Result<Vec<super::ClanData>, polariton_server::operations::SimpleOpError> {
        // TODO support days since active (if it's actually used by client)
        let clan_results = self.db.clans_by_search(
            search.search_string,
            search.start_range as u64,
            search.end_range as u64,
            search.types.into_iter().map(super::ClanType::core_to_db)
        ).await
            .map_err(|e| {
                log::error!("Failed to retrieve clans for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve clans: {}", e),
                )
            })?;
        // FIXME retrieve all member counts in one database call
        let mut member_counts = std::collections::HashMap::with_capacity(clan_results.len());
        for clan in clan_results.iter() {
            let member_count = self.db.count_clan_members_by_clan_id(clan.id).await
                .map_err(|e| {
                    log::error!("Failed to retrieve clan {} member count for user {}: {}", clan.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve clan {} member count: {}", clan.id, e),
                    )
                })?;
            member_counts.insert(clan.id, member_count);
        }
        Ok(clan_results.into_iter()
            .map(|clan| super::ClanData {
                name: clan.name,
                description: clan.description,
                ty: super::ClanType::db_to_core(&clan.variant),
                size: member_counts.get(&clan.id).map(|x| *x as i32).unwrap_or_default(),
            })
            .collect()
        )
    }

    async fn create_clan(&self, clan: super::ClanData, avatar: Vec<u8>)-> Result<Vec<super::ClanMember>, polariton_server::operations::SimpleOpError> {
        // TODO make this a transaction
        let existing_clan = self.db.clan_by_name(clan.name.clone()).await
            .map_err(|e| {
                log::error!("Failed to do exist check on clan {} for user {}: {}", clan.name, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to do exist check on clan {}: {}", clan.name, e),
                )
            })?;
        if existing_clan.is_some() {
            return Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::ClanAlreadyExists as i16,
                format!("Clan with name \"{}\" is already in database", clan.name),
            ));
        }
        let now = chrono::Utc::now().timestamp();
        let new_clan = oj_rc_database::schema::clan::ActiveModel {
            id: oj_rc_database::sea_orm::ActiveValue::NotSet,
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
            name: oj_rc_database::sea_orm::ActiveValue::Set(clan.name.clone()),
            description: oj_rc_database::sea_orm::ActiveValue::Set(clan.description),
            variant: oj_rc_database::sea_orm::ActiveValue::Set(clan.ty.core_to_db()),
        };
        let new_clan = self.db.insert_clan(new_clan).await
            .map_err(|e| {
                log::error!("Failed to save new clan for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to save new clan: {}", e),
                )
            })?;
        let first_member = oj_rc_database::schema::clan_member::ActiveModel {
            id: oj_rc_database::sea_orm::ActiveValue::NotSet,
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
            user_id: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
            clan_id: oj_rc_database::sea_orm::ActiveValue::Set(new_clan.id),
            rank: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberRank::Leader),
            status: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberStatus::Confirmed),
        };
        self.db.insert_clan_member(first_member).await
            .map_err(|e| {
                log::error!("Failed to save first clan {} member of user {}: {}", new_clan.id, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to save first clan {} member: {}", new_clan.id, e),
                )
            })?;
        let avatar_info: Option<i32> = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await
            .map_err(|e| {
                log::error!("Failed to retrieve avatar info of user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve avatar info: {}", e),
                )
            })?
            .map(|x| x.data.parse().ok()).flatten();

        if !avatar.is_empty() {
            self.save_clan_avatar(avatar, &clan.name).await?;
        }

        Ok(vec![
            super::ClanMember {
                public_id: self.account.public_id.clone(),
                display_name: self.account.display_name.clone(),
                is_confirmed: true,
                avatar_id: avatar_info,
                rank: super::ClanMemberRank::Leader,
                season_xp: 0,
            }
        ])
    }

    async fn join_clan(&self, clan_name: &str) -> Result<(super::ClanData, Vec<super::ClanMember>), polariton_server::operations::SimpleOpError> {
        let current_clan = self.db.clan_by_user_id(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve clan for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve clan: {}", e),
                )
            })?;
        if current_clan.is_some() {
            log::debug!("User {} cannot join a clan; they are already in one", self.account.id);
            return Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::AlreadyInClan as i16,
                format!("User {} is already in clan", self.account.id),
            ));
        }
        let existing_clan = self.db.clan_by_name(clan_name.to_owned()).await
            .map_err(|e| {
                log::error!("Failed to retrieve existing clan {} for user {}: {}", clan_name, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve existing clan {}: {}", clan_name, e),
                )
            })?;
        if let Some(clan) = existing_clan {
            // pre-join checks
            let invited_clan_opt = self.db.clan_invited_to_for_user_id_and_clan_id(self.account.id, clan.id).await
                .map_err(|e| {
                    log::error!("Failed to retrieve clan invites of user {}: {}", self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to save new clan invites: {}", e),
                    )
                })?;
            match clan.variant {
                oj_rc_database::schema::clan::ClanType::Public => {
                    // nothing
                },
                oj_rc_database::schema::clan::ClanType::Private => {
                    let is_invited = invited_clan_opt.is_some();
                    if !is_invited {
                        log::debug!("User {} tried to join clan {} without being invited", self.account.id, clan.id);
                        return Err(polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::SocialErrorCode::ClanClosed as i16,
                            format!("Clan \"{}\" is private and user {} is not invited", clan.name, self.account.id),
                        ));
                    }
                },
                oj_rc_database::schema::clan::ClanType::Abandoned => {
                    log::debug!("User {} tried to join abandoned clan {}", self.account.id, clan.id);
                    return Err(polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::ClanClosed as i16,
                        format!("Clan \"{}\" is abandoned", clan.name),
                    ));
                },
                oj_rc_database::schema::clan::ClanType::Banned => {
                    log::debug!("User {} tried to join banned clan {}", self.account.id, clan.id);
                    return Err(polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::ClanClosed as i16,
                        format!("Clan \"{}\" is banned", clan.name),
                    ));
                },
            }
            if let Some((_invited_clan, invited_member)) = invited_clan_opt {
                self.db.update_clan_member(oj_rc_database::schema::clan_member::ActiveModel {
                    id: oj_rc_database::sea_orm::ActiveValue::Set(invited_member.id),
                    status: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberStatus::Confirmed),
                    ..Default::default()
                }).await
                    .map_err(|e| {
                        log::error!("Failed to update invited clan {} member of user {}: {}", clan.id, self.account.id, e);
                        polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                            format!("Failed to update invited clan {} member: {}", clan.id, e),
                        )
                    })?;
            } else {
                let now = chrono::Utc::now().timestamp();
                let new_member = oj_rc_database::schema::clan_member::ActiveModel {
                    id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                    creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                    user_id: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
                    clan_id: oj_rc_database::sea_orm::ActiveValue::Set(clan.id),
                    rank: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberRank::Member),
                    status: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberStatus::Confirmed),
                };
                self.db.insert_clan_member(new_member).await
                    .map_err(|e| {
                        log::error!("Failed to save new clan {} member of user {}: {}", clan.id, self.account.id, e);
                        polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                            format!("Failed to save new clan {} member: {}", clan.id, e),
                        )
                    })?;
            }
            self.db.update_clan_member_decline_all_invites(self.account.id).await
                .map_err(|e| {
                    log::error!("Failed to decline clan invites for user {}: {}", self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to decline clan invites: {}", e),
                    )
                })?;
            let members = self.clan_members_of_clan(clan.id).await?;
            Ok((
                super::ClanData {
                    name: clan.name,
                    description: clan.description,
                    ty: super::ClanType::db_to_core(&clan.variant),
                    size: members.iter().filter(|x| x.is_confirmed).count() as _,
                },
                members,
            ))
        } else {
            log::warn!("Failed to find clan with name \"{}\" in database", clan_name);
            return Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::ClanNotFound as i16,
                format!("Clan with name \"{}\" is not in database", clan_name),
            ));
        }
    }


    async fn leave_clan(&self) -> Result<bool, polariton_server::operations::SimpleOpError> {
        let joined_clan_opt = self.db.clan_by_user_id(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve clan of user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve clan of user: {}", e),
                )
            })?;
        if let Some((joined_clan, member)) = joined_clan_opt {
            let is_leader = matches!(member.rank, oj_rc_database::schema::clan_member::ClanMemberRank::Leader);
            if is_leader {
                log::warn!("User {} tried to leave clan {} that they lead", self.account.id, joined_clan.id);
                return Err(polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::NotClanLeader as i16,
                    format!("Failed to leave clan: you are the leader"),
                ));
            }
            let new_model = oj_rc_database::schema::clan_member::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::Set(member.id),
                status: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberStatus::Deactivated),
                ..Default::default()
            };
            self.db.update_clan_member(new_model).await
                .map_err(|e| {
                    log::error!("Failed to update clan {} member for user {}: {}", joined_clan.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve update clan member for user: {}", e),
                    )
                })?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn remove_user_from_clan(&self, public_id: &str) -> Result<bool, polariton_server::operations::SimpleOpError> {
        let joined_clan_opt = self.db.clan_by_user_id(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve clan of user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve clan of user: {}", e),
                )
            })?;
        if let Some((joined_clan, member)) = joined_clan_opt {
            let is_leader = matches!(member.rank, oj_rc_database::schema::clan_member::ClanMemberRank::Leader);
            let is_officer = matches!(member.rank, oj_rc_database::schema::clan_member::ClanMemberRank::Officer);
            if !(is_leader || is_officer) {
                log::warn!("User {} tried to remove user {} from clan {} without permission", self.account.id, public_id, joined_clan.id);
                return Err(polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::ClanRankTooLow as i16,
                    format!("Failed to remove user from clan: no permission"),
                ));
            }
            let members = self.db.clan_members_by_clan_id(joined_clan.id).await
                .map_err(|e| {
                    log::error!("Failed to retrieve joined clan {} members for user {}: {}", joined_clan.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve joined clan {} members for user: {}", joined_clan.id, e),
                    )
                })?;
            let target_member_opt = members.iter()
                .find(|(_mem, user)| user.public_id == public_id);
            if let Some((target_member, target_user)) = target_member_opt {
                let can_kick = match target_member.rank {
                    oj_rc_database::schema::clan_member::ClanMemberRank::Member => true, // already guaranteed to be leader OR officer
                    oj_rc_database::schema::clan_member::ClanMemberRank::Officer => is_leader,
                    oj_rc_database::schema::clan_member::ClanMemberRank::Leader => false,
                };
                if !can_kick {
                    log::warn!("User {} tried to remove user {} from clan {} with insufficient permission", self.account.id, target_user.id, joined_clan.id);
                    return Err(polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::ClanRankTooLow as i16,
                        format!("Failed to remove user from clan: insufficient permission"),
                    ));
                }
                let new_model = oj_rc_database::schema::clan_member::ActiveModel {
                    id: oj_rc_database::sea_orm::ActiveValue::Set(target_member.id),
                    status: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberStatus::Deactivated),
                    ..Default::default()
                };
                self.db.update_clan_member(new_model).await
                    .map_err(|e| {
                        log::error!("Failed to update clan {} member for user {}: {}", joined_clan.id, self.account.id, e);
                        polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                            format!("Failed to retrieve update clan member for user: {}", e),
                        )
                    })?;
                Ok(true)
            } else {
                log::warn!("User {} tried to remove non-existent user {} from clan {}", self.account.id, public_id, joined_clan.id);
                return Err(polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::UserNotFoundInClan as i16,
                    format!("Failed to remove user from clan: user not in clan"),
                ));
            }
        } else {
            log::debug!("User {} (not in a clan) tried to remove user {} from a clan", self.account.id, public_id);
            Ok(false)
        }
    }

    async fn update_clan(&self, name: Option<String>, description: Option<String>, ty: Option<super::ClanType>, avatar: Option<Vec<u8>>) -> Result<Vec<super::ClanMember>, polariton_server::operations::SimpleOpError> {
        let joined_clan_opt = self.db.clan_by_user_id(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve clan of user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve clan of user: {}", e),
                )
            })?;
        if let Some((joined_clan, member)) = joined_clan_opt {
            let is_leader = matches!(member.rank, oj_rc_database::schema::clan_member::ClanMemberRank::Leader);
            if !is_leader {
                log::debug!("User {} is not leader but tried to modify clan {}", self.account.id, joined_clan.id);
                return Err(polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::NotClanLeader as i16,
                    format!("Failed to modify clan: user is not the leader"),
                ));
            }
            let new_clan = oj_rc_database::schema::clan::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::Set(joined_clan.id),
                creation_time: oj_rc_database::sea_orm::ActiveValue::NotSet,
                name: if let Some(name) = name { oj_rc_database::sea_orm::ActiveValue::Set(name) } else { oj_rc_database::sea_orm::ActiveValue::NotSet },
                description: if let Some(description) = description { oj_rc_database::sea_orm::ActiveValue::Set(description) } else { oj_rc_database::sea_orm::ActiveValue::NotSet },
                variant: if let Some(ty) = ty { oj_rc_database::sea_orm::ActiveValue::Set(ty.core_to_db()) } else { oj_rc_database::sea_orm::ActiveValue::NotSet },
            };
            self.db.update_clan(new_clan).await
                .map_err(|e| {
                    log::error!("Failed to update clan {} for user {}: {}", joined_clan.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to update clan for user: {}", e),
                    )
                })?;
            if let Some(new_avatar) = avatar {
                self.save_clan_avatar(new_avatar, &joined_clan.name).await?;
            }
            self.clan_members_of_clan(joined_clan.id).await
        } else {
            log::debug!("User {} is not in a clan but tried to modify clan", self.account.id);
            return Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::UserNotInClan as i16,
                format!("Failed to modify clan for user not in clan"),
            ));
        }
    }

    async fn invite_to_clan(&self, public_id: &str) -> Result<super::ClanMember, polariton_server::operations::SimpleOpError> {
        // TODO make this a transaction
        let my_clan_opt = self.db.clan_by_user_id(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} to invite to clan for user {}: {}", public_id, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user to invite to clan for user: {}", e),
                )
            })?;
        if let Some((my_clan, my_member)) = my_clan_opt {
            if matches!(my_member.rank, oj_rc_database::schema::clan_member::ClanMemberRank::Member) {
                log::debug!("User {} cannot invite to clan {} (rank is member)", self.account.id, my_clan.id);
                return Err(polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::ClanRankTooLow as i16,
                    "User cannot invite to clan (rank is member)".to_owned(),
                ));
            }
            let target_user = self.db.user_by_public_id(public_id.to_owned()).await
                .map_err(|e| {
                    log::error!("Failed to retrieve user {} to invite to clan for user {}: {}", public_id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve user to invite to clan for user: {}", e),
                    )
                })?
                .ok_or_else(|| {
                    log::debug!("Cannot find user {} to invite to clan {} by user {}", public_id, my_clan.id, self.account.id);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::UserDoesNotExist as i16,
                        "Failed to find user to invite to clan by user".to_owned(),
                    )
                })?;
            let invite_opt = self.db.clan_invited_to_for_user_id_and_clan_id(target_user.id, my_clan.id).await
                .map_err(|e| {
                    log::error!("Failed to retrieve user {} clan invite by user {}: {}", target_user.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve user clan invite by user: {}", e),
                    )
                })?;
            if invite_opt.is_some() {
                log::debug!("User {} is already invited to clan {} by user {}", public_id, my_clan.id, self.account.id);
                return Err(polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::AlreadyInvited as i16,
                    "User is already invited to clan".to_owned(),
                ));
            }
            let target_user_clan_opt = self.db.clan_by_user_id(target_user.id).await
                .map_err(|e| {
                    log::error!("Failed to retrieve user {} clan by user {}: {}", target_user.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve user clan by user: {}", e),
                    )
                })?;
            if let Some((target_user_clan, _target_user_member)) = target_user_clan_opt {
                log::debug!("User {} is already in clan {}; cannot do invite by user {}", target_user.id, target_user_clan.id, self.account.id);
                return Err(polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::AlreadyInClan as i16,
                    "User is already in clan".to_owned(),
                ));
            }
            let now = chrono::Utc::now().timestamp();
            let new_invite = oj_rc_database::schema::clan_member::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                user_id: oj_rc_database::sea_orm::ActiveValue::Set(target_user.id),
                clan_id: oj_rc_database::sea_orm::ActiveValue::Set(my_clan.id),
                rank: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberRank::Member),
                status: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberStatus::Invited),
            };
            self.db.insert_clan_member(new_invite).await
                .map_err(|e| {
                    log::error!("Failed to retrieve user {} clan by user {}: {}", target_user.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve user clan by user: {}", e),
                    )
                })?;
            let avatar_id = self.db.user_aux_by_user_id_and_descriptor(target_user.id, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await
                .map_err(|e| {
                    log::error!("Failed to retrieve user {} avatar info by user {}: {}", target_user.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve user avatar info by user: {}", e),
                    )
                })?
                .and_then(|x| x.data.parse().ok());
            Ok(super::ClanMember {
                public_id: target_user.public_id,
                display_name: target_user.display_name,
                is_confirmed: false,
                avatar_id: avatar_id,
                rank: super::ClanMemberRank::Member,
                season_xp: 0,
            })
        } else {
            log::debug!("User {} tried to invite user {} to their non-existent clan", self.account.id, public_id);
            Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::UserNotInClan as i16,
                format!("Failed to invite user to clan for user: invitee is not in clan"),
            ))
        }
    }

    async fn my_clan_invites(&self) -> Result<Vec<super::ClanInviteData>, polariton_server::operations::SimpleOpError> {
        let invites = self.db.clans_invited_to_for_user_id(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} clan invites: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user clan invites: {}", e),
                )
            })?;
        let clan_ids = invites.iter().map(|(clan, _invite)| clan.id);
        let clan_leaders = self.db.clan_leaders_by_clan_ids(clan_ids).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} clan invite leaders: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user clan invite leaders: {}", e),
                )
            })?;
        let clan_leaders: std::collections::HashMap::<i32, oj_rc_database::schema::user::Model> = clan_leaders.into_iter()
            .map(|(member, user)| (member.clan_id, user))
            .collect();
        let clan_leader_ids = clan_leaders.values().map(|user| user.id);
        let clan_leaders_avatars = self.db.user_auxs_by_user_ids_and_descriptor(
            clan_leader_ids,
            oj_rc_database::schema::user_aux::Descriptor::AvatarId
        ).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} clan invite leaders' avatars: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user clan invite leaders' avatar: {}", e),
                )
            })?;
        let clan_leaders_avatars: std::collections::HashMap::<i32, Option<i32>> = clan_leaders_avatars.into_iter()
            .map(|avatar| (avatar.user_id, avatar.data.parse().ok()))
            .collect();
        let mut clan_invites = Vec::with_capacity(invites.len());
        for (invite_clan, _invite_member) in invites {
            if let Some(invitee) = clan_leaders.get(&invite_clan.id) {
                clan_invites.push(super::ClanInviteData {
                    public_id: invitee.public_id.clone(),
                    display_name: invitee.display_name.clone(),
                    avatar_id: clan_leaders_avatars.get(&invitee.id).copied().flatten(),
                    clan_name: invite_clan.name,
                    clan_description: invite_clan.description,
                    size: 0, // TODO
                });
            }
        }
        Ok(clan_invites)
    }

    async fn decline_clan_invite(&self, clan_name: &str) -> Result<Vec<super::ClanMember>, polariton_server::operations::SimpleOpError> {
        let clan_invite_opt = self.db.clan_invited_to_for_user_id_and_clan_name(self.account.id, clan_name.to_owned()).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} clan {} invite: {}", self.account.id, clan_name, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user clan invite: {}", e),
                )
            })?;
        if let Some((clan, invite)) = clan_invite_opt {
            let to_update = oj_rc_database::schema::clan_member::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::Set(invite.id),
                status: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberStatus::Deactivated),
                ..Default::default()
            };
            self.db.update_clan_member(to_update).await
                .map_err(|e| {
                log::error!("Failed to update user {} clan {} invite {} to declined: {}", self.account.id, clan.id, invite.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to update user clan invite to declined: {}", e),
                )
            })?;
            self.clan_members_of_clan(clan.id).await
        } else {
            log::debug!("User {} cannot decline non-existent invite to clan {}", self.account.id, clan_name);
            Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::NoInvite as i16,
                format!("User cannot decline non-existent clan invite"),
            ))
        }
    }

    async fn decline_all_clan_invites(&self) -> Result<bool, polariton_server::operations::SimpleOpError> {
        self.db.update_clan_member_decline_all_invites(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to update all clan invites to decline for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to update all clan invites to decline for user: {}", e),
                )
            })?;
        Ok(true)
    }

    async fn cancel_invite_to_clan(&self, public_id: &str) -> Result<(super::ClanData, Vec<super::ClanMember>), polariton_server::operations::SimpleOpError> {
        let user_opt = self.db.user_by_public_id(public_id.to_owned()).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} to cancel invite by user {}: {}", public_id, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user to cancel invite by user: {}", e),
                )
            })?;
        if let Some(user) = user_opt {
            let (my_clan, _my_member) = self.db.clan_by_user_id(self.account.id).await
                .map_err(|e| {
                    log::error!("Failed to retrieve user {} clan to cancel invite: {}", self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve user clan to cancel invite by user: {}", e),
                    )
                })?
                .ok_or_else(|| {
                    log::debug!("Cannot find user {} clan to cancel invite", self.account.id);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::UserNotInClan as i16,
                        "Cannot find user clan to cancel invite by user".to_owned(),
                    )
                })?;
            let (_clan, invite) = self.db.clan_invited_to_for_user_id_and_clan_id(user.id, my_clan.id).await
                .map_err(|e| {
                    log::error!("Failed to retrieve user {} clan invite to cancel invite for user {}: {}", user.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to retrieve user clan invite to cancel invite by user: {}", e),
                    )
                })?
                .ok_or_else(|| {
                    log::debug!("Cannot find user {} clan invite to cancel invite for user {}", user.id, self.account.id);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::NoInvite as i16,
                        "Cannot find user clan to cancel invite by user".to_owned(),
                    )
                })?;
            let new_invite = oj_rc_database::schema::clan_member::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::Set(invite.id),
                status: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberStatus::Deactivated),
                ..Default::default()
            };
            self.db.update_clan_member(new_invite).await
                .map_err(|e| {
                    log::error!("Failed to update user {} clan invite to cancel invite for user {}: {}", user.id, self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to udpate user clan invite to cancel invite by user: {}", e),
                    )
                })?;
            let members = self.clan_members_of_clan(my_clan.id).await?;
            Ok((
                super::ClanData {
                    name: my_clan.name,
                    description: my_clan.description,
                    ty: super::ClanType::db_to_core(&my_clan.variant),
                    size: members.len() as _,
                },
                members,
            ))
        } else {
            log::debug!("Cannot find user {} to cancel invite by user {}", public_id, self.account.id);
            Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::UserDoesNotExist as i16,
                "Cannot find user to cancel invite by user".to_owned(),
            ))
        }
    }

    async fn update_clan_member(&self, public_id: &str, rank: super::ClanMemberRank) -> Result<Vec<super::ClanMember>, polariton_server::operations::SimpleOpError> {
        let user = self.db.user_by_public_id(public_id.to_owned()).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} to update clan member for user {}: {}", public_id, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user to update clan member for user: {}", e),
                )
            })?
            .ok_or_else(|| {
                log::debug!("Cannot find user {} to update clan member for user {}", public_id, self.account.id);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::UserDoesNotExist as i16,
                    "Cannot find user to update clan member for user".to_owned(),
                )
            })?;
        let (my_clan, my_member) = self.db.clan_by_user_id(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} clan to update clan member: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user clan to clan member: {}", e),
                )
            })?
            .ok_or_else(|| {
                log::debug!("Cannot find user {} clan to update clan member", self.account.id);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::UserNotInClan as i16,
                    "Cannot find user clan to udpate clan member".to_owned(),
                )
            })?;
        let is_leader = matches!(my_member.rank, oj_rc_database::schema::clan_member::ClanMemberRank::Leader);
        let is_officer = matches!(my_member.rank, oj_rc_database::schema::clan_member::ClanMemberRank::Officer);
        let can_update = match rank {
            super::ClanMemberRank::Member => is_leader,
            super::ClanMemberRank::Officer => is_officer || is_leader,
            super::ClanMemberRank::Leader => is_leader,
        };
        if !can_update {
            log::debug!("User {} does not have enough permissions to update clan member rank of user {}", self.account.id, user.id);
            return Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::ClanRankTooLow as i16,
                "User does not have enough permissions to update clan member rank of user".to_owned(),
            ));
        }
        let (target_clan, target_member) = self.db.clan_by_user_id(user.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve user {} clan to update clan member for user {}: {}", user.id, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to retrieve user clan to clan member: {}", e),
                )
            })?
            .ok_or_else(|| {
                log::debug!("Cannot find user {} clan to update clan member for user {}", user.id, self.account.id);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::UserNotInClan as i16,
                    "Cannot find user clan to udpate clan member".to_owned(),
                )
            })?;
        if target_clan.id != my_clan.id {
            log::debug!("User {} is not in same clan as user {} to update clan member rank", self.account.id, user.id);
            return Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::SocialErrorCode::UserNotInClan as i16,
                "User is not in same clan as user to update clan member rank".to_owned(),
            ));
        }
        let to_update = oj_rc_database::schema::clan_member::ActiveModel {
            id: oj_rc_database::sea_orm::ActiveValue::Set(target_member.id),
            rank: oj_rc_database::sea_orm::ActiveValue::Set(rank.core_to_db()),
            ..Default::default()
        };
        self.db.update_clan_member(to_update).await
            .map_err(|e| {
                log::error!("Failed to update user {} clan member rank by user {}: {}", user.id, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                    format!("Failed to udpate user clan member rank by user: {}", e),
                )
            })?;
        if matches!(rank, super::ClanMemberRank::Leader) {
            let to_update = oj_rc_database::schema::clan_member::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::Set(my_member.id),
                rank: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::clan_member::ClanMemberRank::Officer),
                ..Default::default()
            };
            self.db.update_clan_member(to_update).await
                .map_err(|e| {
                    log::error!("Failed to update own clan member rank by user {} to demote to officer: {}", self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::SocialErrorCode::DatabaseError as i16,
                        format!("Failed to udpate own clan member rank by user to demote to officer: {}", e),
                    )
                })?;
        }
        self.clan_members_of_clan(my_clan.id).await
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
