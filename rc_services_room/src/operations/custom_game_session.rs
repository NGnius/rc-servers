use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 144;

const RESPONSE_CODE_PARAM_KEY: u8 = 168; // int enum; out
const RESPONSE_DATA_PARAM_KEY: u8 = 169; // hashtable; out

pub(super) struct CustomGameRetriever {
    games: std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CustomGameRetriever {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let user_info = user.user()?;
        let my_pub_id = user_info.public_id();
        let game_opt = self.games.get_user_game(my_pub_id).await;
        if let Some(game) = game_opt {
            log::debug!("User {} retrieved their custom game session {} info", my_pub_id, game.session_id);
            params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(crate::data::custom_games::SessionRetrieveResponse::SessionRetrieved as _));
            let pub_ids: Vec<String> = game.users.iter()
                .map(|member| member.public_id.clone())
                .collect();
            let avatars = user_info.list_avatar_info(&pub_ids).await?;
            let avatar_map: std::collections::HashMap<_, _> = avatars.into_iter()
                .map(|avatar| (avatar.public_id.clone(), avatar)).collect();
            let resp = crate::data::custom_games::Session {
                leader: game.users.first().map(|leader| leader.public_id.clone()).unwrap_or_default(),
                session: game.session_id,
                members: game.users.iter().map(|mem| mem.public_id.clone()).collect(),
                members_display_name: game.users.iter()
                    .filter_map(|mem| avatar_map.get(&mem.public_id))
                    .map(|mem| mem.display_name.clone())
                    .collect(),
                invited: game.users.iter()
                    .filter(|mem| mem.is_invited)
                    .map(|mem| mem.public_id.clone())
                    .collect(),
                team_b_members: game.users.iter()
                    .filter(|mem| mem.team == 1)
                    .map(|mem| mem.public_id.clone())
                    .collect(),
                config: game.config,
                avatar_info: avatar_map.iter()
                    .map(|(pub_id, avatar)| (pub_id.to_owned(), oj_rc_core::data::player_data::AvatarInfo {
                        avatar_id: avatar.avatar_id,
                    }))
                    .collect(),
                player_session_state: game.users.iter()
                    .map(|mem| (mem.public_id.clone(), mem.state))
                    .collect(),
            };
            params.insert(RESPONSE_DATA_PARAM_KEY, resp.as_transmissible());
        } else {
            log::debug!("User {} is not in any custom game session", my_pub_id);
            params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(crate::data::custom_games::SessionRetrieveResponse::UserNotInAnySession as _));
        }
        Ok(params)
    }
}

pub(super) fn custom_session_provider<C: Send + 'static>(games: &std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>) -> SimpleOpImpl<C, crate::UserTy, CustomGameRetriever> {
    SimpleOpImpl::new(CustomGameRetriever {
        games: games.to_owned(),
    })
}

