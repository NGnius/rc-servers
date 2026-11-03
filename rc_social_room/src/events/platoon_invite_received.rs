#[derive(Clone)]
pub struct PlatoonInviteReceived {
    pub inviter_public_id: oj_rc_core::types::PublicId,
    pub inviter_display_name: oj_rc_core::types::DisplayName,
    pub avatar_id: Option<i32>, // None when using custom avatar
}

impl PlatoonInviteReceived {
    pub const CODE: u8 = 10;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        use oj_rc_core::types::RawType;
        let mut params = std::collections::HashMap::with_capacity(4);
        params.insert(19, polariton::operation::Typed::Str(self.inviter_public_id.as_raw_ref().into()));
        params.insert(75, polariton::operation::Typed::Str(self.inviter_display_name.as_raw_ref().into()));
        params.insert(13, polariton::operation::Typed::Bool(self.avatar_id.is_none()));
        params.insert(14, polariton::operation::Typed::Int(self.avatar_id.unwrap_or_default()));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for PlatoonInviteReceived {
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
