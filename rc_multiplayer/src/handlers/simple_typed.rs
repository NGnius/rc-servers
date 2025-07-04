pub struct SimpleRlnl<In: byteserde::des_slice::ByteDeserializeSlice<In>, H: RlnlEventCodeHandler<In=In>> {
    //_in: std::marker::PhantomData<In>,
    //_out: std::marker::PhantomData<Out>,
    handler: H,
}

impl <In: byteserde::des_slice::ByteDeserializeSlice<In>, H: RlnlEventCodeHandler<In=In>> SimpleRlnl<In, H> {
    pub fn new(inner: H) -> Self {
        Self {
            handler: inner,
        }
    }
}

#[async_trait::async_trait]
pub trait RlnlEventCodeHandler: Sync + Send {
    type In: byteserde::des_slice::ByteDeserializeSlice<Self::In>;
    //type Out: byteserde::ser_heap::ByteSerializeHeap;
    const CODE: rlnl::event_code::NetworkEvent;

    async fn handle(&self, data: Self::In, peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, sender: &literustlib_server::DataSender<crate::PacketData>);
}

#[async_trait::async_trait]
impl <In: byteserde::des_slice::ByteDeserializeSlice<In>, H: RlnlEventCodeHandler<In=In>> crate::EventCodeHandler for SimpleRlnl<In, H> {
    async fn handle(&self, data: &bytes::Bytes, peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, sender: &literustlib_server::DataSender<crate::PacketData>) {
        let mut des = byteserde::des_slice::ByteDeserializerSlice::new(&data);
        let rlnl_data = In::byte_deserialize(&mut des).expect("Bad serialization");
        self.handler.handle(rlnl_data, peer, user, sender).await;
    }
}

impl <In: byteserde::des_slice::ByteDeserializeSlice<In>, H: RlnlEventCodeHandler<In=In>> crate::EventCode for SimpleRlnl<In, H> {
    const CODE: i16 = H::CODE as i16;
}


