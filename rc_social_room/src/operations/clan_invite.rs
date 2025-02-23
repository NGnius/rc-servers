use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::clan_invite::*;

const PARAM_KEY: u8 = 42;

pub(super) fn clan_invites_provider() -> SimpleFunc<39, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Arr(Arr {
            ty: 104, // hashmap
            items: vec![
                ClanInviteInfo {
                    username: "RE_user1".to_owned(),
                    display_name: "RE_user1".to_owned(),
                    clan_name: "RE_clan1".to_owned(),
                    clan_size: 42,
                    use_custom_avatar: false,
                    avatar_id: 0,
                }.as_transmissible()
            ],
        }));
        Ok(params.into())
    })
}
