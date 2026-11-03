#[derive(Clone)]
pub struct PlatoonMemberStatusUpdate {
    pub member_public_id: oj_rc_core::types::PublicId,
    pub member_display_name: oj_rc_core::types::DisplayName,
    pub member_status: crate::data::platoon::MemberStatus,
}

impl PlatoonMemberStatusUpdate {
    pub const CODE: u8 = 18;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        use oj_rc_core::types::RawType;
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(1, polariton::operation::Typed::Str(self.member_public_id.as_raw_ref().into()));
        params.insert(75, polariton::operation::Typed::Str(self.member_display_name.as_raw_ref().into()));
        params.insert(3, polariton::operation::Typed::Int(self.member_status.as_u8() as i32));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for PlatoonMemberStatusUpdate {
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
