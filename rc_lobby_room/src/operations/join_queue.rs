use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 0;

const GROUP_ID_PARAM_KEY: u8 = 2; // str; in
const GARAGE_SLOT_PARAM_KEY: u8 = 3; // int; in
const GROUP_SIZE_PARAM_KEY: u8 = 4; // int; in
const IS_GROUP_LEADER_PARAM_KEY: u8 = 14; // bool; in
const LOBBY_TY_PARAM_KEY: u8 = 30; // int; in
const EVENT_TO_JOIN_PARAM_KEY: u8 = 41; // str; in

const ESTIMATED_QUEUE_TIME_PARAM_KEY: u8 = 13; // int (seconds); out
const PERSONAL_RANKING_PARAM_KEY: u8 = 17; // double; out

pub(super) struct QueueJoinProvider {
    queue_handler: std::sync::Arc<crate::QueueHandler>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for QueueJoinProvider {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Str(group_id)) = params.remove(&GROUP_ID_PARAM_KEY) {
            if let Some(Typed::Int(slot_id)) = params.remove(&GARAGE_SLOT_PARAM_KEY) {
                if let Some(Typed::Int(group_size)) = params.remove(&GROUP_SIZE_PARAM_KEY) {
                    if let Some(Typed::Bool(is_leader)) = params.remove(&IS_GROUP_LEADER_PARAM_KEY) {
                        if let Some(Typed::Int(lobby_ty)) = params.remove(&LOBBY_TY_PARAM_KEY) {
                            let lobby_ty = oj_rc_core::data::lobby::LobbyType::from_int(lobby_ty)?;
                            log::debug!("Got lobby join queue request of platoon {} ({} players, is_leader:{}) slot {} lobby {:?}", group_id.string, group_size, is_leader, slot_id, lobby_ty);
                            match lobby_ty {
                                oj_rc_core::data::lobby::LobbyType::None => {
                                    log::warn!("Cannot join queue for None mode (???)");
                                },
                                oj_rc_core::data::lobby::LobbyType::CustomGame => {
                                    log::debug!("Joining platoon {} to custom game lobby", group_id.string);
                                    params.insert(ESTIMATED_QUEUE_TIME_PARAM_KEY, Typed::Int(self.queue_handler.wait_time_s()));
                                    params.insert(PERSONAL_RANKING_PARAM_KEY, Typed::Double(42.0));
                                    let events = user.event_sender();
                                    let user_info = user.user()?;
                                    self.queue_handler.join_custom_queue(
                                        user_info.clone(),
                                        events.to_owned(),
                                    ).await;
                                },
                                oj_rc_core::data::lobby::LobbyType::QuickPlay => {
                                    // regular multiplayer
                                    if let Some(Typed::Str(event_to_join)) = params.remove(&EVENT_TO_JOIN_PARAM_KEY) {
                                        log::debug!("Joining platoon {} to multiplayer lobby with event {}", group_id.string, event_to_join.string);
                                        params.insert(ESTIMATED_QUEUE_TIME_PARAM_KEY, Typed::Int(self.queue_handler.wait_time_s()));
                                        params.insert(PERSONAL_RANKING_PARAM_KEY, Typed::Double(42.0));
                                        let events = user.event_sender();
                                        let user_info = user.user()?;
                                        if let Some(current_lobby) = user_info.current_game_event_setter().get_multiplayer().await {
                                            self.queue_handler.join_queue(
                                                current_lobby.map,
                                                current_lobby.mode,
                                                current_lobby.visibility,
                                                current_lobby.auto_heal,
                                                user_info.clone(),
                                                events.to_owned(),
                                                if group_size > 1 {
                                                    Some(crate::lobby::PlatoonInfo {
                                                        total: group_size as _,
                                                        platoon_id: group_id.string,
                                                        is_leader,
                                                    })
                                                } else {
                                                    None
                                                }
                                            ).await;
                                        }
                                    } else {
                                        log::warn!("Missing multiplayer event, not joining platoon {} to multiplayer lobby queue", group_id.string);
                                    }
                                },
                                oj_rc_core::data::lobby::LobbyType::Solo => {
                                    log::warn!("Cannot join queue for solo mode (it's singleplayer!)");
                                },
                            }

                        }
                    }
                }
            }
        }
        Ok(params.into())
    }
}

pub(super) fn join_queue_provider<C: Send + 'static>(queue_handler: &std::sync::Arc<crate::QueueHandler>) -> SimpleOpImpl<C, crate::UserTy, QueueJoinProvider> {
    SimpleOpImpl::new(QueueJoinProvider {
        queue_handler: queue_handler.to_owned(),
    })
}
