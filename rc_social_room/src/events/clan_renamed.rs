pub struct ClanRenamed {
    pub old_name: String,
    pub new_name: String,
    pub admin_public_id: String,
}

impl ClanRenamed {
    pub const CODE: u8 = 28;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(3);
        params.insert(31, polariton::operation::Typed::Str(self.old_name.clone().into()));
        params.insert(44, polariton::operation::Typed::Str(self.new_name.clone().into()));
        params.insert(1, polariton::operation::Typed::Str(self.admin_public_id.clone().into()));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for ClanRenamed {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: Self::CODE,
            params: self.as_event_params(),
        }
    }
}
