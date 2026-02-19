use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::ParameterTable;

const CODE: u8 = 13;

pub(super) struct PlatoonLeaver {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for PlatoonLeaver {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        let my_public_id = user_info.public_id();
        if let Some(platoon_id) = self.social.platoon_of_user(my_public_id).await {
            log::debug!("User {} left platoon {}", my_public_id, platoon_id);
            let members = self.social.users_of_platoon(&platoon_id).await;
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
                self.social.remove_user_from_platoon(my_public_id).await;
                let event = crate::events::platoon_member_left::PlatoonMemberLeft {
                    member_public_id: my_public_id.to_owned(),
                    member_display_name: user_info.display_name().to_owned(),
                };
                for member in members.iter() {
                    if member.public_id == my_public_id { continue; }
                    self.social.send_event_to(&member.public_id, event.clone()).await;
                }
                if members[0].public_id == my_public_id {
                    // current user was the leader; there is now a new leader
                    log::debug!("User {} is now the leader of platoon {}", members[1].public_id, platoon_id);
                    let social_info = user_info.list_social_info(&[members[1].public_id.clone()]).await?;
                    let event = crate::events::platoon_leader_changed::PlatoonLeaderChanged {
                        leader_public_id: social_info[0].public_id.clone(),
                        leader_display_name: social_info[0].display_name.clone(),
                    };
                    for member in members.iter() {
                        if member.public_id == my_public_id { continue; }
                        self.social.send_event_to(&member.public_id, event.clone()).await;
                    }
                }
            }

        }
        Ok(params)
    }
}

pub(super) fn platoon_leave_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, PlatoonLeaver> {
    SimpleOpImpl::new(PlatoonLeaver {
        social: init_ctx.social.clone(),
    })
}
