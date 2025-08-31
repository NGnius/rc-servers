pub struct Broadcaster<const EXCLUDE_SENDER: bool, const CODE_IN: i16, const CODE_OUT: i16, const PROPERTY: u8, InOut: byteserde::des_slice::ByteDeserializeSlice<InOut> + crate::Broadcastable> {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
    code_out: rlnl::event_code::NetworkEvent,
    property: literustlib::packet::Property,
    _in: std::marker::PhantomData<InOut>,
}

impl <const EXCLUDE_SENDER: bool, const CODE_IN: i16, const CODE_OUT: i16, const PROPERTY: u8, InOut: byteserde::des_slice::ByteDeserializeSlice<InOut> + crate::Broadcastable> Broadcaster<EXCLUDE_SENDER, CODE_IN, CODE_OUT, PROPERTY, InOut> {
    pub fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::simple_typed::SimpleRlnl<InOut, Self> {
        crate::handlers::simple_typed::SimpleRlnl::new(Broadcaster::new(init_ctx))
    }

    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
            code_out: crate::handler::i16_to_event_or_panic(CODE_OUT),
            property: literustlib::packet::Property::try_from(PROPERTY).expect("Invalid literustlib packet property"),
            _in: std::marker::PhantomData::default(),
        }
    }
}

#[async_trait::async_trait]
impl <const EXCLUDE_SENDER: bool, const CODE_IN: i16, const CODE_OUT: i16, const PROPERTY: u8, InOut: byteserde::des_slice::ByteDeserializeSlice<InOut> + crate::Broadcastable> crate::handlers::simple_typed::RlnlEventCodeHandler for Broadcaster<EXCLUDE_SENDER, CODE_IN, CODE_OUT, PROPERTY, InOut> {
    type In = InOut;
    const CODE: rlnl::event_code::NetworkEvent = crate::handler::i16_to_event_or_panic(CODE_IN);

    async fn handle(&self, data: Self::In, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {

            if EXCLUDE_SENDER {
                crate::events::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::RebroadcastRlnl {
                    skip_user_id: user_info.user_id(),
                    event: self.code_out,
                    event_in: Self::CODE,
                    property: self.property,
                    data: Some(Box::new(data)),
                }).await);
            } else {
                crate::events::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::BroadcastRlnl {
                    user_id: user_info.user_id(),
                    event: self.code_out,
                    event_in: Self::CODE,
                    property: self.property,
                    data: Some(Box::new(data)),
                }).await);
            }

        } else {
            log::error!("Failed to rebroadcast {:?}->{:?} for user (no auth!)", Self::CODE, self.code_out);
        }
    }
}
