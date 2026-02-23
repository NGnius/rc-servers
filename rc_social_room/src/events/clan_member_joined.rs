#[derive(Clone)]
pub struct ClanMemberJoined {
    pub joiner_public_id: String,
    pub joiner_display_name: String,
    pub avatar_id: Option<i32>,
    pub state: crate::data::clan::ClanMemberState,
    pub season_xp: i32,
}

impl ClanMemberJoined {
    pub const CODE: u8 = 22;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(6);
        params.insert(1, polariton::operation::Typed::Str(self.joiner_public_id.clone().into()));
        params.insert(75, polariton::operation::Typed::Str(self.joiner_display_name.clone().into()));
        params.insert(13, polariton::operation::Typed::Bool(self.avatar_id.is_none()));
        params.insert(14, polariton::operation::Typed::Int(self.avatar_id.unwrap_or_default()));
        params.insert(37, polariton::operation::Typed::Int(self.state as _));
        params.insert(48, polariton::operation::Typed::Int(self.season_xp));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for ClanMemberJoined {
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
