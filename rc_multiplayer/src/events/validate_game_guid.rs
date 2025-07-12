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
                log::error!("Failed {:?} event: {}", Self::CODE, e);
            }
        } else {
            log::error!("Failed to validate game guid for user {} (other packets will probably be ignored)", username);
        }

    }
}
