use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::channel::*;

const PARAM_KEY: u8 = 18;

pub(super) fn all_channels_provider() -> SimpleFunc<11, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::HashMap, // hashtable
            items: vec![
                ChatChannelInfo {
                    channel_name: "RE_public_channel0".to_owned(),
                    members: vec![
                        ChatChannelMember {
                            name: "RE_chat0_username0".to_owned(),
                            use_custom_avatar: false,
                            state: ChatPlayerState::Idk1,
                            custom_avatar: Vec::default(),
                            avatar_id: 2,
                        },
                        ChatChannelMember {
                            name: "RE_chat0_username1".to_owned(),
                            use_custom_avatar: false,
                            state: ChatPlayerState::Idk2,
                            custom_avatar: Vec::default(),
                            avatar_id: 3,
                        },
                    ],
                    channel_ty: ChatChannelType::Public,
                }.as_transmissible(),
                ChatChannelInfo {
                    channel_name: "RE_custom_channel1".to_owned(),
                    members: vec![
                        ChatChannelMember {
                            name: "RE_chat1_username0".to_owned(),
                            use_custom_avatar: false,
                            state: ChatPlayerState::Idk0,
                            custom_avatar: Vec::default(),
                            avatar_id: 2,
                        },
                        ChatChannelMember {
                            name: "RE_chat1_username1".to_owned(),
                            use_custom_avatar: false,
                            state: ChatPlayerState::Idk1,
                            custom_avatar: Vec::default(),
                            avatar_id: 3,
                        },
                    ],
                    channel_ty: ChatChannelType::Custom,
                }.as_transmissible(),
            ],
        }));
        Ok(params.into())
    })
}
