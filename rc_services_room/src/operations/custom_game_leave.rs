use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 145;

const RESPONSE_CODE_PARAM_KEY: u8 = 168; // int enum; out

pub(super) struct CustomGameLeaver {
    games: std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>,
    mesh: std::sync::Arc<crate::user_service::UserMesh>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CustomGameLeaver {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let user_info = user.user()?;
        let (resp_code, session_opt) = self.games.leave_game(user_info.public_id()).await;
        if let Some(session) = session_opt {
            let event = crate::events::CustomGameRefresh {
                session: session.session_id,
            };
            let session_members = session.users.iter()
                .map(|mem| &mem.public_id as &str);
            self.mesh.broadcast_event_to(session_members, event).await;
        }
        params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(resp_code as _));
        Ok(params)
    }
}

pub(super) fn game_leave_provider<C: Send + 'static>(init_ctx: &crate::InitConfig) -> SimpleOpImpl<C, crate::UserTy, CustomGameLeaver> {
    SimpleOpImpl::new(CustomGameLeaver {
        games: init_ctx.custom_games.clone(),
        mesh: init_ctx.user_mesh.clone(),
    })
}
