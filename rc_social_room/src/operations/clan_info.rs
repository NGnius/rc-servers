use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::clan::*;

const CLAN_NAME_PARAM_KEY: u8 = 31; // in and out
const MEMBERS_PARAM_KEY: u8 = 36; // out only
const ROBITS_CONVERSION_PARAM_KEY: u8 = 51; // out only
const CLAN_DESCRIPTION_PARAM_KEY: u8 = 32; // out only
const CLAN_TYPE_PARAM_KEY: u8 = 32; // out only

pub(super) fn clan_info_provider<C: Send + Sync>() -> SimpleFunc<33, crate::UserTy, impl (Fn(ParameterTable<C>, &crate::UserTy) -> Result<ParameterTable<C>, i16>) + Sync + Sync, C> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        if let Some(Typed::Str(clan_name)) = params.get(&CLAN_NAME_PARAM_KEY) {
            log::debug!("Requested info on clan {}", clan_name.string);
            params.insert(MEMBERS_PARAM_KEY, Typed::Arr(Arr {
                ty: polariton::serdes::TypePrefix::HashMap, // hashmap
                custom_ty: None,
                items: vec![
                    ClanMember {
                        username: "RE_clan_user_idk0".to_owned(),
                        display_name: "RE_clan_user_idk0".to_owned(),
                        member_state: ClanMemberState::Idk0,
                        use_custom_avatar: false,
                        avatar_id: 1,
                        rank: ClanMemberRank::Leader,
                        is_online: true,
                        season_xp: 42_000,
                    }.as_transmissible(),
                    ClanMember {
                        username: "RE_clan_user_idk1".to_owned(),
                        display_name: "RE_clan_user_idk1".to_owned(),
                        member_state: ClanMemberState::Idk1,
                        use_custom_avatar: false,
                        avatar_id: 1,
                        rank: ClanMemberRank::Leader,
                        is_online: true,
                        season_xp: 42_001,
                    }.as_transmissible(),
                    ClanMember {
                        username: "RE_clan_user_idk2".to_owned(),
                        display_name: "RE_clan_user_idk2".to_owned(),
                        member_state: ClanMemberState::Idk2,
                        use_custom_avatar: false,
                        avatar_id: 1,
                        rank: ClanMemberRank::Leader,
                        is_online: true,
                        season_xp: 42_002,
                    }.as_transmissible(),
                ],
            }));
            params.insert(ROBITS_CONVERSION_PARAM_KEY, Typed::Float(0.5));
            params.insert(CLAN_DESCRIPTION_PARAM_KEY, Typed::Str("RE_clan_description".into()));
            params.insert(CLAN_TYPE_PARAM_KEY, Typed::Int(ClanType::Closed as _));
        } else {
            log::debug!("Requested info on own clan (returning no info)");
        }

        Ok(params.into())
    })
}
