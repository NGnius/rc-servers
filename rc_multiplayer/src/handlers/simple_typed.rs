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

    async fn handle(&self, data: Self::In, peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>);
}

#[async_trait::async_trait]
impl <In: byteserde::des_slice::ByteDeserializeSlice<In> + Send, H: RlnlEventCodeHandler<In=In>> crate::EventCodeHandler for SimpleRlnl<In, H> {
    async fn handle(&self, data: &bytes::Bytes, peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData, sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        let mut des = byteserde::des_slice::ByteDeserializerSlice::new(&data);
        match In::byte_deserialize(&mut des) {
            Ok(rlnl_data) => {
                self.handler.handle(rlnl_data, peer, user, sender).await;
            },
            Err(e) => {
                log::error!("Bad deserialization for {:?}, bytes {:?}: {}", H::CODE, &data[..], e);
            }
        }
    }
}

impl <In: byteserde::des_slice::ByteDeserializeSlice<In> + Send, H: RlnlEventCodeHandler<In=In>> crate::EventCode for SimpleRlnl<In, H> {
    const CODE: i16 = H::CODE as i16;
}

pub struct RlnlSender<'a> {
    sender: &'a literustlib_server::DataSender<crate::PacketData>,
}

impl <'a> RlnlSender<'a> {
    #[inline]
    pub fn new(inner: &'a literustlib_server::DataSender<crate::PacketData>) -> Self {
        Self {
            sender: inner,
        }
    }

    pub async fn send_data<D: byteserde::ser_heap::ByteSerializeHeap + ?Sized>(&self, data: &D, event: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, conn: &literustlib_server::Connection<crate::PacketData>) -> std::io::Result<usize> {
        let mut ser = byteserde::ser_heap::ByteSerializerHeap::default();
        data.byte_serialize_heap(&mut ser).map_err(|e| std::io::Error::new(std::io::ErrorKind::Unsupported, e.message))?;
        let event_data = crate::handler::EventData::with_data(
            crate::data::MessageType::ServerMsg,
            event,
            bytes::Bytes::copy_from_slice(ser.as_slice()),
        );
        self.sender.send_data(event_data, property, conn).await
    }

    pub async fn send_empty(&self, event: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, conn: &literustlib_server::Connection<crate::PacketData>) -> std::io::Result<usize> {
        let event_data = crate::handler::EventData::without_data(crate::data::MessageType::ServerMsg, event);
        self.sender.send_data(event_data, property, conn).await
    }
}


