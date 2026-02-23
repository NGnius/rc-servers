#[derive(Clone)]
pub struct ClanMemberLeft {
    pub leaver_public_id: String,
    pub new_leader_public_id: Option<String>,
}

impl ClanMemberLeft {
    pub const CODE: u8 = 23;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(1, polariton::operation::Typed::Str(self.leaver_public_id.clone().into()));
        if let Some(new_leader) = &self.new_leader_public_id {
            params.insert(45, polariton::operation::Typed::Str(new_leader.into()));
        }
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for ClanMemberLeft {
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
