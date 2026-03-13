use polariton::operation::Typed;

//#[derive(Clone)]
pub struct CustomGameKick {
    pub session: String,
    pub was_invited: bool,
}

impl <C: Send + Sync + 'static> polariton_server::events::IntoEvent<C> for CustomGameKick {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        let mut params = polariton::operation::ParameterTable::with_capacity(2);
        params.insert(184, polariton::operation::Typed::HashMap(vec![
            (Typed::Str("Session".into()), Typed::Str(self.session.into())),
            (Typed::Str("WasInvited".into()), Typed::Bool(self.was_invited)),
        ].into()));
        polariton::operation::Event {
            code: 11,
            params,
        }
    }
}
