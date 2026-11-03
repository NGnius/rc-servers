pub struct LoadComplete {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::Dataless<LoadComplete> {
    crate::handlers::Dataless::new(LoadComplete::new(init_ctx))
}

impl LoadComplete {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::handlers::DatalessEventCodeHandler for LoadComplete {
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::LoadingComplete;

    async fn handle(&self, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {
            super::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::LoadComplete {
                user_id: user_info.user_id(),
            }).await);
        } else {
            log::error!("Failed to handle sync loading request for unknown user");
        }
    }
}
