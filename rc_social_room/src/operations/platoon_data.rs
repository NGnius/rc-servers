use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 18;

const PLATOON_ID_PARAM_KEY: u8 = 16;
const PLATOON_LEADER_PARAM_KEY: u8 = 17;
const USER_LIST_PARAM_KEY: u8 = 7;

/*pub(super) fn platoon_provider<C: Send + Sync>() -> SimpleFunc<18, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        // if platoon ID is not provided, you're not in a platoon
        params.insert(PLATOON_ID_PARAM_KEY, polariton::operation::Typed::Null);
        Ok(params.into())
    })
}*/

pub(super) struct PlatoonInformer {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for PlatoonInformer {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        let my_public_id = user_info.public_id();
        log::debug!("Checking for existing platoon of user {}", my_public_id);
        if let Some(platoon_id) = self.social.platoon_of_user(my_public_id).await {
            log::debug!("User {} is in platoon {}", my_public_id, platoon_id);
            let members = self.social.users_of_platoon(&platoon_id).await;
            let member_ids = members.iter()
                .map(|mem| mem.public_id.clone())
                .collect::<Vec<_>>();
            let social_infos = user_info.list_social_info(&member_ids).await?;
            if social_infos.len() != members.len() {
                let diff = members.len().wrapping_sub(social_infos.len());
                return Err(SimpleOpError::with_message(
                    oj_rc_core::data::error_codes::SocialErrorCode::UserDoesNotExist as i16,
                    format!("Could not find user avatar info for {} user(s) in platoon {}", diff, platoon_id),
                ));
            }
            params.insert(PLATOON_ID_PARAM_KEY, Typed::Str(platoon_id.into()));
            params.insert(PLATOON_LEADER_PARAM_KEY, Typed::Str(member_ids.first().unwrap().clone().into()));
            params.insert(USER_LIST_PARAM_KEY, Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Custom,
                custom_ty: Some(1),
                items: members.into_iter()
                    .map(|mem| {
                        let social_info = social_infos.iter()
                            .find(|soc| soc.public_id == mem.public_id)
                            .unwrap();
                        Typed::Custom(crate::data::custom::CustomType::PlatoonMember(crate::data::platoon::PlatoonMemberInfo {
                            public_id: mem.public_id,
                            display_name: social_info.display_name.clone(),
                            status: mem.status,
                            added: mem.timestamp,
                            avatar_id: social_info.avatar_id.unwrap_or(0),
                            use_custom_avatar: social_info.avatar_id.is_none(),
                        }))
                    })
                    .collect(),
            }));
        } else {
            log::debug!("User {} is not in a platoon", my_public_id);
            params.insert(PLATOON_ID_PARAM_KEY, Typed::Null);
        }
        Ok(params)
    }
}

pub(super) fn platoon_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, PlatoonInformer> {
    SimpleOpImpl::new(PlatoonInformer {
        social: init_ctx.social.clone(),
    })
}

