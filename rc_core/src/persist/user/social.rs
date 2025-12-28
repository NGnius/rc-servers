use super::account_json::UserData;

#[async_trait::async_trait]
impl super::SocialUser for UserData {
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
