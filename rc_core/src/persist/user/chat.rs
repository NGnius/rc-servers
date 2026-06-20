use super::account_json::UserData;

#[async_trait::async_trait]
impl super::ChatUser for UserData {
    async fn subscribed_channels(&self) -> Result<polariton::operation::Typed<()>, i16> {
        let channels = self.subscribed_channels_strings().await?;
        log::info!("User is subscribed to channels {:?}", channels);
        Ok(polariton::operation::Typed::Arr(polariton::operation::Arr {
            ty: polariton::serdes::TypePrefix::HashMap, // hashtable
            custom_ty: None,
            items: channels.into_iter().map(|name| crate::data::channel::ChatChannelInfo {
                channel_name: name,
                members: vec![
                    crate::data::channel::ChatChannelMember {
                        name: self.account.display_name.clone(),
                        use_custom_avatar: false,
                        state: crate::data::channel::ChatPlayerState::Idk0,
                        custom_avatar: Vec::default(),
                        avatar_id: 0,
                    },
                ],
                channel_ty: crate::data::channel::ChatChannelType::Public,
            }.as_transmissible()).collect()
        }))
    }

    async fn subscribed_channels_strings(&self) -> Result<Vec<String>, i16> {
        let channels = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::SubscribedChannels).await.map_err(|e| {
                log::error!("Failed to retrieve SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
                crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
            })?.ok_or_else(|| {
                log::error!("Failed to find SubscribedChannels (user_aux) for user_id {}", self.account.id);
                crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
            })?;
        let channels = serde_json::from_str::<Vec<String>>(&channels.data).map_err(|e| {
            log::error!("Failed to parse SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
            crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
        })?;
        Ok(channels)
    }

    async fn add_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> Result<polariton::operation::Typed<()>, i16> {
        if let crate::data::channel::ChatChannelType::Public = channel_ty {
            let mut sub_channels = self.subscribed_channels_strings().await?;
            sub_channels.push(channel.clone());
            let new_data = serde_json::to_string(&sub_channels).map_err(|e| {
                log::error!("Failed to convert to JSON SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
                crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
            })?;
            self.db.update_user_aux_by_user_id_and_descriptor(oj_rc_database::schema::user_aux::ActiveModel {
                data: oj_rc_database::sea_orm::ActiveValue::Set(new_data),
                ..Default::default()
            }, self.account.id, oj_rc_database::schema::user_aux::Descriptor::SubscribedChannels).await.map_err(|e| {
                log::error!("Failed to update SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
                crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
            })?;
        }

         Ok(crate::data::channel::ChatChannelInfo {
            channel_name: channel,
            members: Vec::default(),
            channel_ty,
        }.as_transmissible())
    }

    async fn remove_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> Result<(), i16> {
        if let crate::data::channel::ChatChannelType::Public = channel_ty {
            let mut sub_channels = self.subscribed_channels_strings().await?;
            if let Some(index) = sub_channels.iter().position(|chann| chann == &channel) {
                sub_channels.swap_remove(index);
                let new_data = serde_json::to_string(&sub_channels).map_err(|e| {
                    log::error!("Failed to convert to JSON SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
                })?;
                self.db.update_user_aux_by_user_id_and_descriptor(oj_rc_database::schema::user_aux::ActiveModel {
                    data: oj_rc_database::sea_orm::ActiveValue::Set(new_data),
                    ..Default::default()
                }, self.account.id, oj_rc_database::schema::user_aux::Descriptor::SubscribedChannels).await.map_err(|e| {
                    log::error!("Failed to update SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
                })?;
            }
        }
        Ok(())
    }

    /*async fn has_pending_sanctions(&self) -> Result<bool, i16> {
        let count = self.db.count_sanctions_to_ack_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::sanction::Descriptor::Warn).await.map_err(|e| {
            log::error!("Failed to count pending sanctions for user_id {}: {}", self.account.id, e);
            crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
        })?;
        Ok(count != 0)
    }*/

    async fn get_sanctions(&self, username: String) -> Result<polariton::operation::Typed<()>, i16> {
        let user_opt = self.db.user_by_display_name(username.clone()).await.map_err(|e| {
            log::error!("Failed to retrieve user by username {} for user_id {}: {}", username, self.account.id, e);
            crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
        })?;
        if let Some(user) = user_opt {
            let sanctions = self.db.sanctions_by_user_id(user.id).await.map_err(|e| {
                log::error!("Failed to retrieve sanctions by username {} for user_id {}: {}", username, self.account.id, e);
                crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
            })?;
            Ok(polariton::operation::Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Str,
                custom_ty: None,
                items: sanctions.into_iter().map(|x| {
                    let data = crate::data::sanction::SanctionJson {
                        type_: crate::data::sanction::SanctionType::from_db(x.descriptor),
                        reason: x.reason,
                        reporter: x.issuer_name,
                        issued: chrono::DateTime::from_timestamp(x.creation_time, 0).unwrap(),
                    };
                    polariton::operation::Typed::Str(data.as_json().into())
                }).collect(),
            }))
        } else {
            Err(crate::data::error_codes::ChatErrorCodes::DoesNotExist as i16)
        }
    }

    async fn set_sanction(&self, sanction: super::SetSanction) -> Result<(), i16> {
        self.check_perms_to_exec(&sanction.type_)?;
        let user_opt = self.db.user_by_display_name(sanction.username.clone()).await.map_err(|e| {
            log::error!("Failed to retrieve user by username {} for user_id {}: {}", sanction.username, self.account.id, e);
            crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
        })?;
        if let Some(user) = user_opt {
            if sanction.is_adding {
                let now = chrono::Utc::now().timestamp();
                let sanction_ty = crate::data::sanction::SanctionType::from_persist(sanction.type_).to_db();
                let to_add = oj_rc_database::schema::sanction::ActiveModel {
                    user_id: oj_rc_database::sea_orm::ActiveValue::Set(user.id),
                    creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                    issuer_id: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
                    issuer_name: oj_rc_database::sea_orm::ActiveValue::Set(self.account.display_name.clone()),
                    descriptor: oj_rc_database::sea_orm::ActiveValue::Set(sanction_ty.clone()),
                    reason: oj_rc_database::sea_orm::ActiveValue::Set(sanction.reason),
                    duration: oj_rc_database::sea_orm::ActiveValue::Set(if sanction.duration <= 0 { None } else { Some(sanction.duration as i64) }),
                    ..Default::default()
                };
                if matches!(sanction_ty, oj_rc_database::schema::sanction::Descriptor::Ban) {
                    self.db.update_perms_by_user_id(oj_rc_database::schema::permissions::ActiveModel {
                        banned: oj_rc_database::sea_orm::ActiveValue::Set(true),
                        ..Default::default()
                    }, user.id).await.map_err(|e| {
                        log::error!("Failed to update permissions (to ban) for user_id {} by user_id {}: {}", user.id, self.account.id, e);
                        crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
                    })?;
                }
                self.db.insert_sanction(to_add).await.map_err(|e| {
                    log::error!("Failed to insert sanction for user_id {} by user_id {}: {}", user.id, self.account.id, e);
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
                })?;
                Ok(())
            } else {
                // FIXME
                log::error!("Modifying sanctions is not currently supported");
                Err(crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16)
            }
        } else {
            Err(crate::data::error_codes::ChatErrorCodes::DoesNotExist as i16)
        }
    }

    async fn get_total_registered_users(&self) -> Result<u64, polariton_server::operations::SimpleOpError> {
        self.db.user_count().await
            .map_err(|e| {
                log::error!("Failed to retrieve total user count for {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16,
                    format!("Failed to retrieve total user count: {}", e),
                )
            })
    }

    async fn set_permission(&self, username: String, permission: super::UserRole, value: bool) -> Result<(), polariton_server::operations::SimpleOpError> {
        if !(self.perms.developer || self.perms.royalty || self.perms.administrator) {
            // technically this should already be handled by the chat command permission check
            // this is just extra insurance in case of a bad server configuration
            return Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::ChatErrorCodes::AdminsOnly as i16,
                format!("User {} cannot grant permissions for {}", self.account.id, username),
            ))
        }
        let account_opt = self.db.user_by_display_name(username.clone()).await.map_err(|e| {
                log::error!("Failed to retrieve user {} to grant permission by user {}: {}", username, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16,
                    format!("Failed to retrieve user for permission grant: {}", e),
                )
            })?;
        if let Some(account) = account_opt {
            self.db.update_perms_by_user_id(oj_rc_database::schema::permissions::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                user_id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                moderator: if matches!(permission, super::UserRole::Moderator) { oj_rc_database::sea_orm::ActiveValue::Set(value) } else { oj_rc_database::sea_orm::ActiveValue::NotSet },
                administrator: if matches!(permission, super::UserRole::Administrator) { oj_rc_database::sea_orm::ActiveValue::Set(value) } else { oj_rc_database::sea_orm::ActiveValue::NotSet },
                developer: if matches!(permission, super::UserRole::Developer) { oj_rc_database::sea_orm::ActiveValue::Set(value) } else { oj_rc_database::sea_orm::ActiveValue::NotSet },
                royalty: if matches!(permission, super::UserRole::Royalty) { oj_rc_database::sea_orm::ActiveValue::Set(value) } else { oj_rc_database::sea_orm::ActiveValue::NotSet },
                banned: oj_rc_database::sea_orm::ActiveValue::NotSet,
            }, account.id).await
            .map_err(|e| {
                log::error!("Failed to update permissions for user {} by user {}: {}", account.id, self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16,
                    format!("Failed to update user permissions: {}", e),
                )
            })?;
            Ok(())
        } else {
            Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::ChatErrorCodes::DoesNotExist as i16,
                format!("User {} not found", username),
            ))
        }
    }

    async fn clear_factory_flag(&self) -> Result<bool, polariton_server::operations::SimpleOpError> {
        let selected_garage_opt = self.db.garage_selected(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve selected garage for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16,
                    format!("Failed to retrieve selected garage: {}", e),
                )
            })?;
        match selected_garage_opt {
            Some(selected_garage) => {
                self.db.update_garage(oj_rc_database::schema::garage::ActiveModel {
                    id: oj_rc_database::sea_orm::ActiveValue::Set(selected_garage.id),
                    user_id: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
                    crf_id: oj_rc_database::sea_orm::ActiveValue::Set(None),
                    ..Default::default()
                }).await
                    .map_err(|e| {
                        log::error!("Failed to clear garage {} factory flag for user {}: {}", selected_garage.id, self.account.id, e);
                        polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16,
                            format!("Failed to clear garage {} factory flag: {}", selected_garage.id, e),
                        )
                    })?;
                Ok(selected_garage.crf_id.is_some())
            },
            None => {
                log::error!("Failed to find selected garage for user {}", self.account.id);
                return Err(polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16,
                    format!("Failed to find selected garage"),
                ));
            }
        }
    }
}
