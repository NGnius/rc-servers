pub struct RoomJoined {
    pub channel_name: String,
    pub player_name: String,
    pub player_state: oj_rc_core::data::channel::ChatPlayerState,
    pub use_custom_avatar: bool,
    pub custom_avatar: Vec<u8>,
    pub avatar_id: i32,
}

impl RoomJoined {
    pub const CODE: u8 = 4;
    pub const CHANNEL: u8 = 0;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(8);
        params.insert(3, polariton::operation::Typed::Str(self.channel_name.clone().into()));
        params.insert(22, polariton::operation::Typed::Str(self.player_name.clone().into()));
        params.insert(23, polariton::operation::Typed::Int(self.player_state as _));
        params.insert(24, polariton::operation::Typed::Bool(self.use_custom_avatar));
        if self.use_custom_avatar {
            params.insert(26, polariton::operation::Typed::Bytes(self.custom_avatar.clone().into()));
        } else {
            params.insert(25, polariton::operation::Typed::Int(self.avatar_id));
        }
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for RoomJoined {
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

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for &RoomJoined {
    const CHANNEL: u8 = RoomJoined::CHANNEL;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: RoomJoined::CODE,
            params: self.as_event_params().into(),
        }
    }
}
