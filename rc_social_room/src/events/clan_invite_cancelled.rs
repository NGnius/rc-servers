pub struct ClanInviteCancelled {
    pub clan_name: String,
}

impl ClanInviteCancelled {
    pub const CODE: u8 = 25;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(31, polariton::operation::Typed::Str(self.clan_name.clone().into()));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for ClanInviteCancelled {
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
