use polariton::operation::Typed;

#[derive(Clone)]
pub struct CustomGameInviteDecline {
    pub public_id: String,
}

impl <C: Send + Sync + 'static> polariton_server::events::IntoEvent<C> for CustomGameInviteDecline {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        let mut params = polariton::operation::ParameterTable::with_capacity(1);
        params.insert(190, polariton::operation::Typed::HashMap(vec![
            (Typed::Str("UserName".into()), Typed::Str(self.public_id.into()))
        ].into()));
        polariton::operation::Event {
            code: 12,
            params,
        }
    }
}
