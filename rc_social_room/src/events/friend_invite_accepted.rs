pub struct FriendInviteAccepted {
    pub friend_public_id: String,
    pub friend_display_name: String,
}

impl FriendInviteAccepted {
    pub const CODE: u8 = 1;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(1, polariton::operation::Typed::Str(self.friend_public_id.clone().into()));
        params.insert(75, polariton::operation::Typed::Str(self.friend_display_name.clone().into()));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for FriendInviteAccepted {
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
