use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 152;

const NEW_STATE_PARAM_KEY: u8 = 188; // int enum; in

pub(super) struct CustomGameMemberStateUpdate {
    games: std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>,
    mesh: std::sync::Arc<crate::user_service::UserMesh>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CustomGameMemberStateUpdate {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(Typed::Int(new_state)) = params.remove(&NEW_STATE_PARAM_KEY) {
            let new_status_enum = crate::data::custom_games::PlayerSessionStatus::from_u8(new_state as _)
                .ok_or_else(|| SimpleOpError::with_message(
                    oj_rc_core::data::error_codes::WebServicesError::UnexpectedError as _,
                    format!("Unrecognized PlayerSessionStatus {}", new_state),
            ))?;
            let user_info = user.user()?;
            let my_pub_id = user_info.public_id();
            let session_opt = self.games.update_user_status(my_pub_id, new_status_enum).await;
            if let Some(session) = session_opt {
                let update_event = crate::events::CustomGameRefresh {
                    session: session.session_id,
                };
                let members_iter = session.users.iter()
                    .map(|mem| &mem.public_id as &str);
                self.mesh.broadcast_event_to(members_iter, update_event).await;
            }
        }
        Ok(params)
    }
}

pub(super) fn game_player_status_update_provider<C: Send + 'static>(init_ctx: &crate::InitConfig) -> SimpleOpImpl<C, crate::UserTy, CustomGameMemberStateUpdate> {
    SimpleOpImpl::new(CustomGameMemberStateUpdate {
        games: init_ctx.custom_games.clone(),
        mesh: init_ctx.user_mesh.clone(),
    })
}
