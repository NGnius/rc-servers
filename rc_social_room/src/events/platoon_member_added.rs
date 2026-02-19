#[derive(Clone)]
pub struct PlatoonNewMember {
    pub new_member: crate::data::platoon::PlatoonMemberInfo,
}

impl PlatoonNewMember {
    pub const CODE: u8 = 17;

    pub fn as_event_params(&self) -> polariton::operation::ParameterTable<crate::data::custom::CustomType> {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(15, polariton::operation::Typed::Custom(crate::data::custom::CustomType::PlatoonMember(self.new_member.clone())));
        params.into()
    }
}

impl polariton_server::events::IntoEvent<crate::data::custom::CustomType> for PlatoonNewMember {
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
