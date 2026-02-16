use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed, Arr};

use crate::data::friend::*;

const CODE: u8 = 4;

const FRIENDS_PARAM_KEY: u8 = 5;
const AVATAR_PARAM_KEY: u8 = 76;

/*pub(super) fn friends_provider() -> SimpleFunc<4, crate::UserTy, impl (Fn(ParameterTable<crate::data::custom::CustomType>, &crate::UserTy) -> Result<ParameterTable<crate::data::custom::CustomType>, i16>) + Sync + Sync, crate::data::custom::CustomType> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(FRIENDS_PARAM_KEY, Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::Custom, // custom
            items: vec![Typed::Custom(crate::data::custom::CustomType::FriendInfo)] }));
        params.insert(AVATAR_PARAM_KEY, Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::HashMap, // hashmap
            items: vec![AvatarInfo {
                name: "".to_string(),
                use_custom_avatar: false,
                avatar_id: 1,
            }.as_transmissible()],
        }));
        Ok(params.into())
    })
}*/

pub(super) struct FriendsLister {
    social: std::sync::Arc<crate::SocialMesh>,
}

#[async_trait::async_trait]
impl SimpleOperation<crate::data::custom::CustomType> for FriendsLister {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<crate::data::custom::CustomType>, user: &Self::User) -> Result<ParameterTable<crate::data::custom::CustomType>, SimpleOpError> {
        let user_info = user.user()?;
        let friends = user_info.list_friends().await?;
        let mut friend_pub_ids = friends.iter().map(|friend| friend.public_id.clone()).collect();
        self.social.filter_online_only(&mut friend_pub_ids).await;
        let friends_online_pub_ids = friend_pub_ids;
        // Typed::Custom(crate::data::custom::CustomType::FriendInfo)
        params.insert(FRIENDS_PARAM_KEY, Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::Custom, // custom
            items: friends.iter().map(|friend|
                Typed::Custom(crate::data::custom::CustomType::FriendInfo(crate::data::friend::FriendInfo {
                    status: crate::data::friend::InviteStatus::from_core(&friend.state),
                    is_online: friends_online_pub_ids.contains(&friend.public_id),
                    public_id: friend.public_id.clone(),
                    display_name: friend.display_name.clone(),
                    clan_name: friend.clan_name.clone().unwrap_or_default(),
                }))
            ).collect()
        }));
        params.insert(AVATAR_PARAM_KEY, Typed::Arr(Arr {
            ty: polariton::serdes::TypePrefix::HashMap, // hashmap
            items: friends.iter()
                .map(|friend|
                    AvatarInfo {
                        name: friend.public_id.clone(),
                        use_custom_avatar: friend.avatar_id == u32::MAX,
                        avatar_id: friend.avatar_id.try_into().unwrap_or_default(),
                    }.as_transmissible()
                ).collect()
        }));
        tokio::task::spawn(send_online_event_to_friends(
            friends.iter()
                .filter(|friend| friends_online_pub_ids.contains(&friend.public_id))
                .map(|friend| (
                    friend.public_id.clone(),
                    crate::events::friend_status::FriendStatus {
                        friend_public_id: user_info.public_id().to_owned(),
                        friend_display_name: user_info.display_name().to_owned(),
                        is_online: true,
                        invite_status: crate::data::friend::InviteStatus::from_core(&friend.state).reciprocal(),
                    }
                ))
                .collect(),
            self.social.clone(),
        ));
        Ok(params)
    }
}

async fn send_online_event_to_friends(events: Vec<(String, crate::events::friend_status::FriendStatus)>, social: std::sync::Arc<crate::SocialMesh>) {
    for (public_id, event) in events {
        social.send_event_to(&public_id, event).await;
    }
}

pub(super) fn friends_provider(init_ctx: &crate::InitConfig) -> SimpleOpImpl<crate::data::custom::CustomType, crate::UserTy, FriendsLister> {
    SimpleOpImpl::new(FriendsLister {
        social: init_ctx.social.clone(),
    })
}
