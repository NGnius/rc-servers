pub struct FriendInviteCancelled {
    pub friend_public_id: oj_rc_core::types::PublicId,
    pub friend_display_name: oj_rc_core::types::DisplayName,
}

impl FriendInviteCancelled {
    pub const CODE: u8 = 5;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        use oj_rc_core::types::RawType;
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(1, polariton::operation::Typed::Str(self.friend_public_id.as_raw_ref().into()));
        params.insert(75, polariton::operation::Typed::Str(self.friend_display_name.as_raw_ref().into()));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for FriendInviteCancelled {
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
