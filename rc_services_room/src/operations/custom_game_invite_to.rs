use oj_rc_core::types::RawType;
use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 147;

const INVITEE_PARAM_KEY: u8 = 171; // str; in
const IS_TEAM_A_PARAM_KEY: u8 = 175; // bool; in
const RESPONSE_CODE_PARAM_KEY: u8 = 168; // int enum; out

pub(super) struct CustomGameInviter {
    games: std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>,
    mesh: std::sync::Arc<crate::user_service::UserMesh>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CustomGameInviter {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(Typed::Str(invitee_id)) = params.remove(&INVITEE_PARAM_KEY) {
            if let Some(Typed::Bool(is_team_a)) = params.remove(&IS_TEAM_A_PARAM_KEY) {
                let user_info = user.user()?;
                let my_pub_id = user_info.public_id();
                let invitee_pub_id = oj_rc_core::types::PublicId::from_raw(invitee_id.string);
                let avatars = user_info.list_avatar_info(&[my_pub_id.clone(), invitee_pub_id.clone()]).await?;
                // TODO resolve public_id of invitee by provided display name
                let resp_code = if !avatars.iter().any(|x| x.public_id == invitee_pub_id) {
                    crate::data::custom_games::InviteToCustomGameResponseCode::UserDoesNotExist
                } else {
                    let soc_info_invitee = avatars.iter().find(|s| s.public_id == invitee_pub_id);
                    let invitee_name = soc_info_invitee.map(|s| s.display_name.clone())
                        .unwrap_or_else(|| oj_rc_core::types::DisplayName::from_raw(invitee_pub_id.as_raw()));
                    let (resp_code, session_opt) = self.games.invite_user(my_pub_id.clone(), invitee_pub_id.clone(), invitee_name, is_team_a).await;
                    if let Some(session) = session_opt {
                        if self.mesh.is_user_online(&invitee_pub_id).await {
                            let my_avatar = avatars.iter().find(|x| x.public_id == my_pub_id).unwrap();
                            log::debug!("User {} invited {} to custom game {}", my_pub_id, invitee_pub_id, session.session_id.as_raw_ref());
                            let event = crate::events::CustomGameInvite {
                                inviter_public_id: my_pub_id.to_owned(),
                                inviter_display_name: user_info.display_name().to_owned(),
                                session: session.session_id.clone(),
                                avatar_id: my_avatar.avatar_id,
                                invited_to_team_a: is_team_a,
                            };
                            self.mesh.send_event_to(&invitee_pub_id, event).await;
                            let event = crate::events::CustomGameRefresh {
                                session: session.session_id,
                            };
                            let other_members = session.users.iter()
                                .filter(|mem| !mem.is_invited)
                                .map(|mem| &mem.public_id);
                            self.mesh.broadcast_event_to(other_members, event).await;
                            resp_code
                        } else {
                            crate::data::custom_games::InviteToCustomGameResponseCode::UserIsNotOnline
                        }
                    } else {
                        resp_code
                    }
                };
                params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(resp_code as _));
            }
        }
        Ok(params)
    }
}

pub(super) fn game_invite_provider<C: Send + 'static>(init_ctx: &crate::InitConfig) -> SimpleOpImpl<C, crate::UserTy, CustomGameInviter> {
    SimpleOpImpl::new(CustomGameInviter {
        games: init_ctx.custom_games.clone(),
        mesh: init_ctx.user_mesh.clone(),
    })
}
