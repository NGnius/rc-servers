#[derive(Clone)]
pub struct ClanDataUpdated {
    pub description: Option<String>,
    pub ty: Option<crate::data::clan::ClanType>,
}

impl ClanDataUpdated {
    pub const CODE: u8 = 27;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(2);
        if let Some(description) = &self.description {
            params.insert(32, polariton::operation::Typed::Str(description.into()));
        }
        if let Some(ty) = self.ty {
            params.insert(34, polariton::operation::Typed::Int(ty.to_u8() as _));
        }
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for ClanDataUpdated {
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
