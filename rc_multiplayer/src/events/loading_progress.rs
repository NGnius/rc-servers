pub struct GameLoadingProgress {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::SimpleRlnl<rlnl::events::loading::LoadingProgress, GameLoadingProgress> {
    crate::handlers::SimpleRlnl::new(GameLoadingProgress::new(init_ctx))
}

impl GameLoadingProgress {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::handlers::RlnlEventCodeHandler for GameLoadingProgress {
    type In = rlnl::events::loading::LoadingProgress;
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::BroadcastLoadingProgress;

    async fn handle(&self, data: Self::In, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {
            super::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::LoadingProgress {
                user_id: user_info.account_id(),
                user_name: data.user_name.0,
                progress: data.progress,
            }).await);
        } else {
            log::error!("Failed to broadcast loading progress for user {} (no auth!)", data.user_name.0);
        }
    }
}
