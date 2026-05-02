pub struct HealAssistBonus {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::SimpleRlnl<rlnl::events::ingame::HealAssistBonus, HealAssistBonus> {
    crate::handlers::SimpleRlnl::new(HealAssistBonus::new(init_ctx))
}

impl HealAssistBonus {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::handlers::RlnlEventCodeHandler for HealAssistBonus {
    type In = rlnl::events::ingame::HealAssistBonus;
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::HeallingAssistBonusRequest;

    async fn handle(&self, data: Self::In, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {
            //log::info!("Heal assist for player {}", data.healing_player_id);
            super::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::HealAssistBonus {
                user_id: user_info.account_id(),
                healer: data.healing_player_id,
                healee: data.healed_player_id,
            }).await);
        }
    }
}
