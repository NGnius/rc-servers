pub struct WeaponSelect {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::simple_typed::SimpleRlnl<rlnl::events::ingame::SelectWeapon, WeaponSelect> {
    crate::handlers::simple_typed::SimpleRlnl::new(WeaponSelect::new(init_ctx))
}

impl WeaponSelect {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::handlers::simple_typed::RlnlEventCodeHandler for WeaponSelect {
    type In = rlnl::events::ingame::SelectWeapon;
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::WeaponSelect;

    async fn handle(&self, data: Self::In, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {
            if let Some(category) = oj_rc_core::data::weapon_list::ItemCategory::from_smaller(data.item_category as _) {
                if let Some(tier) = oj_rc_core::data::cube_list::ItemTier::from_u32(data.item_category as _) {
                    super::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::WeaponSelect {
                        user_id: user_info.user_id(),
                        machine_id: data.machine_id,
                        category,
                        size: tier,
                    }).await);
                } else { log::warn!("Bad WeaponSelect tier") }
            } else { log::warn!("Bad WeaponSelect category") }
        }
    }
}
