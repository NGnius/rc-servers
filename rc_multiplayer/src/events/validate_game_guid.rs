pub struct AuthUserGame {

}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::simple_typed::SimpleRlnl<rlnl::events::loading::GameGuidInfo, AuthUserGame> {
    crate::handlers::simple_typed::SimpleRlnl::new(AuthUserGame::new(init_ctx))
}

impl AuthUserGame {
    fn new(_init_ctx: &crate::InitConfig) -> Self {
        Self {

        }
    }
}

#[async_trait::async_trait]
impl crate::handlers::simple_typed::RlnlEventCodeHandler for AuthUserGame {
    type In = rlnl::events::loading::GameGuidInfo;
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::ValidateGameGuid;

    async fn handle(&self, data: Self::In, peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, sender: &literustlib_server::DataSender<crate::PacketData>) {
        log::debug!("Got {:?} event with data {:?}", Self::CODE, data);
    }
}
