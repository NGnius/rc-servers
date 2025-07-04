pub type UserData = crate::user::User;
pub type PacketData = crate::handler::EventData;

#[async_trait::async_trait]
pub trait EventCodeHandler: Send + Sync {
    async fn handle(&self, data: &bytes::Bytes, peer: &std::sync::Arc<literustlib_server::Connection<PacketData>>, user: &UserData, sender: &literustlib_server::DataSender<PacketData>);
}

pub trait EventCode: EventCodeHandler {
    const CODE: i16;
}
