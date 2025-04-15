use crate::SimpleChatFunc;
use crate::persist::chat_user::ChatUser;
use polariton::operation::ParameterTable;

const PARAM_KEY: u8 = 18;

pub(super) fn all_channels_provider(chat_system: crate::state::ChatImpl) -> SimpleChatFunc<11, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy, &crate::state::ChatImpl) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleChatFunc::new(|params, user: &crate::UserTy, _chat_system: &crate::state::ChatImpl| {
        let mut params = params.to_dict();
        let user_trait = user.user()?;
        let chat_user = super::get_chat_user(user_trait.as_ref().as_ref());
        //let chat_user: &ChatUserImpl = user_trait.ext(std::any::TypeId::of::<ChatUserImpl>()).unwrap().downcast_ref().unwrap();
        params.insert(PARAM_KEY, chat_user.subscribed_channels());
        /*params.insert(PARAM_KEY, Typed::Arr(Arr {
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
        }));*/
        Ok(params.into())
    }, chat_system)
}
