pub struct AssistBonus {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::SimpleRlnl<rlnl::events::ingame::AssistBonus, AssistBonus> {
    crate::handlers::SimpleRlnl::new(AssistBonus::new(init_ctx))
}

impl AssistBonus {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::handlers::RlnlEventCodeHandler for AssistBonus {
    type In = rlnl::events::ingame::AssistBonus;
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::AssistBonusRequest;

    async fn handle(&self, data: Self::In, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {
            super::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::AssistBonus {
                user_id: user_info.user_id(),
                shootee: data.requester_player_id as u8,
                shooters: data.player_ids.into_iter().map(|x| x.player).collect(),
            }).await);
        }
    }
}
