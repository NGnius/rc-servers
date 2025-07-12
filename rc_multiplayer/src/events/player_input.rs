pub struct PlayerInput {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::simple_typed::SimpleRlnl<rlnl::events::ingame::MultiPlayerInputChanged, PlayerInput> {
    crate::handlers::simple_typed::SimpleRlnl::new(PlayerInput::new(init_ctx))
}

impl PlayerInput {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::handlers::simple_typed::RlnlEventCodeHandler for PlayerInput {
    type In = rlnl::events::ingame::MultiPlayerInputChanged;
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::OnPlayerInputChanged;

    async fn handle(&self, data: Self::In, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {
            super::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::BroadcastRlnl {
                user_id: user_info.user_id(),
                event: rlnl::event_code::NetworkEvent::OnServerReceivedInputChange,
                property: literustlib::packet::Property::Unreliable,
                data: Some(Box::new(data)),
            }).await);
        } else {
            log::error!("Failed to rebroadcast OnPlayerInputChanged for user (no auth!)");
        }
    }
}
