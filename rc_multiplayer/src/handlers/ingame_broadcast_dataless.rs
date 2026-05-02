#![allow(dead_code)]
pub struct DatalessBroadcaster<const EXCLUDE_SENDER: bool, const CODE_IN: i16, const CODE_OUT: i16, const PROPERTY: u8> {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
    code_out: rlnl::event_code::NetworkEvent,
    property: literustlib::packet::Property,
}

impl <const EXCLUDE_SENDER: bool, const CODE_IN: i16, const CODE_OUT: i16, const PROPERTY: u8> DatalessBroadcaster<EXCLUDE_SENDER, CODE_IN, CODE_OUT, PROPERTY> {
    pub fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::dataless::Dataless<Self> {
        crate::handlers::dataless::Dataless::new(Self::new(init_ctx))
    }

    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
            code_out: crate::handler::i16_to_event_or_panic(CODE_OUT),
            property: literustlib::packet::Property::try_from(PROPERTY).expect("Invalid literustlib packet property"),
        }
    }
}

#[async_trait::async_trait]
impl <const EXCLUDE_SENDER: bool, const CODE_IN: i16, const CODE_OUT: i16, const PROPERTY: u8> crate::handlers::dataless::DatalessEventCodeHandler for DatalessBroadcaster<EXCLUDE_SENDER, CODE_IN, CODE_OUT, PROPERTY> {
    const CODE: rlnl::event_code::NetworkEvent = crate::handler::i16_to_event_or_panic(CODE_IN);

    async fn handle(&self, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        if let Some(user_info) = user.user().await {
            if EXCLUDE_SENDER {
                crate::events::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::RebroadcastRlnl {
                    skip_user_id: user_info.account_id(),
                    event: self.code_out,
                    event_in: Self::CODE,
                    property: self.property,
                    data: None,
                }).await);
            } else {
                crate::events::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::BroadcastRlnl {
                    user_id: user_info.account_id(),
                    event: self.code_out,
                    event_in: Self::CODE,
                    property: self.property,
                    data: None,
                }).await);
            }

        } else {
            log::error!("Failed to rebroadcast {:?}-?{:?} for user (no auth!)", Self::CODE, self.code_out);
        }
    }
}
