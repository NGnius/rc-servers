#[derive(Clone)]
pub struct ClanMemberDataUpdated {
    pub member_public_id: String,
    pub member_display_name: String,
    pub avatar_id: Option<i32>,
    pub state: Option<crate::data::clan::ClanMemberState>,
    pub rank: Option<crate::data::clan::ClanMemberRank>,
    pub is_online: Option<bool>,
}

impl ClanMemberDataUpdated {
    pub const CODE: u8 = 26;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(8);
        params.insert(1, polariton::operation::Typed::Str(self.member_public_id.clone().into()));
        params.insert(75, polariton::operation::Typed::Str(self.member_display_name.clone().into()));
        params.insert(13, polariton::operation::Typed::Bool(self.avatar_id.is_none()));
        params.insert(14, polariton::operation::Typed::Int(self.avatar_id.unwrap_or_default()));
        if let Some(state) = self.state {
            params.insert(37, polariton::operation::Typed::Int(state as _));
        }
        if let Some(rank) = self.rank {
            params.insert(38, polariton::operation::Typed::Int(rank as _));
        }
        if let Some(is_online) = self.is_online {
            params.insert(2, polariton::operation::Typed::Bool(is_online));
        }
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for ClanMemberDataUpdated {
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
