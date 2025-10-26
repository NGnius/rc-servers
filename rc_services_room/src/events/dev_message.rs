pub struct DevMessage {
    pub message: String,
    pub duration: i32,
}

impl <C: Send + Sync + 'static> polariton_server::events::IntoEvent<C> for DevMessage {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        let mut params = polariton::operation::ParameterTable::with_capacity(2);
        params.insert(2, polariton::operation::Typed::Bytes(self.message.as_bytes().to_vec().into()));
        params.insert(15, polariton::operation::Typed::Int(self.duration));
        polariton::operation::Event {
            code: 1,
            params,
        }
    }
}
