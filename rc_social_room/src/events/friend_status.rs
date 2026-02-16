#[derive(Clone)]
pub struct FriendStatus {
    pub friend_public_id: String,
    pub friend_display_name: String,
    pub is_online: bool,
    pub invite_status: crate::data::friend::InviteStatus,
}

impl FriendStatus {
    pub const CODE: u8 = 4;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(4);
        params.insert(1, polariton::operation::Typed::Str(self.friend_public_id.clone().into()));
        params.insert(75, polariton::operation::Typed::Str(self.friend_display_name.clone().into()));
        params.insert(2, polariton::operation::Typed::Bool(self.is_online));
        params.insert(3, polariton::operation::Typed::Byte(self.invite_status.as_u8()));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for FriendStatus {
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
