use oj_rc_core::types::RawType;
use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 14;

const PUBLIC_ID_PARAM_KEY: u8 = 1; // str; in

pub(super) struct PlatoonKicker {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for PlatoonKicker {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        let my_public_id = user_info.public_id();
        if let Some(Typed::Str(username)) = params.remove(&PUBLIC_ID_PARAM_KEY) {
            let user_pub_id = oj_rc_core::types::PublicId::from_raw(username.string);
            if let Some(platoon_id) = self.social.platoon_of_user(&user_pub_id).await {
                let members = self.social.users_of_platoon(&platoon_id).await;
                if !members.is_empty() && members[0].public_id != my_public_id {
                    return Err(SimpleOpError::with_message(
                        oj_rc_core::data::error_codes::SocialErrorCode::TheyNotPlatoonLeader as i16,
                        "Only platoon leader can kick members".to_owned(),
                    ));
                }
                log::debug!("{} kicked from platoon {}", user_pub_id, platoon_id.as_raw_ref());
                if members.len() <= 2 {
                    self.social.remove_platoon(&platoon_id).await;
                    let event = crate::events::platoon_removed::PlatoonDisbanded {
                        platoon_id: platoon_id.to_owned(),
                    };
                    for member in members.iter() {
                        if member.public_id == my_public_id { continue; }
                        self.social.send_event_to(&member.public_id, event.clone()).await;
                    }
                    log::debug!("Platoon {} removed", platoon_id.as_raw_ref());
                } else {
                    let social_infos = user_info.list_social_info(std::slice::from_ref(&user_pub_id)).await?;
                    self.social.remove_user_from_platoon(&user_pub_id).await;
                    self.social.send_event_to(&user_pub_id, crate::events::platoon_member_kick::PlatoonMemberKick).await;
                    let event = crate::events::platoon_member_left::PlatoonMemberLeft {
                        member_public_id: user_pub_id.clone(),
                        member_display_name: social_infos.first().map(|soc| soc.display_name.clone())
                            .unwrap_or_else(|| oj_rc_core::types::DisplayName::from_raw(user_pub_id.as_raw())),
                    };
                    for member in members.iter() {
                        if member.public_id == my_public_id { continue; }
                        self.social.send_event_to(&member.public_id, event.clone()).await;
                    }
                }
            } else {
                log::warn!("Cannot kick {} from platoon because they are not in one", user_pub_id);
            }
        }
        Ok(params)
    }
}

pub(super) fn platoon_kick_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, PlatoonKicker> {
    SimpleOpImpl::new(PlatoonKicker {
        social: init_ctx.social.clone(),
    })
}
