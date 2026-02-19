#[derive(Clone)]
pub struct PlatoonMemberKick;

impl PlatoonMemberKick {
    pub const CODE: u8 = 12;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        polariton::operation::ParameterTable::<C>::with_capacity(0)
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for PlatoonMemberKick {
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
