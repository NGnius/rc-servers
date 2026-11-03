use oj_rc_core::types::RawType;
use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 150;

const TO_KICK_PARAM_KEY: u8 = 183; // str; in
const RESPONSE_CODE_PARAM_KEY: u8 = 168; // int enum; out

pub(super) struct CustomGameMemberKicker {
    games: std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>,
    mesh: std::sync::Arc<crate::user_service::UserMesh>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CustomGameMemberKicker {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(Typed::Str(user_to_kick)) = params.remove(&TO_KICK_PARAM_KEY) {
            let user_info = user.user()?;
            let my_pub_id = user_info.public_id();
            let target_user = oj_rc_core::types::PublicId::from_raw(user_to_kick.string);
            let (resp_code, session_opt) = self.games.kick_from_game(my_pub_id.clone(), target_user.clone()).await;
            if let Some((kick_info, session)) = session_opt {
                log::debug!("User {} kicked {} from custom game {}", my_pub_id, target_user, session.session_id.as_raw_ref());
                let kick_event = crate::events::CustomGameKick {
                    session: kick_info.session_id,
                    was_invited: kick_info.was_invited,
                };
                self.mesh.send_event_to(&target_user, kick_event).await;
                let update_event = crate::events::CustomGameRefresh {
                    session: session.session_id.clone(),
                };
                let members_iter = session.users.iter()
                    .map(|mem| &mem.public_id);
                self.mesh.broadcast_event_to(members_iter, update_event).await;
                user_info.update_custom_game(oj_rc_core::persist::user::intercom::IntercomLobbyCustomGameDataMessage {
                    session_id: session.session_id.as_raw(),
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
            params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(resp_code as _));
        }
        Ok(params)
    }
}

pub(super) fn game_kick_provider<C: Send + 'static>(init_ctx: &crate::InitConfig) -> SimpleOpImpl<C, crate::UserTy, CustomGameMemberKicker> {
    SimpleOpImpl::new(CustomGameMemberKicker {
        games: init_ctx.custom_games.clone(),
        mesh: init_ctx.user_mesh.clone(),
    })
}
