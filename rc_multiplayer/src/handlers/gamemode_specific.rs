#![allow(dead_code)]
pub struct GamemodeSpecific<const EVENT: i16, const PROPERTY: u8, InOut: byteserde::des_slice::ByteDeserializeSlice<InOut> + crate::Broadcastable> {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
    event: rlnl::event_code::NetworkEvent,
    property: literustlib::packet::Property,
    _in: std::marker::PhantomData<InOut>,
}

impl <const EVENT: i16, const PROPERTY: u8, InOut: byteserde::des_slice::ByteDeserializeSlice<InOut> + crate::Broadcastable> GamemodeSpecific<EVENT, PROPERTY, InOut> {
    pub fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::simple_typed::SimpleRlnl<InOut, Self> {
        crate::handlers::simple_typed::SimpleRlnl::new(GamemodeSpecific::new(init_ctx))
    }

    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
            event: crate::handler::i16_to_event_or_panic(EVENT),
            property: literustlib::packet::Property::try_from(PROPERTY).expect("Invalid literustlib packet property"),
            _in: std::marker::PhantomData::default(),
        }
    }
}

#[async_trait::async_trait]
impl <const EVENT: i16, const PROPERTY: u8, InOut: byteserde::des_slice::ByteDeserializeSlice<InOut> + crate::Broadcastable> crate::handlers::simple_typed::RlnlEventCodeHandler for GamemodeSpecific<EVENT, PROPERTY, InOut> {
    type In = InOut;
    const CODE: rlnl::event_code::NetworkEvent = crate::handler::i16_to_event_or_panic(EVENT);

    async fn handle(&self, data: Self::In, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {
            crate::events::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::RebroadcastRlnl {
                skip_user_id: user_info.user_id(),
                event: self.event,
                event_in: Self::CODE,
                property: self.property,
                data: Some(Box::new(data)),
            }).await);
        } else {
            log::error!("Failed to send gamemode specifc event {:?} for user (no auth!)", self.event);
        }
    }
}
