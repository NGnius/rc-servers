use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 10;

const USERNAME_PARAM_KEY: u8 = 1; // str; in
const PLATOON_ID_PARAM_KEY: u8 = 16; // str; out
const PLATOON_INVITEE_PARAM_KEY: u8 = 15; // custom (PlatoonMember); out
const AVATAR_ID_PARAM_KEY: u8 = 14; // int; out
const USE_CUSTOM_AVATAR_PARAM_KEY: u8 = 13; // bool; out

pub(super) struct PlatoonInviter {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for PlatoonInviter {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Str(username)) = params.remove(&USERNAME_PARAM_KEY) {
            if let Some(platoon_id) = self.social.platoon_of_user(&username.string).await {
                log::debug!("User {} is already in platoon {}", username.string, platoon_id);
                return Err(SimpleOpError::with_message(
                    oj_rc_core::data::error_codes::SocialErrorCode::AlreadyInPlatoon as i16,
                    format!("User is already in platoon {}", platoon_id),
                ));
            }
            let user_info = user.user()?;
            let (platoon_id, social_infos, timestamp) = if let Some(platoon_id) = self.social.platoon_of_user(user_info.public_id()).await {
                // add to existing platoon
                log::debug!("Inviting {} to platoon {} by user {}", username.string, platoon_id, user_info.public_id());
                let members = self.social.users_of_platoon(&platoon_id).await;
                let member_public_ids: Vec<String> = members.iter()
                    .map(|m| m.public_id.clone())
                    .chain([username.string.clone()])
                    .collect();
                let social_infos = user_info.list_social_info(&member_public_ids).await?;
                if social_infos.len() != members.len() + 1 {
                    log::debug!("Platoon members info could not be retrieved while inviting {} to platoon {}", username.string, platoon_id);
                    return Err(SimpleOpError::with_message(
                        oj_rc_core::data::error_codes::SocialErrorCode::UserDoesNotExist as i16,
                        "User's info could not be retrieved".to_owned(),
                    ));
                }
                if let Some(timestamp) = self.social.add_user_to_platoon(&username.string, &platoon_id, crate::data::platoon::MemberStatus::Invited).await {
                    if social_infos.len() > 2 {
                        // send PlatoonMemberAdded event to other platoon members
                        let invitee_soc = social_infos.iter()
                            .find(|soc| soc.public_id == username.string)
                            .unwrap();
                        let event = crate::events::platoon_member_added::PlatoonNewMember {
                            new_member: crate::data::platoon::PlatoonMemberInfo {
                                public_id: invitee_soc.public_id.clone(),
                                display_name: invitee_soc.display_name.clone(),
                                status: crate::data::platoon::MemberStatus::Invited,
                                added: timestamp,
                                avatar_id: invitee_soc.avatar_id.unwrap_or_default(),
                                use_custom_avatar: invitee_soc.avatar_id.is_none(),
                            },
                        };
                        // unlikely to be more than 3, so no need to run this in its own task
                        for social_info in social_infos.iter() {
                            if social_info.public_id == username.string { continue; }
                            if social_info.public_id == user_info.public_id() { continue; }
                            self.social.send_event_to(&social_info.public_id, event.clone()).await;
                        }
                        /*// send PlatoonMemberAdded event to invitee for other members
                        for social_info in social_infos.iter() {
                            if social_info.public_id == username.string { continue; }
                            if social_info.public_id == user_info.public_id() { continue; }
                            let member = members.iter()
                                .find(|mem| mem.public_id == social_info.public_id)
                                .unwrap();
                            let event = crate::events::platoon_member_added::PlatoonNewMember {
                                new_member: crate::data::platoon::PlatoonMemberInfo {
                                    public_id: social_info.public_id.clone(),
                                    display_name: social_info.display_name.clone(),
                                    status: member.status,
                                    added: member.timestamp,
                                    avatar_id: social_info.avatar_id.unwrap_or_default(),
                                    use_custom_avatar: social_info.avatar_id.is_none(),
                                },
                            };
                            self.social.send_event_to(&username.string, event).await;
                        }*/
                    }
                    (platoon_id, social_infos, timestamp)
                } else {
                    log::debug!("Failed to add user {} to platoon {} (did the platoon disband?)", username.string, platoon_id);
                    return Err(SimpleOpError::with_message(
                        oj_rc_core::data::error_codes::SocialErrorCode::UserNotPlatoonFound as i16,
                        "Failed to invite user to platoon".to_owned(),
                    ));
                }
            } else {
                // create new platoon
                let social_infos = user_info.list_social_info(&[
                    username.string.clone(),
                    user_info.public_id().to_owned(),
                ]).await?;
                if social_infos.len() != 2 {
                    log::debug!("User {} info could not be retrieved while creating new platoon", username.string);
                    return Err(SimpleOpError::with_message(
                        oj_rc_core::data::error_codes::SocialErrorCode::UserDoesNotExist as i16,
                        "User's info could not be retrieved".to_owned(),
                    ));
                }
                let platoon_id = if let Some((_, platoon_key)) = self.social.create_platoon(user_info.public_id()).await {
                    log::debug!("Created new platoon {} for user {} to invite {}", platoon_key, user_info.public_id(), username.string);
                    platoon_key
                } else {
                    return Err(SimpleOpError::with_message(
                        oj_rc_core::data::error_codes::SocialErrorCode::UserNotInPlatoon as i16,
                        "Failed to create platoon".to_owned(),
                    ));
                };
                if let Some(timestamp) = self.social.add_user_to_platoon(&username.string, &platoon_id, crate::data::platoon::MemberStatus::Invited).await {
                    (platoon_id, social_infos, timestamp)
                } else {
                    return Err(SimpleOpError::with_message(
                        oj_rc_core::data::error_codes::SocialErrorCode::UserNotPlatoonFound as i16,
                        "Failed to add user to new platoon".to_owned(),
                    ));
                }
            };
            let invitee = social_infos.iter()
                .find(|soc| soc.public_id == username.string)
                .unwrap();
            let inviter = social_infos.iter()
                .find(|soc| soc.public_id == user_info.public_id())
                .unwrap();
            self.social.send_event_to(&invitee.public_id, crate::events::platoon_invite_received::PlatoonInviteReceived {
                inviter_public_id: user_info.public_id().to_owned(),
                inviter_display_name: user_info.display_name().to_owned(),
                avatar_id: inviter.avatar_id,
            }).await;
            params.insert(PLATOON_ID_PARAM_KEY, Typed::Str(platoon_id.into()));
            params.insert(PLATOON_INVITEE_PARAM_KEY, Typed::Custom(crate::data::custom::CustomType::PlatoonMember(crate::data::platoon::PlatoonMemberInfo {
                public_id: invitee.public_id.clone(),
                display_name: invitee.display_name.clone(),
                status: crate::data::platoon::MemberStatus::Invited,
                added: timestamp,
                avatar_id: invitee.avatar_id.unwrap_or_default(),
                use_custom_avatar: invitee.avatar_id.is_none(),
            })));
            params.insert(AVATAR_ID_PARAM_KEY, Typed::Int(invitee.avatar_id.unwrap_or_default()));
            params.insert(USE_CUSTOM_AVATAR_PARAM_KEY, Typed::Bool(invitee.avatar_id.is_none()));
        }
        Ok(params)
    }
}

pub(super) fn platoon_invite_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, PlatoonInviter> {
    SimpleOpImpl::new(PlatoonInviter {
        social: init_ctx.social.clone(),
    })
}
