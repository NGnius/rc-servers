use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 148;

const ACCEPT_PARAM_KEY: u8 = 173; // bool; in
const RESPONSE_CODE_PARAM_KEY: u8 = 168; // int enum; out

pub(super) struct CustomGameInviteResponder {
    games: std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>,
    mesh: std::sync::Arc<crate::user_service::UserMesh>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CustomGameInviteResponder {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(Typed::Bool(is_accept)) = params.remove(&ACCEPT_PARAM_KEY) {
            let user_info = user.user()?;
            let my_pub_id = user_info.public_id();
            let (resp_code, session_opt) = self.games.update_invite_user(my_pub_id, is_accept).await;
            if let Some(session) = session_opt {
                if !is_accept {
                    let event = crate::events::CustomGameInviteDecline {
                        public_id: my_pub_id.to_owned(),
                    };
                    let session_members_iter = session.users.iter()
                        .filter(|mem| !mem.is_invited && mem.public_id != my_pub_id)
                        .map(|mem| &mem.public_id as &str);
                    self.mesh.broadcast_event_to(session_members_iter, event).await;
                } else {
                    user_info.update_custom_game(oj_rc_core::persist::user::intercom::IntercomLobbyCustomGameDataMessage {
                        session_id: session.session_id.clone(),
                        config: session.config_core,
                        users: session.users.iter()
                            .filter(|user| !user.is_invited)
                            .map(|user| oj_rc_core::persist::user::intercom::IntercomLobbyCustomGameUserData {
                                public_id: user.public_id.clone(),
                                team: user.team,
                            })
                            .collect()
                    }).await;
                }
                let event = crate::events::CustomGameRefresh {
                    session: session.session_id,
                };
                let other_session_members_iter = session.users.iter()
                    .filter(|mem| !mem.is_invited && mem.public_id != my_pub_id)
                    .map(|mem| &mem.public_id as &str);
                self.mesh.broadcast_event_to(other_session_members_iter, event).await;
            }
            params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(resp_code as _));
        }
        Ok(params)
    }
}

pub(super) fn game_invite_respond_provider<C: Send + 'static>(init_ctx: &crate::InitConfig) -> SimpleOpImpl<C, crate::UserTy, CustomGameInviteResponder> {
    SimpleOpImpl::new(CustomGameInviteResponder {
        games: init_ctx.custom_games.clone(),
        mesh: init_ctx.user_mesh.clone(),
    })
}
