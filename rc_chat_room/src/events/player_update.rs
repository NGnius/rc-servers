pub struct PlayerUpdated {
    pub channel_name: String,
    pub player_name: String,
    pub player_state: oj_rc_core::data::channel::ChatPlayerState,
}

impl PlayerUpdated {
    pub const CODE: u8 = 6;
    pub const CHANNEL: u8 = 0;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(3);
        params.insert(3, polariton::operation::Typed::Str(self.channel_name.clone().into()));
        params.insert(22, polariton::operation::Typed::Str(self.player_name.clone().into()));
        params.insert(23, polariton::operation::Typed::Int(self.player_state as _));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for PlayerUpdated {
    const CHANNEL: u8 = Self::CHANNEL;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: Self::CODE,
            params: self.as_event_params().into(),
        }
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for &PlayerUpdated {
    const CHANNEL: u8 = PlayerUpdated::CHANNEL;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: PlayerUpdated::CODE,
            params: self.as_event_params().into(),
        }
    }
}
