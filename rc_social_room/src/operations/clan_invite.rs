use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::clan_invite::*;

//use crate::data::clan_invite::*;

const CODE: u8 = 39;

const INVITES_PARAM_KEY: u8 = 42;

/*pub(super) fn clan_invites_provider<C: Send + Sync>() -> SimpleFunc<39, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::<C>::Arr(Arr {
            ty: polariton::serdes::TypePrefix::HashMap, // hashmap
            custom_ty: None,
            /*items: vec![
                ClanInviteInfo {
                    username: "RE_user1".to_owned(),
                    display_name: "RE_user1".to_owned(),
                    clan_name: "RE_clan_invite1".to_owned(),
                    clan_size: 42,
                    use_custom_avatar: false,
                    avatar_id: 0,
                }.as_transmissible()
            ],*/
            items: vec![],
        }));
        Ok(params.into())
    })
}*/

pub(super) struct ClanInviteLister {
    //social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for ClanInviteLister {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        let invites = user_info.my_clan_invites().await?;
        let typed_invites = invites.into_iter()
            .map(|invite| ClanInviteInfo {
                username: invite.public_id,
                display_name: invite.display_name,
                clan_name: invite.clan_name,
                use_custom_avatar: invite.avatar_id.is_none(),
                avatar_id: invite.avatar_id.unwrap_or_default(),
                clan_size: invite.size,
            }.as_transmissible())
            .collect();
        params.insert(INVITES_PARAM_KEY, Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::HashMap, // hashmap
            custom_ty: None,
            items: typed_invites,
        }));
        Ok(params)
    }
}

pub(super) fn clan_invites_provider(_init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, ClanInviteLister> {
    SimpleOpImpl::new(ClanInviteLister {
        //social: init_ctx.social.clone(),
    })
}
