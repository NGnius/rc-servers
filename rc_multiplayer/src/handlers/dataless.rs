pub struct Dataless<H: DatalessEventCodeHandler> {
    handler: H,
}

impl <H: DatalessEventCodeHandler> Dataless<H> {
    pub fn new(inner: H) -> Self {
        Self {
            handler: inner,
        }
    }
}

#[async_trait::async_trait]
pub trait DatalessEventCodeHandler: Sync + Send {
    const CODE: rlnl::event_code::NetworkEvent;

    async fn handle(&self, peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>);
}

#[async_trait::async_trait]
impl <H: DatalessEventCodeHandler> crate::EventCodeHandler for Dataless<H> {
    async fn handle(&self, _data: &bytes::Bytes, peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        self.handler.handle(peer, user, sender).await;
    }
}

impl <H: DatalessEventCodeHandler> crate::EventCode for Dataless<H> {
    const CODE: i16 = H::CODE as i16;
}
