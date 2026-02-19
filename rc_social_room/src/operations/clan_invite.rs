use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

//use crate::data::clan_invite::*;

const PARAM_KEY: u8 = 42;

pub(super) fn clan_invites_provider<C: Send + Sync>() -> SimpleFunc<39, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
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
}
