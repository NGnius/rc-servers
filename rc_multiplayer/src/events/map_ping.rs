pub struct PingMap {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::SimpleRlnl<rlnl::events::ingame::MapPing, PingMap> {
    crate::handlers::SimpleRlnl::new(PingMap::new(init_ctx))
}

impl PingMap {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::handlers::RlnlEventCodeHandler for PingMap {
    type In = rlnl::events::ingame::MapPing;
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::MapPingEvent;

    async fn handle(&self, data: Self::In, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {
            super::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::MapPing {
                user_id: user_info.user_id(),
                ping: data,
            }).await);
        }
    }
}
