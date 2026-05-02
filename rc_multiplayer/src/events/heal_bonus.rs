pub struct HealBonus {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::SimpleRlnl<rlnl::events::ingame::DestroyedHealedCubesBonus, HealBonus> {
    crate::handlers::SimpleRlnl::new(HealBonus::new(init_ctx))
}

impl HealBonus {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::handlers::RlnlEventCodeHandler for HealBonus {
    type In = rlnl::events::ingame::DestroyedHealedCubesBonus;
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::HealCubesBonusRequest;

    async fn handle(&self, data: Self::In, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {
            super::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::HealCubesBonus {
                user_id: user_info.account_id(),
                info: data,
            }).await);
        }
    }
}
