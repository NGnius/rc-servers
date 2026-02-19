use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 11;

const PLATOON_ID_PARAM_KEY: u8 = 16; // str; out
const PLATOON_LEADER_PARAM_KEY: u8 = 17; // str; out
const PLATOON_MEMBERS_PARAM_KEY: u8 = 7; // array of custom (platoon member); out

pub(super) struct PlatoonInviteAccepter {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for PlatoonInviteAccepter {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        let my_public_id = user_info.public_id();
        if let Some(platoon_id) = self.social.platoon_of_user(my_public_id).await {
            log::debug!("User {} accepted invite to platoon {}", my_public_id, platoon_id);
            let members = self.social.users_of_platoon(&platoon_id).await;
            let member_pub_ids: Vec<String> = members.iter().map(|m| m.public_id.clone()).collect();
            let social_infos = user_info.list_social_info(&member_pub_ids).await?;
            let plat_members: Vec<crate::data::platoon::PlatoonMemberInfo> = members.iter()
                .map(|member| {
                    let soc = social_infos.iter()
                        .find(|soc| soc.public_id == member.public_id)
                        .unwrap();
                    crate::data::platoon::PlatoonMemberInfo {
                        public_id: soc.public_id.clone(),
                        display_name: soc.display_name.clone(),
                        status: member.status,
                        added: member.timestamp,
                        avatar_id: 0,
                        use_custom_avatar: true, // the game client has a bug that makes it need a custom avatar here
                    }
                }).collect();
            params.insert(PLATOON_ID_PARAM_KEY, Typed::Str(platoon_id.into()));
            params.insert(PLATOON_LEADER_PARAM_KEY, Typed::Str(
                members.first()
                    .map(|p| p.public_id.clone())
                    .unwrap_or_default()
                    .into()
            ));
            params.insert(PLATOON_MEMBERS_PARAM_KEY, Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Custom,
                custom_ty: Some(1),
                items: plat_members.iter()
                    .map(|x| Typed::Custom(crate::data::custom::CustomType::PlatoonMember(x.to_owned())))
                    .collect(),
            }));
            self.social.update_user_in_platoon(my_public_id, crate::data::platoon::MemberStatus::Ready).await;
            let event = crate::events::platoon_member_update::PlatoonMemberStatusUpdate {
                member_public_id: my_public_id.to_owned(),
                member_display_name: user_info.display_name().to_owned(),
                member_status: crate::data::platoon::MemberStatus::Ready,
            };
            for member in members.iter() {
                if member.public_id == my_public_id { continue; }
                self.social.send_event_to(&member.public_id, event.clone()).await;
            }
            // send the correct avatars later (to workaround the game client bug)
            let social = self.social.clone();
            let my_id = my_public_id.to_owned();
            tokio::task::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(200)).await; // FIXME figure out a better way to run this shortly after the op has returned
                for member in plat_members.iter() {
                    if member.public_id == my_id { continue; }
                    let social_info = social_infos.iter()
                        .find(|soc| soc.public_id == member.public_id)
                        .unwrap();
                    if social_info.avatar_id.is_none() { continue; }
                    let event = crate::events::platoon_member_avatar_update::PlatoonMemberAvatarChanged {
                        member_public_id: member.public_id.clone(),
                        avatar_id: social_info.avatar_id,
                    };
                    social.send_event_to(&my_id, event).await;
                }
            });

        }
        Ok(params)
    }
}

pub(super) fn platoon_accepter_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, PlatoonInviteAccepter> {
    SimpleOpImpl::new(PlatoonInviteAccepter {
        social: init_ctx.social.clone(),
    })
}
