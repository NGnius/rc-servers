pub struct FriendInviteReceived {
    pub friend_public_id: String,
    pub friend_display_name: String,
    pub clan_name: Option<String>,
    pub is_online: bool, // when would this ever be false?
    pub avatar_id: u32, // direct from database; u32::MAX means it is a custom avatar
}

impl FriendInviteReceived {
    pub const CODE: u8 = 0;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(8);
        params.insert(1, polariton::operation::Typed::Str(self.friend_public_id.clone().into()));
        params.insert(75, polariton::operation::Typed::Str(self.friend_display_name.clone().into()));
        if let Some(clan_name) = &self.clan_name {
            params.insert(31, polariton::operation::Typed::Str(clan_name.into()));
        } else {
            params.insert(31, polariton::operation::Typed::Null);
        }

        params.insert(2, polariton::operation::Typed::Bool(self.is_online));
        params.insert(9, polariton::operation::Typed::HashMap(vec![
            (polariton::operation::Typed::Str("useCustomAvatar".into()), polariton::operation::Typed::Bool(self.avatar_id == u32::MAX)),
            (polariton::operation::Typed::Str("avatarId".into()), polariton::operation::Typed::Int(self.avatar_id.try_into().unwrap_or_default())),
        ].into()));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for FriendInviteReceived {
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
