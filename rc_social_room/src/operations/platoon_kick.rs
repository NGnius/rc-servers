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
            if let Some(platoon_id) = self.social.platoon_of_user(&username.string).await {
                let members = self.social.users_of_platoon(&platoon_id).await;
                if !members.is_empty() && members[0].public_id != my_public_id {
                    return Err(SimpleOpError::with_message(
                        oj_rc_core::data::error_codes::SocialErrorCode::TheyNotPlatoonLeader as i16,
                        "Only platoon leader can kick members".to_owned(),
                    ));
                }
                log::debug!("User {} kicked from platoon {}", username.string, platoon_id);
                if members.len() <= 2 {
                    self.social.remove_platoon(&platoon_id).await;
                    let event = crate::events::platoon_removed::PlatoonDisbanded {
                        platoon_id: platoon_id.to_owned(),
                    };
                    for member in members.iter() {
                        if member.public_id == my_public_id { continue; }
                        self.social.send_event_to(&member.public_id, event.clone()).await;
                    }
                    log::debug!("Platoon {} removed", platoon_id);
                } else {
                    let social_infos = user_info.list_social_info(std::slice::from_ref(&username.string)).await?;
                    self.social.remove_user_from_platoon(&username.string).await;
                    self.social.send_event_to(&username.string, crate::events::platoon_member_kick::PlatoonMemberKick).await;
                    let event = crate::events::platoon_member_left::PlatoonMemberLeft {
                        member_public_id: username.string.clone(),
                        member_display_name: social_infos.first().map(|soc| soc.display_name.clone()).unwrap_or_else(|| username.string.clone()),
                    };
                    for member in members.iter() {
                        if member.public_id == my_public_id { continue; }
                        self.social.send_event_to(&member.public_id, event.clone()).await;
                    }
                }
            } else {
                log::warn!("Cannot kick {} from platoon because they are not in one", username.string);
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
