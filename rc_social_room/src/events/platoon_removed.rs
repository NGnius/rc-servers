#[derive(Clone)]
pub struct PlatoonDisbanded {
    pub platoon_id: String,
}

impl PlatoonDisbanded {
    pub const CODE: u8 = 13;

    pub fn as_event_params(&self) -> polariton::operation::ParameterTable<crate::data::custom::CustomType> {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(16, polariton::operation::Typed::Str(self.platoon_id.clone().into()));
        params.into()
    }
}

impl polariton_server::events::IntoEvent<crate::data::custom::CustomType> for PlatoonDisbanded {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<crate::data::custom::CustomType> {
        polariton::operation::Event {
            code: Self::CODE,
            params: self.as_event_params(),
        }
    }
}
