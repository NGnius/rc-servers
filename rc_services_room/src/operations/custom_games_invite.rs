use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

use crate::data::custom_games::*;

const CODE: u8 = 0;

const RESULT_CODE_PARAM_KEY: u8 = 168; // int enum; out
const INVITE_PARAM_KEY: u8 = 189; // hashtable (refer to C# CheckIfHasBeenInvitedToCustomGameSessionRequest)

pub(super) struct CustomGamePendingInvites {
    games: std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CustomGamePendingInvites {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let user_info = user.user()?;
        let my_pub_id = user_info.public_id();
        if let Some(session) = self.games.get_user_game(my_pub_id).await {
            let myself = session.users.iter().find(|u| u.public_id == my_pub_id).unwrap();
            if myself.is_invited {
                log::debug!("User {} has checked and is invited to custom game {}", my_pub_id, session.session_id);
                params.insert(RESULT_CODE_PARAM_KEY, Typed::Int(CustomGameInviteCode::PendingInvite as _));
                // build invite data
                let leader = session.users.first().unwrap();
                let leader_avatar = user_info.list_avatar_info(std::slice::from_ref(&leader.public_id)).await?;
                let resp = CustomGameInvite {
                    inviter_public_id: leader_avatar[0].public_id.clone(),
                    inviter_display_name: leader_avatar[0].display_name.clone(),
                    session: session.session_id,
                    avatar_id: leader_avatar[0].avatar_id,
                    invited_to_team_b: myself.team == 1,
                };
                params.insert(INVITE_PARAM_KEY, resp.as_transmissible());
            } else {
                log::debug!("User {} has already accepted invite to custom game {}", my_pub_id, session.session_id);
                params.insert(RESULT_CODE_PARAM_KEY, Typed::Int(CustomGameInviteCode::NoInvite as _));
            }
        } else {
            log::debug!("User {} is not invited to any custom game", my_pub_id);
            params.insert(RESULT_CODE_PARAM_KEY, Typed::Int(CustomGameInviteCode::NoInvite as _));
        }
        Ok(params)
    }
}

pub(super) fn pending_invite_provider<C: Send + 'static>(init_ctx: &crate::InitConfig) -> SimpleOpImpl<C, crate::UserTy, CustomGamePendingInvites> {
    SimpleOpImpl::new(CustomGamePendingInvites {
        games: init_ctx.custom_games.clone(),
    })
}

