#[derive(Clone)]
pub struct PlatoonLeaderChanged {
    pub leader_public_id: String,
    pub leader_display_name: String,
}

impl PlatoonLeaderChanged {
    pub const CODE: u8 = 16;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(4);
        params.insert(17, polariton::operation::Typed::Str(self.leader_public_id.clone().into()));
        params.insert(75, polariton::operation::Typed::Str(self.leader_display_name.clone().into()));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for PlatoonLeaderChanged {
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
