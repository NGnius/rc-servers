pub struct TestMessage;

pub(super) fn handler() -> crate::handlers::SimpleRlnl<rlnl::events::loading::MachineDefinition, TestMessage> {
    crate::handlers::SimpleRlnl::new(TestMessage::new())
}

impl TestMessage {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl crate::handlers::RlnlEventCodeHandler for TestMessage {
    type In = rlnl::events::loading::MachineDefinition;
    const CODE: rlnl::event_code::NetworkEvent = rlnl::event_code::NetworkEvent::TestConnection;

    async fn handle(&self, data: Self::In, peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        user.flag_is_testing_connection();
        let send_result = crate::handlers::RlnlSender::new(sender)
            .send_data(
                &data,
                rlnl::event_code::NetworkEvent::TestConnection,
                literustlib::packet::Property::ReliableOrdered,
                peer,
            ).await;
        if let Err(e) = send_result {
            log::warn!("Failed to echo test message to {}: {}", peer.id(), e);
        }
    }
}
