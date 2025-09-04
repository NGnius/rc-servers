pub struct Stub<const CODE_IN: i16, In: byteserde::des_slice::ByteDeserializeSlice<In> + Send + Sync + 'static> {
    _in: std::marker::PhantomData<In>,
}

impl <const CODE_IN: i16, In: byteserde::des_slice::ByteDeserializeSlice<In> + Send + Sync + 'static> Stub<CODE_IN, In> {
    #[allow(dead_code)]
    pub fn handler(init_ctx: &crate::InitConfig) -> crate::handlers::simple_typed::SimpleRlnl<In, Self> {
        crate::handlers::simple_typed::SimpleRlnl::new(Stub::new(init_ctx))
    }

    fn new(_init_ctx: &crate::InitConfig) -> Self {
        Self {
            _in: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl <const CODE_IN: i16, In: byteserde::des_slice::ByteDeserializeSlice<In> + Send + Sync + 'static> crate::handlers::simple_typed::RlnlEventCodeHandler for Stub<CODE_IN, In> {
    type In = In;
    const CODE: rlnl::event_code::NetworkEvent = crate::handler::i16_to_event_or_panic(CODE_IN);

    async fn handle(&self, _data: Self::In, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, _user: &crate::UserData, _sender: &std::sync::Arc<literustlib_server::DataSender<crate::PacketData>>) {
        log::debug!("Got {:?} event", Self::CODE);
        //log::debug!("Got {:?} event: {:?}", Self::CODE, data);
    }
}
