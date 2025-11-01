pub struct MaintenanceMode {
    pub message: String,
}

impl <C: Send + Sync + 'static> polariton_server::events::IntoEvent<C> for MaintenanceMode {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        let mut params = polariton::operation::ParameterTable::with_capacity(1);
        params.insert(19, polariton::operation::Typed::Str(self.message.clone().into()));
        polariton::operation::Event {
            code: 3,
            params,
        }
    }
}
