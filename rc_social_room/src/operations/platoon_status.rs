use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 17;

const STATUS_PARAM_KEY: u8 = 3;

pub(super) struct PlatoonStatusUpdater {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for PlatoonStatusUpdater {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        if let Some(Typed::Int(status)) = params.remove(&STATUS_PARAM_KEY) {
            let user_info = user.user()?;
            let my_public_id = user_info.public_id();
            let member_status = crate::data::platoon::MemberStatus::from_u8(status as u8)
                .ok_or_else(|| SimpleOpError::with_message(
                    oj_rc_core::data::error_codes::SocialErrorCode::UnexpectedError as i16,
                    "Invalid member status".to_owned(),
                ))?;
            if let Some(platoon_id) = self.social.platoon_of_user(my_public_id).await {
                log::debug!("User {} platoon {} status updated to {:?}", my_public_id, platoon_id, member_status);
                let members = self.social.users_of_platoon(&platoon_id).await;
                self.social.update_user_in_platoon(my_public_id, member_status).await;
                let event = crate::events::platoon_member_update::PlatoonMemberStatusUpdate {
                    member_public_id: my_public_id.to_owned(),
                    member_display_name: user_info.display_name().to_owned(),
                    member_status,
                };
                for member in members {
                    if member.public_id == my_public_id { continue; }
                    self.social.send_event_to(&member.public_id, event.clone()).await;
                }
            }
        }


        Ok(params)
    }
}

pub(super) fn platoon_update_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, PlatoonStatusUpdater> {
    SimpleOpImpl::new(PlatoonStatusUpdater {
        social: init_ctx.social.clone(),
    })
}
