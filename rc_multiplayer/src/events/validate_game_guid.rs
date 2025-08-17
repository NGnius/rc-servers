pub struct AuthUserGame {
    matches: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::SimpleRlnl<rlnl::events::loading::GameGuidInfo, AuthUserGame> {
    crate::handlers::SimpleRlnl::new(AuthUserGame::new(init_ctx))
}

impl AuthUserGame {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            matches: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::handlers::RlnlEventCodeHandler for AuthUserGame {
    type In = rlnl::events::loading::GameGuidInfo;
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::ValidateGameGuid;

    async fn handle(&self, data: Self::In, peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        let username = data.player_name.0.clone();
        let game_guid = data.game_guid.0.clone();
        if user.authenticate(data).await {
            let user_info = user.user().await.unwrap();
            match user_info.current_game().await {
                Ok(Some(current_game)) => {
                    if current_game.guid == game_guid {
                        let (tx, rx) = tokio::sync::oneshot::channel();
                        super::log_channel_send_failure(self.matches.send(crate::matches::GameMessage::NewConnection {
                            user: user_info.clone(),
                            game_guid,
                            connection: peer.to_owned(),
                            response: tx,
                            sender: sender.to_owned(),
                        }).await);
                        log::debug!("Sent NewConnection message to matches handler");
                        if let Ok(Some(e)) = rx.await {
                            log::error!("Failed {:?} event: {} [disconnecting...]", Self::CODE, e);
                            super::log_lnl_send_failure(crate::handlers::RlnlSender::new(&sender)
                                .send_data(&rlnl::types::StringCode {
                                    ty: rlnl::types::GameServerErrorCodes::StrErrCustomString,
                                    custom: Some(rlnl::types::BinaryWriterString(e.message)),
                                },
                                rlnl::event_code::NetworkEvent::OnFailedToConnectToServer,
                                literustlib::packet::Property::ReliableOrdered,
                                &peer).await);
                            peer.disconnect();
                        } else {
                            peer.certify();
                        }
                    } else {
                        log::error!("Registered game GUID does not match sent GUID (got: {}, expected: {}) [disconnecting...]", game_guid, current_game.guid);
                        super::log_lnl_send_failure(crate::handlers::RlnlSender::new(&sender)
                            .send_data(&rlnl::types::StringCode {
                                ty: rlnl::types::GameServerErrorCodes::StrErrIncorrectGameGuid,
                                custom: Some(rlnl::types::BinaryWriterString(format!("Send game guid does not equal expected guid; {} != {}", game_guid, current_game.guid))),
                            },
                            rlnl::event_code::NetworkEvent::OnFailedToConnectToServer,
                            literustlib::packet::Property::ReliableOrdered,
                            &peer).await);
                        peer.disconnect();
                    }
                },
                Ok(None) => {
                    log::warn!("Cannot validate game guid for user {} with no ongoing game [disconnecting...]", user_info.user_id());
                    super::log_lnl_send_failure(crate::handlers::RlnlSender::new(&sender)
                        .send_data(&rlnl::types::StringCode {
                            ty: rlnl::types::GameServerErrorCodes::StrErrIncorrectGameGuid,
                            custom: None,
                        },
                        rlnl::event_code::NetworkEvent::OnFailedToConnectToServer,
                        literustlib::packet::Property::ReliableOrdered,
                        &peer).await);
                    peer.disconnect();
                },
                Err(e) => {
                    log::error!("Failed to get current game for user {}: {} [disconnecting...]", user_info.user_id(), e.message);
                    super::log_lnl_send_failure(crate::handlers::RlnlSender::new(&sender)
                        .send_data(&rlnl::types::StringCode {
                            ty: core_to_rlnl_mp_error_code(e.code),
                            custom: Some(rlnl::types::BinaryWriterString(e.message)),
                        },
                        rlnl::event_code::NetworkEvent::OnFailedToConnectToServer,
                        literustlib::packet::Property::ReliableOrdered,
                        &peer).await);
                    peer.disconnect();
                },
            }

        } else {
            log::error!("Failed to validate game guid for user {} [disconnecting...]", username);
            peer.disconnect();
        }

    }
}

fn core_to_rlnl_mp_error_code(core_: oj_rc_core::persist::user::MultiplayerErrorCode) -> rlnl::types::GameServerErrorCodes {
    match core_ {
        oj_rc_core::persist::user::MultiplayerErrorCode::HaxSpeed => rlnl::types::GameServerErrorCodes::StrErrHaxSpeed,
        oj_rc_core::persist::user::MultiplayerErrorCode::HaxException => rlnl::types::GameServerErrorCodes::StrErrHaxException,
        oj_rc_core::persist::user::MultiplayerErrorCode::HaxTeleport => rlnl::types::GameServerErrorCodes::StrErrHaxTeleport,
        oj_rc_core::persist::user::MultiplayerErrorCode::HaxEacViolation => rlnl::types::GameServerErrorCodes::StrErrHaxEacViolation,
        oj_rc_core::persist::user::MultiplayerErrorCode::HaxAfk => rlnl::types::GameServerErrorCodes::StrErrHaxAfk,
        oj_rc_core::persist::user::MultiplayerErrorCode::HaxFirerange => rlnl::types::GameServerErrorCodes::StrErrHaxFirerange,
        oj_rc_core::persist::user::MultiplayerErrorCode::HaxFiredamage => rlnl::types::GameServerErrorCodes::StrErrHaxFiredamage,
        oj_rc_core::persist::user::MultiplayerErrorCode::HaxFirerate => rlnl::types::GameServerErrorCodes::StrErrHaxFirerate,
        oj_rc_core::persist::user::MultiplayerErrorCode::HaxFireposition => rlnl::types::GameServerErrorCodes::StrErrHaxFireposition,
        oj_rc_core::persist::user::MultiplayerErrorCode::IncorrectGameGuid => rlnl::types::GameServerErrorCodes::StrErrIncorrectGameGuid,
        oj_rc_core::persist::user::MultiplayerErrorCode::CustomString => rlnl::types::GameServerErrorCodes::StrErrCustomString,
        oj_rc_core::persist::user::MultiplayerErrorCode::TimedOut => rlnl::types::GameServerErrorCodes::StrErrTimedOut,
        oj_rc_core::persist::user::MultiplayerErrorCode::GameEnded => rlnl::types::GameServerErrorCodes::StrErrGameEnded,
    }
}
